extern crate diesel;

use diesel::result::QueryResult;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use diesel::ExpressionMethods;
use diesel::PgConnection;

use crate::models::user::User;
use crate::schema::users::dsl::{is_admin as admin, *};

pub fn get_by_email(e: String, connection: &PgConnection) -> QueryResult<User> {
    return users.filter(email.eq(e)).first(connection);
}

pub fn get_by_id(i: i64, connection: &PgConnection) -> QueryResult<User> {
    return users.find(i).first(connection);
}

pub fn get_by_confirmation_token(token: String, connection: &PgConnection) -> QueryResult<User> {
    return users.filter(confirmation_token.eq(token)).first(connection);
}

pub fn is_admin(user_id: i64, connection: &PgConnection) -> bool {
    let is_admin: QueryResult<bool> = users.filter(id.eq(user_id)).select(admin).first(connection);

    if is_admin.is_err() {
        return false;
    }

    let is_admin = is_admin.unwrap();

    return is_admin;
}
