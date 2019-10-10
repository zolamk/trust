extern crate diesel;
extern crate handlebars;
extern crate lettre;
extern crate lettre_email;
extern crate native_tls;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

use crate::diesel::Connection;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::{DatabaseError, NotFound};
use rocket::http::Status;

use crate::config::Config;
use crate::crypto::jwt::JWT;
use crate::error::Error;
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

#[post("/token?<username>&<password>&<grant_type>&<refresh_token>")]
pub fn token(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    username: String,
    password: String,
    grant_type: String,
    refresh_token: Option<String>,
) -> Result<status::Custom<JsonValue>, Error> {
    if grant_type == "password" {
        return password_grant(username, password, config, connection_pool);
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
            error!("{}", err);
            return Err(internal_error);
        }
    };

    match crate::models::user::get_by_email(username, &connection) {
        Ok(u) => {
            if !u.confirmed {
                return Err(user_not_confirmed);
            }

            if u.verify_password(password) {
                // let jwt = JWT::new(u.id, u.email, app_metadata: Option<Value>, user_metadata: Option<Value>, config: Config)
            }

            return Err(invalid_email_or_password);
        }
        Err(err) => match err {
            NotFound => {
                return Err(invalid_email_or_password);
            }
            _ => {
                error!("{}", err);
                return Err(internal_error);
            }
        },
    }
}

// fn refresh_token_grant(
//     refresh_token: Option<String>,
//     config: State<Config>,
//     connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
// ) -> status::Custom<JsonValue> {
// }
