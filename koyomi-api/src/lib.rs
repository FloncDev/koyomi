use sqlx::{Pool, Postgres};

pub mod routes;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
}
