pub mod errors;
pub mod models;
pub mod parser;
pub mod timetable_loop;

pub use errors::ParseError;
pub use models::{Lesson, Term, TimetabledLesson};

use reqwest::Client;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub scraper: Client,
}
