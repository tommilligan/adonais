extern crate chrono;
#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

pub mod keats;

use std::convert::TryFrom;

use chrono::{NaiveDate, NaiveTime};

#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub code: String,
    pub groups: Vec<u32>,
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
            groups: keats::parse_group_range(&event.groups.unwrap_or("".to_owned())),
            title: event.title,
            type_: event.type_,
            staff: event.staff,
            room: event.room,
            campus: event.campus,
        })
    }
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
