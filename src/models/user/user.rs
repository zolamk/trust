use crate::{
    models::Error,
    schema::{users, users::dsl::*},
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{NaiveDateTime, Utc};
use diesel::{delete, update, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use serde::Serialize;

#[derive(Queryable, AsChangeset, Serialize, Identifiable, Debug, Clone, GraphQLObject)]
#[changeset_options(treat_none_as_null = "true")]
pub struct User {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[graphql(name = "phone_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    #[graphql(skip)]
    #[serde(skip_serializing)]
    pub is_admin: bool,

    #[graphql(skip)]
    #[serde(skip_serializing)]
    pub password: Option<String>,

    #[graphql(name = "email_confirmed")]
    pub email_confirmed: bool,

    #[graphql(skip)]
    #[serde(skip_serializing)]
    pub email_confirmation_token: Option<String>,

    #[graphql(name = "email_confirmation_token_sent_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_confirmation_token_sent_at: Option<NaiveDateTime>,

    #[graphql(name = "email_confirmed_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_confirmed_at: Option<NaiveDateTime>,

    #[graphql(name = "phone_confirmed")]
    pub phone_confirmed: bool,

    #[graphql(skip)]
    #[serde(skip_serializing)]
    pub phone_confirmation_token: Option<String>,

    #[graphql(name = "phone_confirmation_token_sent_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_confirmation_token_sent_at: Option<NaiveDateTime>,

    #[graphql(name = "phone_confirmed_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_confirmed_at: Option<NaiveDateTime>,

    #[graphql(skip)]
    #[serde(skip_serializing)]
    pub recovery_token: Option<String>,

    #[graphql(name = "recovery_token_sent_at")]
    #[serde(skip_serializing)]
    pub recovery_token_sent_at: Option<NaiveDateTime>,

    #[graphql(skip)]
    #[serde(skip_serializing)]
    pub email_change_token: Option<String>,

    #[graphql(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_email: Option<String>,

    #[graphql(name = "email_change_token_sent_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_change_token_sent_at: Option<NaiveDateTime>,

    #[graphql(name = "last_signin_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_signin_at: Option<NaiveDateTime>,

    #[graphql(name = "created_at")]
    pub created_at: NaiveDateTime,

    #[graphql(name = "updated_at")]
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn delete_by_email(e: String, connection: &PgConnection) -> Result<usize, Error> {
        match delete(users.filter(email.eq(e))).execute(connection) {
            Ok(affected_rows) => Ok(affected_rows),
            Err(err) => Err(Error::from(err)),
        }
    }

    pub fn delete(&self, connection: &PgConnection) -> Result<User, Error> {
        match delete(users.filter(id.eq(self.id.clone()))).get_result(connection) {
            Ok(user) => Ok(user),
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

    pub fn confirm_email(&mut self, connection: &PgConnection) -> Result<User, Error> {
        let n: Option<String> = None;

        let now = Some(Utc::now().naive_utc());

        match update(users.filter(id.eq(self.id.clone())))
            .set((email_confirmed.eq(true), email_confirmation_token.eq(n), email_confirmed_at.eq(now)))
            .get_result(connection)
        {
            Ok(affected_rows) => Ok(affected_rows),
            Err(err) => Err(Error::from(err)),
        }
    }

    pub fn confirm_phone(&mut self, connection: &PgConnection) -> Result<User, Error> {
        let n: Option<String> = None;
        let now = Some(Utc::now().naive_utc());

        match update(users.filter(id.eq(self.id.clone())))
            .set((phone_confirmed.eq(true), phone_confirmation_token.eq(n), phone_confirmed_at.eq(now)))
            .get_result(connection)
        {
            Ok(affected_rows) => Ok(affected_rows),
            Err(err) => Err(Error::from(err)),
        }
    }

    pub fn update_last_sign_in(&mut self, connection: &PgConnection) -> Result<usize, Error> {
        let now = Some(Utc::now().naive_utc());

        match update(users.filter(id.eq(self.id.clone()))).set(last_signin_at.eq(now)).execute(connection) {
            Ok(affected_rows) => Ok(affected_rows),
            Err(err) => Err(Error::from(err)),
        }
    }

    pub fn confirm_email_change(&mut self, connection: &PgConnection) -> Result<User, Error> {
        let n: Option<String> = None;
        match update(users.filter(id.eq(self.id.clone())))
            .set((email_change_token.eq(n.clone()), new_email.eq(n), email.eq(self.new_email.as_ref().unwrap())))
            .get_result(connection)
        {
            Ok(user) => Ok(user),
            Err(err) => Err(Error::from(err)),
        }
    }

    pub fn save(self, connection: &PgConnection) -> Result<User, Error> {
        match update(users.filter(id.eq(self.id.clone()))).set(&self).get_result(connection) {
            Ok(user) => Ok(user),
            Err(err) => Err(Error::from(err)),
        }
    }
}
