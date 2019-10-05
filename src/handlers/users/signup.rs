extern crate diesel;
extern crate handlebars;
extern crate lettre;
extern crate lettre_email;
extern crate native_tls;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

use crate::diesel::Connection;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::{DatabaseError, NotFound};
use rocket::http::Status;

use crate::config::Config;
use crate::crypto::secure_token;
use crate::error::Error;
use crate::hook::{HookEvent, Webhook};
use crate::mailer::EmailTemplates;
use crate::models::operator_signature::OperatorSignature;
use crate::models::user::{NewUser, User};
use chrono::Utc;
use handlebars::Handlebars;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::{ConnectionReuseParameters, SmtpClient};
use lettre::{ClientSecurity, ClientTlsParameters, Transport};
use lettre_email::Email;
use log::error;
use native_tls::TlsConnector;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SignUpForm {
    pub name: Option<String>,
    pub email: String,
    pub avatar: Option<String>,
    pub password: String,
}

#[post("/signup", data = "<signup_form>")]
pub fn signup(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    email_templates: State<EmailTemplates>,
    signup_form: Json<SignUpForm>,
    operator_signature: OperatorSignature,
) -> status::Custom<JsonValue> {
    let mut user = NewUser::default();

    user.avatar = signup_form.avatar.clone();

    user.confirmed = config.auto_confirm;

    user.name = signup_form.name.clone();

    user.email = signup_form.email.clone();

    user.password = Some(signup_form.password.clone());

    user.aud = config.aud.to_string();

    user.hash_password();

    let internal_error = status::Custom(
        Status::InternalServerError,
        json!({
            "code": "internal_error"
        }),
    );

    let conflict_error = status::Custom(
        Status::Conflict,
        json!({
            "code": "email_registered",
            "message": "a user with this email has already been registered",
        }),
    );

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return internal_error;
        }
    };

    // check if user already exists
    match crate::models::user::get_by_email(user.email.clone(), &connection) {
        Ok(u) => {
            if u.confirmed {
                return conflict_error;
            }

            // if user is not confirmed delete the unconfirmed user
            let result = u.delete(&connection);

            if result.is_err() {
                error!("{}", result.err().unwrap());
                return internal_error;
            }
        }
        Err(err) => match err {
            NotFound => {}
            _ => {
                error!("{}", err);
                return internal_error;
            }
        },
    }

    if !config.auto_confirm {
        user.confirmation_token = Some(secure_token(100));
        user.confirmation_sent_at = Some(Utc::now().naive_utc());
    }

    let transaction = connection.transaction::<_, Error, _>(|| {
        let user = user.save(&connection);

        if user.is_err() {
            return Err(Error::DieselError(user.err().unwrap()));
        }

        let user = user.unwrap();

        use serde_json::json;

        let payload = json!({
            "event": HookEvent::Signup,
            "user": user,
        });

        let hook = Webhook::new(
            HookEvent::Signup,
            payload,
            config.inner().clone(),
            operator_signature,
        );

        let hook = hook.trigger();

        if hook.is_err() {
            return Err(Error::HookError(hook.err().unwrap()));
        }

        if !config.auto_confirm {
            let confirmation_url = format!(
                "{}/confirm?confirmation_token={}",
                config.instance_url,
                user.confirmation_token.clone().unwrap(),
            );

            let template = email_templates.clone().confirmation_email_template();

            let email = send_confirmation_email(template, confirmation_url, user, config);

            if email.is_err() {
                return Err(email.err().unwrap());
            }
        }

        return Ok(());
    });

    if transaction.is_ok() {
        return status::Custom(
            Status::Ok,
            json!({
                "code": "success",
                "message": "user has been successfully signed up"
            }),
        );
    }

    let err = transaction.err().unwrap();

    match err {
        Error::DieselError(err) => match err {
            DatabaseError(kind, _info) => match kind {
                DatabaseErrorKind::UniqueViolation => {
                    return status::Custom(
                        Status::Conflict,
                        json!({
                            "code": "email_registered",
                            "message": "a user with this email has already been registered",
                        }),
                    )
                }
                _ => {
                    println!("{:?}", kind);
                    return internal_error;
                }
            },
            _ => {
                println!("{}", err);
                return internal_error;
            }
        },

        Error::HookError(err) => {
            if err.status.code >= 500 {
                return status::Custom(
                    Status::UnprocessableEntity,
                    json!({
                        "code": "hook_error",
                        "message": "error handling webhook"
                    }),
                );
            } else {
                return status::Custom(err.status, JsonValue(err.body));
            }
        }

        _ => return internal_error,
    }
}

fn send_confirmation_email(
    template: String,
    confirmation_url: String,
    user: User,
    config: State<Config>,
) -> Result<(), Error> {
    let tls_connector = TlsConnector::builder().build().unwrap();

    let tls_parameters = ClientTlsParameters::new(config.smtp_host.to_string(), tls_connector);

    let credentials = Credentials::new(
        config.smtp_username.to_string(),
        config.smtp_password.to_string(),
    );

    let mut mailer = SmtpClient::new(
        (&config.smtp_host[..], config.smtp_port),
        ClientSecurity::Required(tls_parameters),
    )
    .unwrap()
    .authentication_mechanism(Mechanism::Login)
    .credentials(credentials)
    .timeout(Some(std::time::Duration::new(10, 0)))
    .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
    .transport();

    let confirmation_email = Handlebars::new().render_template(
        &template,
        &json!({
            "email": user.email.clone(),
            "site_url": config.site_url.clone(),
            "confirmation_url": confirmation_url,
        }),
    );

    if confirmation_email.is_err() {
        let err = confirmation_email.err().unwrap();
        return Err(Error::TemplateError(err));
    }

    let email = Email::builder()
        .from(config.smtp_admin_email.clone())
        .to(user.email)
        .html(confirmation_email.unwrap())
        .build();

    if email.is_err() {
        let err = email.err().unwrap();
        error!("{}", err);
        return Err(Error::EmailError(err));
    }

    let email = mailer.send(email.unwrap().into());

    if email.is_err() {
        let err = email.err().unwrap();
        error!("{}", err);
        return Err(Error::SMTPError(err));
    }

    return Ok(());
}
