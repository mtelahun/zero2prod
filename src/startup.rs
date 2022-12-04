use actix_web::{web, App, HttpServer, dev::Server};
use std::net::TcpListener;
use sqlx::PgPool;

use crate::routes::{health_check, subscriptions};

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server, std::io::Error> {
    // Wrap connection in a smart pointer
    let db_pool = web::Data::new(db_pool);
    // Capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/subscriptions", web::post().to(subscriptions))
        .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
