use crate::{
    config::Config,
    crypto::{jwt::JWT, Error as CryptoError},
    handlers::Error,
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use log::error;
use rocket::{http::Status, response::status, State};
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UpdateForm {
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub confirm: bool,
}

#[put("/users/<id>", data = "<update_form>")]
pub fn update(
    _config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    token: Result<JWT, CryptoError>,
    update_form: Json<UpdateForm>,
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

    let token = token.unwrap();

    let internal_error = Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string());

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return Err(internal_error);
        }
    };

    if !token.is_admin(&connection) {
        return Err(Error::new(403, json!({"code": "only_admin_can_update"}), "Only Admin Can Update Users".to_string()));
    }

    let user = crate::models::user::get_by_id(id, &connection);

    if user.is_err() {
        let err = user.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let mut user = user.unwrap();

    if user.id == token.sub {
        return Err(Error::new(403, json!({"code": "admin_cant_update_self"}), "Admin Can't Update Self".to_string()));
    }

    user.name = update_form.name.clone();

    user.avatar = update_form.avatar.clone();

    let user = user.save(&connection);

    if user.is_err() {
        let err = user.err().unwrap();
        error!("{:?}", err);
        return Err(Error::from(err));
    }

    let user = user.unwrap();

    let data = serde_json::to_value(user);

    if data.is_err() {
        let err = data.err().unwrap();
        error!("{:?}", err);
        return Err(Error::from(err));
    }

    let data = data.unwrap();

    return Ok(status::Custom(Status::Ok, JsonValue(data)));
}
