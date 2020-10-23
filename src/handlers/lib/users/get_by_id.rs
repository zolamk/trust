use crate::{
    crypto::jwt::JWT,
    handlers::Error,
    models::{user::User, Error as ModelError},
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
    result::Error::NotFound,
};
use log::error;

pub fn get_by_id(connection: &PooledConnection<ConnectionManager<PgConnection>>, token: &JWT, id: String) -> Result<User, Error> {
    if token.sub != id && !token.is_admin(connection) {
        return Err(Error::new(403, json!({"code": "only_admin_can_get"}), "Only Admin Can Get Users".to_string()));
    }

    let internal_error = Err(Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string()));

    let user = crate::models::user::get_by_id(id, &connection);

    if user.is_err() {
        let err = user.err().unwrap();

        if let ModelError::DatabaseError(NotFound) = err {
            return Err(Error::new(404, json!({"code": "user_not_found"}), "User Not Found".to_string()));
        }

        error!("{:?}", err);

        return internal_error;
    }

    return Ok(user.unwrap());
}
