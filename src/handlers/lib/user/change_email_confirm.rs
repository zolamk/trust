use crate::{
    config::Config,
    handlers::Error,
    mailer::EmailTemplates,
    models::{
        user::{get_by_email_change_token, User},
        Error as ModelError,
    },
    operator_signature::OperatorSignature,
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
    result::Error::NotFound,
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ConfirmChangeEmailForm {
    pub email_change_token: String,
}

pub fn change_email_confirm(
    _config: &Config,
    connection: &PooledConnection<ConnectionManager<PgConnection>>,
    _email_templates: &EmailTemplates,
    _operator_signature: &OperatorSignature,
    confirm_change_email_form: ConfirmChangeEmailForm,
) -> Result<User, Error> {
    let user = get_by_email_change_token(confirm_change_email_form.email_change_token, connection);

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

    let user = user.confirm_email_change(connection);

    if user.is_err() {
        let err = user.err().unwrap();
        error!("{:?}", err);
        return Err(Error::from(err));
    }

    return Ok(user.unwrap());
}