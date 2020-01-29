
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_rt::System;

async fn nonblocking() -> String {
    "Hello, world.".to_owned()
}

fn blocking() -> Result<String, ()> {
    System::new("").block_on(nonblocking());
    Ok("Hello world!".to_owned())
}

async fn index() -> impl Responder {
    let body = web::block(blocking).await.unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

