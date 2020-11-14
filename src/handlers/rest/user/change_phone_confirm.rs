use crate::{
    handlers::{lib::user::change_phone_confirm, Error},
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use log::error;
use rocket::{http::Status, response::status, State};
use rocket_contrib::json::{Json, JsonValue};

#[patch("/user/phone/confirm", data = "<confirm_change_phone_form>")]
pub fn change_phone_confirm(
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
    confirm_change_phone_form: Json<change_phone_confirm::ConfirmPhoneChangeForm>,
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
            error!("{:?}", err);
            return Err(internal_error);
        }
    };

    let user = change_phone_confirm::change_phone_confirm(&connection, confirm_change_phone_form.into_inner());

    if user.is_err() {
        return Err(user.err().unwrap());
    }

    return Ok(status::Custom(
        Status::Ok,
        JsonValue(json!({
            "code": "success",
            "message": "phone changed successfully",
        })),
    ));
}
