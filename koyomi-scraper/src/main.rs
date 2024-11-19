use dotenvy::var;
use koyomi_scraper::{timetable_loop::timetample_loop, AppState};
use reqwest::{header::HeaderMap, Client};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    dotenvy::dotenv().ok();

    // Setup state
    tracing::debug!("Attempting database connection");

    let pool = PgPoolOptions::new()
        .connect(
            var("DATABASE_URL")
                .expect("DATABASE_URL is not set")
                .as_str(),
        )
        .await
        .expect("Error connecting to database");

    tracing::info!("Connected to database");

    let mut headers = HeaderMap::new();
    headers.append(
        "cookie",
        var("COOKIE").expect("COOKIE is not set").parse().unwrap(),
    );
    headers.append("x-requested-with", "XMLHttpRequest".parse().unwrap());

    let scraper = Client::builder().default_headers(headers).build()?;

    let state = AppState { pool, scraper };

    timetample_loop(state.clone()).await;

    Ok(())
}
