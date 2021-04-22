use crate::{
    config::Config,
    crypto::secure_token,
    handlers::Error,
    models::{
        user::{get_by_email, User},
    },
    mailer::send_email
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
#[graphql(name = "resend_email_form")]
pub struct ResendEmailForm {
    pub email: String
}

pub fn resend_email(config: &Config, connection: &PooledConnection<ConnectionManager<PgConnection>>, resend_email_form: ResendEmailForm) -> Result<User, Error> {

    let too_many_requests = Error::new(429, json!({"code": "too_many_requests"}), "Too many confirmation code requests! please try again later".to_string());

    let transaction = connection.transaction::<User, Error, _>(|| {

        let user = get_by_email(&resend_email_form.email, connection);
            
        if user.is_err() {

            let err = user.err().unwrap();

            error!("{:?}", err);

            return Err(Error::from(err));

        }

        let mut user = user.unwrap();

        if !(config.disable_email || user.email.is_none() || user.email_confirmed) {

            if user.email_confirmation_token_sent_at.is_some() {

                let now = Utc::now();

                let last_sent_at = user.email_confirmation_token_sent_at.unwrap();

                let minutes = now.signed_duration_since(last_sent_at).num_minutes();

                if minutes < config.minutes_between_resend {

                    return Err(too_many_requests);

                }

            }

            user.email_confirmation_token = Some(secure_token(100));

            user.email_confirmation_token_sent_at = Some(Utc::now());

            let u = user.save(connection);

            if u.is_err() {

                let err = u.err().unwrap();
                
                error!("{:?}", err);

                return Err(Error::from(err));

            }

            user = u.unwrap();

            let template = &config.get_confirmation_email_template();

            let to = &user.email.clone().unwrap();

            let subject = &config.get_confirmation_email_subject();

            let data = json!({
                "confirmation_token": user.email_confirmation_token.clone().unwrap(),
                "email": user.email,
                "site_url": config.site_url
            });

            let email = send_email(template, data, to, subject, config);

            if email.is_err() {
                let err = email.err().unwrap();

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
