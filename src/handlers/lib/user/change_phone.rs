use crate::{
    config::Config,
    crypto::{jwt::JWT, secure_token},
    handlers::Error,
    models::{user::User, Error as ModelError},
    sms::send_sms,
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
pub struct ChangePhoneForm {
    pub phone_number: String,
}

pub fn change_phone(config: &Config, connection: &PooledConnection<ConnectionManager<PgConnection>>, token: &JWT, change_phone_form: ChangePhoneForm) -> Result<User, Error> {
    let conflict_error = Error::new(409, json!({"code": "phone_registered"}), "A user with this phone number has already been registered".to_string());

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

    match crate::models::user::get_by_phone_number(change_phone_form.phone_number.clone(), &connection) {
        Ok(mut user) => {
            if user.phone_confirmed {
                return Err(conflict_error);
            }

            // if the user has a phone number confirmed
            // even though the email is not confirmed
            // clear the accounts email otherwise
            // delete the account since neither the phone number or email have been confirmed
            let result = if user.email_confirmed {
                user.phone_number = None;
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
        user.new_phone_number = user.phone_number.clone(); // store the old phone number in new phone number in case we ever need to revert it

        user.phone_number = Some(change_phone_form.phone_number);

        let user = user.save(&connection);

        if user.is_err() {
            let err = user.err().unwrap();

            error!("{:?}", err);

            return Err(Error::from(err));
        }

        return Ok(user.unwrap());
    }

    user.new_phone_number = Some(change_phone_form.phone_number);

    user.phone_number_change_token = Some(secure_token(6));

    user.phone_number_change_token_sent_at = Some(Utc::now().naive_utc());

    let user = user.save(&connection);

    if user.is_err() {
        let err = user.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let user = user.unwrap();

    let template = config.clone().get_change_phone_sms_template();

    let data = json!({
        "phone_number_change_token": user.phone_number_change_token.clone().unwrap(),
        "phone_number": user.phone_number,
        "new_phone_number": user.new_phone_number,
        "site_url": config.site_url
    });

    let sms = send_sms(template, data, user.new_phone_number.clone().unwrap(), &config);

    if sms.is_err() {
        let err = sms.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    return Ok(user);
}
