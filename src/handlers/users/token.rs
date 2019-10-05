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
use crate::crypto::secure_token;
use crate::mailer::EmailTemplates;
use crate::models::user::NewUser;
use chrono::Utc;
use diesel::result::Error;
use handlebars::Handlebars;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::{ConnectionReuseParameters, SmtpClient};
use lettre::{ClientSecurity, ClientTlsParameters, Transport};
use lettre_email::Email;
use log::error;
use native_tls::TlsConnector;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

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
) -> status::Custom<JsonValue> {
    if grant_type == "password" {
        return password_grant(username, password, config, connection_pool);
    } else {
        return status::Custom(
            Status::UnprocessableEntity,
            json!({
                "code": "invalid_grant_type"
            }),
        );
    }
}

fn password_grant(
    username: String,
    password: String,
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
) -> status::Custom<JsonValue> {
    let internal_error = status::Custom(
        Status::InternalServerError,
        json!({
            "code": "internal_error",
        }),
    );

    let user_not_confirmed = status::Custom(
        Status::PreconditionFailed,
        json!({
            "code": "user_not_confirmed"
        }),
    );

    let invalid_email_or_password = status::Custom(
        Status::Unauthorized,
        json!({
            "code": "invalid_email_or_password"
        }),
    );

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(err) => {
            error!("{}", err);
            return internal_error;
        }
    };

    match crate::models::user::get_by_email(username, &connection) {
        Ok(mut u) => {
            if !u.confirmed {
                return user_not_confirmed;
            }

            // if u.verify_password(password) {
            //     let jwt = JWT::new(u.id, u.email, app_metadata: Option<Value>, user_metadata: Option<Value>, config: Config)
            // }

            return invalid_email_or_password;
        }
        Err(err) => match err {
            NotFound => {
                return invalid_email_or_password;
            }
            _ => {
                error!("{}", err);
                return internal_error;
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
