extern crate rocket;

mod state;

mod facebook;
mod provider;
mod user_data;

use crate::handlers::Error;
pub use facebook::FacebookProvider;
pub use provider::Provider;
use rocket::response::status;
use rocket::response::Redirect;
use rocket_contrib::json::JsonValue;
pub use state::ProviderState;
pub use user_data::UserProvidedData;

#[derive(Debug, Responder)]
pub enum ProviderResponse {
    Redirect(Redirect),
    Other(Result<status::Custom<JsonValue>, Error>),
}
