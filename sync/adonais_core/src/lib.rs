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
            start_datetime,
            end_datetime,
            description,
            location,
        }
    }
}

/// Convert a JSON list of events from the Keats API, ready to create Google
/// calendara events
#[wasm_bindgen]
pub fn keats_to_google_calendar_events(keats_json: &str) -> String {
    let keats_events: Vec<keats::Event> = serde_json::from_str(keats_json).unwrap();
    let google_events: Vec<google::Event> = keats_events
        .into_iter()
        .map(|event| Event::try_from(event).unwrap())
        .map(google::Event::from)
        .collect();
    serde_json::to_string(&google_events).unwrap()
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
}
