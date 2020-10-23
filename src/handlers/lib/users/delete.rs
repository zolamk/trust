use crate::{crypto::jwt::JWT, handlers::Error, models::user::User};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
};
use log::error;

pub fn delete(connection: &PooledConnection<ConnectionManager<PgConnection>>, token: &JWT, id: String) -> Result<User, Error> {
    if !token.is_admin(connection) {
        return Err(Error::new(403, json!({"code": "only_admin_can_delete"}), "Only Admin Can Delete Users".to_string()));
    }

    let user = crate::models::user::get_by_id(id, &connection);

    if user.is_err() {
        let err = user.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let user = user.unwrap();

    let user = user.delete(connection);

    if user.is_err() {
        let err = user.err().unwrap();
        error!("{:?}", err);
        return Err(Error::from(err));
    }

    return Ok(user.unwrap());
}
