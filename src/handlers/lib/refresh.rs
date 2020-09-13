use crate::{
    config::Config,
    crypto,
    crypto::jwt::JWT,
    handlers::{lib::token::LoginResponse, Error},
    hook::{HookEvent, Webhook},
    models::{refresh_token::get_refresh_token_by_token, user},
    operator_signature::OperatorSignature,
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct RefreshForm {
    #[graphql(name = "refresh_token")]
    pub refresh_token: String,
}

pub fn refresh(config: &Config, connection: &PooledConnection<ConnectionManager<PgConnection>>, operator_signature: OperatorSignature, refresh_form: RefreshForm) -> Result<LoginResponse, Error> {
    let internal_error = Error::new(500, json!({"code": "internal_server_error"}), "Internal Server Error".to_string());

    let invalid_refresh_token = Error::new(400, json!({"code": "invalid_refresh_token"}), "Invalid Refresh Token".to_string());

    let refresh_token = get_refresh_token_by_token(refresh_form.refresh_token, &connection);

    if refresh_token.is_err() {
        let err = refresh_token.err().unwrap();

        error!("{:?}", err);

        return Err(invalid_refresh_token);
    }

    let mut refresh_token = refresh_token.unwrap();

    let token = crypto::secure_token(50);

    refresh_token.token = token;

    let refresh_token = refresh_token.save(&connection);

    if refresh_token.is_err() {
        let err = refresh_token.err().unwrap();

        error!("{:?}", err);

        return Err(internal_error);
    }

    let refresh_token = refresh_token.unwrap();

    let user = user::get_by_id(refresh_token.user_id, &connection);

    if user.is_err() {
        let err = user.err().unwrap();

        error!("{:?}", err);

        return Err(internal_error);
    }

    let user = user.unwrap();

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

    return Ok(LoginResponse {
        access_token: jwt,
        refresh_token: refresh_token.token,
    });
}
