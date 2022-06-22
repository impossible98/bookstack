use std::env;

mod global;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use local_ip_address::local_ip;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn run(ip: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((ip, port))?
    .run()
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let mode = env::var("MODE").unwrap_or("production".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or("6969".to_string())
        .parse()
        .unwrap();

    let local_ip = local_ip().unwrap();
    println!();
    println!(
        "  {} v2.9.12 dev server running at:",
        global::global::BIN_NAME
    );
    println!();
    println!("  > Local:    http://localhost:{}/", port);
    println!("  > Network:  http://{}:{}/", local_ip, port);
    println!();
    println!("  ready in 473ms.");
    println!();

    if mode == "development" {
        run("0.0.0.0", port).await
    } else {
        run("127.0.0.1", port).await
    }
}
