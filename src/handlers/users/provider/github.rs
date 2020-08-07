use crate::{
    config::Config,
    handlers::users::provider::{Provider, UserProvidedData},
};
use reqwest::{Client, Error};
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
struct GithubUser {
    pub email: Option<String>,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Deserialize)]
struct GithubUserEmail {
    pub email: String,
    pub primary: bool,
    pub verified: bool,
}

#[derive(Clone)]
pub struct GithubProvider {
    config: Config,
}

impl GithubProvider {
    pub fn new(config: Config) -> GithubProvider {
        return GithubProvider { config };
    }
}

impl Provider for GithubProvider {
    fn enabled(&self) -> bool {
        return self.config.github_enabled;
    }

    fn auth_url(&self) -> String {
        return String::from("https://github.com/login/oauth/authorize");
    }

    fn token_url(&self) -> String {
        return String::from("https://github.com/login/oauth/access_token");
    }

    fn client_id(&self) -> String {
        return self.config.github_client_id.clone().unwrap();
    }

    fn client_secret(&self) -> String {
        return self.config.github_client_secret.clone().unwrap();
    }

    fn scopes(&self) -> Vec<String> {
        return vec![String::from("user:email")];
    }

    fn get_user_data(&self, access_token: String) -> Result<UserProvidedData, Error> {
        let client = Client::new();

        let mut response = client.get("https://api.github.com/user").bearer_auth(access_token.clone()).send()?;

        let user: GithubUser = response.json()?;

        response = client.get("https://api.github.com/user/emails").bearer_auth(access_token).send()?;

        let emails: Vec<GithubUserEmail> = response.json()?;

        let mut data = UserProvidedData {
            verified: false,
            email: None,
            name: user.name,
            avatar: user.avatar_url,
        };

        for email in emails {
            if email.primary {
                data.email = Some(email.email);
                data.verified = email.verified;
                break;
            }
        }

        return Ok(data);
    }
}
