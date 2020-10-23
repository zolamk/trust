use crate::{crypto::jwt::JWT, handlers::Error, models::user::User};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
};
use log::error;

pub fn get(connection: &PooledConnection<ConnectionManager<PgConnection>>, token: &JWT, offset: i64, limit: i64) -> Result<Vec<User>, Error> {
    if !token.is_admin(connection) {
        return Err(Error::new(403, json!({"code": "only_admin_can_get"}), "Only Admin Can Get Users".to_string()));
    }

    let users = crate::models::user::get(offset, limit, connection);

    if users.is_err() {
        let err = users.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    return Ok(users.unwrap());
}
