use crate::{
    config::Config,
    crypto::{jwt::JWT, secure_token, Error as CryptoError},
    diesel::Connection,
    handlers::Error,
    mailer::{send_email, EmailTemplates},
    models::{user::NewUser, Error as ModelError},
};
use chrono::Utc;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
    result::{
        DatabaseErrorKind,
        Error::{DatabaseError, NotFound},
    },
};
use log::error;
use rocket::{http::Status, response::status, State};
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateForm {
    pub email: String,
    pub password: String,
    pub confirm: bool,
    pub data: Option<serde_json::Value>,
    pub app_metadata: Option<serde_json::Value>,
}

#[post("/users", data = "<create_form>")]
pub fn create(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    email_templates: State<EmailTemplates>,
    create_form: Json<CreateForm>,
    token: Result<JWT, CryptoError>,
) -> Result<status::Custom<JsonValue>, Error> {
    if token.is_err() {
        let err = token.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let token = token.unwrap();

    let internal_error = Error {
        code: 500,
        body: json!({
            "code": "internal_error"
        }),
    };

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return Err(internal_error);
        }
    };

    if !token.is_admin(&connection) {
        return Err(Error {
            code: 403,
            body: json!({
                "code": "only_admin_can_create"
            }),
        });
    }

    if !config.password_rule.is_match(create_form.password.as_ref()) {
        return Err(Error {
            code: 400,
            body: json!({
                "code": "invalid_password_format",
                "message": "invalid password"
            }),
        });
    }

    let conflict_error = Err(Error {
        code: 409,
        body: json!({
            "code": "email_registered",
            "message": "a user with this email has already been registered",
        }),
    });

    // if users exists and is confirmed return conflict error
    // if not delete the unconfirmed user
    // if the error is user not found proceed with the normal flow
    match crate::models::user::get_by_email(create_form.email.clone(), &connection) {
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

    let mut user = NewUser::default();

    user.confirmed = config.auto_confirm || create_form.confirm;

    user.email = create_form.email.clone();

    user.password = Some(create_form.password.clone());

    user.hash_password();

    user.aud = config.aud.clone();

    if !user.confirmed {
        user.confirmation_token = Some(secure_token(100));

        user.confirmation_token_sent_at = Some(Utc::now().naive_utc());
    }

    user.user_metadata = create_form.data.clone();

    user.app_metadata = create_form.app_metadata.clone();

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

        if !user.confirmed {
            let template = email_templates.clone().confirmation_email_template();

            let data = json!({
                "confirmation_url": format!("{}/confirmation_token={}", config.site_url, user.confirmation_token.clone().unwrap()),
                "email": user.email,
                "site_url": config.site_url
            });

            let email = send_email(template, data, &user, &config);

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
            "confirmation_required": !user.confirmed,
            "message": "user has been successfully created"
        });

        return Ok(status::Custom(Status::Ok, JsonValue(body)));
    }

    let err = transaction.err().unwrap();

    return Err(err);
}
