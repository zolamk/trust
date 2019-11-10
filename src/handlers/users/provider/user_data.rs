use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct UserProvidedData {
    pub email: Option<String>,
    pub verified: bool,
    pub metadata: Option<Value>,
}
