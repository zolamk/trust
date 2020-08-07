use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserProvidedData {
    pub email: Option<String>,
    pub verified: bool,
    pub name: Option<String>,
    pub avatar: Option<String>,
}
