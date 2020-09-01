use crate::{
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
use rocket_contrib::json::JsonValue;

#[get("/user")]
pub fn get(
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
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

    let internal_error = Err(Error {
        code: 500,
        body: json!({
            "code": "internal_error"
        }),
    });

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return internal_error;
        }
    };

    let user = crate::models::user::get_by_id(token.sub, &connection);

    if user.is_err() {
        let err = user.err().unwrap();

        if let ModelError::DatabaseError(NotFound) = err {
            return Err(Error {
                code: 404,
                body: json!({
                    "code": "user_not_found"
                }),
            });
        }

        error!("{:?}", err);

        return internal_error;
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
