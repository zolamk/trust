use crate::{
    config::Config,
    handlers::Error,
    hook::{HookEvent, Webhook},
    models::user::User,
    operator_signature::OperatorSignature,
};
use diesel::PgConnection;

pub fn trigger_hook(event: HookEvent, mut user: User, config: &Config, connection: &PgConnection, operator_signature: OperatorSignature, provider: String) -> Result<User, Error> {
    let payload = json!({
        "event": event,
        "provider": provider,
        "user": user,
    });

    let hook = Webhook::new(HookEvent::Signup, payload, config.clone(), operator_signature);

    let hook_response = hook.trigger();

    if hook_response.is_err() {
        return Err(Error::from(hook_response.err().unwrap()));
    }

    let hook_response = hook_response.unwrap();

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
                    return Err(Error::from(res.err().unwrap()));
                }

                user = res.unwrap();
            }
        }
    }

    return Ok(user);
}
