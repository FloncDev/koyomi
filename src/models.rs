use chrono::{NaiveDate, NaiveTime};
use uuid::Uuid;

#[derive(Debug)]
pub struct Term {
    pub id: i32,
    pub name: String,
    pub start: NaiveDate,
    pub end: NaiveDate,
}

#[derive(Debug)]
pub struct TimetabledLesson {
    pub id: i32,
    pub subject: String,
    pub teachers: String,
    pub location: String,
    pub start: NaiveTime,
    pub end: NaiveTime,
    pub weekday: i16,
}

// Had to implement by myself because I need to ignore id
impl PartialEq for TimetabledLesson {
    fn eq(&self, other: &Self) -> bool {
        if (
            &self.subject,
            &self.teachers,
            &self.location,
            &self.start,
            &self.end,
            &self.weekday,
        ) == (
            &other.subject,
            &other.teachers,
            &other.location,
            &other.start,
            &other.end,
            &other.weekday,
        ) {
            return true;
        }

        false
    }
}

#[derive(Debug)]
pub struct Lesson {
    pub id: i32,
    pub timetabled_lesson: TimetabledLesson,
    pub date: NaiveDate,
    pub uid: Uuid,
}
