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
use serde_json::{Map, Value};
use std::{env, thread, time};
use testcontainers::*;
use trust::{config::Config, handlers, mailer::EmailTemplates, models::user::get_by_email};

embed_migrations!("./migrations");

#[test]
fn end_users_test() {
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

    env::set_var("SMTP_ADMIN_EMAIL", "trust-85efd0@inbox.mailtrap.io");

    env::set_var("SMTP_HOST", "smtp.mailtrap.io");

    env::set_var("SMTP_PORT", "25");

    env::set_var("SMTP_USERNAME", "06b45c64cb46b9");

    env::set_var("SMTP_PASSWORD", "baee5138d7bc80");

    let config = Config::new();

    let manager = ConnectionManager::<PgConnection>::new(config.database_url.clone());

    let pool = Pool::new(manager).expect("expected to connect to database");

    let connection = pool.get().expect("unable to get database connection");

    embedded_migrations::run(&connection).expect("expected migrations to run");

    let email_templates = EmailTemplates::new(config.clone());

    let rocket = rocket::ignite().manage(config).manage(pool).manage(email_templates).mount(
        "/",
        routes![
            handlers::users::signup::signup,
            handlers::users::token::token,
            handlers::users::confirm_email::confirm,
            handlers::users::user::get::get,
            handlers::users::user::change_password::change_password,
            handlers::users::user::change_email::change_email,
            handlers::users::user::change_email_confirm::change_email_confirm,
            handlers::users::reset::reset::reset,
            handlers::users::reset::confirm_reset::confirm_reset,
        ],
    );

    let client = Client::new(rocket).expect("valid rocket instance");

    let signature = Header::new("x-operator-signature", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJmdW5jdGlvbl9ob29rcyI6eyJsb2dpbiI6Imh0dHBzOi8vcnVuLm1vY2t5LmlvL3YzLzQwYzZiYzM0LTRkYTgtNDFmMC05N2I0LWY4ZTgzY2JiMzVjMSIsInNpZ251cCI6Imh0dHBzOi8vcnVuLm1vY2t5LmlvL3YzLzQwYzZiYzM0LTRkYTgtNDFmMC05N2I0LWY4ZTgzY2JiMzVjMSJ9LCJzaXRlX3VybCI6Imh0dHA6Ly9sb2NhbGhvc3Q6OTAwMCIsInJlZGlyZWN0X3VybCI6Imh0dHA6Ly9sb2NhbGhvc3Q6OTAwMC9sb2dpbiJ9.9qZ_6Kr1YrykplVq-nrv19Dzj_Cfgovzcrez3imMneE");

    let req = client.post("/signup").header(signature.clone()).body(r#"{ "email": "zola@programmer.net", "password": "password"}"#);

    let res = req.dispatch();

    assert_eq!(res.status(), Status::Ok);

    let req = client.post("/signup").header(signature.clone()).body(r#"{ "email": "admin@zelalem.me", "password": "password"}"#);

    let res = req.dispatch();

    assert_eq!(res.status(), Status::Ok);

    let second = time::Duration::from_millis(1000);

    thread::sleep(second);

    let req = client
        .post("/token")
        .header(signature.clone())
        .body(r#"{ "username": "zola@programmer.net", "password": "password", "grant_type": "password"}"#);

    let res = req.dispatch();

    assert_eq!(res.status(), Status::PreconditionFailed);

    let user = get_by_email("zola@programmer.net".to_string(), &connection).expect("expected to find user");

    let req = client
        .post("/confirm/email")
        .header(signature.clone())
        .body(format!("{{\"confirmation_token\": \"{}\"}}", user.email_confirmation_token.unwrap()));

    let res = req.dispatch();

    assert_eq!(res.status(), Status::Ok);

    let user = get_by_email("admin@zelalem.me".to_string(), &connection).expect("expected to find user");

    let req = client
        .post("/confirm/email")
        .header(signature.clone())
        .body(format!("{{\"confirmation_token\": \"{}\"}}", user.email_confirmation_token.unwrap()));

    let res = req.dispatch();

    assert_eq!(res.status(), Status::Ok);

    let req = client
        .post("/token")
        .header(signature.clone())
        .body(r#"{ "username": "zola@programmer.net", "password": "wrongpassword", "grant_type": "password"}"#);

    let res = req.dispatch();

    assert_eq!(res.status(), Status::Unauthorized);

    let req = client
        .post("/token")
        .header(signature.clone())
        .body(r#"{ "username": "zola@programmer.net", "password": "password", "grant_type": "password"}"#);

    let mut res = req.dispatch();

    assert_eq!(res.status(), Status::Ok);

    let body = res.body_string().unwrap();

    let res: Map<String, Value> = serde_json::from_str(&body).expect("expected response to parse");

    let access_token = res.get("access_token").unwrap().as_str().unwrap().to_string();

    let authorization = Header::new("authorization", format!("Bearer {}", access_token));

    let req = client.get("/user").header(authorization.clone()).header(signature.clone());

    let mut res = req.dispatch();

    let body = res.body_string().unwrap();

    let body: Map<String, Value> = serde_json::from_str(&body).expect("expected response to parse");

    let email = body.get("email").unwrap().as_str().unwrap().to_string();

    assert_eq!(res.status(), Status::Ok);

    assert_eq!(email, "zola@programmer.net");

    let req = client.get("/user").header(signature.clone());

    let res = req.dispatch();

    assert_eq!(res.status(), Status::Unauthorized);

    let req = client
        .patch("/user/password")
        .header(authorization.clone())
        .header(signature.clone())
        .body(r#"{ "old_password": "wrongpassword", "new_password": "newpassword"}"#);

    let mut res = req.dispatch();

    let body = res.body_string().unwrap();

    let body: Map<String, Value> = serde_json::from_str(&body).expect("expected response to parse");

    let code = body.get("code").unwrap().as_str().unwrap().to_string();

    assert_eq!(res.status(), Status::BadRequest);

    assert_eq!(code, "invalid_old_password");

    let req = client
        .patch("/user/password")
        .header(authorization.clone())
        .header(signature.clone())
        .body(r#"{ "old_password": "password", "new_password": "pass"}"#);

    let mut res = req.dispatch();

    let body = res.body_string().unwrap();

    let body: Map<String, Value> = serde_json::from_str(&body).expect("expected response to parse");

    let code = body.get("code").unwrap().as_str().unwrap().to_string();

    assert_eq!(res.status(), Status::BadRequest);

    assert_eq!(code, "invalid_password_format");

    let req = client
        .patch("/user/password")
        .header(authorization.clone())
        .header(signature.clone())
        .body(r#"{ "old_password": "password", "new_password": "newpassword"}"#);

    let res = req.dispatch();

    assert_eq!(res.status(), Status::Ok);

    let req = client
        .patch("/user/email")
        .header(authorization.clone())
        .header(signature.clone())
        .body(r#"{ "email": "admin@zelalem.me"}"#);

    let res = req.dispatch();

    assert_eq!(res.status(), Status::Conflict);

    thread::sleep(second);

    let req = client.patch("/user/email").header(authorization).header(signature.clone()).body(r#"{ "email": "zola@zelalem.me"}"#);

    let res = req.dispatch();

    assert_eq!(res.status(), Status::Ok);

    let user = get_by_email("zola@programmer.net".to_string(), &connection).expect("expected to find user");

    let req = client
        .patch("/user/email/confirm")
        .header(signature.clone())
        .body(format!("{{\"email_change_token\": \"{}\"}}", "wrongtoken"));

    let res = req.dispatch();

    assert_eq!(res.status(), Status::NotFound);

    let req = client
        .patch("/user/email/confirm")
        .header(signature.clone())
        .body(format!("{{\"email_change_token\": \"{}\"}}", user.email_change_token.unwrap()));

    let res = req.dispatch();

    assert_eq!(res.status(), Status::Ok);

    get_by_email("zola@zelalem.me".to_string(), &connection).expect("expected to find user");

    let req = client.post("/reset").header(signature.clone()).body(r#"{ "email": "non@existent.email"}"#);

    let res = req.dispatch();

    assert_eq!(res.status(), Status::Accepted);

    thread::sleep(second);

    let req = client.post("/reset").header(signature.clone()).body(r#"{ "email": "zola@zelalem.me"}"#);

    let res = req.dispatch();

    assert_eq!(res.status(), Status::Accepted);

    thread::sleep(second);

    let user = get_by_email("zola@zelalem.me".to_string(), &connection).expect("expected to find user");

    let req = client
        .post("/reset/confirm")
        .header(signature.clone())
        .body(format!("{{\"recovery_token\": \"{}\", \"new_password\": \"newpassword\"}}", "wrongtoken"));

    let res = req.dispatch();

    assert_eq!(res.status(), Status::NotFound);

    let req = client
        .post("/reset/confirm")
        .header(signature.clone())
        .body(format!("{{\"recovery_token\": \"{}\", \"new_password\": \"newpassword\"}}", user.recovery_token.unwrap()));

    let res = req.dispatch();

    assert_eq!(res.status(), Status::Ok);
}
