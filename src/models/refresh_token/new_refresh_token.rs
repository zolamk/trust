use crate::{
    crypto::secure_token,
    models::{refresh_token::RefreshToken, Error},
    schema::{refresh_tokens, refresh_tokens::dsl::*},
};
use diesel::{insert_into, PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Default, Insertable, Deserialize, Serialize)]
#[table_name = "refresh_tokens"]
pub struct NewRefreshToken {
    token: String,
    user_id: String,
}

impl NewRefreshToken {
    pub fn new(uid: String) -> NewRefreshToken {
        return NewRefreshToken {
            user_id: uid,
            token: secure_token(50),
        };
    }

    pub fn save(self, connection: &PgConnection) -> Result<RefreshToken, Error> {
        match insert_into(refresh_tokens).values(self).get_result(connection) {
            Ok(refresh_token) => Ok(refresh_token),
            Err(err) => Err(Error::from(err)),
        }
    }
}
