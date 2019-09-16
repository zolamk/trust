extern crate dotenv;
extern crate envy;
extern crate serde;

use dotenv::dotenv;
use serde::Deserialize;
use std::path::PathBuf;
use std::process::exit;

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub site_url: String,
    pub database_url: String,
    pub db_driver: String,
    pub log_level: String,
    pub log_file: Option<PathBuf>,
    pub facebook_enabled: bool,
    pub facebook_client_id: Option<String>,
    pub facebook_client_secret: Option<String>,
    pub google_enabled: bool,
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
    pub smtp_host: String,
    pub smtp_port: u32,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_admin_email: String,
    pub auto_confirm: bool,
}

impl Config {
    pub fn new() -> Config {
        dotenv().ok();

        match envy::from_env::<Config>() {
            Ok(config) => config,
            Err(e) => {
                println!("{}", e);
                exit(1);
            }
        }
    }
}
