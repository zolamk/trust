mod error;
mod send;
mod templates;

pub use error::Error;
pub use send::send_confirmation_email;
pub use send::send_invitation_email;
pub use templates::EmailTemplates;
