use axum::{extract::State, http::StatusCode, routing::get, Router};
use icalendar::{Calendar, Component, Event, EventLike};
use koyomi_core::Lesson;

use crate::AppState;

// Seperate function so I can do testing
pub fn generate_calendar(lessons: Vec<Lesson>) -> Calendar {
    let mut calendar = Calendar::new();

    for lesson in lessons {
        calendar.push(
            Event::new()
                .summary(&lesson.subject)
                .description(&lesson.teachers)
                .location(&lesson.location)
                .starts(lesson.start.to_utc())
                .ends(lesson.end.to_utc())
                .uid(format!("{}-{}", lesson.subject, lesson.id.unwrap_or(-1)).as_str())
                .done(),
        );
    }

    calendar
}

async fn get_ical(State(state): State<AppState>) -> Result<String, StatusCode> {
    let lessons = match sqlx::query_as!(Lesson, "select * from lessons")
        .fetch_all(&state.pool)
        .await
    {
        Ok(lessons) => lessons,
        Err(err) => {
            tracing::error!("An error occured while fetching lessons {:#?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(generate_calendar(lessons).to_string())
}

pub fn get_router() -> Router<AppState> {
    Router::new().route("/calendar.ical", get(get_ical))
}
