extern crate bcrypt;
extern crate chrono;
extern crate serde_json;

use chrono::NaiveDateTime;

use bcrypt::{hash, verify, DEFAULT_COST};

use diesel::insert_into;

use diesel::PgConnection;

use crate::schema::users;

#[derive(Default, Queryable)]
pub struct User {
    pub id: i64,
    pub instance_id: String,
    pub name: Option<String>,
    pub email: String,
    pub avatar: Option<String>,
    pub aud: String,
    pub role: Option<String>,
    pub password: Option<String>,
    pub confirmed: bool,
    pub invited_at: Option<NaiveDateTime>,
    pub confirmation_token: Option<String>,
    pub confirmation_sent_at: Option<NaiveDateTime>,
    pub recovery_token: Option<String>,
    pub recovery_sent_at: Option<NaiveDateTime>,
    pub email_change_token: Option<String>,
    pub email_change: Option<String>,
    pub email_change_sent_at: Option<NaiveDateTime>,
    pub last_signin_at: Option<NaiveDateTime>,
    pub app_metadata: Option<serde_json::Value>,
    pub user_metadata: Option<serde_json::Value>,
    pub is_super_admin: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Default, Insertable, Queryable)]
#[table_name = "users"]
pub struct NewUser {
    pub instance_id: String,
    pub name: Option<String>,
    pub email: String,
    pub avatar: Option<String>,
    pub aud: String,
    pub role: Option<String>,
    pub password: Option<String>,
    pub is_super_admin: bool,
    pub confirmed: bool,
}

impl User {
    pub fn hash_password(&mut self) {
        match &self.password {
            Some(v) => self.password = Some(hash(v, DEFAULT_COST).unwrap()),
            None => self.password = None,
        }
    }

    pub fn verify_password(&mut self, password: String) -> bool {
        match &self.password {
            Some(v) => verify(password, v).unwrap(),
            None => false,
        }
    }
}

impl NewUser {
    pub fn hash_password(&mut self) {
        match &self.password {
            Some(v) => self.password = Some(hash(v, DEFAULT_COST).unwrap()),
            None => self.password = None,
        }
    }

    pub fn save(&self, connection: &PgConnection) -> diesel::QueryResult<usize> {
        use crate::schema::users::dsl::*;

        use diesel::RunQueryDsl;

        return insert_into(users).values(self).execute(connection);
    }
}
