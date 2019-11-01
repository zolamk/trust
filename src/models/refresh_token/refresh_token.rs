extern crate frank_jwt;
extern crate rocket;
extern crate serde;
extern crate serde_json;

use crate::schema::refresh_tokens;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Queryable, AsChangeset, Serialize, Identifiable)]
pub struct RefreshToken {
    pub id: i64,
    pub token: String,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
