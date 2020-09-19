use crate::{
    config::Config,
    crypto::{jwt::JWT, Error as CryptoError},
    handlers::{lib::users::get_by_id, Error},
    mailer::EmailTemplates,
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use log::error;
use rocket::{http::Status, response::status, State};
use rocket_contrib::json::JsonValue;

#[get("/users/<id>")]
pub fn user(
    config: State<Config>,
    email_templates: State<EmailTemplates>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    token: Result<JWT, CryptoError>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
    id: String,
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

    let operator_signature = operator_signature.unwrap();

    let token = token.unwrap();

    let internal_error = Err(Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string()));

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return internal_error;
        }
    };

    let user = get_by_id::get_by_id(config.inner(), &connection, email_templates.inner(), &operator_signature, &token, id);

    if user.is_err() {
        return Err(user.err().unwrap());
    }

    let user = user.unwrap();

    let user = serde_json::to_value(&user);

    if user.is_err() {
        let err = user.err().unwrap();

        error!("{:?}", err);

        return internal_error;
    }

    let user = user.unwrap();

    return Ok(status::Custom(Status::Ok, JsonValue(user)));
}
