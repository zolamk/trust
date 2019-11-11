use crate::{config::Config, mailer::Error, models::user::User};
use handlebars::Handlebars;
use lettre::{
    smtp::{
        authentication::{Credentials, Mechanism},
        ConnectionReuseParameters, SmtpClient,
    },
    ClientSecurity, ClientTlsParameters, Transport,
};
use lettre_email::Email;
use native_tls::TlsConnector;

pub fn send_confirmation_email(template: String, confirmation_url: String, user: &User, config: &Config) -> Result<(), Error> {
    let tls_connector = TlsConnector::builder().build().unwrap();

    let tls_parameters = ClientTlsParameters::new(config.smtp_host.to_string(), tls_connector);

    let credentials = Credentials::new(config.smtp_username.to_string(), config.smtp_password.to_string());

    let mut mailer = SmtpClient::new((&config.smtp_host[..], config.smtp_port), ClientSecurity::Required(tls_parameters))
        .unwrap()
        .authentication_mechanism(Mechanism::Login)
        .credentials(credentials)
        .timeout(Some(std::time::Duration::new(10, 0)))
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .transport();

    let confirmation_email = Handlebars::new().render_template(
        &template,
        &json!({
            "email": user.email.clone(),
            "site_url": config.site_url.clone(),
            "confirmation_url": confirmation_url,
        }),
    );

    if confirmation_email.is_err() {
        return Err(Error::from(confirmation_email.err().unwrap()));
    }

    let email = Email::builder().from(config.smtp_admin_email.clone()).to(user.email.clone()).html(confirmation_email.unwrap()).build();

    if email.is_err() {
        return Err(Error::from(email.err().unwrap()));
    }

    let email = mailer.send(email.unwrap().into());

    if email.is_err() {
        return Err(Error::from(email.err().unwrap()));
    }

    return Ok(());
}

pub fn send_invitation_email(template: String, invitation_url: String, user: &User, config: &Config) -> Result<(), Error> {
    let tls_connector = TlsConnector::builder().build().unwrap();

    let tls_parameters = ClientTlsParameters::new(config.smtp_host.to_string(), tls_connector);

    let credentials = Credentials::new(config.smtp_username.to_string(), config.smtp_password.to_string());

    let mut mailer = SmtpClient::new((&config.smtp_host[..], config.smtp_port), ClientSecurity::Required(tls_parameters))
        .unwrap()
        .authentication_mechanism(Mechanism::Login)
        .credentials(credentials)
        .timeout(Some(std::time::Duration::new(10, 0)))
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .transport();

    let invitation_email = Handlebars::new().render_template(
        &template,
        &json!({
            "email": user.email.clone(),
            "site_url": config.site_url.clone(),
            "invitation_url": invitation_url,
        }),
    );

    if invitation_email.is_err() {
        return Err(Error::from(invitation_email.err().unwrap()));
    }

    let email = Email::builder().from(config.smtp_admin_email.clone()).to(user.email.clone()).html(invitation_email.unwrap()).build();

    if email.is_err() {
        return Err(Error::from(email.err().unwrap()));
    }

    let email = mailer.send(email.unwrap().into());

    if email.is_err() {
        return Err(Error::from(email.err().unwrap()));
    }

    return Ok(());
}
