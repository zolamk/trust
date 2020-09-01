use crate::{
    config::Config,
    handlers::Error,
    models::{user::get_by_recovery_token, Error as ModelError},
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
    result::Error::NotFound,
};
use log::error;
use rocket::{http::Status, response::status, State};
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ConfirmResetForm {
    pub recovery_token: String,
    pub new_password: String,
}

#[post("/reset/confirm", data = "<reset_form>")]
pub fn confirm_reset(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    reset_form: Json<ConfirmResetForm>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
) -> Result<status::Custom<JsonValue>, Error> {
    if operator_signature.is_err() {
        let err = operator_signature.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let internal_error = Error {
        code: 500,
        body: json!({
            "code": "internal_error"
        }),
    };

    if !config.password_rule.is_match(reset_form.new_password.as_ref()) {
        return Err(Error {
            code: 400,
            body: json!({
                "code": "invalid_password_format",
                "message": "invalid password"
            }),
        });
    }

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return Err(internal_error);
        }
    };

    let user = get_by_recovery_token(reset_form.recovery_token.clone(), &connection);

    if user.is_err() {
        let err = user.err().unwrap();
        if let ModelError::DatabaseError(NotFound) = err {
            return Ok(status::Custom(Status::NotFound, JsonValue(json!({"code": "recovery_token_not_found"}))));
        }
        error!("{:?}", err);
        return Err(Error::from(err));
    }

    let mut user = user.unwrap();

    user.password = Some(reset_form.new_password.clone());

    user.hash_password();

    let user = user.save(&connection);

    if user.is_err() {
        let err = user.err().unwrap();
        error!("{:?}", err);
        return Err(Error::from(err));
    }

    return Ok(status::Custom(Status::Ok, JsonValue(json!({"code": "success"}))));
}
