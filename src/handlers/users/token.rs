use crate::{
    config::Config,
    handlers::Error,
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
};

use crate::handlers::lib::token;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use log::error;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

#[post("/token", data = "<login_form>")]
pub fn token(
    login_form: Json<token::LoginForm>,
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
) -> Result<JsonValue, Error> {
    if operator_signature.is_err() {
        let err = operator_signature.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let operator_signature = operator_signature.unwrap();

    let internal_error = Error::new(500, json!({"code": "internal_server_error"}), "Internal Server Error".to_string());

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(err) => {
            error!("{:?}", err);
            return Err(internal_error);
        }
    };

    let token = token::token(config.inner(), &connection, operator_signature, login_form.into_inner());

    if token.is_err() {
        return Err(token.err().unwrap());
    }

    return Ok(JsonValue(json!(token.unwrap())));
}
