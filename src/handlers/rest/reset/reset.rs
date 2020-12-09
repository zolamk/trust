use crate::{
    config::Config,
    handlers::{lib::reset::reset, Error},
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use log::error;
use rocket::{http::Status, response::status, State};
use rocket_contrib::json::{Json, JsonValue};

#[post("/reset", data = "<reset_form>")]
pub fn reset(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    reset_form: Json<reset::ResetForm>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
) -> Result<status::Custom<JsonValue>, Error> {
    if operator_signature.is_err() {
        let err = operator_signature.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let internal_error = Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string());

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return Err(internal_error);
        }
    };

    let res = reset::reset(&config, &connection, reset_form.into_inner());

    if res.is_err() {
        let err = res.err().unwrap();
        error!("{:?}", err);
    }

    return Ok(status::Custom(Status::Accepted, JsonValue(json!({"code": "accepted"}))));
}