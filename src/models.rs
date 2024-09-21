use chrono::{NaiveDate, NaiveTime};
use uuid::Uuid;

#[derive(Debug)]
pub struct Term {
    pub name: String,
    pub start: NaiveDate,
    pub end: NaiveDate,
}

#[derive(Debug)]
pub struct Subject {
    pub name: String,
}

#[derive(Debug)]
pub struct TimetabledLesson {
    pub subject: String,
    pub teachers: String,
    pub location: String,
    pub start: NaiveTime,
    pub end: NaiveTime,
    pub weekday: i32,
}

#[derive(Debug)]
pub struct Lesson {
    pub timetabled_lesson: TimetabledLesson,
    pub uid: Uuid,
}
