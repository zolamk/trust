use crate::{
    config::Config,
    handlers::Error,
    models::{
        user::{get_by_recovery_token, User},
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

#[derive(Deserialize, Serialize)]
pub struct ConfirmResetForm {
    pub recovery_token: String,
    pub new_password: String,
}

pub fn confirm_reset(config: &Config, connection: &PooledConnection<ConnectionManager<PgConnection>>, reset_form: ConfirmResetForm) -> Result<User, Error> {
    if !config.password_rule.is_match(reset_form.new_password.as_ref()) {
        return Err(Error::new(400, json!({"code": "invalid_password_format"}), "Invalid Password Format".to_string()));
    }

    let user = get_by_recovery_token(reset_form.recovery_token.clone(), connection);

    if user.is_err() {
        let err = user.err().unwrap();

        if let ModelError::DatabaseError(NotFound) = err {
            return Err(Error::new(404, json!({"code": "recovery_token_not_found"}), "Recovery Token Not Found".to_string()));
        }

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let mut user = user.unwrap();

    user.password = Some(reset_form.new_password);

    user.hash_password(config.password_hash_cost);

    let user = user.save(&connection);

    if user.is_err() {
        let err = user.err().unwrap();
        error!("{:?}", err);
        return Err(Error::from(err));
    }

    return Ok(user.unwrap());
}
