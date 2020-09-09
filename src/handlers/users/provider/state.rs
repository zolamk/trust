use crate::{config::Config, handlers::Error};
use chrono::{Duration, Utc};
use frank_jwt::{decode, encode, Algorithm, ValidationOptions};
use log::error;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProviderState {
    pub provider: String,
}

impl ProviderState {
    pub fn new(provider: String) -> ProviderState {
        return ProviderState { provider };
    }

    pub fn sign(self, config: &Config) -> Result<String, frank_jwt::Error> {
        let header = json!({});

        let exp = Utc::now() + Duration::minutes(5);

        let exp = exp.timestamp();

        let payload = json!({
            "exp": exp,
            "aud": "trust",
            "provider": self.provider,
        });

        return encode(header, &config.jwt_secret, &payload, Algorithm::HS512);
    }

    pub fn verify(state: String, config: &Config) -> Result<ProviderState, Error> {
        let state = decode(state.as_str(), &config.jwt_secret, Algorithm::HS512, &ValidationOptions::default());

        if state.is_err() {
            let err = state.err().unwrap();

            error!("{:?}", err);

            return Err(Error::new(400, json!({"code": "error_decoding_state"}), "Error Decoding State".to_string()));
        }

        let (_, state) = state.unwrap();

        let state = serde_json::from_value(state);

        if state.is_err() {
            let err = state.err().unwrap();

            error!("{:?}", err);

            return Err(Error::new(400, json!({"code": "error_deserializing_state"}), "Error Deserializing State".to_string()));
        }

        return Ok(state.unwrap());
    }
}
