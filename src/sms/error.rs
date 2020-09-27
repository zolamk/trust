#[derive(Debug)]
pub enum Error {
    TemplateError(handlebars::TemplateRenderError),
    SMSError(reqwest::Error),
    SMSResponseError,
    InvalidMethodError(oauth2::http::method::InvalidMethod),
}

impl From<handlebars::TemplateRenderError> for Error {
    fn from(e: handlebars::TemplateRenderError) -> Self {
        return Error::TemplateError(e);
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        return Error::SMSError(e);
    }
}

impl From<oauth2::http::method::InvalidMethod> for Error {
    fn from(e: oauth2::http::method::InvalidMethod) -> Self {
        return Error::InvalidMethodError(e);
    }
}
