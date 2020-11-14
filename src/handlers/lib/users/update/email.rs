use crate::{
    config::Config,
    crypto::{jwt::JWT, secure_token},
    diesel::Connection,
    handlers::Error,
    mailer::send_email,
    models::user::User,
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
    pub email: String,
    pub confirm: Option<bool>,
}

pub fn update_email(config: &Config, connection: &PooledConnection<ConnectionManager<PgConnection>>, token: &JWT, update_form: UpdateForm, id: String) -> Result<User, Error> {
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
        user.new_email = user.email.clone(); // store the old email in new email in case we ever need to revert it

        user.email = Some(update_form.email);

        let user = user.save(connection);

        if user.is_err() {
            let err = user.err().unwrap();

            error!("{:?}", err);

            return Err(Error::from(err));
        }

        return Ok(user.unwrap());
    }

    let transaction = connection.transaction::<User, Error, _>(|| {
        user.new_email = Some(update_form.email.clone());

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

        let subject = &config.get_confirmation_email_subject();

        let data = json!({
            "change_email_token": user.email_change_token.clone().unwrap(),
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
    });

    if transaction.is_err() {
        return Err(transaction.err().unwrap());
    }

    return Ok(transaction.unwrap());
}
