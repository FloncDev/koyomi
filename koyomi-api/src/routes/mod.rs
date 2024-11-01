use axum::Router;

use crate::AppState;

pub mod ical;

pub fn get_router() -> Router<AppState> {
    Router::new()
}
