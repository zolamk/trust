extern crate chrono;
extern crate frank_jwt;
extern crate reqwest;
extern crate rocket;
extern crate serde;

use crate::config::Config;
use crate::error::Error;
use crate::hook::HookEvent;
use crate::models::operator_signature::OperatorSignature;
use chrono::Utc;
use frank_jwt::{encode, Algorithm};
use log::error;
use reqwest::header::AUTHORIZATION;
use serde_json::{json, Value};

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
            config,
            payload,
            event,
            operator_signature,
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

            return Err(Error::new(
                500,
                json!({
                    "code": "hook_signature_error"
                }),
            ));
        }

        let client = reqwest::Client::new();

        let res = client
            .post(url)
            .header(AUTHORIZATION, signature.unwrap())
            .json(&payload)
            .send();

        if res.is_err() {
            let err = res.err().unwrap();

            error!("{}", err);

            return Err(Error::new(
                422,
                json!({
                    "code": "hook_error"
                }),
            ));
        }

        let mut res = res.unwrap();

        let status = res.status();

        if status.is_success() {
            match res.json() {
                Ok(res) => return Ok(res),
                Err(err) => {
                    error!("{:?}", err);
                    return Err(Error::new(
                        422,
                        json!({
                            "code": "hook_success_response_parsing_error"
                        }),
                    ));
                }
            };
        }

        match res.json() {
            Ok(body) => {
                return Err(Error::new(status.as_u16(), body));
            }
            Err(err) => {
                error!("{}", err);
                return Err(Error::new(
                    422,
                    json!({
                        "code": "hook_error_response_parsing_error"
                    }),
                ));
            }
        };
    }
}
