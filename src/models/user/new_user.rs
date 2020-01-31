use crate::{
    models::{user::User, Error},
    schema::{users, users::dsl::*},
};
use bcrypt::{hash, DEFAULT_COST};
use chrono::NaiveDateTime;
use diesel::{insert_into, PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Default, Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub aud: String,
    pub is_admin: bool,
    pub password: Option<String>,
    pub confirmed: bool,
    pub confirmation_token: Option<String>,
    pub confirmation_sent_at: Option<NaiveDateTime>,
    pub invitation_sent_at: Option<NaiveDateTime>,
    pub user_metadata: Option<serde_json::Value>,
    pub app_metadata: Option<serde_json::Value>,
}

impl NewUser {
    pub fn hash_password(&mut self) {
        match &self.password {
            Some(v) => self.password = Some(hash(v, DEFAULT_COST).unwrap()),
            None => self.password = None,
        }
    }

    pub fn save(&self, connection: &PgConnection) -> Result<User, Error> {
        match insert_into(users).values(self).get_result(connection) {
            Ok(user) => Ok(user),
            Err(err) => Err(Error::from(err)),
        }
    }
}
