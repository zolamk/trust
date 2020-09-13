use crate::{
    handlers::{
        graphql::context::Context,
        lib::{refresh, token, user::get},
        Error as HandlerError,
    },
    models::user::User,
};
use juniper::FieldResult;

#[derive(Debug)]
pub struct Query {}

#[juniper::object(Context = Context)]
impl Query {
    fn user(context: &Context, id: String) -> Result<User, HandlerError> {
        unimplemented!();
    }

    fn users(context: &Context) -> FieldResult<Vec<User>> {
        unimplemented!();
    }

    fn me(context: &Context) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = get::get(&context.config, &context.connection, &context.email_templates, &context.operator_signature, token);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    fn token(context: &Context, username: String, password: String) -> Result<token::LoginResponse, HandlerError> {
        let token = token::token(&context.config, &context.connection, context.operator_signature.clone(), token::LoginForm { username, password });

        if token.is_err() {
            return Err(token.err().unwrap());
        }

        return Ok(token.unwrap());
    }

    fn refresh(context: &Context, token: String) -> Result<token::LoginResponse, HandlerError> {
        let token = refresh::refresh(&context.config, &context.connection, context.operator_signature.clone(), refresh::RefreshForm { refresh_token: token });

        if token.is_err() {
            return Err(token.err().unwrap());
        }

        return Ok(token.unwrap());
    }
}
