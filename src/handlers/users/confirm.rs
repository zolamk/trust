use crate::{
    handlers::Error,
    models::Error as ModelError,
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
pub struct ConfirmForm {
    pub confirmation_token: String,
}

#[post("/confirm", data = "<confirm_form>")]
pub fn confirm(
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    confirm_form: Json<ConfirmForm>,
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

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(err) => {
            error!("{}", err);
            return Err(internal_error);
        }
    };

    let user = crate::models::user::get_by_confirmation_token(confirm_form.confirmation_token.clone(), &connection);

    if user.is_err() {
        match user.err().unwrap() {
            ModelError::DatabaseError(NotFound) => {
                return Err(Error {
                    code: 404,
                    body: json!({
                        "code": "user_not_found"
                    }),
                })
            }

            err => {
                error!("{:?}", err);

                return Err(internal_error);
            }
        }
    }

    let mut user = user.unwrap();

    let user = user.confirm(&connection);

    if user.is_err() {
        error!("{:?}", user.err().unwrap());

        return Err(internal_error);
    }

    let body = json!({
        "code": "success",
        "message": "email has been confirmed successfully"
    });

    return Ok(status::Custom(Status::Ok, JsonValue(body)));
}
