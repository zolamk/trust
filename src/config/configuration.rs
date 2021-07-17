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
    pub extra: HashMap<String, String>,
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

    pub jwt_secret: Option<String>,

    #[serde(default = "default_log_level")]
    pub log_level: String,

    pub confirmation_email_template_path: Option<String>,

    pub recovery_email_template_path: Option<String>,

    pub change_email_template_path: Option<String>,

    pub invitation_email_template_path: Option<String>,

    pub confirmation_email_subject: Option<String>,

    pub recovery_email_subject: Option<String>,

    pub change_email_subject: Option<String>,

    pub invitation_email_subect: Option<String>,

    mailer_template_confirmation: Option<String>,

    mailer_template_recovery: Option<String>,

    mailer_template_change: Option<String>,

    mailer_template_invite: Option<String>,

    pub confirmation_sms_template: Option<String>,

    pub recovery_sms_template: Option<String>,

    pub change_phone_sms_template: Option<String>,

    pub invitation_sms_template: Option<String>,

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

    #[serde(default = "default_disable_email")]
    pub disable_email: bool,

    #[serde(default = "default_password_hash_cost")]
    pub password_hash_cost: u32,

    #[serde(default = "max_connection_pool_size")]
    pub max_connection_pool_size: u32,

    pub sms: Option<SMSConfig>,

    #[serde(skip_serializing, skip_deserializing)]
    private_key: Option<String>,

    #[serde(skip_serializing, skip_deserializing)]
    public_key: Option<String>,

    #[serde(skip_serializing, skip_deserializing)]
    jwt_type: String,

    #[serde(default = "default_admin_only_list")]
    pub admin_only_list: bool,

    #[serde(default = "default_minutes_between_resend")]
    pub minutes_between_resend: i64,

    pub login_hook: Option<String>,
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
            if config.jwt_secret.is_none() {
                panic!("JWT_SECRET env variable not set");
            }

            config.jwt_type = String::from("symmetric");
        }
        other => {
            error!("unsupported algorithm {}", other);
            exit(1);
        }
    }

    if config.confirmation_email_template_path.is_some() {
        config.mailer_template_confirmation = match fs::read_to_string(Path::new(&config.confirmation_email_template_path.clone().unwrap())) {
            Ok(key) => Some(key),
            Err(err) => {
                panic!("unable to read confirmation template file: {}", err);
            }
        }
    }

    if config.change_email_template_path.is_some() {
        config.mailer_template_change = match fs::read_to_string(Path::new(&config.change_email_template_path.clone().unwrap())) {
            Ok(key) => Some(key),
            Err(err) => {
                panic!("unable to read change email template file: {}", err);
            }
        }
    }

    if config.recovery_email_template_path.is_some() {
        config.mailer_template_recovery = match fs::read_to_string(Path::new(&config.recovery_email_template_path.clone().unwrap())) {
            Ok(key) => Some(key),
            Err(err) => {
                panic!("unable to read recovery email template file: {}", err);
            }
        }
    }

    if config.invitation_email_template_path.is_some() {
        config.mailer_template_invite = match fs::read_to_string(Path::new(&config.invitation_email_template_path.clone().unwrap())) {
            Ok(key) => Some(key),
            Err(err) => {
                panic!("unable to read invitation email template file: {}", err);
            }
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

    if config.disable_email && config.disable_phone {
        panic!("can't disable phone and email at the same time");
    }

    return config;
}

impl Config {
    #[allow(dead_code)]
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
        let h = HoconLoader::new().load_file(".conf");

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
        return self.jwt_secret.unwrap();
    }

    pub fn get_decoding_key(self) -> String {
        if self.jwt_type.eq("assymetric") {
            return self.get_public_key();
        }
        return self.jwt_secret.unwrap();
    }

    pub fn get_confirmation_email_template(&self) -> String {
        if self.mailer_template_confirmation.is_none() {
            return "<h2>Confirm your email</h2><p>Follow this link to confirm your email</p><p><a href='{{ site_url }}?confirmation_token={{ confirmation_token }}'>Confirm</a></p>".to_string();
        }

        return self.mailer_template_confirmation.clone().unwrap();
    }

    pub fn get_invitation_email_template(&self) -> String {
        if self.mailer_template_invite.is_none() {
            return "<h2>You have been invited</h2><p>Follow this link to accept your invitation</p><p><a href='{{ site_url }}?invitation_token={{ invitation_token }}'>Accept Invite</a></p>"
                .to_string();
        }

        return self.mailer_template_invite.clone().unwrap();
    }

    pub fn get_invitation_email_subject(&self) -> String {
        if self.invitation_email_subect.is_none() {
            return "You've been invited".to_string();
        }

        return self.invitation_email_subect.clone().unwrap();
    }

    pub fn get_confirmation_email_subject(&self) -> String {
        if self.confirmation_email_subject.is_none() {
            return "Confirm Your Account".to_string();
        }

        return self.confirmation_email_subject.clone().unwrap();
    }

    pub fn get_recovery_email_template(&self) -> String {
        if self.mailer_template_recovery.is_none() {
            return "<h2>Recover Your Account</h2><p>Follow this link to recover you account</p><p><a href='{{ site_url }}?recovery_token={{ recovery_token }}'>Recover</a></p>".to_string();
        }

        return self.mailer_template_recovery.clone().unwrap();
    }

    pub fn get_recovery_email_subject(&self) -> String {
        if self.recovery_email_subject.is_none() {
            return "Recover Your Account".to_string();
        }

        return self.recovery_email_subject.clone().unwrap();
    }

    pub fn get_change_email_template(&self) -> String {
        if self.mailer_template_change.is_none() {
            return "<h2>Change Your Email Address<h2><p>Follow this link to confirm your email address change</p><p><a href='{{ site_url }}?change_email_token={{ change_email_token }}'>Confirm</a></p>".to_string();
        }

        return self.mailer_template_change.clone().unwrap();
    }

    pub fn get_change_email_subject(&self) -> String {
        if self.change_email_subject.is_none() {
            return "Confirm Email Change".to_string();
        }

        return self.change_email_subject.clone().unwrap();
    }

    pub fn get_invitation_sms_template(&self) -> String {
        if self.invitation_sms_template.is_none() {
            return "Invitation acceptance code - {{ invitation_token }}".to_string();
        }

        return self.invitation_sms_template.clone().unwrap();
    }

    pub fn get_confirmation_sms_template(&self) -> String {
        if self.confirmation_sms_template.is_none() {
            return "Phone confirmation code - {{ confirmation_token }}".to_string();
        }

        return self.confirmation_sms_template.clone().unwrap();
    }

    pub fn get_recovery_sms_template(&self) -> String {
        if self.recovery_sms_template.is_none() {
            return "Phone recovery code - {{ recovery_token }}".to_string();
        }

        return self.recovery_sms_template.clone().unwrap();
    }

    pub fn get_change_phone_sms_template(&self) -> String {
        if self.change_phone_sms_template.is_none() {
            return "Phone change code -  {{ phone_change_token }}".to_string();
        }

        return self.change_phone_sms_template.clone().unwrap();
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

fn default_disable_email() -> bool {
    false
}

fn default_admin_only_list() -> bool {
    true
}

fn default_log_level() -> String {
    "error".to_string()
}

fn default_password_hash_cost() -> u32 {
    10
}

fn max_connection_pool_size() -> u32 {
    10
}

fn default_minutes_between_resend() -> i64 {
    1
}
