use crate::{
    config::Config,
    handlers::users::provider::{Provider, UserProvidedData},
};
use reqwest::{blocking::Client, Error};
use serde::Deserialize;

#[derive(Deserialize)]
struct PictureData {
    pub is_silhouette: bool,
    pub url: String,
}

#[derive(Deserialize)]
struct Data {
    pub data: PictureData,
}

#[derive(Deserialize)]
struct FacebookUser {
    pub email: Option<String>,
    pub name: Option<String>,
    pub picture: Data,
}

#[derive(Clone)]
pub struct FacebookProvider {
    config: Config,
}

impl FacebookProvider {
    pub fn new(config: Config) -> FacebookProvider {
        return FacebookProvider { config };
    }
}

impl Provider for FacebookProvider {
    fn enabled(&self) -> bool {
        return self.config.facebook_enabled;
    }

    fn auth_url(&self) -> String {
        return String::from("https://www.facebook.com/v5.0/dialog/oauth");
    }

    fn token_url(&self) -> String {
        return String::from("https://graph.facebook.com/v5.0/oauth/access_token");
    }

    fn client_id(&self) -> String {
        return self.config.facebook_client_id.clone().unwrap();
    }

    fn client_secret(&self) -> String {
        return self.config.facebook_client_secret.clone().unwrap();
    }

    fn scopes(&self) -> Vec<String> {
        return vec![String::from("email")];
    }

    fn get_user_data(&self, access_token: String) -> Result<UserProvidedData, Error> {
        let client = Client::new();

        let response = client
            .get("https://graph.facebook.com/me?fields=name,email,picture{url,is_silhouette}")
            .bearer_auth(access_token)
            .send()?;

        let data: FacebookUser = response.json()?;

        let d = UserProvidedData {
            email: data.email,
            verified: true,
            name: data.name,
            avatar: if !data.picture.data.is_silhouette { Some(data.picture.data.url) } else { None },
        };

        return Ok(d);
    }
}
