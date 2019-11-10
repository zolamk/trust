use crate::{
    config::Config, crypto::jwt::JWT, handlers::{trigger_hook, Error}, hook::HookEvent, models::{refresh_token::NewRefreshToken, Error as ModelError}, operator_signature::OperatorSignature
};
use diesel::{
    pg::PgConnection, r2d2::{ConnectionManager, Pool}, result::Error::NotFound
};
use log::error;
use rocket::{http::Status, response::status, State};
use rocket_contrib::json::JsonValue;

pub fn password_grant(
    username: String, password: String, config: State<Config>, connection_pool: State<Pool<ConnectionManager<PgConnection>>>, operator_signature: OperatorSignature,
) -> Result<status::Custom<JsonValue>, Error> {
    let internal_error = Error {
        code: 500,
        body: json!({
            "code": "internal_error",
        }),
    };

    let user_not_confirmed = Error {
        code: 412,
        body: json!({
            "code": "user_not_confirmed",
        }),
    };

    let invalid_email_or_password = Error {
        code: 401,
        body: json!({
            "code": "invalid_email_or_password",
        }),
    };

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(err) => {
            error!("{:?}", err);
            return Err(internal_error);
        }
    };

    let user = crate::models::user::get_by_email(username, &connection);

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

    let user = user.unwrap();

    if !user.confirmed {
        return Err(user_not_confirmed);
    }

    if !user.verify_password(password) {
        return Err(invalid_email_or_password);
    }

    let user = trigger_hook(HookEvent::Signup, user, config.inner(), &connection, operator_signature, "email".to_string());

    if user.is_err() {
        let err = user.err().unwrap();

        error!("{:?}", err);

        return Err(err);
    }

    let user = user.unwrap();

    let jwt = JWT::new(&user, config.aud.clone());

    let jwt = jwt.sign(config.inner());

    if jwt.is_err() {
        let err = jwt.err().unwrap();

        error!("{:?}", err);

        return Err(internal_error);
    }

    let jwt = jwt.unwrap();

    let refresh_token = NewRefreshToken::new(user.id);

    let refresh_token = refresh_token.save(&connection);

    if refresh_token.is_err() {
        error!("{:?}", refresh_token.err().unwrap());

        return Err(Error {
            code: 500,
            body: json!({
                "code": "unable_to_create_refresh_token",
            }),
        });
    }

    let refresh_token = refresh_token.unwrap().token;

    return Ok(status::Custom(
        Status::Ok,
        JsonValue(json!({
            "access_token": jwt,
            "refresh_token": refresh_token,
            "type": "bearer"
        })),
    ));
}
