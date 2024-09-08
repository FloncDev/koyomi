pub mod models;
pub use models::{Lesson, Subject, Term, TimetabledLesson};

use reqwest::Client;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub scraper: Client,
}
