use axum::{routing::get, Router};
use dotenvy::var;
use koyomi_api::{routes, AppState};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
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

    let state = AppState { pool };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(routes::get_router())
        .with_state(state);

    let listner = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Cannot bind to port 3000");

    tracing::info!("Running server on http://{}", listner.local_addr().unwrap());

    axum::serve(listner, app.into_make_service()).await.unwrap();
}
