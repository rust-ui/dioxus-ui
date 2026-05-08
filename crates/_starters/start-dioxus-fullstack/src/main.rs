#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]

mod app;
mod components;
mod domain;
mod utils;

use app::App;

fn main() {
    #[cfg(feature = "server")]
    {
        use sqlx::postgres::PgPoolOptions;
        use tracing_subscriber::{EnvFilter, FmtSubscriber};

        let subscriber = FmtSubscriber::builder()
            .with_ansi(true)
            .with_file(true)
            .with_line_number(true)
            .with_env_filter(EnvFilter::from_default_env())
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set tracing subscriber");

        dotenv::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = tokio::runtime::Runtime::new()
            .expect("Failed to create tokio runtime")
            .block_on(async {
                let pool = PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&database_url)
                    .await
                    .expect("Failed to connect to database");

                sqlx::migrate!("./migrations")
                    .run(&pool)
                    .await
                    .expect("Failed to run migrations");

                tracing::info!("Migrations complete.");
                pool
            });

        dioxus::LaunchBuilder::new()
            .with_context(pool)
            .launch(App);

        return;
    }

    dioxus::launch(App);
}
