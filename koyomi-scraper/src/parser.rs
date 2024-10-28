use chrono::{Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use regex::Regex;
use scraper::{Html, Selector};

use crate::ParseError;
use koyomi_core::Lesson;

pub fn parse_timetable(html: &str, monday: NaiveDate) -> Result<Vec<Lesson>, ParseError> {
    let regex =
        Regex::new(r"<strong>(.*?) - (.*?):<\/strong> (.*?) in (.*?) with  ((?:[a-zA-Z0-9_\-\/]+(?: +[a-zA-Z0-9_\-\/])*)*?)(?:\s{2,}|$)").unwrap();

    let html = Html::parse_fragment(html);

    let mut lessons = vec![];

    for day in html.select(&Selector::parse(".well")?) {
        let weekday = day
            .select(&Selector::parse("h3")?)
            .next()
            .expect("Weekday <h3> tag not found")
            .inner_html()
            .parse::<Weekday>()?
            .num_days_from_monday();

        for lesson in day.select(&Selector::parse("li")?) {
            let inner_html = lesson.inner_html();

            let day = monday + Duration::days(weekday.into());

            let captures = regex
                .captures(inner_html.trim())
                .expect("Could not capture information from lesson string")
                .iter()
                .map(|capture| {
                    capture
                        .expect("Could not find capture")
                        .as_str()
                        .to_string()
                })
                .collect::<Vec<String>>();

            if let [_, start, end, name, location, teachers] = &captures[..] {
                let start = NaiveDateTime::new(day, NaiveTime::parse_from_str(start, "%H:%M")?)
                    .and_local_timezone(Local)
                    .unwrap();
                let end = NaiveDateTime::new(day, NaiveTime::parse_from_str(end, "%H:%M")?)
                    .and_local_timezone(Local)
                    .unwrap();

                lessons.push(Lesson {
                    id: None,
                    start,
                    end,
                    location: location.clone(),
                    teachers: teachers.clone(),
                    subject: name.clone(),
                })
            } else {
                tracing::error!(
                    "Could not get all capture groups from string. {:#?}",
                    captures
                );
            }
        }
    }

    Ok(lessons)
}
