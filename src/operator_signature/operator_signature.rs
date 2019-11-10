use crate::{config::Config, hook::HookEvent, operator_signature::Error};
use frank_jwt::{decode, encode, Algorithm, ValidationOptions};
use rocket::{
    http::Status, request::{self, FromRequest, Request}, Outcome, State
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

#[derive(Deserialize, Serialize, Clone)]

pub struct OperatorSignature {
    pub site_url: String,
    pub redirect_url: String,
    pub function_hooks: Map<String, Value>,
}

impl OperatorSignature {
    pub fn new(site_url: String, redirect_url: String, function_hooks: Map<String, Value>) -> OperatorSignature {
        return OperatorSignature {
            site_url,
            redirect_url,
            function_hooks,
        };
    }

    pub fn encode(self, operator_token: &str) -> Result<String, Error> {
        let header = json!({});

        let payload = serde_json::to_value(self);

        if payload.is_err() {
            return Err(Error::SignatureMissing);
        }

        let payload = payload.unwrap();

        match encode(header, &operator_token.to_string(), &payload, Algorithm::HS256) {
            Ok(token) => Ok(token),
            Err(err) => Err(Error::from(err)),
        }
    }

    pub fn decode(operator_signature: &str, operator_token: &str) -> Result<OperatorSignature, Error> {
        let decoded_token = decode(operator_signature, &operator_token.to_string(), Algorithm::HS256, &ValidationOptions::dangerous());

        if decoded_token.is_err() {
            return Err(Error::from(decoded_token.err().unwrap()));
        }

        let (_header, payload) = decoded_token.unwrap();

        let operator_signature = serde_json::from_value(payload);

        if operator_signature.is_err() {
            return Err(Error::from(operator_signature.err().unwrap()));
        }

        let operator_signature: OperatorSignature = operator_signature.unwrap();

        return Ok(operator_signature);
    }

    pub fn get_hook_url_for_event(self, event: HookEvent) -> Option<String> {
        if event == HookEvent::Login {
            return match self.function_hooks.get("login") {
                Some(hook) => Some(hook.as_str().unwrap().to_string()),
                None => None,
            };
        }

        return match self.function_hooks.get("signup") {
            Some(hook) => Some(hook.as_str().unwrap().to_string()),
            None => None,
        };
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for OperatorSignature {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let operator_signature = request.headers().get_one("x-operator-signature");

        if operator_signature.is_none() {
            return Outcome::Failure((Status::BadRequest, Error::SignatureMissing));
        }

        let config = request.guard::<State<Config>>();

        if config.is_failure() {
            return Outcome::Failure((Status::BadRequest, Error::SignatureMissing));
        }

        let config = config.unwrap();

        let operator_signature = operator_signature.unwrap();

        let operator_signature = OperatorSignature::decode(operator_signature, config.operator_token.as_ref());

        if operator_signature.is_err() {
            return Outcome::Failure((Status::BadRequest, operator_signature.err().unwrap()));
        }

        let operator_signature = operator_signature.unwrap();

        return Outcome::Success(operator_signature);
    }
}
