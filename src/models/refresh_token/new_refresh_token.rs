use crate::crypto::secure_token;
use crate::models::refresh_token::RefreshToken;
use crate::models::Error;
use crate::schema::refresh_tokens;
use crate::schema::refresh_tokens::dsl::*;
use diesel::insert_into;
use diesel::PgConnection;
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};

#[derive(Default, Insertable, Deserialize, Serialize)]
#[table_name = "refresh_tokens"]
pub struct NewRefreshToken {
    token: String,
    user_id: i64,
}

impl NewRefreshToken {
    pub fn new(uid: i64) -> NewRefreshToken {
        return NewRefreshToken {
            user_id: uid,
            token: secure_token(50),
        };
    }

    pub fn save(self, connection: &PgConnection) -> Result<RefreshToken, Error> {
        match insert_into(refresh_tokens)
            .values(self)
            .get_result(connection)
        {
            Ok(refresh_token) => Ok(refresh_token),
            Err(err) => Err(Error::from(err)),
        }
    }
}