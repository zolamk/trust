use crate::{
    config::Config, handlers::users::provider::{Provider, UserProvidedData}
};
use reqwest::{Client, Error};
use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Deserialize)]
struct GoogleUser {
    pub email: Option<String>,
    pub name: Option<String>,
    pub verified_email: bool,
    pub picture: Option<String>,
}

#[derive(Clone)]
pub struct GoogleProvider {
    config: Config,
}

impl GoogleProvider {
    pub fn new(config: Config) -> GoogleProvider {
        return GoogleProvider { config };
    }
}

impl Provider for GoogleProvider {
    fn enabled(&self) -> bool {
        return self.config.google_enabled;
    }

    fn auth_url(&self) -> String {
        return String::from("https://accounts.google.com/o/oauth2/auth");
    }

    fn token_url(&self) -> String {
        return String::from("https://oauth2.googleapis.com/token");
    }

    fn client_id(&self) -> String {
        return self.config.google_client_id.clone().unwrap();
    }

    fn client_secret(&self) -> String {
        return self.config.google_client_secret.clone().unwrap();
    }

    fn scopes(&self) -> Vec<String> {
        return vec![String::from("email"), String::from("profile")];
    }

    fn get_user_data(&self, access_token: String) -> Result<UserProvidedData, Error> {
        let client = Client::new();

        let mut response = client.get("https://www.googleapis.com/oauth2/v1/userinfo?alt=json").bearer_auth(access_token).send()?;

        let data: GoogleUser = response.json()?;

        let mut metadata = Map::<String, Value>::new();

        if let Some(avatar_url) = data.picture {
            metadata.insert("avatar".to_string(), Value::String(avatar_url));
        }

        if let Some(name) = data.name {
            metadata.insert("name".to_string(), Value::String(name));
        }

        return Ok(UserProvidedData {
            email: data.email,
            verified: data.verified_email,
            metadata: Some(Value::Object(metadata)),
        });
    }
}
