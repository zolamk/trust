use crate::{
    config::Config,
    hook::{Error, HookError, HookEvent},
    operator_signature::OperatorSignature,
};
use chrono::Utc;
use frank_jwt::{encode, Algorithm};
use reqwest::header::AUTHORIZATION;
use serde_json::{json, Value};

pub fn trigger_hook(event: HookEvent, payload: Value, config: &Config, operator_signature: &OperatorSignature) -> Result<Option<Value>, Error> {
    let url = operator_signature.get_hook_url_for_event(event);

    if url.is_none() {
        return Ok(None);
    }

    let url = url.unwrap();

    let url: &str = url.as_ref();

    let header = json!({
        "iat": Utc::now().timestamp(),
        "issuer": "trust"
    });

    let payload = json!({});

    let signature = encode(header, &config.jwt_secret, &payload, Algorithm::HS256);

    if signature.is_err() {
        let err = signature.err().unwrap();
        return Err(Error::from(err));
    }

    let signature = signature.unwrap();

    let client = reqwest::blocking::Client::new();

    let res = client.post(url).header(AUTHORIZATION, signature).json(&payload).send();

    if res.is_err() {
        return Err(Error::from(res.err().unwrap()));
    }

    let res = res.unwrap();

    let status = res.status();

    if status.is_success() {
        match res.json() {
            Ok(res) => return Ok(res),
            Err(err) => return Err(Error::from(err)),
        };
    }

    match res.json() {
        Ok(body) => return Err(Error::from(HookError { code: status.as_u16(), body })),
        Err(err) => return Err(Error::from(err)),
    };
}
