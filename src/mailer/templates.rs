use crate::config::Config;
use std::{fs::read_to_string, path::Path};

static DEFAULT_CONFIRMATION_EMAIL: &str = "<h2>Confirm your email</h2><p>Follow this link to confirm your email</p><p><a href='{{ confirmation_url }}'>Confirm</a></p>";

static DEFAULT_INVITATION_EMAIL: &str = "<h2>You've Been Invited</h2><p>Follow this link to accept your invitation</p><p><a href='{{ invitation_url }}'>Accept Invite</a></p>";

static DEFAULT_RECOVERY_EMAIL: &str = "<h2>Recover Your Account</h2><p>Follow this link to recover you account</p><p><a href='{{ recovery_url }}'>Recover</a></p>";

#[derive(Clone)]
pub struct EmailTemplates {
    config: Config,
    confirmation_email: String,
    invitation_email: String,
    recovery_email: String,
}

impl EmailTemplates {
    pub fn new(config: Config) -> EmailTemplates {
        let confirmation_email = DEFAULT_CONFIRMATION_EMAIL.to_string();

        let invitation_email = DEFAULT_INVITATION_EMAIL.to_string();

        let recovery_email = DEFAULT_RECOVERY_EMAIL.to_string();

        let mut email_templates = EmailTemplates {
            config,
            confirmation_email,
            invitation_email,
            recovery_email,
        };

        if email_templates.config.mailer_template_confirmation.is_some() {
            email_templates.confirmation_email = read_to_string(Path::new(&email_templates.config.mailer_template_confirmation.clone().unwrap())).unwrap();
        }

        if email_templates.config.mailer_template_invitation.is_some() {
            email_templates.invitation_email = read_to_string(Path::new(&email_templates.config.mailer_template_invitation.clone().unwrap())).unwrap();
        }

        if email_templates.config.mailer_template_recovery.is_some() {
            email_templates.recovery_email = read_to_string(Path::new(&email_templates.config.mailer_template_recovery.clone().unwrap())).unwrap();
        }

        return email_templates;
    }

    pub fn confirmation_email_template(self) -> String {
        return self.confirmation_email;
    }

    pub fn invitation_email_template(self) -> String {
        return self.invitation_email;
    }

    pub fn recovery_email_template(self) -> String {
        return self.recovery_email;
    }
}
