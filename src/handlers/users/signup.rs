use crate::{
    config::Config,
    crypto::secure_token,
    handlers::{trigger_hook, Error},
    hook::HookEvent,
    mailer::{send_confirmation_email, EmailTemplates},
    models::{user::NewUser, Error as ModelError},
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
};
use chrono::Utc;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
    result::{DatabaseErrorKind, Error::DatabaseError},
    Connection, NotFound,
};
use log::error;
use rocket::{http::Status, response::status, State};
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SignUpForm {
    pub email: String,
    pub password: String,
}

#[post("/signup", data = "<signup_form>")]
pub fn signup(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    email_templates: State<EmailTemplates>,
    signup_form: Json<SignUpForm>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
) -> Result<status::Custom<JsonValue>, Error> {
    if config.disable_signup {
        let err = Error {
            code: 422,
            body: json!({
                "code": "signup_disabled",
                "message": "trust instance has signup disabled"
            }),
        };
        return Err(err);
    }

    if operator_signature.is_err() {
        let err = operator_signature.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let operator_signature = operator_signature.unwrap();

    if !config.password_rule.is_match(signup_form.password.as_ref()) {
        return Err(Error {
            code: 400,
            body: json!({
                "code": "invalid_password_format",
                "message": "invalid password"
            }),
        });
    }

    let mut user = NewUser::default();

    user.email = signup_form.email.clone();

    user.password = Some(signup_form.password.clone());

    user.aud = config.aud.to_string();

    user.confirmed = config.auto_confirm;

    user.confirmation_token = if config.auto_confirm { None } else { Some(secure_token(100)) };

    user.confirmation_token_sent_at = if config.auto_confirm { None } else { Some(Utc::now().naive_utc()) };

    user.hash_password();

    let internal_error = Error {
        code: 500,
        body: json!({
            "code": "internal_error"
        }),
    };

    let conflict_error = Err(Error {
        code: 409,
        body: json!({
            "code": "email_registered",
            "message": "a user with this email has already been registered",
        }),
    });

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return Err(internal_error);
        }
    };

    // if users exists and is confirmed return conflict error
    // if not delete the unconfirmed user
    // if the error is user not found proceed with the normal flow
    match crate::models::user::get_by_email(user.email.clone(), &connection) {
        Ok(user) => {
            if user.confirmed {
                return conflict_error;
            }

            let result = user.delete(&connection);

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

    let transaction = connection.transaction::<_, Error, _>(|| {
        let user = user.save(&connection);

        if user.is_err() {
            let err = user.err().unwrap();

            if let ModelError::DatabaseError(DatabaseError(DatabaseErrorKind::UniqueViolation, _info)) = err {
                let err = Error {
                    code: 409,
                    body: json!({
                        "code": "email_already_registered"
                    }),
                };
                return Err(err);
            }

            error!("{:?}", err);

            return Err(internal_error);
        }

        let user = user.unwrap();

        let user = trigger_hook(HookEvent::Signup, user, config.inner(), &connection, operator_signature, "email".to_string());

        if user.is_err() {
            let err = user.err().unwrap();

            error!("{:?}", err);

            return Err(err);
        }

        let user = user.unwrap();

        if !config.auto_confirm {
            let confirmation_url = format!("{}/confirm?confirmation_token={}", config.site_url, user.confirmation_token.clone().unwrap(),);

            let template = email_templates.clone().confirmation_email_template();

            let email = send_confirmation_email(template, confirmation_url, &user, config.inner());

            if email.is_err() {
                let err = email.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));
            }
        }

        return Ok(());
    });

    if transaction.is_ok() {
        let body = json!({
            "code": "success",
            "message": "user has been successfully signed up"
        });

        return Ok(status::Custom(Status::Ok, JsonValue(body)));
    }

    let err = transaction.err().unwrap();

    return Err(err);
}
