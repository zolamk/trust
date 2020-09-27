use crate::config::Config;
use std::{fs::read_to_string, path::Path};

static DEFAULT_CONFIRMATION_SMS: &str = "Your Phone Confirmation Code Is\n{{ confirmation_code }}";

static DEFAULT_RECOVERY_SMS: &str = "Your Account Recovery Code Is\n{{ recovery_code }}";

#[derive(Clone)]
pub struct SMSTemplates {
    config: Config,
    confirmation_sms: String,
    recovery_sms: String,
}

impl SMSTemplates {
    pub fn new(config: Config) -> SMSTemplates {
        let confirmation_sms = DEFAULT_CONFIRMATION_SMS.to_string();

        let recovery_sms = DEFAULT_RECOVERY_SMS.to_string();

        let mut sms_templates = SMSTemplates {
            config,
            confirmation_sms,
            recovery_sms,
        };

        if sms_templates.config.sms_template_confirmation.is_some() {
            sms_templates.confirmation_sms = read_to_string(Path::new(&sms_templates.config.sms_template_confirmation.clone().unwrap())).unwrap();
        }

        if sms_templates.config.mailer_template_recovery.is_some() {
            sms_templates.recovery_sms = read_to_string(Path::new(&sms_templates.config.sms_template_recovery.clone().unwrap())).unwrap();
        }

        return sms_templates;
    }

    pub fn confirmation_sms_template(self) -> String {
        return self.confirmation_sms;
    }

    pub fn recovery_sms_template(self) -> String {
        return self.recovery_sms;
    }
}
