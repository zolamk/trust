mod facebook;
mod github;
mod google;
mod provider;
mod state;
mod user_data;

use crate::handlers::Error;
pub use facebook::FacebookProvider;
pub use github::GithubProvider;
pub use google::GoogleProvider;
pub use provider::Provider;
use rocket::response::{status, Redirect};
use rocket_contrib::json::JsonValue;
pub use state::ProviderState;
pub use user_data::UserProvidedData;

#[derive(Debug, Responder)]
pub enum ProviderResponse {
    Redirect(Redirect),
    Other(Result<status::Custom<JsonValue>, Error>),
}
