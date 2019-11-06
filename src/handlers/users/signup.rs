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
    operator_signature: Result<OperatorSignature, Error>,
) -> Result<status::Custom<JsonValue>, Error> {
    if config.disable_signup {
        let err = Error {
            code: 422,
            body: json!({
                "code": "signup_disabled",
                "message": "trust instance has signup disabled"
            }),
        };
        return Err(err);
    }

    if operator_signature.is_err() {
        let err = operator_signature.err().unwrap();
        return Err(err);
    }

    let operator_signature = operator_signature.unwrap();

    let mut user = NewUser::default();

    // user.avatar = signup_form.avatar.clone();

    user.confirmed = config.auto_confirm;

    // user.name = signup_form.name.clone();

    user.email = signup_form.email.clone();

    user.password = Some(signup_form.password.clone());

    user.aud = config.aud.to_string();

    user.hash_password();

    let internal_error = Error {
        code: 500,
        body: json!({
            "code": "internal_error"
        }),
    };

    let conflict_error = Error {
        code: 409,
        body: json!({
            "code": "email_registered",
            "message": "a user with this email has already been registered",
        }),
    };

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return Err(internal_error);
        }
    };

    // check if user already exists
    match crate::models::user::get_by_email(user.email.clone(), &connection) {
        Ok(u) => {
            if u.confirmed {
                return Err(conflict_error);
            }

            // if user is not confirmed delete the unconfirmed user
            let result = u.delete(&connection);

            if result.is_err() {
                error!("{}", result.err().unwrap());
                return Err(internal_error);
            }
        }
        Err(err) => match err {
            NotFound => {}
            _ => {
                error!("{}", err);
                return Err(internal_error);
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
            let err = user.err().unwrap();

            match err {
                DatabaseError(kind, _info) => match kind {
                    DatabaseErrorKind::UniqueViolation => {
                        let err = Error {
                            code: 409,
                            body: json!({
                                "code": "email_already_registered"
                            }),
                        };
                        return Err(err);
                    }
                    _ => {
                        println!("{:?}", kind);
                        return Err(internal_error);
                    }
                },
                _ => {
                    println!("{}", err);
                    return Err(internal_error);
                }
            }
        }

        let mut user = user.unwrap();

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
            return Err(hook.err().unwrap());
        }

        let hook_response = hook.unwrap();

        if let Some(hook_response) = hook_response {
            if hook_response.is_object() {
                let hook_response = hook_response.as_object().unwrap();

                let update = if hook_response.contains_key("app_metadata") {
                    let app_metdata = hook_response.get("app_metadata").unwrap().clone();

                    user.app_metadata = Some(app_metdata);

                    true
                } else if hook_response.contains_key("user_metadata") {
                    let user_metadata = hook_response.get("user_metadata").unwrap().clone();

                    user.user_metadata = Some(user_metadata);

                    true
                } else {
                    false
                };

                if update {
                    let res = user.save(&connection);

                    if res.is_err() {
                        let err = res.err().unwrap();

                        error!("{:?}", err);

                        return Err(internal_error);
                    }

                    user = res.unwrap();
                }
            }
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
                let err = email.err().unwrap();

                error!("{:?}", err);

                return Err(err);
            }
        }

        return Ok(());
    });

    if transaction.is_ok() {
        let body = json!({
            "code": "success",
            "message": "user has been successfully signed up"
        });

        return Ok(status::Custom(Status::Ok, JsonValue(body)));
    }

    let err = transaction.err().unwrap();

    return Err(err);
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

        error!("{}", err);

        return Err(Error {
            code: 500,
            body: json!({
                "code": "confirmation_email_template_render_error"
            }),
        });
    }

    let email = Email::builder()
        .from(config.smtp_admin_email.clone())
        .to(user.email)
        .html(confirmation_email.unwrap())
        .build();

    if email.is_err() {
        let err = email.err().unwrap();
        error!("{}", err);
        return Err(Error {
            code: 500,
            body: json!({
                "code": "confirmation_email_build_error"
            }),
        });
    }

    let email = mailer.send(email.unwrap().into());

    if email.is_err() {
        let err = email.err().unwrap();
        error!("{}", err);
        return Err(Error {
            code: 500,
            body: json!({
                "code": "confirmation_email_send_error"
            }),
        });
    }

    return Ok(());
}
