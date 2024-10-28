use chrono::{DateTime, Local, NaiveDate};
use koyomi_core::Lesson;
use koyomi_scraper::{parser::parse_timetable, ParseError};

// Just has plain timetable data, no marks e.g. "Present"
#[test]
fn simple_week() {
    // Has been abstracted to not include useless HTML
    let html = r#"
<div>
    <div class="well">
      <h3>Monday</h3>
      <ul>
         <li>
            <strong>09:00 - 10:15:</strong> Maths in A123 with  A Teacher                 
         </li>
         <li>
            <strong>10:30 - 11:35:</strong> Maths in A123 with  A Teacher                 
         </li>
         <li>
            <strong>11:35 - 12:30:</strong> Tutorial (A12) in B123 with  B Teacher                 
         </li>
      </ul>
    </div>
</div>
    "#;

    let week_starting = NaiveDate::from_ymd_opt(2024, 10, 28).unwrap();

    let timetable = parse_timetable(html, week_starting);

    assert_eq!(
        timetable,
        Ok(vec![
            Lesson {
                id: 0,
                subject: String::from("Maths"),
                start: DateTime::from_timestamp(1730106000, 0)
                    .unwrap()
                    .with_timezone(&Local),
                end: DateTime::from_timestamp(1730110500, 0)
                    .unwrap()
                    .with_timezone(&Local),
                location: String::from("A123"),
                teachers: String::from("A Teacher"),
                // uid is not compared
                uid: uuid::Uuid::new_v4(),
            },
            Lesson {
                id: 0,
                subject: String::from("Maths"),
                start: DateTime::from_timestamp(1730111400, 0)
                    .unwrap()
                    .with_timezone(&Local),
                end: DateTime::from_timestamp(1730115300, 0)
                    .unwrap()
                    .with_timezone(&Local),
                location: String::from("A123"),
                teachers: String::from("A Teacher"),
                uid: uuid::Uuid::new_v4(),
            },
            Lesson {
                id: 0,
                subject: String::from("Tutorial (A12)"),
                start: DateTime::from_timestamp(1730115300, 0)
                    .unwrap()
                    .with_timezone(&Local),
                end: DateTime::from_timestamp(1730118600, 0)
                    .unwrap()
                    .with_timezone(&Local),
                location: String::from("B123"),
                teachers: String::from("B Teacher"),
                uid: uuid::Uuid::new_v4(),
            },
        ])
    )
}

// Test with some marks
#[test]
fn marked_week() {
    // Has been abstracted to not include useless HTML
    let html = r#"
<div>
    <div class="well">
      <h3>Monday</h3>
      <ul>
         <li>
            <strong>09:00 - 10:15:</strong> Maths in A123 with  A Teacher                 
            <p class="mark-text mark-text-present"><i class="fa fa-angle-right"></i> Marked "Present"</p>
         </li>
         <li>
            <strong>10:30 - 11:35:</strong> Maths in A123 with  A Teacher                 
            <p class="mark-text mark-text-present"><i class="fa fa-angle-right"></i> Marked "Present"</p>
         </li>
         <li>
            <strong>11:35 - 12:30:</strong> Tutorial (A12) in B123 with  B Teacher                 
            <p class="mark-text mark-text-present"><i class="fa fa-angle-right"></i> Marked "4 minutes Late (Unauthorised)"</p>
         </li>
      </ul>
    </div>
    <div class="well">
        <h3>Friday</h3>
        <ul>
            <li>
                <strong>10:30 - 11:45:</strong> Lesson in A123 with  A Teacher / B Teacher                 
                <p class="mark-text mark-text-present"><i class="fa fa-angle-right"></i> Marked "Present"</p>
                <p class="tapStatusIndicator tapStatusIndicatorOut">Tapped Out at 11:41 AM <i class="fa fa-rss-square"></i></p>
            </li>
        </ul>
    </div>
</div>
    "#;

    let week_starting = NaiveDate::from_ymd_opt(2024, 10, 28).unwrap();

    let timetable = parse_timetable(html, week_starting);

    assert_eq!(
        timetable,
        Ok(vec![
            Lesson {
                id: 0,
                subject: String::from("Maths"),
                start: DateTime::from_timestamp(1730106000, 0)
                    .unwrap()
                    .with_timezone(&Local),
                end: DateTime::from_timestamp(1730110500, 0)
                    .unwrap()
                    .with_timezone(&Local),
                location: String::from("A123"),
                teachers: String::from("A Teacher"),
                // uid is not compared
                uid: uuid::Uuid::new_v4(),
            },
            Lesson {
                id: 0,
                subject: String::from("Maths"),
                start: DateTime::from_timestamp(1730111400, 0)
                    .unwrap()
                    .with_timezone(&Local),
                end: DateTime::from_timestamp(1730115300, 0)
                    .unwrap()
                    .with_timezone(&Local),
                location: String::from("A123"),
                teachers: String::from("A Teacher"),
                uid: uuid::Uuid::new_v4(),
            },
            Lesson {
                id: 0,
                subject: String::from("Tutorial (A12)"),
                start: DateTime::from_timestamp(1730115300, 0)
                    .unwrap()
                    .with_timezone(&Local),
                end: DateTime::from_timestamp(1730118600, 0)
                    .unwrap()
                    .with_timezone(&Local),
                location: String::from("B123"),
                teachers: String::from("B Teacher"),
                uid: uuid::Uuid::new_v4(),
            },
            Lesson {
                id: 0,
                subject: String::from("Lesson"),
                start: DateTime::from_timestamp(1730457000, 0)
                    .unwrap()
                    .with_timezone(&Local),
                end: DateTime::from_timestamp(1730461500, 0)
                    .unwrap()
                    .with_timezone(&Local),
                location: String::from("A123"),
                teachers: String::from("A Teacher / B Teacher"),
                uid: uuid::Uuid::new_v4(),
            },
        ])
    )
}
