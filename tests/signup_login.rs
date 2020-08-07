#[cfg(test)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use rocket::{
    http::{Header, Status},
    local::Client,
};
use std::env;
use testcontainers::*;
use trust::{config::Config, handlers, mailer::EmailTemplates, models::user::get_by_email};

embed_migrations!("./migrations");

#[test]
fn signup_login_test() {
    let docker = clients::Cli::default();

    let db = "trust";

    let user = "trust";

    let password = "trust";

    let postgres = images::generic::GenericImage::new("postgres:12-alpine")
        .with_wait_for(images::generic::WaitFor::message_on_stderr("database system is ready to accept connections"))
        .with_env_var("POSTGRES_DB", db)
        .with_env_var("POSTGRES_USER", user)
        .with_env_var("POSTGRES_PASSWORD", password);

    let node = docker.run(postgres);

    let connection_string = &format!("postgres://{}:{}@localhost:{}/{}", user, password, node.get_host_port(5432).unwrap(), db);

    env::set_var("AUD", "trust");

    env::set_var("DATABASE_URL", connection_string);

    env::set_var("INSTANCE_URL", "localhost:1996");

    env::set_var("JWT_ALGORITHM", "HS256");

    env::set_var("JWT_SECRET", "supersecret");

    env::set_var("SITE_URL", "localhost");

    env::set_var("SMTP_ADMIN_EMAIL", "d07bdd869f-85efd0@inbox.mailtrap.io");

    env::set_var("SMTP_HOST", "smtp.mailtrap.io");

    env::set_var("SMTP_PORT", "25");

    env::set_var("SMTP_USERNAME", "06b45c64cb46b9");

    env::set_var("SMTP_PASSWORD", "baee5138d7bc80");

    let config = Config::new();

    let manager = ConnectionManager::<PgConnection>::new(config.database_url.clone());

    let pool = Pool::new(manager).unwrap();

    let connection = pool.get().expect("unable to get database connection");

    embedded_migrations::run(&connection).expect("expected migrations to run");

    let email_templates = EmailTemplates::new(config.clone());

    let rocket = rocket::ignite()
        .manage(config)
        .manage(pool)
        .manage(email_templates)
        .mount("/", routes![handlers::users::signup::signup, handlers::users::token::token, handlers::users::confirm::confirm]);

    let client = Client::new(rocket).expect("valid rocket instance");

    let signature = Header::new("x-operator-signature", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJmdW5jdGlvbl9ob29rcyI6eyJsb2dpbiI6Imh0dHBzOi8vdHJ1c3QuZnJlZS5iZWVjZXB0b3IuY29tIiwic2lnbnVwIjoiaHR0cHM6Ly90cnVzdC5mcmVlLmJlZWNlcHRvci5jb20ifSwic2l0ZV91cmwiOiJodHRwOi8vbG9jYWxob3N0OjkwMDAiLCJyZWRpcmVjdF91cmwiOiJodHRwOi8vbG9jYWxob3N0OjkwMDAvbG9naW4ifQ.C5ESOhYXOuJtfXhFQvZVwfCwz1x7GsCKp1jpP9cTp7Y");

    let req = client.post("/signup").header(signature.clone()).body(r#"{ "email": "zola@programmer.net", "password": "password"}"#);

    let res = req.dispatch();

    assert_eq!(res.status(), Status::Ok);

    let req = client
        .post("/token")
        .header(signature.clone())
        .body(r#"{ "username": "zola@programmer.net", "password": "password", "grant_type": "password"}"#);

    let res = req.dispatch();

    assert_eq!(res.status(), Status::PreconditionFailed);

    let user = get_by_email("zola@programmer.net".to_string(), &connection).expect("expected to find user");

    let req = client
        .post("/confirm")
        .header(signature.clone())
        .body(format!("{{\"confirmation_token\": \"{}\"}}", user.confirmation_token.unwrap()));

    let res = req.dispatch();

    assert_eq!(res.status(), Status::Ok);

    let req = client
        .post("/token")
        .header(signature.clone())
        .body(r#"{ "username": "zola@programmer.net", "password": "password", "grant_type": "password"}"#);

    let res = req.dispatch();

    assert_eq!(res.status(), Status::Ok);
}
