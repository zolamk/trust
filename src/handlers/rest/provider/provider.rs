use crate::handlers::rest::provider::user_data::UserProvidedData;
use reqwest::Error;

pub trait Provider {
    fn enabled(&self) -> bool;
    fn auth_url(&self) -> String;
    fn token_url(&self) -> String;
    fn client_id(&self) -> String;
    fn client_secret(&self) -> String;
    fn scopes(&self) -> Vec<String>;
    fn get_user_data(&self, access_token: String) -> Result<UserProvidedData, Error>;
}
