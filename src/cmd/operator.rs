use crate::config::Config;
use crate::operator_signature::OperatorSignature;
use clap::ArgMatches;
use serde_json::{Map, Value};

pub fn operator(matches: Option<&ArgMatches>, config: Config) {
    let matches = matches.unwrap();

    if let ("create-signature", sub_m) = matches.subcommand() {
        new_signuature(sub_m, config);
    }
}

fn new_signuature(matches: Option<&ArgMatches>, config: Config) {
    let matches = matches.unwrap();

    let site_url = matches.value_of("site_url").unwrap().to_string();

    let redirect_url = matches
        .value_of("confirmed_redirect_url")
        .unwrap()
        .to_string();

    let mut function_hooks = Map::with_capacity(2);

    let login_hook = matches.value_of("login_hook");

    let signup_hook = matches.value_of("signup_hook");

    if let Some(login_hook) = login_hook {
        function_hooks.insert("login".to_string(), Value::String(login_hook.to_string()));
    }

    if let Some(signup_hook) = signup_hook {
        function_hooks.insert("signup".to_string(), Value::String(signup_hook.to_string()));
    }

    let operator_signature = OperatorSignature::new(site_url, redirect_url, function_hooks);

    let operator_signature = operator_signature.encode(config.operator_token.as_ref());

    if operator_signature.is_err() {
        println!("{:?}", operator_signature.err().unwrap());
        std::process::exit(1);
    }

    println!("{}", operator_signature.unwrap());
}