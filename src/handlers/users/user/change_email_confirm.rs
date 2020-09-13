use crate::{
    config::Config,
    handlers::{lib::user::change_email_confirm, Error},
    mailer::EmailTemplates,
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use log::error;
use rocket::{http::Status, response::status, State};
use rocket_contrib::json::{Json, JsonValue};

#[patch("/user/email/confirm", data = "<confirm_change_email_form>")]
pub fn change_email_confirm(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    email_templates: State<EmailTemplates>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
    confirm_change_email_form: Json<change_email_confirm::ConfirmChangeEmailForm>,
) -> Result<status::Custom<JsonValue>, Error> {
    if operator_signature.is_err() {
        let err = operator_signature.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let operator_signature = operator_signature.unwrap();

    let internal_error = Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string());

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(err) => {
            error!("{:?}", err);
            return Err(internal_error);
        }
    };

    let user = change_email_confirm::change_email_confirm(config.inner(), &connection, email_templates.inner(), &operator_signature, confirm_change_email_form.into_inner());

    if user.is_err() {
        return Err(user.err().unwrap());
    }

    return Ok(status::Custom(
        Status::Ok,
        JsonValue(json!({
            "code": "success",
            "message": "email changed successfully",
        })),
    ));
}
