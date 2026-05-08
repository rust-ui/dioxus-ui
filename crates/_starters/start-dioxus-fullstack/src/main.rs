#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]

mod app;
mod components;
mod domain;
mod utils;

use app::App;

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use tracing_subscriber::{EnvFilter, FmtSubscriber};

    let subscriber = FmtSubscriber::builder()
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    tracing::info!("Migrations complete. Launching server...");

    dioxus::LaunchBuilder::new()
        .with_context(pool)
        .launch(App);
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}
