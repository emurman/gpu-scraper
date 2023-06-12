use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Error};
use std::{env};

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
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "server" => {
            server();
        }
        _ => println!("hello")
    };
}