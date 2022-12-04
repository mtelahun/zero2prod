use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if we can't read config
    let configuration = get_configuration().expect("Failed to read config");
    let connection_pool =
        PgPool::connect(configuration.database.connection_string().expose_secret())
            .await
            .expect("Faile to connect to postgres");
    let address = format!("localhost:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind to local address");
    run(listener, connection_pool)?.await
}
