use crate::{
    config::Config,
    crypto::{jwt::JWT, Error as CryptoError},
    handlers::{lib::users::get, Error},
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use log::error;
use rocket::{http::Status, response::status, State};
use rocket_contrib::json::JsonValue;

#[get("/users?<offset>&<limit>")]
pub fn users(
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    token: Result<JWT, CryptoError>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
    offset: i64,
    limit: i64,
    config: State<Config>,
) -> Result<status::Custom<JsonValue>, Error> {
    if operator_signature.is_err() {
        let err = operator_signature.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    if token.is_err() {
        let err = token.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let token = token.unwrap();

    let internal_error = Err(Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string()));

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return internal_error;
        }
    };

    let users = get::get(&connection, &token, offset, limit, config.inner());

    if users.is_err() {
        return Err(users.err().unwrap());
    }

    let users = users.unwrap();

    let users = serde_json::to_value(&users);

    if users.is_err() {
        let err = users.err().unwrap();

        error!("{:?}", err);

        return internal_error;
    }

    let users = users.unwrap();

    return Ok(status::Custom(Status::Ok, JsonValue(users)));
}
