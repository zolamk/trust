use crate::{config::Config, sms::Error};
use handlebars::Handlebars;
use log::error;
use reqwest::{
    self,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde_json::Value;
use std::{collections::HashMap, str::FromStr};

pub fn send_sms(template: String, data: Value, to: String, config: &Config) -> Result<(), Error> {
    let sms_config = config.sms.clone().unwrap();

    let sms_headers: HeaderMap = sms_config
        .headers
        .iter()
        .map(|(name, val)| (HeaderName::from_str(name.as_ref()), HeaderValue::from_str(val.as_ref())))
        .filter(|(k, v)| k.is_ok() && v.is_ok())
        .map(|(k, v)| (k.unwrap(), v.unwrap()))
        .collect();

    let client = reqwest::blocking::Client::builder().default_headers(sms_headers).build();

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

    for (key, value) in sms_config.extra {
        body.insert(key, value);
    }

    let res = client.json(&body).send();

    if res.is_err() {
        let err = res.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let res = res.unwrap();

    let status = res.status();

    if status.is_success() {
        return Ok(());
    }

    let res = res.text().unwrap_or(String::from("SMS Response Error"));

    return Err(Error::SMSResponseError(res));
}
