mod error;
mod send;
mod templates;

pub use error::Error;
pub use send::send_email;
pub use templates::EmailTemplates;
