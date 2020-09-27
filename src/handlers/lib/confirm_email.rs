use crate::{
    handlers::Error,
    models::{user::User, Error as ModelError},
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
    result::Error::NotFound,
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct ConfirmForm {
    #[graphql(name = "confirmation_token")]
    pub confirmation_token: String,
}

pub fn confirm(connection: &PooledConnection<ConnectionManager<PgConnection>>, confirm_form: ConfirmForm) -> Result<User, Error> {
    let internal_error = Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string());

    let user = crate::models::user::get_by_email_confirmation_token(confirm_form.confirmation_token, connection);

    if user.is_err() {
        match user.err().unwrap() {
            ModelError::DatabaseError(NotFound) => return Err(Error::new(404, json!({"code": "user_not_found"}), "User Not Found".to_string())),
            err => {
                error!("{:?}", err);
                return Err(internal_error);
            }
        }
    }

    let mut user = user.unwrap();

    let u = user.confirm_email(connection);

    if u.is_err() {
        error!("{:?}", u.err().unwrap());

        return Err(internal_error);
    }

    return Ok(u.unwrap());
}
