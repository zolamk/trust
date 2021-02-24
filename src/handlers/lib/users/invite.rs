use crate::{
    config::Config,
    crypto::secure_token,
    handlers::Error,
    mailer::send_email,
    models::{
        user::{get_by_email, get_by_phone, NewUser, User},
        Error as ModelError,
    },
    sms::send_sms,
};
use chrono::Utc;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
    Connection, NotFound,
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, GraphQLInputObject)]
#[graphql(name = "invite_form")]
pub struct InviteForm {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

pub fn invite(config: &Config, connection: &PooledConnection<ConnectionManager<PgConnection>>, invite_form: InviteForm) -> Result<User, Error> {
    let internal_error = Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string());

    if invite_form.email.is_none() && invite_form.phone.is_none() {
        return Err(Error::new(409, json!({"code": "email_or_phone_required"}), "Invite Requires Email Or Phone Number".to_string()));
    }

    if invite_form.email.is_some() && invite_form.phone.is_some() {
        return Err(Error::new(
            409,
            json!({"code": "only_email_or_phone_for_invite"}),
            "Invitation Can Only Be Sent Using Either Email Or Phone".to_string(),
        ));
    }

    if invite_form.phone.is_some() && config.disable_phone {
        return Err(Error::new(
            409,
            json!({"code": "invitation_by_phone_but_phone_disabled"}),
            "You invited the user by phone but trust has phone support disabled".to_string(),
        ));
    }

    if invite_form.email.is_some() && config.disable_email {
        return Err(Error::new(
            409,
            json!({"code": "invitation_by_email_but_email_disabled"}),
            "You invited the user by email but trust has email support disabled".to_string(),
        ));
    }

    let user = NewUser {
        email: invite_form.email,
        phone: invite_form.phone,
        name: invite_form.name.clone(),
        ..Default::default()
    };

    if user.email.is_some() {
        match get_by_email(&user.email.clone().unwrap(), &connection) {
            Ok(user) => {
                return Err(Error::new(
                    409,
                    json!({"code": "email_registered", "email": user.email, "id": user.id, "password_set": user.password.is_some(), "phone": user.phone}),
                    "A user with this email address has already been registered".to_string(),
                ));
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

    if user.phone.is_some() {
        match get_by_phone(user.phone.clone().unwrap(), &connection) {
            Ok(user) => {
                return Err(Error::new(
                    409,
                    json!({"code": "phone_registered", "phone": user.phone, "id": user.id, "password_set": user.password.is_some(), "email": user.email}),
                    "A user with this phone number has already been registered".to_string(),
                ));
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

            error!("{:?}", err);

            return Err(internal_error);
        }

        let mut user = user.unwrap();

        if user.email.is_some() {
            user.email_invitation_token = Some(secure_token(100));

            user.invitation_token_sent_at = Some(Utc::now());

            let u = user.save(connection);

            if u.is_err() {
                let err = u.err().unwrap();
                error!("{:?}", err);
                return Err(Error::from(err));
            }

            user = u.unwrap();

            let template = &config.get_invitation_email_template();

            let to = &user.email.clone().unwrap();

            let subject = &config.get_invitation_email_subject();

            let data = json!({
                "invitation_token": user.email_invitation_token.clone().unwrap(),
                "email": user.email,
                "site_url": config.site_url
            });

            let email = send_email(template, data, to, subject, config);

            if email.is_err() {
                let err = email.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));
            }
        }

        if user.phone.is_some() {
            user.phone_invitation_token = Some(secure_token(12));

            user.invitation_token_sent_at = Some(Utc::now());

            let u = user.save(connection);

            if u.is_err() {
                let err = u.err().unwrap();
                error!("{:?}", err);
                return Err(Error::from(err));
            }

            user = u.unwrap();

            let template = config.clone().get_invitation_sms_template();

            let data = json!({
                "invitation_token": user.phone_invitation_token.clone().unwrap(),
                "phone": user.phone,
                "site_url": config.site_url
            });

            let sms = send_sms(template, data, user.phone.clone().unwrap(), &config);

            if sms.is_err() {
                let err = sms.err().unwrap();

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
