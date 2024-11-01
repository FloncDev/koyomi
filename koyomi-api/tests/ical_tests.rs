use chrono::{Duration, Local};
use icalendar::{Calendar, Component, Event, EventLike};
use koyomi_api::routes::ical::generate_calendar;
use koyomi_core::Lesson;

#[test]
fn simple_timetable() {
    let start = Local::now();
    let end = start + Duration::hours(1);
    let lessons = vec![
        Lesson {
            id: None,
            subject: String::from("Math"),
            teachers: String::from("Math Teacher"),
            location: String::from("A123"),
            start,
            end,
        },
        Lesson {
            id: None,
            subject: String::from("Computer Science"),
            teachers: String::from("CS Teacher"),
            location: String::from("B123"),
            start: end,
            end: end + Duration::hours(1),
        },
    ];

    let expected_cal = Calendar::new()
        .push(
            Event::new()
                .summary("Math")
                .description("Math Teacher")
                .location("A123")
                .starts(start.to_utc())
                .ends(end.to_utc())
                .uid(format!("Math-{}-{}", start, end).as_str())
                .done(),
        )
        .push(
            Event::new()
                .summary("Computer Science")
                .description("CS Teacher")
                .location("B123")
                .starts(end.to_utc())
                .ends((end + Duration::hours(1)).to_utc())
                .uid(format!("Computer Science-{}-{}", end, end + Duration::hours(1)).as_str())
                .done(),
        )
        .done();

    let cal = generate_calendar(lessons);

    assert_eq!(expected_cal, cal);
}
