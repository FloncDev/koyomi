use chrono::{NaiveDate, NaiveTime};
use uuid::Uuid;

pub struct Term {
    pub name: String,
    pub start: NaiveDate,
    pub end: NaiveDate,
}

pub struct Subject {
    pub name: String,
}

pub struct TimetabledLesson {
    pub subject: String,
    pub teachers: String,
    pub location: String,
    pub start: NaiveTime,
    pub end: NaiveTime,
    pub weekday: i32,
}

pub struct Lesson {
    pub timetabled_lesson: TimetabledLesson,
    pub uid: Uuid,
}
