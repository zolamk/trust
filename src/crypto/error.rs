use frank_jwt;
use serde_json;

#[derive(Debug)]
pub enum Error {
    JWTError(frank_jwt::Error),
    JSONError(serde_json::Error),
    TokenMissing,
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
