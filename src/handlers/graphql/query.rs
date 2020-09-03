use crate::{handlers::graphql::context::Context, models::user::User};
use juniper::FieldResult;

#[derive(Debug)]
pub struct Query {}

juniper::graphql_object!(Query: Context |&self| {
    field api_version() -> &str {
        return "1.0"
    }

    field user(&executor, id: String) -> FieldResult<User> {
        unimplemented!();
    }

    field users(&executor) -> FieldResult<Vec<User>> {
        unimplemented!();
    }
});
