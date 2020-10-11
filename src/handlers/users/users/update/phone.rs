use crate::{
    config::Config,
    crypto::{jwt::JWT, Error as CryptoError},
    handlers::{lib::users::update::phone, Error},
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
    sms::SMSTemplates,
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use log::error;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

#[patch("/users/<id>/phone", data = "<update_form>")]
pub fn update_phone(
    config: State<Config>,
    sms_templates: State<SMSTemplates>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    token: Result<JWT, CryptoError>,
    update_form: Json<phone::UpdateForm>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
    id: String,
) -> Result<JsonValue, Error> {
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

    let user = phone::update_phone(config.inner(), &connection, sms_templates.inner(), &token, update_form.into_inner(), id);

    if user.is_err() {
        return Err(user.err().unwrap());
    }

    return Ok(JsonValue(json!({
        "code": "success",
    })));
}
