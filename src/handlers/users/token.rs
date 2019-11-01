extern crate diesel;
extern crate handlebars;
extern crate lettre;
extern crate lettre_email;
extern crate native_tls;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error::NotFound;
use rocket::http::Status;

use crate::config::Config;
use crate::crypto::jwt::JWT;
use crate::error::Error;
use crate::hook::{HookEvent, Webhook};
use crate::models::operator_signature::OperatorSignature;
use crate::models::refresh_token::NewRefreshToken;
use log::error;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::JsonValue;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SignUpForm {
    pub name: Option<String>,
    pub email: String,
    pub avatar: Option<String>,
    pub password: String,
}

#[post("/token?<username>&<password>&<grant_type>")]
pub fn token(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    username: String,
    password: String,
    grant_type: String,
    operator_signature: Result<OperatorSignature, Error>,
) -> Result<status::Custom<JsonValue>, Error> {
    if operator_signature.is_err() {
        let err = operator_signature.err().unwrap();
        return Err(err);
    }

    let operator_signature = operator_signature.unwrap();

    if grant_type == "password" {
        return password_grant(
            username,
            password,
            config,
            connection_pool,
            operator_signature,
        );
    } else {
        let err = Error {
            code: 429,
            body: json!({
                "code": "invalid_grant_type",
            }),
        };

        return Err(err);
    }
}

fn password_grant(
    username: String,
    password: String,
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    operator_signature: OperatorSignature,
) -> Result<status::Custom<JsonValue>, Error> {
    let internal_error = Error {
        code: 500,
        body: json!({
            "code": "internal_error",
        }),
    };

    let user_not_confirmed = Error {
        code: 412,
        body: json!({
            "code": "user_not_confirmed",
        }),
    };

    let invalid_email_or_password = Error {
        code: 401,
        body: json!({
            "code": "invalid_email_or_password",
        }),
    };

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(err) => {
            error!("{:?}", err);
            return Err(internal_error);
        }
    };

    let user = crate::models::user::get_by_email(username, &connection);

    if user.is_err() {
        let err = user.err().unwrap();

        match err {
            NotFound => {
                return Err(invalid_email_or_password);
            }
            _ => {
                error!("{:?}", err);
                return Err(internal_error);
            }
        }
    }

    let mut user = user.unwrap();

    if !user.confirmed {
        return Err(user_not_confirmed);
    }

    if !user.verify_password(password) {
        return Err(invalid_email_or_password);
    }

    let payload = json!({
        "event": HookEvent::Signup,
        "user": user,
    });

    let hook = Webhook::new(
        HookEvent::Login,
        payload,
        config.inner().clone(),
        operator_signature,
    );

    let hook = hook.trigger();

    if hook.is_err() {
        let err = hook.err().unwrap();

        error!("{:?}", err);

        return Err(err);
    }

    let hook_response = hook.unwrap();

    if let Some(hook_response) = hook_response {
        if hook_response.is_object() {
            let hook_response = hook_response.as_object().unwrap();

            let update = if hook_response.contains_key("app_metadata") {
                let app_metdata = hook_response.get("app_metadata").unwrap().clone();

                user.app_metadata = Some(app_metdata);

                true
            } else if hook_response.contains_key("user_metadata") {
                let user_metadata = hook_response.get("user_metadata").unwrap().clone();

                user.user_metadata = Some(user_metadata);

                true
            } else {
                false
            };

            if update {
                let res = user.save(&connection);

                if res.is_err() {
                    let err = res.err().unwrap();

                    error!("{:?}", err);

                    return Err(internal_error);
                }

                user = res.unwrap();
            }
        }
    }

    let app_metadata = user.app_metadata;

    let user_metadata = user.user_metadata.clone();

    let jwt = JWT::new(user.id, user.email.clone(), app_metadata, user_metadata);

    let jwt = jwt.sign(config.inner().clone());

    if jwt.is_err() {
        let err = jwt.err().unwrap();

        error!("{:?}", err);

        return Err(internal_error);
    }

    let jwt = jwt.unwrap();

    let refresh_token = NewRefreshToken::new(user.id);

    let refresh_token = refresh_token.save(&connection);

    if refresh_token.is_err() {
        let err = refresh_token.err();
        error!("{:?}", err);
        return Err(Error {
            code: 500,
            body: json!({
                "code": "unable_to_create_refresh_token",
            }),
        });
    }

    let refresh_token = refresh_token.unwrap().token;

    return Ok(status::Custom(
        Status::Ok,
        JsonValue(json!({
            "access_token": jwt,
            "refresh_token": refresh_token,
            "type": "bearer"
        })),
    ));
}
