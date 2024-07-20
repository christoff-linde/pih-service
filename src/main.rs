use server::{telemetry, Configuration, Db};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    telemetry::setup_tracing();

    tracing::debug!("Initialising configuration");
    let config = Configuration::new();

    tracing::debug!("Initialising DB pool");
    let db = Db::new(&config.db_dsn, config.db_pool_max_size)
        .await
        .expect("Failed to initialise db");

    // Initialize db pool.
    tracing::debug!("Initializing db pool");
    let db = Db::new(&cfg.db_dsn, cfg.db_pool_max_size)
        .await
        .expect("Failed to initialize db");

    tracing::debug!("Running migrations");
    db.migrate().await.expect("Failed to run migrations");

    // Spin up our server.
    tracing::info!("Starting server on {}", cfg.listen_address);
    let listener = TcpListener::bind(&cfg.listen_address)
        .await
        .expect("Failed to bind address");
    let router = server::router(cfg, db);
    axum::serve(listener, router)
        .await
        .expect("Failed to start server")
}
