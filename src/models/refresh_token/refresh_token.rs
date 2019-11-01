extern crate frank_jwt;
extern crate rocket;
extern crate serde;
extern crate serde_json;
use diesel;

use crate::models::user::User;
use crate::schema::refresh_tokens;
use crate::schema::refresh_tokens::dsl::*;
use chrono::{DateTime, Utc};
use diesel::RunQueryDsl;
use serde::Serialize;

use diesel::{update, PgConnection, QueryResult};

#[derive(Queryable, AsChangeset, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
pub struct RefreshToken {
    pub id: i64,
    pub token: String,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl RefreshToken {
    pub fn save(self, connection: &PgConnection) -> QueryResult<RefreshToken> {
        return update(refresh_tokens).set(&self).get_result(connection);
    }
}
