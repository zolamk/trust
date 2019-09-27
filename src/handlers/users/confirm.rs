extern crate diesel;
extern crate handlebars;
extern crate lettre;
extern crate lettre_email;
extern crate native_tls;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

use crate::diesel::Connection;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::{DatabaseError, NotFound};
use rocket::http::Status;

use crate::config::Config;
use log::error;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ConfirmForm {
    pub token: String,
}

#[post("/confirm", data = "<confirm_form>")]
pub fn confirm(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    confirm_form: Json<ConfirmForm>,
) -> status::Custom<JsonValue> {
    let internal_error = status::Custom(
        Status::InternalServerError,
        json!({
            "code": "internal_error"
        }),
    );

    let no_user_err = status::Custom(
        Status::UnprocessableEntity,
        json!({
            "code": "user_not_found"
        }),
    );

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(err) => {
            error!("{}", err);
            return internal_error;
        }
    };

    let user =
        crate::models::get_user_by_confirmation_token(confirm_form.token.clone(), &connection);

    if user.is_err() {
        match user.err().unwrap() {
            NotFound => return no_user_err,
            err => {
                error!("{}", err);
                return internal_error;
            }
        }
    }

    let mut user = user.unwrap();

    let user = user.confirm(&connection);

    if user.is_err() {
        error!("{}", user.err().unwrap());
        return internal_error;
    }

    return status::Custom(
        Status::Ok,
        json!({
            "code": "confirmed_successfully"
        }),
    );
}
