use crate::{
    config::Config,
    crypto::{jwt::JWT, secure_token},
    diesel::Connection,
    handlers::Error,
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
    result::{
        DatabaseErrorKind,
        Error::{DatabaseError, NotFound},
    },
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct CreateForm {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub confirm: bool,
}

pub fn create(
    config: &Config,
    connection: &PooledConnection<ConnectionManager<PgConnection>>,
    email_templates: &EmailTemplates,
    _operator_signature: &OperatorSignature,
    token: &JWT,
    create_form: CreateForm,
) -> Result<User, Error> {
    let internal_error = Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string());

    if !token.is_admin(&connection) {
        return Err(Error::new(403, json!({"code": "only_admin_can_create"}), "Only Admin Can Create Users".to_string()));
    }

    if !config.password_rule.is_match(create_form.password.as_ref()) {
        return Err(Error::new(400, json!({"code": "invalid_password_format"}), "Invalid Password Format".to_string()));
    }

    let conflict_error = Err(Error::new(
        409,
        json!({"code": "email_registered"}),
        "A user with this email address has already been registered".to_string(),
    ));

    // if users exists and is confirmed return conflict error
    // if not delete the unconfirmed user
    // if the error is user not found proceed with the normal flow
    match crate::models::user::get_by_email(create_form.email.clone(), &connection) {
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

    let mut user = NewUser::default();

    user.confirmed = config.auto_confirm || create_form.confirm;

    user.email = create_form.email.clone();

    user.name = create_form.name.clone();

    user.avatar = create_form.avatar.clone();

    user.password = Some(create_form.password);

    user.hash_password();

    if !user.confirmed {
        user.confirmation_token = Some(secure_token(100));

        user.confirmation_token_sent_at = Some(Utc::now().naive_utc());
    }

    let transaction = connection.transaction::<User, Error, _>(|| {
        let user = user.save(&connection);

        if user.is_err() {
            let err = user.err().unwrap();

            if let ModelError::DatabaseError(DatabaseError(DatabaseErrorKind::UniqueViolation, _info)) = err {
                let err = Error::new(
                    409,
                    json!({"code": "email_already_registered"}),
                    "A user with this email address has already been registered".to_string(),
                );
                return Err(err);
            }

            error!("{:?}", err);

            return Err(internal_error);
        }

        let user = user.unwrap();

        if !user.confirmed {
            let template = email_templates.clone().confirmation_email_template();

            let data = json!({
                "confirmation_url": format!("{}/confirmation_token={}", config.site_url, user.confirmation_token.clone().unwrap()),
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
        return Err(transaction.err().unwrap());
    }

    return Ok(transaction.unwrap());
}
