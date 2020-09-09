use crate::{
    config::Config,
    crypto::{jwt::JWT, secure_token, Error as CryptoError},
    diesel::Connection,
    handlers::Error,
    mailer::{send_email, EmailTemplates},
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
};
use chrono::Utc;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use log::error;
use rocket::{http::Status, response::status, State};
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UpdateForm {
    pub email: String,
    pub confirm: Option<bool>,
}

#[patch("/users/<id>/email", data = "<update_form>")]
pub fn update_email(
    config: State<Config>,
    email_templates: State<EmailTemplates>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    token: Result<JWT, CryptoError>,
    update_form: Json<UpdateForm>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
    id: String,
) -> Result<status::Custom<JsonValue>, Error> {
    if operator_signature.is_err() {
        let err = operator_signature.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    if token.is_err() {
        let err = token.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let token = token.unwrap();

    let internal_error = Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string());

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return Err(internal_error);
        }
    };

    if !token.is_admin(&connection) {
        return Err(Error::new(403, json!({"code": "only_admin_can_update"}), "Only Admin Can Update Users".to_string()));
    }

    let user = crate::models::user::get_by_id(id, &connection);

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
        user.new_email = Some(user.email.clone()); // store the old email in new email in case we ever need to revert it

        user.email = update_form.email.clone();

        let user = user.save(&connection);

        if user.is_err() {
            let err = user.err().unwrap();

            error!("{:?}", err);

            return Err(Error::from(err));
        }

        return Ok(status::Custom(
            Status::Ok,
            JsonValue(json!({
                "code": "success",
                "confirmation_required": false,
                "message": "email changed successfully",
            })),
        ));
    }

    let transaction = connection.transaction::<_, Error, _>(|| {
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

        let template = email_templates.clone().confirmation_email_template();

        let data = json!({
            "confirmation_url": format!("{}/email_change_token={}", config.site_url, user.email_change_token.clone().unwrap()),
            "email": user.email,
            "new_email": user.new_email,
            "site_url": config.site_url
        });

        let email = send_email(template, data, user.new_email.unwrap(), &config);

        if email.is_err() {
            let err = email.err().unwrap();

            error!("{:?}", err);

            return Err(Error::from(err));
        }

        return Ok(());
    });

    if transaction.is_err() {
        return Err(transaction.err().unwrap());
    }

    return Ok(status::Custom(
        Status::Ok,
        JsonValue(json!({
            "code": "success",
            "confirmation_required": true,
            "message": "please confirm the email address",
        })),
    ));
}
