extern crate diesel;
extern crate frank_jwt;
extern crate handlebars;
extern crate lettre;
extern crate lettre_email;
extern crate serde_json;

#[derive(Debug)]
pub enum Error {
    HookError(crate::hook::Error),
    DieselError(diesel::result::Error),
    EmailError(lettre_email::error::Error),
    TemplateError(handlebars::TemplateRenderError),
    SMTPError(lettre::smtp::error::Error),
    JWTError(frank_jwt::Error),
    JSONError(serde_json::Error),
    MissingOperatorSignature,
    InvalidOperatorSignature,
}

impl From<crate::hook::Error> for Error {
    fn from(e: crate::hook::Error) -> Self {
        return Error::HookError(e);
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        return Error::DieselError(e);
    }
}

impl From<lettre_email::error::Error> for Error {
    fn from(e: lettre_email::error::Error) -> Self {
        return Error::EmailError(e);
    }
}

impl From<handlebars::TemplateRenderError> for Error {
    fn from(e: handlebars::TemplateRenderError) -> Self {
        return Error::TemplateError(e);
    }
}

impl From<lettre::smtp::error::Error> for Error {
    fn from(e: lettre::smtp::error::Error) -> Self {
        return Error::SMTPError(e);
    }
}

impl From<frank_jwt::Error> for Error {
    fn from(e: frank_jwt::Error) -> Self {
        return Error::JWTError(e);
    }
}
