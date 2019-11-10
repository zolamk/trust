use crate::models::refresh_token::RefreshToken;
use crate::models::Error;
use crate::schema::refresh_tokens::dsl::*;
use diesel::ExpressionMethods;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub fn get_refresh_token(
    refresh_token: String,
    connection: &PgConnection,
) -> Result<RefreshToken, Error> {
    match refresh_tokens
        .filter(token.eq(refresh_token))
        .first(connection)
    {
        Ok(refresh_token) => Ok(refresh_token),
        Err(err) => Err(Error::from(err)),
    }
}