use secrecy::ExposeSecret;
use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;
use zero2prod::configuration::get_configuration;
use zero2prod::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");


    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(&address).expect("Failed to bind port.");
    let connection_pool = PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy(&configuration.database.connection_string().expose_secret())
            .expect("Failed to connect to Postgres.");
    // println!("Listening on http://{}", listener.local_addr().unwrap());
    run(listener, connection_pool)?.await
}
