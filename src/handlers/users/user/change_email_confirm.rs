use crate::{
    handlers::Error,
    models::{user::get_by_email_change_token, Error as ModelError},
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
pub struct ConfirmChangeEmailForm {
    pub email_change_token: String,
}

#[patch("/user/email/confirm", data = "<confirm_change_email_form>")]
pub fn change_email_confirm(
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
    confirm_change_email_form: Json<ConfirmChangeEmailForm>,
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

    let user = get_by_email_change_token(confirm_change_email_form.email_change_token.clone(), &connection);

    if user.is_err() {
        let err = user.err().unwrap();
        match err {
            ModelError::DatabaseError(NotFound) => return Err(Error::new(404, json!({"code": "user_not_found"}), "User Not Found".to_string())),
            _ => {
                error!("{:?}", err);
                return Err(Error::from(err));
            }
        }
    }

    let mut user = user.unwrap();

    match user.confirm_email_change(&connection) {
        Ok(_) => {
            return Ok(status::Custom(
                Status::Ok,
                JsonValue(json!({
                    "code": "success",
                    "message": "email changed successfully",
                })),
            ));
        }
        Err(err) => {
            error!("{:?}", err);
            return Err(Error::from(err));
        }
    }
}
