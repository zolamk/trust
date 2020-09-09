use crate::{
    config::Config,
    crypto::jwt::JWT,
    handlers::Error,
    hook::{HookEvent, Webhook},
    models::{refresh_token::NewRefreshToken, Error as ModelError},
    operator_signature::OperatorSignature,
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
    NotFound,
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, GraphQLObject)]
pub struct LoginResponse {
    #[graphql(name = "access_token")]
    pub access_token: String,
    #[graphql(name = "refresh_token")]
    pub refresh_token: String,
}

pub fn token(config: &Config, connection: &PooledConnection<ConnectionManager<PgConnection>>, operator_signature: OperatorSignature, form: LoginForm) -> Result<LoginResponse, Error> {
    let internal_error = Error::new(500, json!({"code": "internal_server_error"}), "Internal Server Error".to_string());

    let user_not_confirmed = Error::new(412, json!({"code": "user_not_confirmed"}), "User hasn't been confirmed".to_string());

    let invalid_email_or_password = Error::new(401, json!({"code": "invalid_email_or_password"}), "Invalid Email or Password".to_string());

    let user = crate::models::user::get_by_email(form.username, &connection);

    if user.is_err() {
        let err = user.err().unwrap();

        match err {
            ModelError::DatabaseError(NotFound) => {
                return Err(invalid_email_or_password);
            }
            _ => {
                error!("{:?}", err);
                return Err(internal_error);
            }
        }
    }

    let mut user = user.unwrap();

    if !user.confirmed {
        return Err(user_not_confirmed);
    }

    if !user.verify_password(form.password) {
        return Err(invalid_email_or_password);
    }

    let payload = json!({
        "event": HookEvent::Login,
        "provider": "email",
        "user": user,
    });

    let hook = Webhook::new(HookEvent::Login, payload, config.clone(), operator_signature);

    let hook_response = hook.trigger();

    if hook_response.is_err() {
        return Err(Error::from(hook_response.err().unwrap()));
    }

    let hook_response = hook_response.unwrap();

    let jwt = JWT::new(&user, config.aud.clone(), hook_response);

    let jwt = jwt.sign(config);

    if jwt.is_err() {
        let err = jwt.err().unwrap();

        error!("{:?}", err);

        return Err(internal_error);
    }

    let jwt = jwt.unwrap();

    let refresh_token = NewRefreshToken::new(user.id.clone());

    let refresh_token = refresh_token.save(&connection);

    if refresh_token.is_err() {
        error!("{:?}", refresh_token.err().unwrap());

        return Err(Error::new(500, json!({"code": "unable_to_create_refresh_token"}), "Unable To Create Refresh Token".to_string()));
    }

    let refresh_token = refresh_token.unwrap().token;

    let user = user.update_last_sign_in(&connection);

    if user.is_err() {
        let err = user.err().unwrap();
        error!("{:?}", err);
        return Err(Error::from(err));
    }

    return Ok(LoginResponse { access_token: jwt, refresh_token });
}
