use crate::{config::Config, sms::Error};
use handlebars::Handlebars;
use reqwest::{
    self,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde_json::Value;
use std::{collections::HashMap, str::FromStr};

pub fn send_sms(template: String, data: Value, to: String, config: &Config) -> Result<(), Error> {
    let sms_config = config.sms_config.clone().unwrap();

    let sms_headers: HeaderMap = sms_config
        .headers
        .iter()
        .map(|(name, val)| (HeaderName::from_str(name.as_ref()), HeaderValue::from_str(val.as_ref())))
        .filter(|(k, v)| k.is_ok() && v.is_ok())
        .map(|(k, v)| (k.unwrap(), v.unwrap()))
        .collect();

    let client = reqwest::Client::builder().default_headers(sms_headers).build();

    if client.is_err() {
        return Err(Error::from(client.err().unwrap()));
    }

    let method = reqwest::Method::from_str(&sms_config.method);

    if method.is_err() {
        return Err(Error::from(method.err().unwrap()));
    }

    let client = client.unwrap().request(method.unwrap(), &sms_config.url);

    let sms = Handlebars::new().render_template(&template, &data);

    if sms.is_err() {
        return Err(Error::from(sms.err().unwrap()));
    }

    let mut body = HashMap::new();

    body.insert(sms_config.mapping.source, sms_config.source);

    body.insert(sms_config.mapping.message, sms.unwrap());

    body.insert(sms_config.mapping.destination, to);

    let res = client.json(&body).send();

    if res.is_err() {
        return Err(Error::from(res.err().unwrap()));
    }

    let res = res.unwrap();

    let status = res.status().as_u16();

    if status < 200 || status > 299 {
        return Err(Error::SMSResponseError);
    }

    return Ok(());
}
