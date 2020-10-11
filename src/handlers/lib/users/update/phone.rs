use crate::{
    config::Config,
    crypto::{jwt::JWT, secure_token},
    diesel::Connection,
    handlers::Error,
    models::user::User,
    sms::{send_sms, SMSTemplates},
};
use chrono::Utc;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct UpdateForm {
    pub phone_number: String,
    pub confirm: Option<bool>,
}

pub fn update_phone(
    config: &Config,
    connection: &PooledConnection<ConnectionManager<PgConnection>>,
    sms_templates: &SMSTemplates,
    token: &JWT,
    update_form: UpdateForm,
    id: String,
) -> Result<User, Error> {
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
        user.phone_number = Some(update_form.phone_number);

        let user = user.save(connection);

        if user.is_err() {
            let err = user.err().unwrap();

            error!("{:?}", err);

            return Err(Error::from(err));
        }

        return Ok(user.unwrap());
    }

    let transaction = connection.transaction::<User, Error, _>(|| {
        user.new_phone_number = Some(update_form.phone_number.clone());

        user.phone_number_change_token = Some(secure_token(6));

        user.phone_number_change_token_sent_at = Some(Utc::now().naive_utc());

        let user = user.save(&connection);

        if user.is_err() {
            let err = user.err().unwrap();

            error!("{:?}", err);

            return Err(Error::from(err));
        }

        let user = user.unwrap();

        let template = sms_templates.clone().confirmation_sms_template();

        let data = json!({
            "confirmation_code": user.phone_number_change_token,
            "phone_number": user.phone_number,
            "new_phone_number": user.new_phone_number,
            "site_url": config.site_url
        });

        let sms = send_sms(template, data, user.new_phone_number.clone().unwrap(), config);

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
