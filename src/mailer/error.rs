#[derive(Debug)]
pub enum Error {
    TemplateError(handlebars::TemplateRenderError),
    EmailError(lettre_email::error::Error),
    SMTPError(lettre::smtp::error::Error),
}

impl From<handlebars::TemplateRenderError> for Error {
    fn from(e: handlebars::TemplateRenderError) -> Self {
        return Error::TemplateError(e);
    }
}

impl From<lettre_email::error::Error> for Error {
    fn from(e: lettre_email::error::Error) -> Self {
        return Error::EmailError(e);
    }
}

impl From<lettre::smtp::error::Error> for Error {
    fn from(e: lettre::smtp::error::Error) -> Self {
        return Error::SMTPError(e);
    }
}
