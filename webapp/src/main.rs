use actix_web::{get, post, web::{self}, App, HttpServer, HttpResponse, Responder, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
	password: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Rust service prototype")
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/login")]
async fn login(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}, password start:{}!", info.username, info.password.chars().take(1).collect::<String>()))
}

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(index)
            .service(healthcheck)
            .service(login)
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(init)
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
