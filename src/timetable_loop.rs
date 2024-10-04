use crate::{parser::parse_timetable, AppState, TimetabledLesson};
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

        let week_starting = "2024-09-23";

        form.insert("week", week_starting);
        form.insert("student_user_id", &student_id);

        let response = state
            .scraper
            .post(url.clone())
            .form(&form)
            .send()
            .await
            .unwrap();

        // TODO: Figure out what to do with expired tokens
        if response.url().as_str() != url {
            tracing::error!("Session token expired");
            panic!()
        }

        tracing::debug!(
            "Recieved response from server. status={}",
            response.status()
        );

        let html = response.text().await.unwrap();

        let timetable = parse_timetable(&html).unwrap();

        let current_lessons = sqlx::query_as!(TimetabledLesson, "SELECT * FROM timetabled_lessons")
            .fetch_all(&state.pool)
            .await
            .unwrap();

        let to_add: Vec<&TimetabledLesson> = timetable
            .iter()
            .filter(|lesson| !current_lessons.contains(lesson))
            .collect();

        let to_remove: Vec<&TimetabledLesson> = current_lessons
            .iter()
            .filter(|lesson| !timetable.contains(lesson))
            .collect();

        tracing::debug!("Updating database");
        for lesson in to_add {
            // TODO: fix this
            tracing::debug!("Adding {} to database", lesson.subject);
            sqlx::query!(
                r#"
            INSERT INTO timetabled_lessons
            (subject, teachers, location, start, "end", weekday)
            VALUES
            ($1, $2, $3, $4, $5, $6)
            "#,
                lesson.subject,
                lesson.teachers,
                lesson.location,
                lesson.start,
                lesson.end,
                lesson.weekday
            )
            .execute(&state.pool)
            .await
            .unwrap();
        }

        for lesson in to_remove {
            tracing::debug!("Removing {} from database", lesson.subject);
            sqlx::query!("DELETE FROM timetabled_lessons WHERE id = $1", lesson.id)
                .execute(&state.pool)
                .await
                .unwrap();
        }

        tracing::debug!("Ticked");
    }
}
