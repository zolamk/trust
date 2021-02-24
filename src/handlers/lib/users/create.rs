use crate::{
    config::Config,
    crypto::{jwt::JWT, secure_token},
    diesel::Connection,
    handlers::Error,
    mailer::send_email,
    models::{
        user::{get_by_email, get_by_phone, NewUser, User},
        Error as ModelError,
    },
    sms::send_sms,
};
use chrono::Utc;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
    result::Error::NotFound,
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, GraphQLInputObject, Clone, Debug)]
#[graphql(name = "create_user_form")]
pub struct CreateForm {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub confirm: Option<bool>,
}

pub fn create(config: &Config, connection: &PooledConnection<ConnectionManager<PgConnection>>, token: &JWT, create_form: CreateForm) -> Result<User, Error> {
    let internal_error = Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string());

    if !token.is_admin(&connection) {
        return Err(Error::new(403, json!({"code": "only_admin_can_create"}), "Only Admin Can Create Users".to_string()));
    }

    if create_form.password.is_some() && !config.password_rule.is_match(create_form.password.clone().unwrap().as_ref()) {
        return Err(Error::new(400, json!({"code": "invalid_password_format"}), "Invalid Password Format".to_string()));
    }

    let mut user = NewUser {
        email: create_form.email.clone(),
        phone: create_form.phone.clone(),
        name: create_form.name.clone(),
        password: create_form.password.clone(),
        ..Default::default()
    };

    if user.email.is_some() {
        // if the user is signing up with email and
        // if user exists and is confirmed return conflict error
        // if not delete the unconfirmed user and proceed with the normal flow
        // if the error is user not found proceed with the normal flow
        match get_by_email(&user.email.clone().unwrap(), &connection) {
            Ok(user) => {
                return Err(Error::new(
                    409,
                    json!({"code": "email_registered", "id": user.id.clone(), "password_set": user.password.is_some() }),
                    "A user with this email address has already been registered".to_string(),
                ));
            }
            Err(err) => match err {
                ModelError::DatabaseError(NotFound) => {}
                _ => {
                    error!("{:?}", err);

                    return Err(Error::from(err));
                }
            },
        }
    }

    if user.phone.is_some() {
        match get_by_phone(user.phone.clone().unwrap(), &connection) {
            Ok(user) => {
                return Err(Error::new(
                    409,
                    json!({"code": "phone_registered", "id": user.id.clone(), "password_set": user.password.is_some() }),
                    "A user with this phone number has already been registered".to_string(),
                ));
            }
            Err(err) => match err {
                ModelError::DatabaseError(NotFound) => {}
                _ => {
                    error!("{:?}", err);

                    return Err(Error::from(err));
                }
            },
        }
    }

    let transaction = connection.transaction::<User, Error, _>(|| {
        user.hash_password(config.password_hash_cost);

        let user = user.save(&connection);

        if user.is_err() {
            let err = user.err().unwrap();

            error!("{:?}", err);

            return Err(internal_error);
        }

        let mut user = user.unwrap();

        if user.email.is_some() && !config.auto_confirm && !(create_form.confirm.is_some() && create_form.confirm.unwrap()) {
            user.email_confirmation_token = Some(secure_token(100));

            user.email_confirmation_token_sent_at = Some(Utc::now());

            let u = user.save(connection);

            if u.is_err() {
                let err = u.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));
            }

            user = u.unwrap();

            let template = &config.get_confirmation_email_template();

            let to = &user.email.clone().unwrap();

            let subject = &config.get_confirmation_email_subject();

            let data = json!({
                "confirmation_token": user.email_confirmation_token.clone().unwrap(),
                "email": user.email,
                "site_url": config.site_url
            });

            let email = send_email(template, data, to, subject, config);

            if email.is_err() {
                let err = email.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));
            }
        } else {
            let u = user.confirm_email(connection);

            if u.is_err() {
                let err = u.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));
            }

            user = u.unwrap()
        }

        if user.phone.is_some() && !config.auto_confirm && !(create_form.confirm.is_some() && create_form.confirm.unwrap()) {
            user.phone_confirmation_token = Some(secure_token(6));

            user.phone_confirmation_token_sent_at = Some(Utc::now());

            let u = user.save(connection);

            if u.is_err() {
                let err = u.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));
            }

            user = u.unwrap();

            let template = config.clone().get_confirmation_sms_template();

            let data = json!({
                "confirmation_token": user.phone_confirmation_token.clone().unwrap(),
                "phone": user.phone,
                "site_url": config.site_url
            });

            let sms = send_sms(template, data, user.phone.clone().unwrap(), &config);

            if sms.is_err() {
                let err = sms.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));
            }
        } else {
            let u = user.confirm_phone(connection);

            if u.is_err() {
                let err = u.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));
            }

            user = u.unwrap()
        }

        return Ok(user);
    });

    if transaction.is_err() {
        return Err(transaction.err().unwrap());
    }

    return Ok(transaction.unwrap());
}
