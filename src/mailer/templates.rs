extern crate handlebars;
extern crate lettre;
extern crate lettre_email;

use crate::config::Config;
use std::fs::read_to_string;
use std::path::Path;

static DEFAULT_CONFIRMATION_EMAIL: &str = "<h2>Confirm your signup</h2><p>Follow this link to confirm your signup</p><p><a href='{{ confirmation_url }}'>Confirm</a></p>";

static DEFAULT_INVITATION_EMAIL: &str ="<h2>You've Been Invited</h2><p>Follow this link to accept your invitation</p><p><a href='{{ invitation_url }}'>Accept Invite</a></p>";

#[derive(Clone)]
pub struct EmailTemplates {
    config: Config,
    confirmation_email: String,
    invitation_email: String,
}

impl EmailTemplates {
    pub fn new(config: Config) -> EmailTemplates {
        let confirmation_email = DEFAULT_CONFIRMATION_EMAIL.to_string();

        let invitation_email = DEFAULT_INVITATION_EMAIL.to_string();

        let mut email_templates = EmailTemplates {
            config,
            confirmation_email,
            invitation_email,
        };

        if email_templates
            .config
            .mailer_template_confirmation
            .is_some()
        {
            email_templates.confirmation_email = read_to_string(Path::new(
                &email_templates
                    .config
                    .mailer_template_confirmation
                    .clone()
                    .unwrap(),
            ))
            .unwrap();
        }

        if email_templates.config.mailer_template_invitation.is_some() {
            email_templates.invitation_email = read_to_string(Path::new(
                &email_templates
                    .config
                    .mailer_template_invitation
                    .clone()
                    .unwrap(),
            ))
            .unwrap();
        }

        return email_templates;
    }

    pub fn confirmation_email_template(self) -> String {
        return self.confirmation_email;
    }

    pub fn invitation_email_template(self) -> String {
        return self.invitation_email;
    }
}
