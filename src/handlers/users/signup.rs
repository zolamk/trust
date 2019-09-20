extern crate diesel;
extern crate handlebars;
extern crate lettre;
extern crate lettre_email;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

use crate::diesel::Connection;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::{DatabaseError, NotFound};
use rocket::http::Status;

use crate::mailer::EmailTemplates;
use chrono::Utc;
use diesel::result::Error;
use handlebars::Handlebars;
use lettre::smtp::authentication::Credentials;
use lettre::SmtpClient;
use lettre::SmtpTransport;
use lettre::Transport;
use lettre_email::Email;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

use crate::config::Config;
use crate::crypto::secure_token;
use crate::models::user::NewUser;

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
) -> status::Custom<JsonValue> {
    let mut user = NewUser::default();

    user.avatar = signup_form.avatar.clone();
    user.confirmed = config.auto_confirm;
    user.name = signup_form.name.clone();
    user.email = signup_form.email.clone();
    user.password = Some(signup_form.password.clone());
    user.aud = config.aud.clone();

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
    match crate::models::get_user_by_email(user.email.clone(), &connection) {
        Ok(u) => {
            if u.confirmed {
                return conflict_error;
            }
        }
        Err(err) => match err {
            NotFound => {}
            _ => {
                println!("{}", err);
                return internal_error;
            }
        },
    }

    if !config.auto_confirm {
        user.confirmation_token = Some(secure_token(60));
        user.confirmation_sent_at = Some(Utc::now().naive_utc());
    }

    let transaction = connection.transaction::<_, Error, _>(|| {
        let result = user.save(&connection);

        if !result.is_ok() {
            return Err(result.err().unwrap());
        }

        if !config.auto_confirm {
            let confirmation_url = format!(
                "{}/confirmation_token={}",
                config.site_url,
                user.confirmation_token.clone().unwrap(),
            );

            let template = email_templates.clone().confirmation_email_template();

            let email = send_confirmation_email(template, confirmation_url, user, config);

            if !email.is_ok() {
                return Err(result.err().unwrap());
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
    }
}

fn send_confirmation_email(
    template: String,
    confirmation_url: String,
    user: NewUser,
    config: State<Config>,
) -> Result<(), Error> {
    let confirmation_email = Handlebars::new().render_template(
        &template,
        &json!({
            "email": user.email.clone(),
            "site_url": config.site_url.clone(),
            "confirmatio_url": confirmation_url,
        }),
    );

    let email = Email::builder()
        .from(config.smtp_admin_email.clone())
        .to(user.email)
        .text(confirmation_email.unwrap())
        .build();

    if !email.is_ok() {
        return Err(Error::RollbackTransaction);
    }

    let credentials = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());

    let mut mailer = SmtpClient::new_simple(&config.smtp_host.clone())
        .unwrap()
        .credentials(credentials)
        .transport();

    let email = mailer.send(email.unwrap().into());

    if !email.is_ok() {
        return Err(Error::RollbackTransaction);
    }

    return Ok(());
}
