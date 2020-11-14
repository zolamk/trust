use crate::{
    config::Config,
    crypto::secure_token,
    handlers::Error,
    hook::{HookEvent, Webhook},
    mailer::send_email,
    models::{
        user::{get_by_email, get_by_phone_number, NewUser, User},
        Error as ModelError,
    },
    operator_signature::OperatorSignature,
    sms::send_sms,
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
    pub email: Option<String>,
    #[graphql(name = "phone_number")]
    pub phone_number: Option<String>,
    pub password: String,
}

pub fn signup(config: &Config, connection: &PooledConnection<ConnectionManager<PgConnection>>, operator_signature: OperatorSignature, signup_form: SignUpForm) -> Result<User, Error> {
    let internal_error = Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string());

    if config.disable_signup {
        let err = Error::new(422, json!({"code": "signup_disabled"}), "Trust instance has signup disabled".to_string());
        return Err(err);
    }

    if signup_form.email.is_none() && signup_form.phone_number.is_none() {
        return Err(Error::new(409, json!({"code": "email_or_phone_number_required"}), "Signup Requires Email Or Phone Number".to_string()));
    }

    if !config.password_rule.is_match(signup_form.password.as_ref()) {
        return Err(Error::new(400, json!({"code": "invalid_password_format"}), "Invalid Password Format".to_string()));
    }

    let mut user = NewUser {
        email: signup_form.email,
        phone_number: signup_form.phone_number,
        password: Some(signup_form.password),
        name: signup_form.name.clone(),
        avatar: signup_form.avatar,
        ..Default::default()
    };

    user.hash_password();

    if user.email.is_some() {
        // if the user is signing up with email and
        // if user exists and is confirmed return conflict error
        // if not delete the unconfirmed user and proceed with the normal flow
        // if the error is user not found proceed with the normal flow
        match get_by_email(user.email.clone().unwrap(), &connection) {
            Ok(mut user) => {
                if user.email_confirmed {
                    return Err(Error::new(
                        409,
                        json!({"code": "email_registered"}),
                        "A user with this email address has already been registered".to_string(),
                    ));
                }

                // if the user has a phone number confirmed
                // even though the email is not confirmed
                // clear the accounts email otherwise
                // delete the account since neither the phone number or email have been confirmed
                let result = if user.phone_confirmed {
                    user.email = None;

                    user.save(&connection)
                } else {
                    user.delete(&connection)
                };

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
    }

    if user.phone_number.is_some() && !config.disable_phone {
        // if the user is signing up with phone number and
        // if user exists and is confirmed return conflict error
        // if not delete the unconfirmed user and proceed with the normal flow
        // if the error is user not found proceed with the normal flow
        match get_by_phone_number(user.phone_number.clone().unwrap(), &connection) {
            Ok(mut user) => {
                if user.phone_confirmed {
                    return Err(Error::new(
                        409,
                        json!({"code": "phone_registered"}),
                        "A user with this phone number has already been registered".to_string(),
                    ));
                }

                let result = if user.email_confirmed {
                    user.phone_number = None;
                    println!("{:?}", user.phone_number);
                    user.save(&connection)
                } else {
                    user.delete(&connection)
                };

                if result.is_err() {
                    let err = result.err().unwrap();

                    error!("{:?}", err);

                    return Err(Error::from(err));
                }

                println!("{:?}", result.unwrap());
            }
            Err(err) => match err {
                ModelError::DatabaseError(NotFound) => {}
                _ => {
                    error!("{:?}", err);

                    return Err(Error::from(err));
                }
            },
        }
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

        let mut user = user.unwrap();

        if user.email.is_some() && !config.disable_email {
            if config.auto_confirm {
                let u = user.confirm_email(connection);

                if u.is_err() {
                    let err = u.err().unwrap();
                    error!("{:?}", err);
                    return Err(Error::from(err));
                }

                user = u.unwrap();
            } else {
                user.email_confirmation_token = Some(secure_token(100));

                user.email_confirmation_token_sent_at = Some(Utc::now().naive_utc());

                let u = user.save(connection);

                if u.is_err() {
                    let err = u.err().unwrap();
                    error!("{:?}", err);
                    return Err(Error::from(err));
                }

                user = u.unwrap();

                let template = config.clone().get_confirmation_email_template();

                let data = json!({
                    "confirmation_token": user.email_confirmation_token.clone().unwrap(),
                    "email": user.email,
                    "site_url": config.site_url
                });

                let email = send_email(template, data, user.email.clone().unwrap(), config.clone().get_confirmation_email_subject(), config);

                if email.is_err() {
                    let err = email.err().unwrap();

                    error!("{:?}", err);

                    return Err(Error::from(err));
                }
            }
        }

        if user.phone_number.is_some() && !config.disable_phone {
            if config.auto_confirm {
                let u = user.confirm_phone(connection);

                if u.is_err() {
                    let err = u.err().unwrap();
                    error!("{:?}", err);
                    return Err(Error::from(err));
                }

                user = u.unwrap();
            } else {
                user.phone_confirmation_token = Some(secure_token(6));

                user.phone_confirmation_token_sent_at = Some(Utc::now().naive_utc());

                let u = user.save(connection);

                if u.is_err() {
                    let err = u.err().unwrap();
                    error!("{:?}", err);
                    return Err(Error::from(err));
                }

                user = u.unwrap();

                let template = config.clone().get_confirmation_sms_template();

                let data = json!({
                    "confirmation_token": user.phone_confirmation_token.clone().unwrap(),
                    "phone_number": user.phone_number,
                    "site_url": config.site_url
                });

                let sms = send_sms(template, data, user.phone_number.clone().unwrap(), &config);

                if sms.is_err() {
                    let err = sms.err().unwrap();

                    error!("{:?}", err);

                    return Err(Error::from(err));
                }
            }
        }

        // trigger signup hook only if the user has confirmed
        if user.phone_confirmed || user.email_confirmed {
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
        }

        return Ok(user);
    });

    if transaction.is_err() {
        let err = transaction.err().unwrap();

        return Err(err);
    }

    return Ok(transaction.unwrap());
}
