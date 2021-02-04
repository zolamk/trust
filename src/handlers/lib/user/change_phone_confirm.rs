use crate::{
    handlers::Error,
    models::{
        user::{get_by_phone_change_token, User},
        Error as ModelError,
    },
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
    result::Error::NotFound,
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct ConfirmPhoneChangeForm {
    #[graphql(name = "phone_change_token")]
    pub phone_change_token: String,
}

pub fn change_phone_confirm(connection: &PooledConnection<ConnectionManager<PgConnection>>, confirm_change_phone_form: ConfirmPhoneChangeForm) -> Result<User, Error> {
    let user = get_by_phone_change_token(confirm_change_phone_form.phone_change_token, connection);

    if user.is_err() {
        let err = user.err().unwrap();
        match err {
            ModelError::DatabaseError(NotFound) => return Err(Error::new(404, json!({"code": "user_not_found"}), "User Not Found".to_string())),
            _ => {
                error!("{:?}", err);
                return Err(Error::from(err));
            }
        }
    }

    let mut user = user.unwrap();

    let user = user.confirm_phone_change(connection);

    if user.is_err() {
        let err = user.err().unwrap();
        error!("{:?}", err);
        return Err(Error::from(err));
    }

    return Ok(user.unwrap());
}
