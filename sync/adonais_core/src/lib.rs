extern crate chrono;
extern crate chrono_tz;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate wasm_bindgen;

pub mod google;
pub mod keats;

use std::convert::TryFrom;

use chrono::{NaiveDate, NaiveTime, TimeZone};
use chrono_tz::Europe::London;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub code: String,
    pub groups: Vec<u32>,
    pub groups_raw: Option<String>,
    pub title: Option<String>,
    pub type_: Option<String>,
    pub staff: Option<String>,
    pub room: Option<String>,
    pub campus: Option<String>,
}

impl TryFrom<keats::Event> for Event {
    type Error = chrono::ParseError;

    fn try_from(event: keats::Event) -> Result<Self, Self::Error> {
        Ok(Event {
            date: NaiveDate::parse_from_str(&event.date, "%Y-%m-%dT%H:%M:%S")?,
            start_time: NaiveTime::parse_from_str(&event.start_time, "%H:%M")?,
            end_time: NaiveTime::parse_from_str(&event.end_time, "%H:%M")?,
            code: event.code,
            groups_raw: event.groups.clone(),
            groups: keats::parse_group_range(&event.groups.unwrap_or("".to_owned())),
            title: event.title,
            type_: event.type_,
            staff: event.staff,
            room: event.room,
            campus: event.campus,
        })
    }
}

fn join_some_strings(some_strings: Vec<Option<String>>, separator: &str) -> String {
    let parts = some_strings
        .into_iter()
        .filter_map(|o| o)
        .collect::<Vec<String>>();
    match parts.len() {
        0 => "".to_owned(),
        _ => parts.join(separator),
    }
}

impl From<Event> for google::Event {
    fn from(event: Event) -> google::Event {
        let start_datetime = London
            .from_local_datetime(&event.date.and_time(event.start_time))
            .unwrap()
            .to_rfc3339();
        let end_datetime = London
            .from_local_datetime(&event.date.and_time(event.end_time))
            .unwrap()
            .to_rfc3339();

        let location = join_some_strings(vec![event.room, event.campus], ", ");
        let summary = join_some_strings(
            vec![
                Some(event.title.unwrap_or(event.code.clone())),
                event.groups_raw,
            ],
            ", ",
        );
        let description = join_some_strings(
            vec![Some(event.code.clone()), event.staff, event.type_],
            "\n",
        );

        // Pull other fields together into description
        google::Event {
            summary,
            start: google::Time {
                datetime: start_datetime,
            },
            end: google::Time {
                datetime: end_datetime,
            },
            description,
            location,
        }
    }
}

#[wasm_bindgen]
pub fn keats_to_google_calendar_events(js_value: &JsValue) -> JsValue {
    let keats_events: Vec<keats::Event> = js_value.into_serde().unwrap();
    let google_events: Vec<google::Event> = keats_events
        .into_iter()
        .map(keats_to_google_calendar_event)
        .collect();
    JsValue::from_serde(&google_events).unwrap()
}

pub fn keats_to_google_calendar_event(keats_event: keats::Event) -> google::Event {
    google::Event::from(Event::try_from(keats_event).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_from_keats_event() {
        let base_event = Event {
            date: NaiveDate::from_ymd(2017, 11, 12),
            start_time: NaiveTime::from_hms(14, 03, 00),
            end_time: NaiveTime::from_hms(15, 00, 00),
            code: "CODE001".to_owned(),
            groups: vec![253, 254, 255, 256],
            groups_raw: Some("253-256".to_owned()),
            title: Some("Introduction to Clinical Pharmacology".to_owned()),
            type_: Some("Lecture".to_owned()),
            staff: Some("John Keats".to_owned()),
            room: Some("Room 3b".to_owned()),
            campus: Some("Unseen University".to_owned()),
        };
        let base_keats_event = keats::Event {
            date: "2017-11-12T00:00:00".to_owned(),
            start_time: "14:03".to_owned(),
            end_time: "15:00".to_owned(),
            code: "CODE001".to_owned(),
            groups: Some("253-256".to_owned()),
            title: Some("Introduction to Clinical Pharmacology".to_owned()),
            type_: Some("Lecture".to_owned()),
            staff: Some("John Keats".to_owned()),
            room: Some("Room 3b".to_owned()),
            campus: Some("Unseen University".to_owned()),
        };

        // All fields present
        assert_eq!(
            Event::try_from(base_keats_event.clone()).unwrap(),
            base_event
        );

        // Missing groups is assigned to everyone
        assert_eq!(
            Event::try_from(keats::Event {
                groups: None,
                ..base_keats_event.clone()
            })
            .unwrap(),
            Event {
                groups: (200..300).collect(),
                groups_raw: None,
                ..base_event.clone()
            }
        );

        // Time parse error is raised up
        assert!(Event::try_from(keats::Event {
            date: "spam".to_owned(),
            ..base_keats_event.clone()
        })
        .is_err())
    }

    #[test]
    fn test_google_event_from_event() {
        let base_event = Event {
            date: NaiveDate::from_ymd(2019, 08, 12),
            start_time: NaiveTime::from_hms(14, 03, 00),
            end_time: NaiveTime::from_hms(15, 00, 00),
            code: "CODE001".to_owned(),
            groups: vec![253, 254, 255, 256],
            groups_raw: Some("253-256".to_owned()),
            title: Some("Introduction to Clinical Pharmacology".to_owned()),
            type_: Some("Lecture".to_owned()),
            staff: Some("John Keats".to_owned()),
            room: Some("Room 3b".to_owned()),
            campus: Some("Unseen University".to_owned()),
        };
        let base_google_event = google::Event {
            start: google::Time {
                datetime: "2019-08-12T14:03:00+01:00".to_owned(),
            },
            end: google::Time {
                datetime: "2019-08-12T15:00:00+01:00".to_owned(),
            },
            summary: "Introduction to Clinical Pharmacology, 253-256".to_owned(),
            description: "CODE001\nJohn Keats\nLecture".to_owned(),
            location: "Room 3b, Unseen University".to_owned(),
        };

        // All fields present (with DST!)
        assert_eq!(google::Event::from(base_event.clone()), base_google_event);

        // Timezones handled (non-DST)
        assert_eq!(
            google::Event::from(Event {
                date: NaiveDate::from_ymd(2019, 12, 21),
                ..base_event.clone()
            }),
            google::Event {
                start: google::Time {
                    datetime: "2019-12-21T14:03:00+00:00".to_owned(),
                },
                end: google::Time {
                    datetime: "2019-12-21T15:00:00+00:00".to_owned(),
                },
                ..base_google_event.clone()
            }
        );

        // Description & location concat nicely
        assert_eq!(
            google::Event::from(Event {
                staff: None,
                room: None,
                ..base_event.clone()
            }),
            google::Event {
                description: "CODE001\nLecture".to_owned(),
                location: "Unseen University".to_owned(),
                ..base_google_event.clone()
            }
        );
    }
}
