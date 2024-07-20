use pih_service::{telemetry, Configuration, Db};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    telemetry::setup_tracing();

    tracing::debug!("Initialising configuration");
    let config = Configuration::new();

    // Initialize db pool.
    tracing::debug!("Initializing db pool");
    let db = Db::new(&config.db_dsn, config.db_pool_max_size)
        .await
        .expect("Failed to initialize db");

    tracing::debug!("Running migrations");
    db.migrate().await.expect("Failed to run migrations");

    // Spin up our server.
    tracing::info!("Starting server on {}", config.listen_address);
    let listener = TcpListener::bind(&config.listen_address)
        .await
        .expect("Failed to bind address");
    let router = pih_service::router(config, db);
    axum::serve(listener, router)
        .await
        .expect("Failed to start server")
}
