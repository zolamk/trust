use crate::{
    config::Config,
    crypto::secure_token,
    handlers::Error,
    mailer::{send_email, EmailTemplates},
    models::user::get_by_email,
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
};
use chrono::Utc;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
    Connection,
};
use log::error;
use rocket::{http::Status, response::status, State};
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ResetForm {
    pub email: String,
}

#[post("/reset", data = "<reset_form>")]
pub fn reset(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    email_templates: State<EmailTemplates>,
    reset_form: Json<ResetForm>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
) -> Result<status::Custom<JsonValue>, Error> {
    if operator_signature.is_err() {
        let err = operator_signature.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let internal_error = Error {
        code: 500,
        body: json!({
            "code": "internal_error"
        }),
    };

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return Err(internal_error);
        }
    };

    let user = get_by_email(reset_form.email.clone(), &connection);

    if user.is_err() {
        let err = user.err().unwrap();
        error!("{:?}", err);
        return Ok(status::Custom(Status::Accepted, JsonValue(json!({"code": "accepted"}))));
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

        let email = send_email(template, data, user.email, config.inner());

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

    return Ok(status::Custom(Status::Accepted, JsonValue(json!({"code": "accepted"}))));
}
