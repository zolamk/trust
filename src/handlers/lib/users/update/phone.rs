use crate::{
    config::Config,
    crypto::{jwt::JWT, secure_token},
    diesel::Connection,
    handlers::Error,
    models::user::User,
    sms::send_sms,
};
use chrono::Utc;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct UpdatePhoneForm {
    pub phone: String,
    pub confirm: Option<bool>,
}

pub fn update_phone(config: &Config, connection: &PooledConnection<ConnectionManager<PgConnection>>, token: &JWT, update_form: UpdatePhoneForm, id: String) -> Result<User, Error> {
    if !token.is_admin(connection) {
        return Err(Error::new(403, json!({"code": "only_admin_can_update"}), "Only Admin Can Update Users".to_string()));
    }

    let user = crate::models::user::get_by_id(id, connection);

    if user.is_err() {
        let err = user.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let mut user = user.unwrap();

    if user.id == token.sub {
        return Err(Error::new(422, json!({"code": "admin_cant_update_self"}), "Admin Can't Update Self".to_string()));
    }

    if config.auto_confirm || update_form.confirm.is_some() && update_form.confirm.unwrap() {
        user.phone = Some(update_form.phone);

        let user = user.save(connection);

        if user.is_err() {
            let err = user.err().unwrap();

            error!("{:?}", err);

            return Err(Error::from(err));
        }

        return Ok(user.unwrap());
    }

    let transaction = connection.transaction::<User, Error, _>(|| {
        user.new_phone = Some(update_form.phone.clone());

        user.phone_change_token = Some(secure_token(6));

        user.phone_change_token_sent_at = Some(Utc::now());

        let user = user.save(&connection);

        if user.is_err() {
            let err = user.err().unwrap();

            error!("{:?}", err);

            return Err(Error::from(err));
        }

        let user = user.unwrap();

        let template = config.clone().get_change_phone_sms_template();

        let data = json!({
            "phone_change_token": user.phone_change_token,
            "phone": user.phone,
            "new_phone": user.new_phone,
            "site_url": config.site_url
        });

        let sms = send_sms(template, data, user.new_phone.clone().unwrap(), config);

        if sms.is_err() {
            let err = sms.err().unwrap();

            error!("{:?}", err);

            return Err(Error::from(err));
        }

        return Ok(user);
    });

    if transaction.is_err() {
        return Err(transaction.err().unwrap());
    }

    return Ok(transaction.unwrap());
}
