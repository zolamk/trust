use crate::{
    config::Config,
    handlers::Error,
    models::{
        user::{get_by_invitation_token, User},
        Error as ModelError,
    },
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
    result::Error::NotFound,
    Connection,
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct AcceptForm {
    #[graphql(name = "invitation_token")]
    pub invitation_token: String,
    pub password: String,
}

pub fn accept_invite(config: &Config, connection: &PooledConnection<ConnectionManager<PgConnection>>, accept_form: AcceptForm) -> Result<User, Error> {
    let internal_error = Error::new(500, json!({"code": "internal_error"}), "Internal Server Error".to_string());

    if !config.password_rule.is_match(accept_form.password.as_ref()) {
        return Err(Error::new(400, json!({"code": "invalid_password_format"}), "Invalid Password Format".to_string()));
    }

    let user = get_by_invitation_token(accept_form.invitation_token.clone(), connection);

    if user.is_err() {
        match user.err().unwrap() {
            ModelError::DatabaseError(NotFound) => return Err(Error::new(404, json!({"code": "user_not_found"}), "User Not Found".to_string())),
            err => {
                error!("{:?}", err);
                return Err(internal_error);
            }
        }
    }

    let transaction = connection.transaction::<User, Error, _>(|| {
        let mut user = user.unwrap();

        if user.phone_invitation_token.is_some() {
            let user = user.confirm_phone(connection);

            if user.is_err() {
                let err = user.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));
            }
        } else if user.email_invitation_token.is_some() {
            let user = user.confirm_email(connection);

            if user.is_err() {
                let err = user.err().unwrap();

                error!("{:?}", err);

                return Err(Error::from(err));
            }
        }

        let user = user.accept_invitation(connection);

        if user.is_err() {
            error!("{:?}", user.err().unwrap());

            return Err(internal_error);
        }

        let mut user = user.unwrap();

        user.password = Some(accept_form.password);

        user.hash_password(config.password_hash_cost);

        let user = user.save(connection);

        if user.is_err() {
            let err = user.err().unwrap();

            error!("{:?}", err);

            return Err(Error::from(err));
        }

        return Ok(user.unwrap());
    });

    if transaction.is_err() {
        return Err(transaction.err().unwrap());
    }

    return Ok(transaction.unwrap());
}
