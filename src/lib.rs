pub mod errors;
pub mod models;
pub mod parser;

pub use errors::ParseError;
pub use models::{Lesson, Subject, Term, TimetabledLesson};

use reqwest::Client;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub scraper: Client,
}
