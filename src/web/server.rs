use actix_web::HttpServer;
use actix_web::{App, web};
use crate::web::api::*;

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}