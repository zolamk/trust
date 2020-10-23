use crate::{config::Config, crypto::secure_token, handlers::Error, mailer::send_email, models::user::get_by_email_or_phone_number, sms::send_sms};
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
    pub username: String,
}

pub fn reset(config: &Config, connection: &PooledConnection<ConnectionManager<PgConnection>>, reset_form: ResetForm) -> Result<(), Error> {
    if !config.email_rule.is_match(&reset_form.username) && !config.phone_rule.is_match(&reset_form.username) {
        return Err(Error::new(
            422,
            json!({"code": "email_or_phone_number_required"}),
            "Email or Phone Number Is Required To Recover Account".to_string(),
        ));
    }

    let transaction = connection.transaction::<_, Error, _>(|| {
        let user = get_by_email_or_phone_number(reset_form.username.clone(), reset_form.username.clone(), connection);

        if user.is_err() {
            let err = user.err().unwrap();
            error!("{:?}", err);
            return Ok(());
        }

        let mut user = user.unwrap();

        user.recovery_token_sent_at = Some(Utc::now().naive_utc());

        if config.email_rule.is_match(&reset_form.username) && user.email.is_some() && user.email_confirmed {
            if !user.email_confirmed {
                return Ok(());
            }

            let template = config.clone().get_recovery_email_template();

            user.recovery_token = Some(secure_token(100));

            let user = user.save(&connection);

            if user.is_err() {
                let err = user.err().unwrap();
                error!("{:?}", err);
                return Err(Error::from(err));
            }

            let user = user.unwrap();

            let data = json!({
                "recovery_token": user.recovery_token.clone().unwrap(),
                "site_url": config.site_url,
                "email": user.email
            });

            let email = send_email(template, data, user.email.unwrap(), config);

            if email.is_err() {
                let err = email.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));
            }

            return Ok(());
        } else if config.phone_rule.is_match(&reset_form.username) && user.phone_number.is_some() && user.phone_confirmed {
            user.recovery_token = Some(secure_token(6));

            let user = user.save(&connection);

            if user.is_err() {
                let err = user.err().unwrap();
                error!("{:?}", err);
                return Err(Error::from(err));
            }

            let user = user.unwrap();

            let template = config.clone().get_recovery_sms_template();

            let data = json!({
                "recovery_token": user.recovery_token.clone().unwrap(),
                "site_url": config.site_url,
                "phone_number": user.phone_number
            });

            let sms = send_sms(template, data, user.phone_number.unwrap(), config);

            if sms.is_err() {
                let err = sms.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));
            }

            return Ok(());
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
