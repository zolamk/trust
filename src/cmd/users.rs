use crate::{
    config::Config,
    crypto::secure_token,
    mailer::{send_email, EmailTemplates},
    models::{
        user::{NewUser, User},
        Error as ModelError,
    },
};
use chrono::Utc;
use clap::ArgMatches;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
    result::{DatabaseErrorKind, Error::DatabaseError},
};
use log::error;

fn new_user(matches: Option<&ArgMatches>, connection_pool: Pool<ConnectionManager<PgConnection>>, config: Config, email_templates: EmailTemplates) {
    let matches = matches.unwrap();

    let mut user = NewUser::default();

    user.email = matches.value_of("email").unwrap().to_string();

    user.password = Some(matches.value_of("password").unwrap().to_string());

    user.is_admin = matches.is_present("admin");

    user.confirmed = config.auto_confirm || matches.is_present("confirm");

    user.hash_password();

    if !user.confirmed {
        user.confirmation_token = Some(secure_token(100));

        user.confirmation_token_sent_at = Some(Utc::now().naive_utc())
    }

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(err) => {
            error!("unable to connect to database: {:?}", err);
            std::process::exit(1);
        }
    };

    match user.save(&connection) {
        Ok(user) => {
            if !user.confirmed {
                let template = email_templates.confirmation_email_template();

                let data = json!({
                    "confirmation_url": format!("{}/confirm?confirmation_token={}", config.site_url, user.confirmation_token.clone().unwrap()),
                    "site_url": config.site_url,
                    "email": user.email
                });

                let email = send_email(template, data, user.email.clone(), &config);

                if email.is_err() {
                    let err = email.err().unwrap();

                    error!("{:?}", err);

                    return;
                }
            }
            println!("{} created successfully", user.email);
        }
        Err(err) => match err {
            ModelError::DatabaseError(DatabaseError(DatabaseErrorKind::UniqueViolation, _info)) => panic!("{} already exists!", user.email),
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

    let database_url = config.database_url.clone();

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let connection_pool = Pool::new(manager);

    if connection_pool.is_err() {
        let err = connection_pool.err().unwrap();
        error!("{:?}", err);
        return;
    }

    let connection_pool = connection_pool.unwrap();

    let email_templates = EmailTemplates::new(config.clone());

    match matches.subcommand() {
        ("create", sub_m) => new_user(sub_m, connection_pool, config, email_templates),
        ("remove", sub_m) => remove_user(sub_m, connection_pool),
        _ => {}
    }
}
