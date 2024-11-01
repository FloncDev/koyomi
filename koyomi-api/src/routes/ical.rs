use axum::Router;

use crate::AppState;

pub fn get_router() -> Router<AppState> {
    Router::new()
}
