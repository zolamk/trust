extern crate bcrypt;
extern crate chrono;
extern crate serde;
extern crate serde_json;

use chrono::NaiveDateTime;

use bcrypt::{hash, DEFAULT_COST};

use diesel::insert_into;

use diesel::PgConnection;

use crate::schema::users;

use crate::schema::users::dsl::*;

use serde::{Deserialize, Serialize};

use diesel::RunQueryDsl;

use crate::models::user::User;

#[derive(Default, Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: Option<String>,
    pub email: String,
    pub avatar: Option<String>,
    pub aud: String,
    pub role: Option<String>,
    pub password: Option<String>,
    pub is_super_admin: bool,
    pub confirmed: bool,
    pub confirmation_token: Option<String>,
    pub confirmation_sent_at: Option<NaiveDateTime>,
}

impl NewUser {
    pub fn hash_password(&mut self) {
        match &self.password {
            Some(v) => self.password = Some(hash(v, DEFAULT_COST).unwrap()),
            None => self.password = None,
        }
    }

    pub fn save(&self, connection: &PgConnection) -> diesel::QueryResult<User> {
        return insert_into(users).values(self).get_result(connection);
    }
}
