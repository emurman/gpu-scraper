use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use db::lib::establish_connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use scraper::scrape;
use std::{env};

extern crate diesel_migrations;

mod db;
mod scraper;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!!!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut pg_connection = establish_connection();
    pg_connection.run_pending_migrations(MIGRATIONS).unwrap();

    match args[1].as_str() {
        "server" => {
            let _ = server();
        }
        _ => scrape(pg_connection)
    };
}