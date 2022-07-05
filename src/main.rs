use sqlx::PgPool;
use std::net::TcpListener;
use secrecy::ExposeSecret;

use zero2prod2022::configuration::get_configuration;
use zero2prod2022::startup::run;
use zero2prod2022::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //Logging
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    //Configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    //Database
    let connection_pool = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    //Run
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
