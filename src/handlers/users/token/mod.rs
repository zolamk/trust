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

mod password_grant;
mod refresh_token_grant;

use password_grant::password_grant;
use refresh_token_grant::refresh_token_grant;

use crate::config::Config;
use crate::error::Error;
use crate::models::operator_signature::OperatorSignature;
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

#[post("/token?<username>&<password>&<grant_type>&<refresh_token>")]
pub fn token(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    username: Option<String>,
    password: Option<String>,
    grant_type: String,
    refresh_token: Option<String>,
    operator_signature: Result<OperatorSignature, Error>,
) -> Result<status::Custom<JsonValue>, Error> {
    if operator_signature.is_err() {
        let err = operator_signature.err().unwrap();
        return Err(err);
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

        return password_grant(
            username,
            password,
            config,
            connection_pool,
            operator_signature,
        );
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
