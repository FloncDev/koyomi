use crate::{parser::parse_timetable, AppState};
use dotenvy::var;
use std::{collections::HashMap, time::Duration};
use tokio::time;

/// Starts a loop that will check for updates on the timetable, and update database as needed
pub async fn timetample_loop(state: AppState) {
    let mut interval = time::interval(Duration::from_secs(60));

    let student_id = var("STUDENT_ID").expect("STUDENT_ID not set");
    let url = var("ENDPOINT").expect("ENDPOINT not set");

    loop {
        interval.tick().await;

        let mut form = HashMap::new();

        let week_starting = "2024-09-23";

        form.insert("week", week_starting);
        form.insert("student_user_id", &student_id);

        // TODO: Remove unwraps as this needs to run forever
        let response = state
            .scraper
            .post(url.clone())
            .form(&form)
            .send()
            .await
            .unwrap();

        tracing::debug!("Got status {} from timetable server.", response.status());

        let html = response.text().await.unwrap();

        let timetable = parse_timetable(&html).unwrap();

        tracing::debug!("Ticked")
    }
}
