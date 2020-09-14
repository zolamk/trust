use crate::{config::Config, crypto::Error, models::user};
use chrono::{Duration, Utc};
use diesel::PgConnection;
use frank_jwt::{decode, encode, Algorithm, ValidationOptions};
use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
    Outcome, State,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Serialize, Debug)]
pub struct JWT {
    pub sub: String,
    pub email: String,
    pub aud: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl JWT {
    pub fn new(user: &user::User, aud: String, metadata: Option<Value>) -> JWT {
        return JWT {
            sub: user.id.clone(),
            exp: None,
            aud,
            email: user.email.clone(),
            metadata,
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
        return user::is_admin(self.sub.clone(), connection);
    }

    pub fn sign(mut self, config: &Config) -> Result<String, Error> {
        let exp = config.jwt_exp;

        let jwt_algorithm = config.jwt_algorithm.clone();

        let signing_key = config.clone().get_signing_key();

        let header = json!({});

        if exp > 0 {
            let now = Utc::now() + Duration::seconds(exp);

            let exp = now.timestamp();

            self.exp = Some(exp);
        }

        let payload = serde_json::to_value(self);

        if payload.is_err() {
            return Err(Error::JSONError(payload.err().unwrap()));
        }

        let payload = payload.unwrap();

        let res = encode(header, &signing_key, &payload, JWT::get_algorithm(&jwt_algorithm));

        if res.is_err() {
            return Err(Error::from(res.err().unwrap()));
        }

        return Ok(res.unwrap());
    }

    pub fn decode(encoded_token: String, config: Config) -> Result<JWT, Error> {
        let algorithm = config.jwt_algorithm.clone();

        let decoded_token = decode(&encoded_token, &config.get_decoding_key(), JWT::get_algorithm(&algorithm), &ValidationOptions::default());

        if decoded_token.is_err() {
            return Err(Error::from(decoded_token.err().unwrap()));
        }

        let (_, payload) = decoded_token.unwrap();

        let decoded_token = serde_json::from_value(payload);

        if decoded_token.is_err() {
            return Err(Error::from(decoded_token.err().unwrap()));
        }

        let decoded_token: JWT = decoded_token.unwrap();

        return Ok(decoded_token);
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for JWT {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let encoded_token = request.headers().get_one("authorization");

        if encoded_token.is_none() {
            return Outcome::Failure((Status::BadRequest, Error::TokenMissing));
        }

        let config = request.guard::<State<Config>>();

        if config.is_failure() {
            return Outcome::Failure((Status::BadRequest, Error::TokenMissing));
        }

        let config = config.unwrap();

        let encoded_token = encoded_token.unwrap();

        let parts = encoded_token.split(' ').collect::<Vec<&str>>();

        let encoded_token = parts.get(1);

        if encoded_token.is_none() {
            return Outcome::Failure((Status::BadRequest, Error::TokenMissing));
        }

        let encoded_token = encoded_token.unwrap();

        let decoded_token = Self::decode(encoded_token.to_string(), config.inner().clone());

        if decoded_token.is_err() {
            return Outcome::Failure((Status::BadRequest, decoded_token.err().unwrap()));
        }

        let decoded_token = decoded_token.unwrap();

        return Outcome::Success(decoded_token);
    }
}
