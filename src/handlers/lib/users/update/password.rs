use crate::{config::Config, crypto::jwt::JWT, handlers::Error, models::user::User};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UpdateForm {
    pub password: String,
}

pub fn update_password(config: &Config, connection: &PooledConnection<ConnectionManager<PgConnection>>, token: &JWT, update_form: UpdateForm, id: String) -> Result<User, Error> {
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

    if !config.password_rule.is_match(update_form.password.as_ref()) {
        return Err(Error::new(400, json!({"code": "invalid_password_format"}), "Invalid Password Format".to_string()));
    }

    user.password = Some(update_form.password);

    user.hash_password(config.password_hash_cost);

    let user = user.save(&connection);

    if user.is_err() {
        let err = user.err().unwrap();
        error!("{:?}", err);
        return Err(Error::from(err));
    }

    return Ok(user.unwrap());
}
