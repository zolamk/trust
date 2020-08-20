use crate::{
    config::Config,
    crypto::{jwt::JWT, Error as CryptoError},
    handlers::Error,
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
    pub password: String,
}

#[put("/users/<id>/password", data = "<update_form>")]
pub fn change_password(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    token: Result<JWT, CryptoError>,
    update_form: Json<UpdateForm>,
    id: i64,
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

    if !token.is_admin(&connection) {
        return Err(Error {
            code: 403,
            body: json!({
                "code": "only_admin_can_update"
            }),
        });
    }

    let user = crate::models::user::get_by_id(id, &connection);

    if user.is_err() {
        let err = user.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let mut user = user.unwrap();

    if user.id == token.sub {
        return Err(Error {
            code: 422,
            body: json!({
                "code": "admin_cant_update_self"
            }),
        });
    }

    if !config.password_rule.is_match(update_form.password.as_ref()) {
        return Err(Error {
            code: 400,
            body: json!({
                "code": "invalid_password_format",
                "message": "invalid password"
            }),
        });
    }

    user.password = Some(update_form.password.clone());

    user.hash_password();

    let user = user.save(&connection);

    if user.is_err() {
        let err = user.err().unwrap();
        error!("{:?}", err);
        return Err(Error::from(err));
    }

    return Ok(status::Custom(
        Status::Ok,
        JsonValue(json!({
            "code": "success",
            "message": "password changed successfully"
        })),
    ));
}
