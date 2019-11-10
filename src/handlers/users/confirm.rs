use crate::config::Config;
use crate::models::Error as ModelError;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error::NotFound;
use log::error;
use rocket::response::Redirect;
use rocket::State;

#[get("/confirm?<confirmation_token>")]
pub fn confirm(
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    config: State<Config>,
    confirmation_token: String,
) -> Redirect {
    let internal_error_redirect =
        Redirect::to(format!("{}?code=internal_error", config.confirmed_redirect));

    let user_not_found_redirect =
        Redirect::to(format!("{}?code=user_not_found", config.confirmed_redirect));

    let success_redirect = Redirect::to(format!("{}?code=success", config.confirmed_redirect));

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(err) => {
            error!("{}", err);
            return internal_error_redirect;
        }
    };

    let user = crate::models::user::get_by_confirmation_token(confirmation_token, &connection);

    if user.is_err() {
        match user.err().unwrap() {
            ModelError::DatabaseError(NotFound) => return user_not_found_redirect,

            err => {
                error!("{:?}", err);

                return internal_error_redirect;
            }
        }
    }

    let mut user = user.unwrap();

    let user = user.confirm(&connection);

    if user.is_err() {
        error!("{:?}", user.err().unwrap());

        return internal_error_redirect;
    }

    success_redirect
}
