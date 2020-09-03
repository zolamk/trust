use crate::{
    handlers::graphql::context::Context,
    models::user::{NewUser, User},
};
use juniper::FieldResult;

#[derive(Debug)]
pub struct Mutation {}

juniper::graphql_object!(Mutation: Context |&self| {
    field signup(&executor, input: NewUser) -> FieldResult<User> {
        unimplemented!();
    }
});
