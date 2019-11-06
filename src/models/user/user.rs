extern crate bcrypt;
extern crate chrono;
extern crate serde;
extern crate serde_json;

use chrono::NaiveDateTime;

use bcrypt::{hash, verify, DEFAULT_COST};

use diesel::{delete, update};

use diesel::PgConnection;

use crate::schema::users;

use crate::schema::users::dsl::*;

use serde::Serialize;

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

#[derive(Queryable, AsChangeset, Serialize, Identifiable)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub aud: String,
    #[serde(skip_serializing)]
    pub is_admin: bool,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    #[serde(skip_serializing)]
    pub confirmed: bool,
    pub invitation_sent_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub confirmation_token: Option<String>,
    pub confirmation_sent_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub recovery_token: Option<String>,
    #[serde(skip_serializing)]
    pub recovery_sent_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub email_change_token: Option<String>,
    pub email_change: Option<String>,
    pub email_change_sent_at: Option<NaiveDateTime>,
    pub last_signin_at: Option<NaiveDateTime>,
    pub app_metadata: Option<serde_json::Value>,
    pub user_metadata: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn delete_by_email(e: String, connection: &PgConnection) -> diesel::QueryResult<usize> {
        return delete(users.filter(email.eq(e))).execute(connection);
    }

    pub fn delete(&self, connection: &PgConnection) -> diesel::QueryResult<usize> {
        return delete(users.filter(id.eq(self.id))).execute(connection);
    }

    pub fn hash_password(&mut self) {
        match &self.password {
            Some(v) => self.password = Some(hash(v, DEFAULT_COST).unwrap()),
            None => self.password = None,
        }
    }

    pub fn verify_password(&self, pass: String) -> bool {
        match &self.password {
            Some(v) => verify(pass, v).unwrap(),
            None => false,
        }
    }

    pub fn confirm(&mut self, connection: &PgConnection) -> diesel::QueryResult<usize> {
        return update(users.filter(id.eq(self.id)))
            .set((confirmed.eq(true), confirmation_token.eq("")))
            .execute(connection);
    }

    pub fn save(self, connection: &PgConnection) -> diesel::QueryResult<User> {
        return update(users).set(&self).get_result(connection);
    }
}
