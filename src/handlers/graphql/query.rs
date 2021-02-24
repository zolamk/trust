use crate::{
    handlers::{
        graphql::context::Context,
        lib::{refresh, token, user::get, users},
        Error as HandlerError,
    },
    models::user::User,
};
use log::debug;

#[derive(Debug)]
pub struct Query {}

#[juniper::object(Context = Context)]
impl Query {
    fn user(context: &Context, id: String) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            debug!("{:?}", err);

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = users::get_by_id::get_by_id(&context.connection, token, id, &context.config);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    fn users(context: &Context, limit: i32, offset: i32) -> Result<Vec<User>, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            debug!("{:?}", err);

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let users = users::get::get(&context.connection, token, offset as i64, limit as i64, &context.config);

        if users.is_err() {
            return Err(users.err().unwrap());
        }

        return Ok(users.unwrap());
    }

    fn me(context: &Context) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            debug!("{:?}", err);

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = get::get(&context.connection, token);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    fn token(context: &Context, username: String, password: String) -> Result<token::LoginResponse, HandlerError> {
        let token = token::token(&context.config, &context.connection, &context.operator_signature, token::LoginForm { username, password });

        if token.is_err() {
            return Err(token.err().unwrap());
        }

        return Ok(token.unwrap());
    }

    fn refresh(context: &Context, token: String) -> Result<token::LoginResponse, HandlerError> {
        let token = refresh::refresh(&context.config, &context.connection, &context.operator_signature, refresh::RefreshForm { refresh_token: token });

        if token.is_err() {
            return Err(token.err().unwrap());
        }

        return Ok(token.unwrap());
    }
}
