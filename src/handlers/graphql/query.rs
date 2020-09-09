use crate::{
    handlers::{graphql::context::Context, lib::token, Error as HandlerError},
    models::user::User,
};
use juniper::FieldResult;

#[derive(Debug)]
pub struct Query {}

juniper::graphql_object!(Query: Context |&self| {
    field api_version() -> &str {
        return "1.0"
    }

    field user(&executor, id: String) -> Result<User, HandlerError> {
        unimplemented!();
    }

    field users(&executor) -> FieldResult<Vec<User>> {
        unimplemented!();
    }

    field token(&executor, user: token::LoginForm) -> Result<token::LoginResponse, HandlerError> {
        let context = executor.context();

        let token = token::token(&context.config, &context.connection, context.operator_signature.clone(), user);

        if token.is_err() {
            return Err(token.err().unwrap());
        }

        return Ok(token.unwrap());

    }
});
