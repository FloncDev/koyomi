use chrono::{DateTime, Local, NaiveDate};

#[derive(Debug)]
pub struct Term {
    pub id: i32,
    pub name: String,
    pub start: NaiveDate,
    pub end: NaiveDate,
}

#[derive(Debug)]
pub struct Lesson {
    pub id: Option<i32>,
    pub subject: String,
    pub teachers: String,
    pub location: String,
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
}

// Had to implement by myself because I need to ignore id
impl PartialEq for Lesson {
    fn eq(&self, other: &Self) -> bool {
        if (
            &self.subject,
            &self.teachers,
            &self.location,
            &self.start,
            &self.end,
        ) == (
            &other.subject,
            &other.teachers,
            &other.location,
            &other.start,
            &other.end,
        ) {
            return true;
        }

        false
    }
}
