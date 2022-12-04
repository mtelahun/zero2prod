use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read config
    let configuration = get_configuration().expect("Failed to read config");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Faile to connect to postgres");
    let address = format!("localhost:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind to local address");
    run(listener, connection_pool)?.await
}
