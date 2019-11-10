use crate::models::user::User;
use crate::models::Error;
use crate::schema::users::dsl::{is_admin as admin, *};
use diesel::result::QueryResult;
use diesel::ExpressionMethods;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub fn get_by_email(e: String, connection: &PgConnection) -> Result<User, Error> {
    match users.filter(email.eq(e)).first(connection) {
        Ok(user) => Ok(user),
        Err(err) => Err(Error::from(err)),
    }
}

pub fn get_by_id(i: i64, connection: &PgConnection) -> Result<User, Error> {
    match users.find(i).first(connection) {
        Ok(user) => Ok(user),
        Err(err) => Err(Error::from(err)),
    }
}

pub fn get_by_confirmation_token(token: String, connection: &PgConnection) -> Result<User, Error> {
    match users.filter(confirmation_token.eq(token)).first(connection) {
        Ok(user) => Ok(user),
        Err(err) => Err(Error::from(err)),
    }
}

pub fn is_admin(user_id: i64, connection: &PgConnection) -> bool {
    let is_admin: QueryResult<bool> = users.filter(id.eq(user_id)).select(admin).first(connection);

    if is_admin.is_err() {
        return false;
    }

    let is_admin = is_admin.unwrap();

    return is_admin;
}