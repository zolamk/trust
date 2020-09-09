use crate::{
    config::Config,
    crypto::secure_token,
    handlers::Error,
    hook::{HookEvent, Webhook},
    mailer::{send_email, EmailTemplates},
    models::{
        user::{NewUser, User},
        Error as ModelError,
    },
    operator_signature::OperatorSignature,
};
use chrono::Utc;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
    result::{DatabaseErrorKind, Error::DatabaseError},
    Connection, NotFound,
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct SignUpForm {
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub email: String,
    pub password: String,
}

pub fn signup(
    config: &Config,
    connection: &PooledConnection<ConnectionManager<PgConnection>>,
    operator_signature: OperatorSignature,
    email_templates: &EmailTemplates,
    signup_form: SignUpForm,
) -> Result<User, Error> {
    let internal_error = Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string());

    let conflict_error = Err(Error::new(
        409,
        json!({"code": "email_registered"}),
        "A user with this email address has already been registered".to_string(),
    ));

    if config.disable_signup {
        let err = Error::new(422, json!({"code": "signup_disabled"}), "Trust instance has signup disabled".to_string());
        return Err(err);
    }

    if !config.password_rule.is_match(signup_form.password.as_ref()) {
        return Err(Error::new(400, json!({"code": "invalid_password_format"}), "Invalid Password Format".to_string()));
    }

    let mut user = NewUser::default();

    user.email = signup_form.email.clone();

    user.password = Some(signup_form.password.clone());

    user.name = signup_form.name.clone();

    user.avatar = signup_form.avatar;

    user.confirmed = config.auto_confirm;

    user.confirmation_token = if config.auto_confirm { None } else { Some(secure_token(100)) };

    user.confirmation_token_sent_at = if config.auto_confirm { None } else { Some(Utc::now().naive_utc()) };

    user.hash_password();

    // if users exists and is confirmed return conflict error
    // if not delete the unconfirmed user
    // if the error is user not found proceed with the normal flow
    match crate::models::user::get_by_email(user.email.clone(), &connection) {
        Ok(user) => {
            if user.confirmed {
                return conflict_error;
            }

            let result = user.delete(&connection);

            if result.is_err() {
                let err = result.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));
            }
        }
        Err(err) => match err {
            ModelError::DatabaseError(NotFound) => {}
            _ => {
                error!("{:?}", err);

                return Err(Error::from(err));
            }
        },
    }

    let transaction = connection.transaction::<User, Error, _>(|| {
        let user = user.save(&connection);

        if user.is_err() {
            let err = user.err().unwrap();

            if let ModelError::DatabaseError(DatabaseError(DatabaseErrorKind::UniqueViolation, _info)) = err {
                let err = Error::new(409, json!({"code": "email_already_registered"}), "A User With This Email Has Already Been Registered".to_string());
                return Err(err);
            }

            error!("{:?}", err);

            return Err(internal_error);
        }

        let user = user.unwrap();

        let payload = json!({
            "event": HookEvent::Signup,
            "provider": "email",
            "user": user,
        });

        let hook = Webhook::new(HookEvent::Signup, payload, config.clone(), operator_signature);

        let hook_response = hook.trigger();

        if hook_response.is_err() {
            return Err(Error::from(hook_response.err().unwrap()));
        }

        if !config.auto_confirm {
            let template = email_templates.clone().confirmation_email_template();

            let data = json!({
                "confirmation_url": format!("{}/confirm?confirmation_token={}", config.site_url, user.confirmation_token.clone().unwrap()),
                "email": user.email,
                "site_url": config.site_url
            });

            let email = send_email(template, data, user.email.clone(), &config);

            if email.is_err() {
                let err = email.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));
            }
        }

        return Ok(user);
    });

    if transaction.is_err() {
        let err = transaction.err().unwrap();

        return Err(err);
    }

    return Ok(transaction.unwrap());
}
