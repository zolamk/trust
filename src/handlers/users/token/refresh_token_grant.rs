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
use rocket::http::Status;

use crate::config::Config;
use crate::crypto;
use crate::crypto::jwt::JWT;
use crate::handlers::Error;
use crate::models::refresh_token::get_refresh_token_by_token;
use crate::models::user;
use log::error;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::JsonValue;

pub fn refresh_token_grant(
    refresh_token: Option<String>,
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
) -> Result<status::Custom<JsonValue>, Error> {
    if refresh_token.is_none() {
        return Err(Error {
            code: 400,
            body: json!({
                "code": "refresh_token_missing"
            }),
        });
    }

    let refresh_token = refresh_token.unwrap();

    let internal_error = Error {
        code: 500,
        body: json!({
            "code": "internal_error",
        }),
    };

    let invalid_refresh_token = Error {
        code: 400,
        body: json!({
            "code": "invalid_refresh_token"
        }),
    };

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(err) => {
            error!("{:?}", err);
            return Err(internal_error);
        }
    };

    let refresh_token = get_refresh_token_by_token(refresh_token, &connection);

    if refresh_token.is_err() {
        let err = refresh_token.err().unwrap();

        error!("{:?}", err);

        return Err(invalid_refresh_token);
    }

    let mut refresh_token = refresh_token.unwrap();

    let token = crypto::secure_token(50);

    refresh_token.token = token;

    let refresh_token = refresh_token.save(&connection);

    if refresh_token.is_err() {
        let err = refresh_token.err().unwrap();

        error!("{:?}", err);

        return Err(internal_error);
    }

    let refresh_token = refresh_token.unwrap();

    let user = user::get_by_id(refresh_token.user_id, &connection);

    if user.is_err() {
        let err = user.err().unwrap();

        error!("{:?}", err);

        return Err(internal_error);
    }

    let user = user.unwrap();

    let jwt = JWT::new(&user, config.aud.clone());

    let jwt = jwt.sign(config.inner());

    if jwt.is_err() {
        let err = jwt.err().unwrap();

        error!("{:?}", err);

        return Err(internal_error);
    }

    let jwt = jwt.unwrap();

    return Ok(status::Custom(
        Status::Ok,
        JsonValue(json!({
            "access_token": jwt,
            "refresh_token": refresh_token.token,
            "type": "bearer"
        })),
    ));
}