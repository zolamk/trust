use crate::{
    config::Config,
    crypto::{jwt::JWT, Error as CryptoError},
    handlers::{lib::user::change_phone, Error},
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
    sms::SMSTemplates,
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use log::error;
use rocket::{http::Status, response::status, State};
use rocket_contrib::json::{Json, JsonValue};

#[patch("/user/phone", data = "<change_phone_form>")]
pub fn change_phone(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    sms_templates: State<SMSTemplates>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
    change_phone_form: Json<change_phone::ChangePhoneForm>,
    token: Result<JWT, CryptoError>,
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

    let internal_error = Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string());

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return Err(internal_error);
        }
    };

    let user = change_phone::change_phone(config.inner(), &connection, sms_templates.inner(), &token, change_phone_form.into_inner());

    if user.is_err() {
        return Err(user.err().unwrap());
    }

    let user = user.unwrap();

    return Ok(status::Custom(
        Status::Ok,
        JsonValue(json!({
            "code": "success",
            "message": "phone changed successfully",
            "confirmation_required": !user.phone_confirmed
        })),
    ));
}
