use serde_json;

pub struct UserProvidedData {
    pub email: String,
    pub verified: bool,
    pub metadata: Option<serde_json::Value>,
}
