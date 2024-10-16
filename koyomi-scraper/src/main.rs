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
    dotenvy::dotenv().expect("Could not find .env file");

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

    // Spawn the loop
    tokio::task::spawn(timetample_loop(state.clone()));

    // let app = Router::new()
    //     .route("/", get(|| async { "Hello, World!" }))
    //     .with_state(state);
    //
    // let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    //     .await
    //     .expect("Cannot bind to port 3000");

    // tracing::info!(
    //     "Running server on http://{}",
    //     listener.local_addr().unwrap()
    // );

    // axum::serve(listener, app.into_make_service())
    //     .await
    //     .unwrap();

    Ok(())
}
