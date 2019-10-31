extern crate diesel;
extern crate handlebars;
extern crate lettre;
extern crate lettre_email;
extern crate native_tls;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

use crate::diesel::Connection;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::{DatabaseError, NotFound};
use rocket::http::Status;

use crate::config::Config;
use crate::crypto::jwt::JWT;
use crate::crypto::secure_token;
use crate::error::Error;
use crate::mailer::EmailTemplates;
use crate::models::user::{NewUser, User};
use chrono::Utc;
use handlebars::Handlebars;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::{ConnectionReuseParameters, SmtpClient};
use lettre::{ClientSecurity, ClientTlsParameters, Transport};
use lettre_email::Email;
use log::{error, info};
use native_tls::TlsConnector;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct InviteForm {
    pub name: Option<String>,
    pub email: String,
}

#[post("/invite", data = "<invite_form>")]
pub fn invite(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    email_templates: State<EmailTemplates>,
    invite_form: Json<InviteForm>,
    token: Result<JWT, Error>,
) -> Result<status::Custom<JsonValue>, Error> {
    if token.is_err() {
        let err = token.err().unwrap();
        return Err(err);
    }

    let token = token.unwrap();

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

    if !token.is_admin(&connection) {
        return Err(Error {
            code: 403,
            body: json!({
                "code": "only_admin_can_invite"
            }),
        });
    }

    let mut user = NewUser::default();

    user.confirmed = config.auto_confirm;

    user.name = invite_form.name.clone();

    user.email = invite_form.email.clone();

    user.aud = config.aud.clone();

    let conflict_error = Error {
        code: 409,
        body: json!({
            "code": "email_registered",
            "message": "a user with this email has already been registered",
        }),
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

    let token = secure_token(100);

    let sent_at = Utc::now().naive_utc();

    user.confirmation_token = Some(token);

    user.confirmation_sent_at = Some(sent_at);

    user.invitation_sent_at = Some(sent_at);

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

        let user = user.unwrap();

        let invitation_url = format!(
            "{}/invitation_token={}",
            config.site_url,
            user.confirmation_token.clone().unwrap(),
        );

        let template = email_templates.clone().invitation_email_template();

        let email = send_invitation_email(template, invitation_url, user, config);

        if email.is_err() {
            let err = email.err().unwrap();

            error!("{:?}", err);

            return Err(err);
        }

        return Ok(());
    });

    if transaction.is_ok() {
        let body = json!({
            "code": "success",
            "message": "user has been successfully invited"
        });

        return Ok(status::Custom(Status::Ok, JsonValue(body)));
    }

    let err = transaction.err().unwrap();

    return Err(err);
}

fn send_invitation_email(
    template: String,
    invitation_url: String,
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

    let invitation_email = Handlebars::new().render_template(
        &template,
        &json!({
            "email": user.email.clone(),
            "site_url": config.site_url.clone(),
            "invitation_url": invitation_url,
        }),
    );

    if invitation_email.is_err() {
        let err = invitation_email.err().unwrap();

        error!("{:?}", err);

        return Err(Error {
            code: 500,
            body: json!({
                "code": "invitation_email_template_render_error"
            }),
        });
    }

    let email = Email::builder()
        .from(config.smtp_admin_email.clone())
        .to(user.email)
        .html(invitation_email.unwrap())
        .build();

    if email.is_err() {
        let err = email.err().unwrap();
        error!("{:?}", err);
        return Err(Error {
            code: 500,
            body: json!({
                "code": "invitation_email_build_error"
            }),
        });
    }

    let email = mailer.send(email.unwrap().into());

    if email.is_err() {
        let err = email.err().unwrap();
        error!("{:?}", err);
        return Err(Error {
            code: 500,
            body: json!({
                "code": "invitation_email_send_error"
            }),
        });
    }

    return Ok(());
}
