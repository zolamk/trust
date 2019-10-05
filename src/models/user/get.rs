extern crate diesel;

use diesel::result::QueryResult;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use diesel::ExpressionMethods;
use diesel::PgConnection;

use crate::models::user::User;
use crate::schema::users::dsl::*;

pub fn get_by_email(e: String, connection: &PgConnection) -> QueryResult<User> {
    return users.filter(email.eq(e)).first(connection);
}

pub fn get_by_confirmation_token(token: String, connection: &PgConnection) -> QueryResult<User> {
    return users.filter(confirmation_token.eq(token)).first(connection);
}
