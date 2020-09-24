use crate::{
    config::Config,
    crypto::{jwt::JWT, secure_token},
    diesel::Connection,
    handlers::Error,
    mailer::{send_email, EmailTemplates},
    models::{
        user::{get_by_email, get_by_phone_number, NewUser, User},
        Error as ModelError,
    },
    operator_signature::OperatorSignature,
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

#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct CreateForm {
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: String,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub confirm: bool,
}

pub fn create(
    config: &Config,
    connection: &PooledConnection<ConnectionManager<PgConnection>>,
    email_templates: &EmailTemplates,
    _operator_signature: &OperatorSignature,
    token: &JWT,
    create_form: CreateForm,
) -> Result<User, Error> {
    let internal_error = Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string());

    if !token.is_admin(&connection) {
        return Err(Error::new(403, json!({"code": "only_admin_can_create"}), "Only Admin Can Create Users".to_string()));
    }

    if !config.password_rule.is_match(create_form.password.as_ref()) {
        return Err(Error::new(400, json!({"code": "invalid_password_format"}), "Invalid Password Format".to_string()));
    }

    let conflict_error = Err(Error::new(
        409,
        json!({"code": "email_registered"}),
        "A user with this email address has already been registered".to_string(),
    ));

    let mut user = NewUser::default();

    user.email_confirmed = config.auto_confirm || create_form.confirm;

    user.phone_confirmed = user.email_confirmed;

    user.email = create_form.email.clone();

    user.phone_number = create_form.phone_number.clone();

    user.name = create_form.name.clone();

    user.avatar = create_form.avatar.clone();

    user.password = Some(create_form.password);

    user.hash_password();

    if user.email.is_some() {
        // if the user is signing up with email and
        // if user exists and is confirmed return conflict error
        // if not delete the unconfirmed user and proceed with the normal flow
        // if the error is user not found proceed with the normal flow
        match get_by_email(user.email.clone().unwrap(), &connection) {
            Ok(mut user) => {
                if user.email_confirmed {
                    return conflict_error;
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

        if !user.email_confirmed {
            user.email_confirmation_token = Some(secure_token(100));
            user.email_confirmation_token_sent_at = Some(Utc::now().naive_utc());
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
                    return conflict_error;
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

        let user = user.unwrap();

        if user.email.is_some() && !user.email_confirmed {
            let template = email_templates.clone().confirmation_email_template();

            let data = json!({
                "confirmation_url": format!("{}/confirmation_token={}", config.site_url, user.email_confirmation_token.clone().unwrap()),
                "email": user.email,
                "site_url": config.site_url
            });

            let email = send_email(template, data, user.email.clone().unwrap(), &config);

            if email.is_err() {
                let err = email.err().unwrap();

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
