extern crate diesel;

use diesel::result::QueryResult;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use diesel::ExpressionMethods;
use diesel::PgConnection;

use crate::models::user::User;
use crate::schema::users::dsl::*;

pub fn get_user_by_email(e: String, connection: &PgConnection) -> QueryResult<User> {
    users.filter(email.eq(e)).first(connection)
}
