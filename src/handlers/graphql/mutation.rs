use crate::{
    handlers::{
        graphql::context::Context,
        lib::{confirm, signup},
        Error as HandlerError,
    },
    models::user::User,
};

#[derive(Debug)]
pub struct Mutation {}

juniper::graphql_object!(Mutation: Context |&self| {
    field signup(&executor, user: signup::SignUpForm) -> Result<User, HandlerError> {

        let context = executor.context();

        let user = signup::signup(&context.config, &context.connection, context.operator_signature.clone(), &context.email_templates, user);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());

    }

    field confirm(&executor, confirmation: confirm::ConfirmForm) -> Result<User, HandlerError> {

        let context = executor.context();

        let user = confirm::confirm(&context.connection, confirmation);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());

    }
});
