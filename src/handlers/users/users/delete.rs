use crate::{
    config::Config,
    crypto::{jwt::JWT, Error as CryptoError},
    handlers::{lib::users::delete, Error},
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

#[delete("/users/<id>")]
pub fn delete(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    token: Result<JWT, CryptoError>,
    email_templates: State<EmailTemplates>,
    id: String,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
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

    let internal_error = Err(Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string()));

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return internal_error;
        }
    };

    let user = delete::delete(config.inner(), &connection, email_templates.inner(), &operator_signature, &token, id);

    if user.is_err() {
        return Err(user.err().unwrap());
    }

    let body = json!({
        "code": "success",
        "message": "user has been successfully deleted"
    });

    return Ok(status::Custom(Status::Ok, JsonValue(body)));
}
