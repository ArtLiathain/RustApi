mod data_manager;
mod http_handler;
mod stock_parsing;

use actix_web::{web, App, HttpServer};
use std::fs::{self};
use http_handler::http_handler::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
