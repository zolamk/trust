#[derive(Debug)]
pub struct HookError {
    pub code: u16,
    pub body: serde_json::Value,
}

#[derive(Debug)]
pub enum Error {
    JWTError(frank_jwt::Error),
    JSONError(serde_json::Error),
    RequestError(reqwest::Error),
    HookError(HookError),
}

impl From<frank_jwt::Error> for Error {
    fn from(e: frank_jwt::Error) -> Self {
        return Error::JWTError(e);
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::JSONError(e);
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        return Error::RequestError(e);
    }
}

impl From<HookError> for Error {
    fn from(e: HookError) -> Self {
        return Error::HookError(e);
    }
}
