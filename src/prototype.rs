use std::collections::HashMap;

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use icalendar::{Calendar, Component, Event, EventLike};
use regex::Regex;
use reqwest::{header::HeaderMap, Client};
use scraper::{selectable::Selectable, Html, Selector};
use uuid::Uuid;

#[derive(Debug)]
struct Lesson {
    name: String,
    teachers: String,
    location: String,
    start: NaiveDateTime,
    end: NaiveDateTime,
    uid: Uuid,
}

impl From<Lesson> for Event {
    fn from(value: Lesson) -> Self {
        Event::new()
            .location(&value.location)
            .summary(&value.name)
            .description(&value.teachers)
            .starts(value.start)
            .ends(value.end)
            .uid(&value.uid.to_string())
            .done()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let mut headers = HeaderMap::new();
    headers.append("cookie", "redacted".parse().unwrap());
    headers.append("x-requested-with", "XMLHttpRequest".parse().unwrap());

    let client = Client::builder().default_headers(headers).build()?;

    let mut form = HashMap::new();

    let week_starting = "2024-09-09";

    form.insert("week", week_starting);
    form.insert("student_user_id", "redacted");

    let html = client
        .post("redacted")
        .form(&form)
        .send()
        .await?
        .text()
        .await?;

    let html = Html::parse_fragment(&html);

    let well_selector = Selector::parse(".well")?;
    let h3_selector = Selector::parse("h3")?;
    let lesson_selector = Selector::parse("li")?;

    let regex =
        Regex::new(r"<strong>(.*?) - (.*?):<\/strong> (.*?) in (.*?) with  (.*?$)").unwrap();

    let week_start = NaiveDate::parse_from_str(week_starting, "%Y-%m-%d").unwrap();

    let mut lessons = vec![];

    for element in html.select(&well_selector) {
        let weekday = element
            .select(&h3_selector)
            .next()
            .unwrap()
            .inner_html()
            .parse::<Weekday>()?;

        for lesson in element.select(&lesson_selector) {
            let inner_html = lesson.inner_html();
            let captures = regex
                .captures(inner_html.trim())
                .unwrap()
                .iter()
                .map(|capture| capture.unwrap().as_str().to_string())
                .collect::<Vec<String>>();

            if let [_, start_time, end_time, name, location, teachers] = &captures[..] {
                let start_time = NaiveTime::parse_from_str(start_time, "%H:%M").unwrap();
                let end_time = NaiveTime::parse_from_str(end_time, "%H:%M").unwrap();

                let start = NaiveDateTime::new(
                    week_start + Duration::days(weekday.num_days_from_monday().into()),
                    start_time,
                );

                let end = NaiveDateTime::new(
                    week_start + Duration::days(weekday.num_days_from_monday().into()),
                    end_time,
                );

                lessons.push(Lesson {
                    name: name.to_owned(),
                    teachers: teachers.to_owned(),
                    location: location.to_owned(),
                    start,
                    end,
                    uid: Uuid::new_v4(),
                });
            } else {
                panic!();
            };
        }
    }

    let mut cal = Calendar::new();

    for lesson in lessons {
        cal.push(std::convert::Into::<Event>::into(lesson));
    }

    std::fs::write("cal.ical", cal.to_string()).unwrap();

    Ok(())
}
