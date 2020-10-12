use hocon::HoconLoader;
use log::error;
use regex::Regex;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path, process::exit};

#[derive(Deserialize, Clone, Debug)]
pub struct SMSMapping {
    pub source: String,
    pub destination: String,
    pub message: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SMSConfig {
    pub url: String,
    pub method: String,
    pub source: String,
    pub mapping: SMSMapping,
    pub headers: HashMap<String, String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub aud: String,

    #[serde(default = "default_auto_confirm")]
    pub auto_confirm: bool,

    pub database_url: String,

    #[serde(default = "default_disable_signup")]
    pub disable_signup: bool,

    #[serde(default = "default_social_enabled")]
    pub facebook_enabled: bool,

    #[serde(default = "default_social_enabled")]
    pub google_enabled: bool,

    #[serde(default = "default_social_enabled")]
    pub github_enabled: bool,

    pub facebook_client_id: Option<String>,

    pub facebook_client_secret: Option<String>,

    pub google_client_id: Option<String>,

    pub google_client_secret: Option<String>,

    pub github_client_id: Option<String>,

    pub github_client_secret: Option<String>,

    #[serde(default = "default_host")]
    pub host: String,

    pub instance_url: String,

    pub jwt_algorithm: String,

    #[serde(default = "default_jwt_exp")]
    pub jwt_exp: i64,

    pub jwt_private_key_path: Option<String>,

    pub jwt_public_key_path: Option<String>,

    pub jwt_secret: String,

    #[serde(default = "default_log_level")]
    pub log_level: String,

    pub mailer_template_confirmation: Option<String>,

    pub mailer_template_recovery: Option<String>,

    pub sms_template_confirmation: Option<String>,

    pub sms_template_recovery: Option<String>,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_password_rule")]
    #[serde(with = "serde_regex")]
    pub password_rule: Regex,

    #[serde(default = "default_email_rule")]
    #[serde(with = "serde_regex")]
    pub email_rule: Regex,

    #[serde(default = "default_phone_rule")]
    #[serde(with = "serde_regex")]
    pub phone_rule: Regex,

    pub site_url: String,

    pub smtp_admin_email: String,

    pub smtp_host: String,

    pub smtp_password: String,

    pub smtp_port: u16,

    pub smtp_username: String,

    #[serde(default = "default_disable_phone")]
    pub disable_phone: bool,

    pub sms: Option<SMSConfig>,

    #[serde(skip_serializing, skip_deserializing)]
    private_key: Option<String>,

    #[serde(skip_serializing, skip_deserializing)]
    public_key: Option<String>,

    #[serde(skip_serializing, skip_deserializing)]
    jwt_type: String,
}

fn complete(c: Config) -> Config {
    let mut config = c;

    match config.jwt_algorithm.as_ref() {
        "RS256" | "RS384" | "RS512" | "ES256" | "ES384" | "ES512" => {
            assert_eq!(
                config.jwt_private_key_path.is_some(),
                true,
                "expected jwt_private_key_path to be set for all supported assymetric algorithms"
            );

            assert_eq!(
                config.jwt_public_key_path.is_some(),
                true,
                "expected jwt_public_key_path to be set for all supported assymetric algorithms"
            );

            config.private_key = match fs::read_to_string(Path::new(&config.jwt_private_key_path.clone().unwrap())) {
                Ok(key) => Some(key),
                Err(err) => {
                    panic!("unable to read private key file: {}", err);
                }
            };

            config.public_key = match fs::read_to_string(Path::new(&config.jwt_public_key_path.clone().unwrap())) {
                Ok(key) => Some(key),
                Err(err) => {
                    panic!("unable to read public key file: {}", err);
                }
            };

            config.jwt_type = String::from("assymetric");
        }
        "HS256" | "HS384" | "HS512" => {
            config.jwt_type = String::from("symmetric");
        }
        other => {
            error!("unsupported algorithm {}", other);
            exit(1);
        }
    }

    if config.google_enabled {
        assert_eq!(config.google_client_id.is_some(), true, "expected \"google_client_id\" to be set if google provider is enabled");

        assert_eq!(config.google_client_secret.is_some(), true, "expected \"google_client_secret\" to be set if google provider is enabled")
    }

    if config.facebook_enabled {
        assert_eq!(config.facebook_client_id.is_some(), true, "expected \"facebook_client_id\" to be set if facebook provider is enabled");

        assert_eq!(
            config.facebook_client_secret.is_some(),
            true,
            "expected \"facebook_client_secret\" to be set if google provider is enabled"
        )
    }

    if config.github_enabled {
        assert_eq!(config.github_client_id.is_some(), true, "expected \"github_client_id\" to be set if github provider is enabled");

        assert_eq!(config.github_client_secret.is_some(), true, "expected \"github_client_secret\" to be set if github provider is enabled");
    }

    if !config.disable_phone && config.sms.is_none() {
        panic!("expected \"sms\" to be set if phone is enabled");
    }

    println!("{:?}", config);

    return config;
}

impl Config {
    pub fn new_from_string(s: String) -> Config {
        let h = HoconLoader::new().load_str(s.as_ref());

        if h.is_err() {
            let err = h.err().unwrap();
            panic!("{:?}", err);
        }

        let h = h.unwrap();

        let config = h.resolve();

        if config.is_err() {
            let err = config.err().unwrap();
            panic!("{:?}", err);
        }

        let config: Config = config.unwrap();

        return complete(config);
    }

    pub fn new() -> Config {
        let h = HoconLoader::new().load_file(".conf").expect("expected to find \".conf\" configuration file");

        let config = h.resolve();

        if config.is_err() {
            let err = config.err().unwrap();
            panic!("{:?}", err);
        }

        let config: Config = config.unwrap();

        return complete(config);
    }

    fn get_private_key(self) -> String {
        return self.private_key.unwrap();
    }

    fn get_public_key(self) -> String {
        return self.public_key.unwrap();
    }

    pub fn get_signing_key(self) -> String {
        if self.jwt_type.eq("assymetric") {
            return self.get_private_key();
        }
        return self.jwt_secret;
    }

    pub fn get_decoding_key(self) -> String {
        if self.jwt_type.eq("assymetric") {
            return self.get_public_key();
        }
        return self.jwt_secret;
    }
}

fn default_disable_signup() -> bool {
    false
}

fn default_auto_confirm() -> bool {
    false
}

fn default_jwt_exp() -> i64 {
    3600
}

fn default_password_rule() -> Regex {
    regex::Regex::new(r".{8,1000}").unwrap()
}

fn default_email_rule() -> Regex {
    regex::Regex::new(r"^[\w\-\.]+@([\w\-]+\.)+[\w\-]{1,}$").unwrap()
}

fn default_phone_rule() -> Regex {
    regex::Regex::new(r"\+\d{5,15}").unwrap()
}

fn default_host() -> String {
    "localhost".to_string()
}

fn default_port() -> u16 {
    1995
}

fn default_social_enabled() -> bool {
    false
}

fn default_disable_phone() -> bool {
    false
}

fn default_log_level() -> String {
    "error".to_string()
}
