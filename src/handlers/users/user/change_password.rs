use crate::{
    config::Config,
    crypto::{jwt::JWT, Error as CryptoError},
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
pub struct ChangePasswordForm {
    pub old_password: String,
    pub new_password: String,
}

#[patch("/user/password", data = "<change_password_form>")]
pub fn change_password(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
    change_password_form: Json<ChangePasswordForm>,
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

    if !config.password_rule.is_match(change_password_form.new_password.as_ref()) {
        return Err(Error::new(400, json!({"code": "invalid_password_format"}), "Invalid Password Format".to_string()));
    }

    match crate::models::user::get_by_id(token.sub, &connection) {
        Ok(mut user) => {
            if !user.verify_password(change_password_form.old_password.clone()) {
                return Err(Error::new(400, json!({"code": "invalid_old_password"}), "Invalid Old Password".to_string()));
            }

            user.password = Some(change_password_form.new_password.clone());

            user.hash_password();

            if let Err(err) = user.save(&connection) {
                error!("{:?}", err);

                return Err(internal_error);
            }

            return Ok(status::Custom(
                Status::Ok,
                JsonValue(json!({
                    "code": "success",
                    "message": "password changed successfully",
                })),
            ));
        }
        Err(err) => match err {
            ModelError::DatabaseError(NotFound) => {
                return Err(Error::new(422, json!({"code": "user_not_found"}), "User Not Found".to_string()));
            }
            _ => {
                error!("{:?}", err);

                return Err(Error::from(err));
            }
        },
    }
}
