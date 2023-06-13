use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use db::lib::establish_connection;
use scraper::scrape;
use std::{env};

mod db;
mod scraper;

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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let pg_connection = establish_connection();

    match args[1].as_str() {
        "server" => {
            let _ = server();
        }
        _ => scrape(pg_connection)
    };
}