use crate::{parser::parse_timetable, AppState};
use chrono::{DateTime, Datelike, Local};
use dotenvy::var;
use koyomi_core::Lesson;
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

        let today = Local::now().date_naive();
        let monday = today - chrono::Duration::days(today.weekday().num_days_from_monday().into());

        // EXTREME edgecase, only happens like 2 weeks every year, for an hour a day
        // (ofcourse due to daylight savings)
        // Don't need if using naive date, but keeping it just in case
        // if monday.offset() != today.offset() {
        //     tracing::debug!("{}", monday);
        //     monday -= chrono::Duration::seconds(
        //         (today.offset().utc_minus_local() - monday.offset().utc_minus_local()).into(),
        //     );
        // }

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

        let lessons = parse_timetable(&html, monday).unwrap();
        tracing::info!("Found {} lessons for {}", lessons.len(), week_starting);

        let monday = monday
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();
        let stored_lessons = sqlx::query_as!(
            Lesson,
            r#"
                select * from lessons
                where start between $1 and $2
            "#,
            monday,
            monday + chrono::Duration::weeks(1)
        )
        .fetch_all(&state.pool)
        .await
        .unwrap();

        let mut lesson_starts: HashMap<DateTime<Local>, &Lesson> = stored_lessons
            .iter()
            .map(|lesson| (lesson.start, lesson))
            .collect();

        let mut lessons_added = 0;
        let mut lessons_updated = 0;
        let mut lessons_deleted = 0;

        for lesson in lessons {
            match lesson_starts.get(&lesson.start) {
                None => {
                    // Add to database
                    sqlx::query!(
                        r#"
                        insert into lessons
                            (subject, teachers, location, start, "end")
                        select $1, $2, $3, $4, $5
                        "#,
                        lesson.subject,
                        lesson.teachers,
                        lesson.location,
                        lesson.start,
                        lesson.end,
                    )
                    .execute(&state.pool)
                    .await
                    .unwrap();

                    lessons_added += 1;
                }
                Some(stored_lesson) => {
                    if &&lesson != stored_lesson {
                        // Update database
                        sqlx::query!(
                            r#"
                            update lessons
                            set
                                subject = $1,
                                teachers = $2,
                                location = $3,
                                "end" = $4
                            "#,
                            lesson.subject,
                            lesson.teachers,
                            lesson.location,
                            lesson.end
                        )
                        .execute(&state.pool)
                        .await
                        .unwrap();

                        lessons_updated += 1;
                    }
                }
            }

            lesson_starts.remove(&lesson.start);
        }

        // Remove any lessons that were not detected
        for (_, lesson) in lesson_starts {
            sqlx::query!(
                r#"
                delete from lessons
                where
                    id = $1
                "#,
                lesson.id
            )
            .execute(&state.pool)
            .await
            .unwrap();
            lessons_deleted += 1;
        }

        tracing::info!(
            "Updated database. {} added, {} updated, and {} removed",
            lessons_added,
            lessons_updated,
            lessons_deleted
        );
    }
}
