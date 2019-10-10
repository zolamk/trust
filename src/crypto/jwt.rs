extern crate frank_jwt;
extern crate serde_json;

use crate::config::Config;
use frank_jwt::{encode, Algorithm};
use serde_json::json;
use serde_json::Value;

pub struct JWT {
    user_id: i64,
    pub email: String,
    pub app_metadata: Option<Value>,
    pub user_metadata: Option<Value>,
    config: Config,
}

impl JWT {
    pub fn new(
        user_id: i64,
        email: String,
        app_metadata: Option<Value>,
        user_metadata: Option<Value>,
        config: Config,
    ) -> JWT {
        return JWT {
            user_id,
            email,
            app_metadata,
            user_metadata,
            config,
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

    pub fn sign(self) -> Result<String, frank_jwt::Error> {
        let config = self.config.clone();

        let aud = config.aud.clone();

        let jwt_algorithm = config.jwt_algorithm.clone();

        let signing_key = config.get_signing_key();

        let header = json!({
            "sub": self.user_id,
            "aud": aud,
        });

        let payload = json!({
            "email": self.email,
            "app_metadata": self.app_metadata,
            "user_metadata": self.user_metadata,
        });

        return encode(
            header,
            &signing_key,
            &payload,
            JWT::get_algorithm(&jwt_algorithm),
        );
    }
}
