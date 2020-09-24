use crate::{
    config::Config,
    crypto::{jwt::JWT, Error as CryptoError},
    handlers::{lib::users::create, Error},
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

#[post("/users", data = "<create_form>")]
pub fn create(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    email_templates: State<EmailTemplates>,
    create_form: Json<create::CreateForm>,
    token: Result<JWT, CryptoError>,
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

    let internal_error = Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string());

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return Err(internal_error);
        }
    };

    let user = create::create(config.inner(), &connection, email_templates.inner(), &operator_signature, &token, create_form.into_inner());

    if user.is_err() {
        return Err(user.err().unwrap());
    }

    let user = user.unwrap();

    let body = json!({
        "code": "success",
        "email_confirmation_required": !user.email_confirmed,
        "message": "user has been successfully created"
    });

    return Ok(status::Custom(Status::Ok, JsonValue(body)));
}
