use crate::{
    config::Config,
    crypto::get_algorithm,
    hook::{Error, HookError, HookEvent},
};
use chrono::Utc;
use frank_jwt::encode;
use reqwest::header::AUTHORIZATION;
use serde_json::{json, Value};

pub fn trigger_hook(_event: HookEvent, payload: Value, config: &Config) -> Result<Option<Value>, Error> {
    let url = config.login_hook.clone();

    if url.is_none() {
        return Ok(None);
    }

    let url = url.unwrap();

    let url: &str = url.as_ref();

    let header = json!({
        "iat": Utc::now().timestamp(),
        "issuer": "trust"
    });

    let p = json!({
        "metadata": {
            "roles": ["trust"]
        },
        "sub": "trust"
    });

    let signing_key = config.clone().get_signing_key();

    let signature = encode(header, &signing_key, &p, get_algorithm(&config.jwt_algorithm));

    if signature.is_err() {
        let err = signature.err().unwrap();
        return Err(Error::from(err));
    }

    let signature = signature.unwrap();

    let client = reqwest::blocking::Client::new();

    let res = client.post(url).header(AUTHORIZATION, format!("Bearer {}", signature)).json(&payload).send();

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
