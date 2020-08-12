use crate::{
    config::Config,
    crypto::{jwt::JWT, secure_token, Error as CryptoError},
    handlers::Error,
    mailer::{send_email, EmailTemplates},
    models::Error as ModelError,
};
use chrono::Utc;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
    result::Error::NotFound,
};
use log::error;
use rocket::{http::Status, response::status, State};
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ChangeEmailFrom {
    pub email: String,
}

#[patch("/user/email", data = "<change_email_form>")]
pub fn change_email(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    email_templates: State<EmailTemplates>,
    change_email_form: Json<ChangeEmailFrom>,
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

    let conflict_error = Error {
        code: 409,
        body: json!({
            "code": "email_registered",
            "message": "a user with this email has already been registered",
        }),
    };

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return Err(internal_error);
        }
    };

    let user = crate::models::user::get_by_id(token.sub, &connection);

    if user.is_err() {
        let err = user.err().unwrap();

        if let ModelError::DatabaseError(NotFound) = err {
            return Err(Error {
                code: 422,
                body: json!({
                    "code": "user_not_found"
                }),
            });
        }

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let mut user = user.unwrap();

    match crate::models::user::get_by_email(change_email_form.email.clone(), &connection) {
        Ok(user) => {
            if user.confirmed {
                return Err(conflict_error);
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

    if config.auto_confirm {
        user.new_email = Some(user.email.clone()); // store the old email in new email in case we ever need to revert it

        user.email = change_email_form.email.clone();

        let user = user.save(&connection);

        if user.is_err() {
            let err = user.err().unwrap();

            error!("{:?}", err);

            return Err(Error::from(err));
        }

        return Ok(status::Custom(
            Status::Ok,
            JsonValue(json!({
                "code": "success",
                "confirmation_required": false,
                "message": "email changed successfully",
            })),
        ));
    }

    user.new_email = Some(change_email_form.email.clone());

    user.email_change_token = Some(secure_token(100));

    user.email_change_token_sent_at = Some(Utc::now().naive_utc());

    let user = user.save(&connection);

    if user.is_err() {
        let err = user.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let user = user.unwrap();

    let template = email_templates.clone().confirmation_email_template();

    let data = json!({
        "confirmation_url": format!("{}/email_change_token={}", config.site_url, user.email_change_token.clone().unwrap()),
        "email": user.email,
        "site_url": config.site_url
    });

    let email = send_email(template, data, &user, &config);

    if email.is_err() {
        let err = email.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    return Ok(status::Custom(
        Status::Ok,
        JsonValue(json!({
            "code": "success",
            "confirmation_required": !config.auto_confirm,
            "message": "email changed successfully",
        })),
    ));
}
