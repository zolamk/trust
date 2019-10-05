extern crate chrono;
extern crate frank_jwt;
extern crate reqwest;
extern crate rocket;
extern crate serde;

use crate::config::Config;
use crate::hook::Error;
use crate::models::operator_signature::OperatorSignature;
use chrono::Utc;
use frank_jwt::{encode, Algorithm};
use log::error;
use reqwest::header::AUTHORIZATION;
use rocket::http::Status;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(PartialEq, Serialize, Copy, Clone)]
#[serde(untagged)]
pub enum HookEvent {
    Login,
    Signup,
}

pub struct Webhook {
    config: Config,
    payload: Value,
    event: HookEvent,
    operator_signature: OperatorSignature,
}

impl Webhook {
    pub fn new(
        event: HookEvent,
        payload: Value,
        config: Config,
        operator_signature: OperatorSignature,
    ) -> Webhook {
        return Webhook {
            config: config,
            payload: payload,
            event: event,
            operator_signature: operator_signature,
        };
    }

    fn generate_signature(self) -> Result<String, frank_jwt::Error> {
        let header = json!({
            "iat": Utc::now().timestamp(),
            "issuer": "trust"
        });

        let payload = json!({});

        return encode(header, &self.config.jwt_secret, &payload, Algorithm::HS256);
    }

    pub fn trigger(self) -> Result<Option<Value>, Error> {
        let client = reqwest::Client::new();

        let event = self.event;

        let operator_signature = self.operator_signature.clone();

        let url = operator_signature.get_hook_url_for_event(event);

        if url.is_none() {
            return Ok(None);
        }

        let url = url.unwrap();

        let url: &str = url.as_ref();

        let payload = self.payload.clone();

        let signature = self.generate_signature();

        if signature.is_err() {
            let err = signature.err().unwrap();

            error!("{}", err);

            let status = Status::from_code(500).unwrap();

            return Err(Error::new(status, json!({})));
        }

        let unprocessable = Status::from_code(422).unwrap();

        let res = client
            .post(url)
            .header(AUTHORIZATION, signature.unwrap())
            .json(&payload)
            .send();

        if res.is_err() {
            let err = res.err().unwrap();
            error!("{}", err);
            return Err(Error::new(unprocessable, json!({})));
        }

        let mut res = res.unwrap();

        let status = res.status();

        if status.is_success() {
            match res.json() {
                Ok(res) => return Ok(res),
                Err(err) => {
                    error!("{}", err);
                    return Err(Error::new(unprocessable, json!({})));
                }
            };
        }

        let status = Status::from_code(status.as_u16()).unwrap();

        if status.code >= 500 {
            return Err(Error::new(status, json!({})));
        }

        match res.json() {
            Ok(body) => {
                return Err(Error::new(status, body));
            }
            Err(err) => {
                error!("{}", err);
                return Err(Error::new(unprocessable, json!({})));
            }
        };
    }
}
