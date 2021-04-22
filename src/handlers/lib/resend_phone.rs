use crate::{
    config::Config,
    crypto::secure_token,
    handlers::Error,
    models::{
        user::{get_by_phone, User},
    },
    sms::send_sms,
};
use chrono::Utc;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
    Connection,
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, GraphQLInputObject)]
#[graphql(name = "resend_phone_form")]
pub struct ResendPhoneForm {
    pub phone: String
}

pub fn resend_phone(config: &Config, connection: &PooledConnection<ConnectionManager<PgConnection>>, resend_phone_form: ResendPhoneForm) -> Result<User, Error> {

    let too_many_requests = Error::new(429, json!({"code": "too_many_requests"}), "Too many confirmation code requests! please try again later".to_string());

    let transaction = connection.transaction::<User, Error, _>(|| {

        let user = get_by_phone(resend_phone_form.phone, connection);
            
        if user.is_err() {

            let err = user.err().unwrap();

            error!("{:?}", err);

            return Err(Error::from(err));

        }

        let mut user = user.unwrap();

        if !(config.disable_phone || user.phone.is_none() || user.phone_confirmed) {

            if user.phone_confirmation_token_sent_at.is_some() {

                let now = Utc::now();

                let last_sent_at = user.phone_confirmation_token_sent_at.unwrap();

                let minutes = now.signed_duration_since(last_sent_at).num_minutes();

                if minutes < config.minutes_between_resend {

                    return Err(too_many_requests);

                }

            }

            user.phone_confirmation_token = Some(secure_token(6));

            user.phone_confirmation_token_sent_at = Some(Utc::now());

            let u = user.save(connection);

            if u.is_err() {

                let err = u.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));

            }

            user = u.unwrap();

            let template = config.clone().get_confirmation_sms_template();

            let data = json!({
                "confirmation_token": user.phone_confirmation_token.clone().unwrap(),
                "phone": user.phone,
                "site_url": config.site_url
            });

            let sms = send_sms(template, data, user.phone.clone().unwrap(), &config);

            if sms.is_err() {
                let err = sms.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));
            }

        }

        return Ok(user)

    });

    if transaction.is_err() {

        let err = transaction.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err))

    }

    return Ok(transaction.unwrap())

}
