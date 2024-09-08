use chrono::{NaiveTime, Weekday};
use regex::Regex;
use scraper::{Html, Selector};

use crate::{ParseError, TimetabledLesson};

pub fn parse_timetable(html: &str) -> Result<Vec<TimetabledLesson>, ParseError> {
    let regex =
        Regex::new(r"<strong>(.*?) - (.*?):<\/strong> (.*?) in (.*?) with  (.*?$)").unwrap();

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
                let start = NaiveTime::parse_from_str(start, "%H:%M")?;
                let end = NaiveTime::parse_from_str(end, "%H:%M")?;

                lessons.push(TimetabledLesson {
                    start,
                    end,
                    location: location.clone(),
                    teachers: teachers.clone(),
                    subject: name.clone(),
                    weekday: weekday as i32,
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
