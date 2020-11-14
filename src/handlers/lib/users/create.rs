use crate::{
    config::Config,
    crypto::{jwt::JWT, secure_token},
    diesel::Connection,
    handlers::Error,
    mailer::send_email,
    models::{
        user::{get_by_email, get_by_phone_number, NewUser, User},
        Error as ModelError,
    },
    sms::send_sms,
};
use chrono::Utc;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
    result::{
        DatabaseErrorKind,
        Error::{DatabaseError, NotFound},
    },
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, GraphQLInputObject, Clone, Debug)]
pub struct CreateForm {
    pub email: Option<String>,
    #[graphql(name = "phone_number")]
    pub phone_number: Option<String>,
    pub password: String,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub confirm: Option<bool>,
}

pub fn create(config: &Config, connection: &PooledConnection<ConnectionManager<PgConnection>>, token: &JWT, create_form: CreateForm) -> Result<User, Error> {
    let internal_error = Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string());

    if !token.is_admin(&connection) {
        return Err(Error::new(403, json!({"code": "only_admin_can_create"}), "Only Admin Can Create Users".to_string()));
    }

    if !config.password_rule.is_match(create_form.password.as_ref()) {
        return Err(Error::new(400, json!({"code": "invalid_password_format"}), "Invalid Password Format".to_string()));
    }

    let mut user = NewUser {
        email: create_form.email.clone(),
        phone_number: create_form.phone_number.clone(),
        name: create_form.name.clone(),
        password: Some(create_form.clone().password),
        ..Default::default()
    };

    if user.email.is_some() {
        // if the user is signing up with email and
        // if user exists and is confirmed return conflict error
        // if not delete the unconfirmed user and proceed with the normal flow
        // if the error is user not found proceed with the normal flow
        match get_by_email(&user.email.unwrap(), &connection) {
            Ok(mut user) => {
                if user.email_confirmed {
                    return Err(Error::new(
                        409,
                        json!({"code": "email_registered"}),
                        "A user with this email address has already been registered".to_string(),
                    ));
                }

                // if the user has a phone number confirmed
                // even though the email is not confirmed
                // clear the accounts email otherwise
                // delete the account since neither the phone number or email have been confirmed
                let result = if user.phone_confirmed {
                    user.email = None;

                    user.save(&connection)
                } else {
                    user.delete(&connection)
                };

                if result.is_err() {
                    let err = result.err().unwrap();

                    error!("{:?}", err);

                    return Err(Error::from(err));
                }
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

    if user.phone_number.is_some() {
        // if the user is signing up with phone number and
        // if user exists and is confirmed return conflict error
        // if not delete the unconfirmed user and proceed with the normal flow
        // if the error is user not found proceed with the normal flow
        match get_by_phone_number(user.phone_number.clone().unwrap(), &connection) {
            Ok(mut user) => {
                if user.phone_confirmed {
                    return Err(Error::new(
                        409,
                        json!({"code": "phone_registered"}),
                        "A user with this phone number has already been registered".to_string(),
                    ));
                }

                let result = if user.email_confirmed {
                    user.phone_number = None;

                    user.save(&connection)
                } else {
                    user.delete(&connection)
                };

                if result.is_err() {
                    let err = result.err().unwrap();

                    error!("{:?}", err);

                    return Err(Error::from(err));
                }
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
        user.hash_password();

        let user = user.save(&connection);

        if user.is_err() {
            let err = user.err().unwrap();

            if let ModelError::DatabaseError(DatabaseError(DatabaseErrorKind::UniqueViolation, _info)) = err {
                let err = Error::new(
                    409,
                    json!({"code": "email_already_registered"}),
                    "A user with this email address has already been registered".to_string(),
                );
                return Err(err);
            }

            error!("{:?}", err);

            return Err(internal_error);
        }

        let mut user = user.unwrap();

        if user.email.is_some() && !config.auto_confirm && !(create_form.confirm.is_some() && create_form.confirm.unwrap()) {
            user.email_confirmation_token = Some(secure_token(100));

            user.email_confirmation_token_sent_at = Some(Utc::now().naive_utc());

            let u = user.save(connection);

            if u.is_err() {
                let err = u.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));
            }

            user = u.unwrap();

            let template = &config.get_confirmation_email_template();

            let to = &user.email.unwrap();

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
            let user = user.confirm_email(connection);

            if user.is_err() {
                let err = user.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));
            }
        }

        if user.phone_number.is_some() && !config.auto_confirm && !(create_form.confirm.is_some() && create_form.confirm.unwrap()) {
            user.phone_confirmation_token = Some(secure_token(6));

            user.phone_confirmation_token_sent_at = Some(Utc::now().naive_utc());

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
                "phone_number": user.phone_number,
                "site_url": config.site_url
            });

            let sms = send_sms(template, data, user.phone_number.clone().unwrap(), &config);

            if sms.is_err() {
                let err = sms.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));
            }
        } else {
            let user = user.confirm_phone(connection);

            if user.is_err() {
                let err = user.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));
            }
        }

        return Ok(user);
    });

    if transaction.is_err() {
        return Err(transaction.err().unwrap());
    }

    return Ok(transaction.unwrap());
}
