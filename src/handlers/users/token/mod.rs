use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
mod password_grant;
mod refresh_token_grant;
use crate::{
    config::Config,
    handlers::Error,
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
};
use log::error;
use password_grant::password_grant;
use refresh_token_grant::refresh_token_grant;
use rocket::{response::status, State};
use rocket_contrib::json::JsonValue;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SignUpForm {
    pub name: Option<String>,
    pub email: String,
    pub avatar: Option<String>,
    pub password: String,
}

#[post("/token?<username>&<password>&<grant_type>&<refresh_token>")]
pub fn token(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    username: Option<String>,
    password: Option<String>,
    grant_type: String,
    refresh_token: Option<String>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
) -> Result<status::Custom<JsonValue>, Error> {
    if operator_signature.is_err() {
        let err = operator_signature.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let operator_signature = operator_signature.unwrap();

    if grant_type == "password" {
        if username.is_none() || password.is_none() {
            return Err(Error {
                code: 400,
                body: json!({
                    "code": "username_and_password_required_for_password_grant"
                }),
            });
        }

        let username = username.unwrap();

        let password = password.unwrap();

        return password_grant(username, password, config, connection_pool, operator_signature);
    } else if grant_type == "refresh_token" {
        return refresh_token_grant(refresh_token, config, connection_pool);
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
