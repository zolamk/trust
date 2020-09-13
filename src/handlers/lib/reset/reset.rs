use crate::{
    config::Config,
    crypto::secure_token,
    handlers::Error,
    mailer::{send_email, EmailTemplates},
    models::user::get_by_email,
    operator_signature::OperatorSignature,
};
use chrono::Utc;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
    Connection,
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ResetForm {
    pub email: String,
}

pub fn reset(
    config: &Config,
    connection: &PooledConnection<ConnectionManager<PgConnection>>,
    email_templates: &EmailTemplates,
    _operator_signature: &OperatorSignature,
    reset_form: ResetForm,
) -> Result<(), Error> {
    let user = get_by_email(reset_form.email, &connection);

    if user.is_err() {
        let err = user.err().unwrap();
        error!("{:?}", err);
        return Ok(());
    }

    let mut user = user.unwrap();

    let template = email_templates.clone().recovery_email_template();

    let transaction = connection.transaction::<_, Error, _>(|| {
        user.recovery_token = Some(secure_token(100));

        user.recovery_token_sent_at = Some(Utc::now().naive_utc());

        let user = user.save(&connection);

        if user.is_err() {
            let err = user.err().unwrap();
            error!("{:?}", err);
            return Err(Error::from(err));
        }

        let user = user.unwrap();

        let recovery_url = format!("{}/recovery?recovery_token={}", config.site_url, user.recovery_token.clone().unwrap());

        let data = json!({
            "recovery_url": recovery_url,
            "site_url": config.site_url,
            "email": user.email
        });

        let email = send_email(template, data, user.email, config);

        if email.is_err() {
            let err = email.err().unwrap();

            error!("{:?}", err);

            return Err(Error::from(err));
        }

        return Ok(());
    });

    if transaction.is_err() {
        let err = transaction.err().unwrap();
        error!("{:?}", err);
        return Err(err);
    }

    return Ok(());
}
