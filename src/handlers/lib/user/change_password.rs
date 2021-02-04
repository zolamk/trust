use crate::{
    config::Config,
    crypto::jwt::JWT,
    handlers::Error,
    models::{
        user::{get_by_id, User},
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
pub struct ChangePasswordForm {
    #[graphql(name = "old_password")]
    pub old_password: String,
    #[graphql(name = "new_password")]
    pub new_password: String,
}

pub fn change_password(config: &Config, connection: &PooledConnection<ConnectionManager<PgConnection>>, token: &JWT, change_password_form: ChangePasswordForm) -> Result<User, Error> {
    let internal_error = Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string());

    if !config.password_rule.is_match(change_password_form.new_password.as_ref()) {
        return Err(Error::new(400, json!({"code": "invalid_password_format"}), "Invalid Password Format".to_string()));
    }

    let user = get_by_id(token.sub.clone(), connection);

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

    if !user.verify_password(change_password_form.old_password.clone()) {
        return Err(Error::new(400, json!({"code": "invalid_old_password"}), "Invalid Old Password".to_string()));
    }

    user.password = Some(change_password_form.new_password);

    user.hash_password(config.password_hash_cost);

    let user = user.save(&connection);

    if user.is_err() {
        let err = user.err().unwrap();
        error!("{:?}", err);
        return Err(Error::from(err));
    }

    let user = user.unwrap();

    return Ok(user);
}
