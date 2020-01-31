use crate::{
    config::Config,
    crypto::{jwt::JWT, Error as CryptoError},
    handlers::Error,
    models::Error as ModelError,
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
pub struct UpdatePasswordForm {
    pub old_password: String,
    pub new_password: String,
}

#[patch("/user/password", data = "<update_password_form>")]
pub fn update_password(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    update_password_form: Json<UpdatePasswordForm>,
    token: Result<JWT, CryptoError>,
) -> Result<status::Custom<JsonValue>, Error> {
    if token.is_err() {
        let err = token.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let token = token.unwrap();

    let internal_error = Error {
        code: 500,
        body: json!({
            "code": "internal_error"
        }),
    };

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return Err(internal_error);
        }
    };

    if !config.password_rule.is_match(update_password_form.new_password.as_ref()) {
        return Err(Error {
            code: 400,
            body: json!({
                "code": "invalid_password_format",
                "message": "invalid password"
            }),
        });
    }

    match crate::models::user::get_by_id(token.sub, &connection) {
        Ok(mut user) => {
            if !user.verify_password(update_password_form.old_password.clone()) {
                return Err(Error {
                    code: 400,
                    body: json!({
                        "code": "invalid_old_password"
                    }),
                });
            }

            user.password = Some(update_password_form.new_password.clone());

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
                return Err(Error {
                    code: 422,
                    body: json!({
                        "code": "user_not_found"
                    }),
                });
            }
            _ => {
                error!("{:?}", err);

                return Err(Error::from(err));
            }
        },
    }
}
