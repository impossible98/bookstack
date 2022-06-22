use std::env;

mod global;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let mode = env::var("MODE").unwrap_or("development".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or("6969".to_string())
        .parse()
        .unwrap();

    println!();
    println!(
        "  {} v2.9.12 dev server running at:",
        global::global::BIN_NAME
    );
    println!();
    println!("  > Local:    http://localhost:{}/", port);
    println!("  > Network:  http://192.168.1.192:{}/", port);
    println!();
    println!("  ready in 473ms.");
    println!();

    if mode == "production" {
        HttpServer::new(|| {
            App::new()
                .service(hello)
                .service(echo)
                .route("/hey", web::get().to(manual_hello))
        })
        .bind(("127.0.0.1", port))?
        .run()
        .await
    } else {
        HttpServer::new(|| {
            App::new()
                .service(hello)
                .service(echo)
                .route("/hey", web::get().to(manual_hello))
        })
        .bind(("0.0.0.0", port))?
        .run()
        .await
    }
}
