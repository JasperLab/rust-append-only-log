pub(crate) mod log;
pub(crate) mod server;

use actix_web::{web, App, HttpServer};
use server::{consume, produce};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::post().to(produce))
            .route("/", web::get().to(consume))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
