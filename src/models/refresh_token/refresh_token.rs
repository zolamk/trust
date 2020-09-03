use crate::{
    models::{user::User, Error},
    schema::{refresh_tokens, refresh_tokens::dsl::*},
};
use chrono::{DateTime, Utc};
use diesel::{update, PgConnection, RunQueryDsl};
use serde::Serialize;

#[derive(Queryable, AsChangeset, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
pub struct RefreshToken {
    pub id: i64,
    pub token: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl RefreshToken {
    pub fn save(self, connection: &PgConnection) -> Result<RefreshToken, Error> {
        match update(refresh_tokens).set(&self).get_result(connection) {
            Ok(refresh_token) => Ok(refresh_token),
            Err(err) => Err(Error::from(err)),
        }
    }
}
