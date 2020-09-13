use crate::{
    config::Config,
    crypto::{jwt::JWT, Error as CryptoError},
    handlers::{lib::users::update::password, Error},
    mailer::EmailTemplates,
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use log::error;
use rocket::{http::Status, response::status, State};
use rocket_contrib::json::{Json, JsonValue};

#[put("/users/<id>/password", data = "<update_form>")]
pub fn change_password(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    token: Result<JWT, CryptoError>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
    email_templates: State<EmailTemplates>,
    update_form: Json<password::UpdateForm>,
    id: String,
) -> Result<status::Custom<JsonValue>, Error> {
    if operator_signature.is_err() {
        let err = operator_signature.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let operator_signature = operator_signature.unwrap();

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

    let user = password::update_password(config.inner(), &connection, email_templates.inner(), &operator_signature, &token, update_form.into_inner(), id);

    if user.is_err() {
        return Err(user.err().unwrap());
    }

    return Ok(status::Custom(
        Status::Ok,
        JsonValue(json!({
            "code": "success",
            "message": "password changed successfully"
        })),
    ));
}
