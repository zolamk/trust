extern crate diesel;

use crate::models::refresh_token::RefreshToken;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use crate::schema::refresh_tokens::dsl::*;
use diesel::ExpressionMethods;
use diesel::PgConnection;
use diesel::QueryResult;

pub fn get_refresh_token(
    refresh_token: String,
    connection: &PgConnection,
) -> QueryResult<RefreshToken> {
    return refresh_tokens
        .filter(token.eq(refresh_token))
        .first(connection);
}
