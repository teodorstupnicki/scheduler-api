use scheduler_api::tcp::thread_pool::ThreadPool;
use actix_web::{App, web, HttpServer};
use scheduler_api::web::api::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
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