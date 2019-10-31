extern crate chrono;
extern crate frank_jwt;
extern crate serde;
extern crate serde_json;

use crate::config::Config;
use crate::error::Error;
use crate::models::user;
use chrono::{Duration, Utc};
use diesel::PgConnection;
use frank_jwt::{decode, encode, Algorithm};
use log::error;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome, State};
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct JWT {
    #[serde(skip_deserializing)]
    user_id: i64,
    pub email: String,
    pub app_metadata: Option<Value>,
    pub user_metadata: Option<Value>,
}

impl JWT {
    pub fn new(
        user_id: i64,
        email: String,
        app_metadata: Option<Value>,
        user_metadata: Option<Value>,
    ) -> JWT {
        return JWT {
            user_id,
            email,
            app_metadata,
            user_metadata,
        };
    }

    fn get_algorithm(alg: &str) -> Algorithm {
        return match alg {
            "HS256" => Algorithm::HS256,
            "HS384" => Algorithm::HS384,
            "HS512" => Algorithm::HS512,
            "RS256" => Algorithm::RS256,
            "RS384" => Algorithm::RS384,
            "RS512" => Algorithm::RS512,
            "ES256" => Algorithm::ES256,
            "ES384" => Algorithm::ES384,
            "ES512" => Algorithm::ES512,
            a => panic!("unsupported algorithm: {}", a),
        };
    }

    pub fn is_admin(&self, connection: &PgConnection) -> bool {
        return user::is_admin(self.user_id, connection);
    }

    pub fn sign(self, config: Config) -> Result<String, frank_jwt::Error> {
        let aud = config.aud.clone();

        let exp = config.jwt_exp;

        let jwt_algorithm = config.jwt_algorithm.clone();

        let signing_key = config.get_signing_key();

        let header = json!({});

        let payload = if exp > 0 {
            let now = Utc::now() + Duration::seconds(exp);

            let exp = now.timestamp();

            json!({
                "aud":aud,
                "sub":self.user_id,
                "email": self.email,
                "app_metadata": self.app_metadata,
                "user_metadata": self.user_metadata,
                "exp": exp,
            })
        } else {
            json!({
                "aud":aud,
                "sub":self.user_id,
                "email": self.email,
                "app_metadata": self.app_metadata,
                "user_metadata": self.user_metadata,
            })
        };

        return encode(
            header,
            &signing_key,
            &payload,
            JWT::get_algorithm(&jwt_algorithm),
        );
    }

    pub fn decode(encoded_token: String, config: Config) -> Result<Self, Error> {
        let algorithm = config.jwt_algorithm.clone();

        let decoded_token = decode(
            &encoded_token,
            &config.get_decoding_key(),
            JWT::get_algorithm(&algorithm),
        );

        if decoded_token.is_err() {
            let err = decoded_token.err().unwrap();

            error!("{:?}", err);

            let err = Error {
                code: 400,
                body: json!({
                    "code": "bearer_token_decoding_error",
                }),
            };

            return Err(err);
        }

        let (header, payload) = decoded_token.unwrap();

        let decoded_token = serde_json::from_value(payload);

        if decoded_token.is_err() {
            let err = decoded_token.err().unwrap();

            error!("{:?}", err);

            let err = Error {
                code: 400,
                body: json!({
                    "code": "bearer_token_deserializing_error",
                }),
            };

            return Err(err);
        }

        let mut decoded_token: JWT = decoded_token.unwrap();

        let user_id = header.get("sub").unwrap().as_i64().unwrap();

        decoded_token.user_id = user_id;

        return Ok(decoded_token);
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for JWT {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let encoded_token = request.headers().get_one("authorization");

        let token_missing = Error {
            code: 400,
            body: json!({
                "code": "bearer_token_missing",
            }),
        };

        if encoded_token.is_none() {
            return Outcome::Failure((Status::BadRequest, token_missing));
        }

        let config = request.guard::<State<Config>>();

        if config.is_failure() {
            return Outcome::Failure((Status::BadRequest, token_missing));
        }

        let config = config.unwrap();

        let encoded_token = encoded_token.unwrap();

        let decoded_token = Self::decode(encoded_token.to_string(), config.inner().clone());

        if decoded_token.is_err() {
            return Outcome::Failure((Status::BadRequest, decoded_token.err().unwrap()));
        }

        let decoded_token = decoded_token.unwrap();

        return Outcome::Success(decoded_token);
    }
}
