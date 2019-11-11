use crate::{
    models::Error,
    schema::{users, users::dsl::*},
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::NaiveDateTime;
use diesel::{delete, update, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use serde::Serialize;

#[derive(Queryable, AsChangeset, Serialize, Identifiable, Debug)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_sent_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub confirmation_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_sent_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub recovery_token: Option<String>,
    #[serde(skip_serializing)]
    pub recovery_sent_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub email_change_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_change: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_change_sent_at: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_signin_at: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn delete_by_email(e: String, connection: &PgConnection) -> Result<usize, Error> {
        match delete(users.filter(email.eq(e))).execute(connection) {
            Ok(affected_rows) => Ok(affected_rows),
            Err(err) => Err(Error::from(err)),
        }
    }

    pub fn delete(&self, connection: &PgConnection) -> Result<usize, Error> {
        match delete(users.filter(id.eq(self.id))).execute(connection) {
            Ok(affected_rows) => Ok(affected_rows),
            Err(err) => Err(Error::from(err)),
        }
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

    pub fn confirm(&mut self, connection: &PgConnection) -> Result<usize, Error> {
        match update(users.filter(id.eq(self.id))).set((confirmed.eq(true), confirmation_token.eq(""))).execute(connection) {
            Ok(affected_rows) => Ok(affected_rows),
            Err(err) => Err(Error::from(err)),
        }
    }

    pub fn save(self, connection: &PgConnection) -> Result<User, Error> {
        match update(users.filter(id.eq(self.id))).set(&self).get_result(connection) {
            Ok(user) => Ok(user),
            Err(err) => Err(Error::from(err)),
        }
    }
}
