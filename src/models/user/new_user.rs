use crate::{
    models::{user::User, Error},
    schema::{users, users::dsl::*},
};
use bcrypt::hash;
use chrono::NaiveDateTime;
use diesel::{insert_into, PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Default, Insertable, Deserialize, Serialize, GraphQLInputObject)]
#[graphql(description = "New User")]
#[table_name = "users"]
pub struct NewUser {
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub is_admin: bool,
    pub password: Option<String>,
    pub email_confirmed: bool,
    pub email_confirmation_token: Option<String>,
    pub email_confirmation_token_sent_at: Option<NaiveDateTime>,
    pub phone_confirmed: bool,
    pub phone_confirmation_token: Option<String>,
    pub phone_confirmation_token_sent_at: Option<NaiveDateTime>,
}

impl NewUser {
    pub fn hash_password(&mut self, cost: u32) {
        match &self.password {
            Some(v) => self.password = Some(hash(v, cost).unwrap()),
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
