use dotenv::dotenv;
use log::error;
use regex::Regex;
use serde::Deserialize;
use std::{fs, path::Path, process::exit};

#[derive(Deserialize, Clone)]
pub struct Config {
    pub aud: String,
    pub auto_confirm: bool,
    pub confirmed_redirect: String,
    pub database_url: String,
    pub db_driver: String,
    #[serde(default = "default_disable_signup")]
    pub disable_signup: bool,
    pub facebook_client_id: Option<String>,
    pub facebook_client_secret: Option<String>,
    pub facebook_enabled: bool,
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
    pub google_enabled: bool,
    pub github_enabled: bool,
    pub github_client_id: Option<String>,
    pub github_client_secret: Option<String>,
    pub host: String,
    pub instance_url: String,
    pub jwt_algorithm: String,
    #[serde(default = "default_jwt_exp")]
    pub jwt_exp: i64,
    pub jwt_private_key_path: Option<String>,
    pub jwt_public_key_path: Option<String>,
    pub jwt_secret: String,
    pub log_level: String,
    pub mailer_template_confirmation: Option<String>,
    pub mailer_template_email_change: Option<String>,
    pub mailer_template_invitation: Option<String>,
    pub mailer_template_recovery: Option<String>,
    pub mailer_template_revert: Option<String>,
    pub operator_token: String,
    pub port: u16,
    #[serde(default = "default_password_rule")]
    #[serde(with = "serde_regex")]
    pub password_rule: Regex,
    pub site_url: String,
    pub smtp_admin_email: String,
    pub smtp_host: String,
    pub smtp_password: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    #[serde(skip_serializing, skip_deserializing)]
    private_key: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    public_key: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    jwt_type: String,
}

impl Config {
    pub fn new() -> Config {
        dotenv().ok();

        match envy::from_env::<Config>() {
            Ok(mut config) => {
                match config.jwt_algorithm.as_ref() {
                    "RS256" | "RS384" | "RS512" | "ES256" | "ES384" | "ES512" => {
                        assert_eq!(
                            config.jwt_private_key_path.is_some(),
                            true,
                            "expected JWT_PRIVATE_KEY_PATH to be set for all supported assymetric algorithms"
                        );

                        assert_eq!(
                            config.jwt_public_key_path.is_some(),
                            true,
                            "expected JWT_PUBLIC_KEY_PATH to be set for all supported assymetric algorithms"
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
                        std::process::exit(1);
                    }
                }

                if config.google_enabled {
                    assert_eq!(config.google_client_id.is_some(), true, "expected GOOGLE_CLIENT_ID to be set if google provider is enabled");

                    assert_eq!(config.google_client_secret.is_some(), true, "expected GOOGLE_CLIENT_SECRET to be set if google provider is enabled")
                }

                if config.facebook_enabled {
                    assert_eq!(config.facebook_client_id.is_some(), true, "expected FACEBOOK_CLIENT_ID to be set if facebook provider is enabled");

                    assert_eq!(config.facebook_client_secret.is_some(), true, "expected FACEBOOK_CLIENT_SECRET to be set if google provider is enabled")
                }

                if config.github_enabled {
                    assert_eq!(config.github_client_id.is_some(), true, "expected GITHUB_CLIENT_ID to be set if github provider is enabled");

                    assert_eq!(config.github_client_secret.is_some(), true, "expected GITHUB_CLIENT_SECRET to be set if github provider is enabled")
                }

                return config;
            }
            Err(e) => {
                println!("{}", e);
                exit(1);
            }
        }
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

fn default_jwt_exp() -> i64 {
    3600
}

fn default_password_rule() -> Regex {
    regex::Regex::new(r".{8,1000}").unwrap()
}
