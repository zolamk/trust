use crate::{
    handlers::{lib::confirm_phone, Error},
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use log::error;
use rocket::{http::Status, response::status, State};
use rocket_contrib::json::{Json, JsonValue};

#[post("/confirm/phone", data = "<confirm_form>")]
pub fn confirm(
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    confirm_form: Json<confirm_phone::ConfirmForm>,
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
        Err(err) => {
            error!("{}", err);
            return Err(internal_error);
        }
    };

    let user = confirm_phone::confirm(&connection, confirm_form.into_inner());

    if user.is_err() {
        return Err(user.err().unwrap());
    }

    let body = json!({
        "code": "success",
        "message": "phone number has been confirmed successfully"
    });

    return Ok(status::Custom(Status::Ok, JsonValue(body)));
}
