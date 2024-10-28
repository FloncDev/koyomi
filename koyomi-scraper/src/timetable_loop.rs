use crate::{parser::parse_timetable, AppState};
use chrono::{Datelike, Local};
use dotenvy::var;
use std::{collections::HashMap, time::Duration};
use tokio::time;

/// Starts a loop that will check for updates on the timetable, and update database as needed
pub async fn timetample_loop(state: AppState) {
    let mut interval = time::interval(Duration::from_secs(60));

    let student_id = var("STUDENT_ID").expect("STUDENT_ID not set");
    let url = var("ENDPOINT").expect("ENDPOINT not set");

    tracing::debug!("Starting loop");

    // TODO: Remove unwraps as this needs to run forever
    loop {
        interval.tick().await;

        let mut form = HashMap::new();

        let now = Local::now();
        let mut monday = now - chrono::Duration::days(now.weekday().num_days_from_monday().into());

        // EXTREME edgecase, only happens like 2 weeks every year, for an hour a day
        // (ofcourse due to daylight savings)
        if monday.offset() != now.offset() {
            tracing::debug!("{}", monday);
            monday -= chrono::Duration::seconds(
                (now.offset().utc_minus_local() - monday.offset().utc_minus_local()).into(),
            );
        }

        let week_starting = &monday.format("%Y-%m-%d").to_string();

        form.insert("week", week_starting.as_str());
        form.insert("student_user_id", &student_id);

        let response = state
            .scraper
            .post(url.clone())
            .form(&form)
            .send()
            .await
            .unwrap();

        // TODO: Figure out what to do with expired tokens
        // Might write some web extension to automatically get
        // my token from my prowser
        if response.url().as_str() != url {
            tracing::error!("Session token expired");
            panic!()
        }

        tracing::debug!(
            "Recieved response from {}. status={}",
            response.url(),
            response.status()
        );

        let html = response.text().await.unwrap();

        let lessons = parse_timetable(&html, monday.date_naive()).unwrap();
        tracing::debug!("Found {} lessons for {}", lessons.len(), week_starting);

        for lesson in lessons {
            sqlx::query!(
                r#"
                    insert into lessons
                        (subject, teachers, location, start, "end", uid)
                    select $1, $2, $3, $4, $5, $6
                    where not exists (
                        select id from lessons where subject = $2 and start = $4
                    )
                "#,
                lesson.subject,
                lesson.teachers,
                lesson.location,
                lesson.start,
                lesson.end,
                lesson.uid
            )
            .execute(&state.pool)
            .await
            .unwrap();
        }

        tracing::debug!("Ticked");
    }
}
