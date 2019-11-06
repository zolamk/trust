use crate::handlers::users::provider::user_data::UserProvidedData;
use oauth2::Scope;
use reqwest::Error;

pub trait Provider {
    fn enabled(self) -> bool;
    fn auth_url(self) -> String;
    fn token_url(self) -> String;
    fn client_id(self) -> String;
    fn client_secret(self) -> String;
    fn scopes(self) -> Vec<String>;
    fn get_user_data(self) -> Result<UserProvidedData, Error>;
}
