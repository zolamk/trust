use crate::{
    config::Config,
    crypto::{jwt::JWT, secure_token},
    handlers::Error,
    mailer::send_email,
    models::{
        user::{get_by_email, User},
        Error as ModelError,
    },
};
use chrono::Utc;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
    result::Error::NotFound,
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ChangeEmailFrom {
    pub email: String,
}

pub fn change_email(config: &Config, connection: &PooledConnection<ConnectionManager<PgConnection>>, token: &JWT, change_email_form: ChangeEmailFrom) -> Result<User, Error> {
    let conflict_error = Error::new(409, json!({"code": "email_registered"}), "A user with this email address has already been registered".to_string());

    let user = crate::models::user::get_by_id(token.sub.clone(), &connection);

    if user.is_err() {
        let err = user.err().unwrap();

        if let ModelError::DatabaseError(NotFound) = err {
            return Err(Error::new(422, json!({"code": "user_not_found"}), "User Not Found".to_string()));
        }

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let mut user = user.unwrap();

    match get_by_email(&change_email_form.email, &connection) {
        Ok(mut user) => {
            if user.email_confirmed {
                return Err(conflict_error);
            }

            // if the user has a phone number confirmed
            // even though the email is not confirmed
            // clear the accounts email otherwise
            // delete the account since neither the phone number or email have been confirmed
            let result = if user.phone_confirmed {
                user.email = None;

                user.save(&connection)
            } else {
                user.delete(&connection)
            };

            if result.is_err() {
                let err = result.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));
            }
        }
        Err(err) => match err {
            ModelError::DatabaseError(NotFound) => {}
            _ => {
                error!("{:?}", err);

                return Err(Error::from(err));
            }
        },
    }

    if config.auto_confirm {
        user.new_email = user.email.clone(); // store the old email in new email in case we ever need to revert it

        user.email = Some(change_email_form.email);

        let user = user.save(&connection);

        if user.is_err() {
            let err = user.err().unwrap();

            error!("{:?}", err);

            return Err(Error::from(err));
        }

        return Ok(user.unwrap());
    }

    user.new_email = Some(change_email_form.email);

    user.email_change_token = Some(secure_token(100));

    user.email_change_token_sent_at = Some(Utc::now().naive_utc());

    let user = user.save(&connection);

    if user.is_err() {
        let err = user.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let user = user.unwrap();

    let template = &config.get_change_email_template();

    let to = &user.new_email.unwrap();

    let subject = &config.get_change_email_subject();

    let data = json!({
        "change_email_token": user.email_change_token.unwrap(),
        "email": user.email,
        "new_email": user.new_email,
        "site_url": config.site_url
    });

    let email = send_email(template, data, to, subject, config);

    if email.is_err() {
        let err = email.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    return Ok(user);
}
