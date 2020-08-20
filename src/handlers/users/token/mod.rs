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
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
    pub grant_type: String,
    pub refresh_token: Option<String>,
}

#[post("/token", data = "<form>")]
pub fn token(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    form: Json<LoginForm>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
) -> Result<status::Custom<JsonValue>, Error> {
    if operator_signature.is_err() {
        let err = operator_signature.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let operator_signature = operator_signature.unwrap();

    if form.grant_type == "password" {
        return password_grant(form.username.clone(), form.password.clone(), config, connection_pool, operator_signature);
    } else if form.grant_type == "refresh_token" {
        return refresh_token_grant(form.refresh_token.clone(), config, connection_pool, operator_signature);
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
