use axum::{response::IntoResponse, routing::get, Router};
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
                .uid(format!("{}-{}-{}", lesson.subject, lesson.start, lesson.end).as_str())
                .done(),
        );
    }

    calendar
}

async fn get_ical() -> impl IntoResponse {}

pub fn get_router() -> Router<AppState> {
    Router::new().route("/calendar.ical", get(get_ical))
}
