use crate::config::Config;
use crate::handlers::users::provider::Provider;
use crate::handlers::users::provider::UserProvidedData;
use oauth2::Scope;
use reqwest::Error;

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
    fn enabled(self) -> bool {
        return self.config.facebook_enabled;
    }

    fn auth_url(self) -> String {
        return String::from("https://www.facebook.com/v5.0/dialog/oauth");
    }

    fn token_url(self) -> String {
        return String::from("https://www.facebook.com/v5.0/dialog/oauth");
    }

    fn client_id(self) -> String {
        return self.config.facebook_client_id.unwrap();
    }

    fn client_secret(self) -> String {
        return self.config.facebook_client_secret.unwrap();
    }

    fn scopes(self) -> Vec<String> {
        return vec![String::from("email")];
    }

    fn get_user_data(self) -> Result<UserProvidedData, Error> {
        unimplemented!();
    }
}
