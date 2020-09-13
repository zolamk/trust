use crate::{
    config::Config,
    crypto::jwt::JWT,
    handlers::Error,
    mailer::EmailTemplates,
    models::{user::User, Error as ModelError},
    operator_signature::OperatorSignature,
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
    result::Error::NotFound,
};
use log::error;

pub fn get(
    _config: &Config,
    connection: &PooledConnection<ConnectionManager<PgConnection>>,
    _email_templates: &EmailTemplates,
    _operator_signature: &OperatorSignature,
    token: &JWT,
) -> Result<User, Error> {
    let internal_error = Err(Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string()));

    let user = crate::models::user::get_by_id(token.sub.clone(), &connection);

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
