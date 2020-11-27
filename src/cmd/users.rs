use crate::{
    config::Config,
    crypto::secure_token,
    mailer::send_email,
    models::{
        user::{NewUser, User},
        Error as ModelError,
    },
    sms::send_sms,
};
use chrono::Utc;
use clap::ArgMatches;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
    result::{DatabaseErrorKind, Error::DatabaseError},
};
use log::error;
use std::str::FromStr;

fn new_user(matches: Option<&ArgMatches>, connection_pool: Pool<ConnectionManager<PgConnection>>, config: &Config) {
    let matches = matches.unwrap();

    let phone_number = matches.value_of("phone_number");

    let email = matches.value_of("email");

    let mut user = NewUser {
        password: Some(matches.value_of("password").unwrap().to_string()),
        is_admin: matches.is_present("admin"),
        ..Default::default()
    };

    if let Some(email) = email {
        user.email = Some(email.to_string());
        if !config.email_rule.is_match(user.email.clone().unwrap().as_ref()) {
            error!("email address doesn't match email rule");
            std::process::exit(1);
        }
    }

    if let Some(phone_number) = phone_number {
        user.phone_number = Some(phone_number.to_string());
        if !config.phone_rule.is_match(user.phone_number.clone().unwrap().as_ref()) {
            error!("phone number doesn't match phone number rule");
            std::process::exit(1);
        }
    }

    user.hash_password();

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(err) => {
            error!("unable to connect to database: {:?}", err);
            std::process::exit(1);
        }
    };

    match user.save(&connection) {
        Ok(mut user) => {
            if user.email.is_some() && !config.auto_confirm && !matches.is_present("confirm") {
                user.email_confirmation_token = Some(secure_token(100));

                user.email_confirmation_token_sent_at = Some(Utc::now().naive_utc());

                let template = &config.get_confirmation_email_template();

                let to = &user.email.clone().unwrap();

                let subject = &config.get_confirmation_email_subject();

                let data = json!({
                    "confirmation_token": user.email_confirmation_token.clone().unwrap(),
                    "site_url": config.site_url,
                    "email": user.email
                });

                let email = send_email(template, data, to, subject, &config);

                if email.is_err() {
                    let err = email.err().unwrap();

                    error!("{:?}", err);

                    std::process::exit(1);
                }
            }

            let user = user.confirm_email(&connection);

            if user.is_err() {
                let err = user.err().unwrap();

                error!("{:?}", err);

                return;
            }

            let mut user = user.unwrap();

            if user.phone_number.is_some() && !config.auto_confirm && !matches.is_present("confirm") {
                user.phone_confirmation_token = Some(secure_token(6));

                user.phone_confirmation_token_sent_at = Some(Utc::now().naive_utc());

                let template = config.clone().get_confirmation_sms_template();

                let data = json!({
                    "confirmation_token": user.phone_confirmation_token.clone().unwrap(),
                    "phone_number": user.phone_number,
                    "site_url": config.site_url
                });

                let sms = send_sms(template, data, user.phone_number.clone().unwrap(), &config);

                if sms.is_err() {
                    let err = sms.err().unwrap();

                    error!("{:?}", err);

                    std::process::exit(1);
                }
            }

            println!("{} created successfully", user.email.unwrap());
        }
        Err(err) => match err {
            ModelError::DatabaseError(DatabaseError(DatabaseErrorKind::UniqueViolation, _info)) => panic!("{} already exists!", user.email.unwrap()),
            _ => panic!("{:?}", err),
        },
    }
}

fn remove_user(matches: Option<&ArgMatches>, connection_pool: Pool<ConnectionManager<PgConnection>>) {
    let connection = connection_pool.get().expect("unable to get connection to database");

    let matches = matches.unwrap();

    let email = matches.value_of("email").unwrap().to_string();

    match User::delete_by_email(email, &connection) {
        Ok(_val) => println!("user deleted successfully"),
        Err(err) => println!("unable to delete user: {:?}", err),
    }
}

pub fn users(matches: Option<&ArgMatches>) {
    let matches = matches.unwrap();

    let config = Config::new();

    let log_level = config.log_level.clone();

    let logger = simple_logger::SimpleLogger::new().with_level(log::LevelFilter::from_str(&log_level).unwrap()).init();

    if logger.is_err() {
        let err = logger.err().unwrap();
        panic!("{}", err);
    }

    let database_url = config.database_url.clone();

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let connection_pool = Pool::new(manager);

    if connection_pool.is_err() {
        let err = connection_pool.err().unwrap();
        error!("{:?}", err);
        return;
    }

    let connection_pool = connection_pool.unwrap();

    match matches.subcommand() {
        ("create", sub_m) => new_user(sub_m, connection_pool, &config),
        ("remove", sub_m) => remove_user(sub_m, connection_pool),
        _ => {}
    }
}
