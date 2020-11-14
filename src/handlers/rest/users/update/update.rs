use crate::{
    crypto::{jwt::JWT, Error as CryptoError},
    handlers::{lib::users::update::update, Error},
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use log::error;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

#[put("/users/<id>", data = "<update_form>")]
pub fn update(
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    token: Result<JWT, CryptoError>,
    update_form: Json<update::UpdateForm>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
    id: String,
) -> Result<JsonValue, Error> {
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

    let user = update::update(&connection, &token, update_form.into_inner(), id);

    if user.is_err() {
        return Err(user.err().unwrap());
    }

    let data = serde_json::to_value(user.unwrap());

    if data.is_err() {
        let err = data.err().unwrap();
        error!("{:?}", err);
        return Err(Error::from(err));
    }

    return Ok(JsonValue(data.unwrap()));
}
