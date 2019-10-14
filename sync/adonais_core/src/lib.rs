extern crate chrono;
extern crate chrono_tz;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate siphasher;
extern crate wasm_bindgen;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub mod google;
pub mod keats;

use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime, TimeZone};
use chrono_tz::Europe::London;
use data_encoding::BASE32HEX;
use siphasher::sip::SipHasher24;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct EventInner {
    pub start: DateTime<FixedOffset>,
    pub end: DateTime<FixedOffset>,
    pub code: String,
    pub groups: Vec<u32>,
    pub groups_raw: Option<String>,
    pub title: Option<String>,
    pub type_: Option<String>,
    pub staff: Option<String>,
    pub room: Option<String>,
    pub campus: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub id: String,
    pub inner: EventInner,
}

impl TryFrom<keats::Event> for Event {
    type Error = chrono::ParseError;

    fn try_from(event: keats::Event) -> Result<Self, Self::Error> {
        // Timezones! Parse the date and time given a naive London times
        // then convert everything to FixedOffset for consistency.
        let date = NaiveDate::parse_from_str(&event.date, "%Y-%m-%dT%H:%M:%S")?;
        let start_time = NaiveTime::parse_from_str(&event.start_time, "%H:%M")?;
        let end_time = NaiveTime::parse_from_str(&event.end_time, "%H:%M")?;
        let start = London
            .from_local_datetime(&date.and_time(start_time))
            .earliest()
            .expect("Start time invalid.")
            .with_timezone(&FixedOffset::east(0));
        let end = London
            .from_local_datetime(&date.and_time(end_time))
            .latest()
            .expect("End time invalid.")
            .with_timezone(&FixedOffset::east(0));

        // There's some funky formatting of which groups an event is for
        let groups = keats::parse_group_range(&event.groups.clone().unwrap_or("".to_owned()));

        let inner = EventInner {
            start,
            end,
            code: event.code,
            groups_raw: event.groups,
            groups,
            title: event.title,
            type_: event.type_,
            staff: event.staff,
            room: event.room,
            campus: event.campus,
        };

        // We need a unique id for each event. Hash everything and convert it to
        // a valid Google Event id format.
        let mut hasher = SipHasher24::new();
        inner.hash(&mut hasher);
        let id = BASE32HEX.encode(&hasher.finish().to_le_bytes());

        Ok(Event { id, inner })
    }
}

impl Event {
    fn has_group(&self, group: u32) -> bool {
        self.inner.groups.contains(&group)
    }

    fn is_after(&self, min: &DateTime<FixedOffset>) -> bool {
        &self.inner.end > min
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
        let Event { id, inner } = event;

        let location = join_some_strings(vec![inner.room, inner.campus], ", ");
        let summary = join_some_strings(
            vec![
                Some(inner.title.unwrap_or(inner.code.clone())),
                inner.groups_raw,
            ],
            ", ",
        );
        let description = join_some_strings(
            vec![Some(inner.code.clone()), inner.staff, inner.type_],
            "\n",
        );

        // Pull other fields together into description
        google::Event {
            id,
            summary,
            start: google::Time {
                datetime: inner.start.to_rfc3339(),
            },
            end: google::Time {
                datetime: inner.end.to_rfc3339(),
            },
            description,
            location,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct CalendarUpdateRequest {
    /// Ids of existing events in the Google calendar.
    /// If a matching keats event is not found, they will be deleted.
    pub existing: Vec<String>,
    /// New events obtained from the KEATS API.
    /// These will overwrite Google events if they are updated.
    pub new: Vec<keats::Event>,
    /// Only return events relevant to this group.
    pub group: u32,
    /// The `timeMin` argument passed to the [Google Events List API](https://developers.google.com/calendar/v3/reference/events/list)
    /// when generating the list of `existing` ids. Any `new` events before this time will be filtered out.
    pub time_min: DateTime<FixedOffset>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CalendarUpdateResponse {
    /// New Google Events that should be created.
    pub created: Vec<google::Event>,
    /// Existing Google Events that should be deleted.
    pub deleted: Vec<String>,
}

/// The main entrypoint of the library.
///
/// Given information from both the KEATS and Google APIs, calculates the diff
/// that needs to be applied to update the calendar successfully.
pub fn calculate_calendar_update(request: CalendarUpdateRequest) -> CalendarUpdateResponse {
    let CalendarUpdateRequest {
        existing,
        new,
        group,
        time_min,
    } = request;

    let events: Vec<Event> = new
        .into_iter()
        .map(|keats_event| Event::try_from(keats_event).unwrap())
        .collect();

    // Filtered down to only events for the user now
    let group_events: Vec<Event> = events
        .into_iter()
        .filter(|e| e.has_group(group) && e.is_after(&time_min))
        .collect();
    let new_ids = group_events.iter().map(|e| e.id.clone()).collect();

    let mut new_events_by_id: HashMap<String, Event> = group_events
        .into_iter()
        .map(|e| (e.id.clone(), e))
        .collect();

    let existing_ids: HashSet<String> = existing.into_iter().collect();

    let deleted_ids: Vec<&String> = existing_ids.difference(&new_ids).collect();
    let created_ids: Vec<&String> = new_ids.difference(&existing_ids).collect();

    CalendarUpdateResponse {
        created: created_ids
            .into_iter()
            .map(|id| {
                google::Event::from(
                    new_events_by_id
                        .remove(id)
                        .expect("Created event not in map."),
                )
            })
            .collect(),
        deleted: deleted_ids.into_iter().map(|id| id.to_owned()).collect(),
    }
}

#[wasm_bindgen]
pub fn calculate_calendar_update_wasm(js_value: &JsValue) -> JsValue {
    let request = js_value.into_serde().unwrap();
    let response = calculate_calendar_update(request);
    JsValue::from_serde(&response).unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    lazy_static! {
        static ref BASE_KEATS_EVENT: keats::Event = {
            keats::Event {
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
            }
        };
        static ref BASE_EVENT: Event = {
            Event {
                id: "M9P6FJN06OLGM===".to_owned(),
                inner: EventInner {
                    start: DateTime::parse_from_rfc3339("2017-11-12T14:03:00+00:00").unwrap(),
                    end: DateTime::parse_from_rfc3339("2017-11-12T15:00:00+00:00").unwrap(),
                    code: "CODE001".to_owned(),
                    groups: vec![253, 254, 255, 256],
                    groups_raw: Some("253-256".to_owned()),
                    title: Some("Introduction to Clinical Pharmacology".to_owned()),
                    type_: Some("Lecture".to_owned()),
                    staff: Some("John Keats".to_owned()),
                    room: Some("Room 3b".to_owned()),
                    campus: Some("Unseen University".to_owned()),
                },
            }
        };
        static ref BASE_GOOGLE_EVENT: google::Event = {
            google::Event {
                id: "M9P6FJN06OLGM===".to_owned(),
                start: google::Time {
                    datetime: "2017-11-12T14:03:00+00:00".to_owned(),
                },
                end: google::Time {
                    datetime: "2017-11-12T15:00:00+00:00".to_owned(),
                },
                summary: "Introduction to Clinical Pharmacology, 253-256".to_owned(),
                description: "CODE001\nJohn Keats\nLecture".to_owned(),
                location: "Room 3b, Unseen University".to_owned(),
            }
        };
    }

    #[test]
    fn test_event_from_keats_event() {
        // All fields present
        assert_eq!(
            Event::try_from(BASE_KEATS_EVENT.clone()).unwrap(),
            BASE_EVENT.clone()
        );

        // Missing groups is assigned to everyone
        assert_eq!(
            Event::try_from(keats::Event {
                groups: None,
                ..BASE_KEATS_EVENT.clone()
            })
            .unwrap()
            .inner,
            EventInner {
                groups: (200..300).collect(),
                groups_raw: None,
                ..BASE_EVENT.inner.clone()
            }
        );

        // Time parse error is raised up
        assert!(Event::try_from(keats::Event {
            date: "spam".to_owned(),
            ..BASE_KEATS_EVENT.clone()
        })
        .is_err())
    }

    #[test]
    fn test_google_event_from_event() {
        // All fields present
        assert_eq!(
            google::Event::from(BASE_EVENT.clone()),
            BASE_GOOGLE_EVENT.clone()
        );

        // Timezones handled (London DST)
        assert_eq!(
            google::Event::from(Event {
                id: "id1".to_owned(),
                inner: EventInner {
                    start: DateTime::parse_from_rfc3339("2019-08-12T14:03:00+01:00").unwrap(),
                    end: DateTime::parse_from_rfc3339("2019-08-12T15:00:00+01:00").unwrap(),
                    ..BASE_EVENT.inner.clone()
                }
            }),
            google::Event {
                id: "id1".to_owned(),
                start: google::Time {
                    datetime: "2019-08-12T14:03:00+01:00".to_owned(),
                },
                end: google::Time {
                    datetime: "2019-08-12T15:00:00+01:00".to_owned(),
                },
                ..BASE_GOOGLE_EVENT.clone()
            }
        );

        // Description & location concat nicely
        assert_eq!(
            google::Event::from(Event {
                id: "id2".to_owned(),
                inner: EventInner {
                    staff: None,
                    room: None,
                    ..BASE_EVENT.inner.clone()
                }
            }),
            google::Event {
                id: "id2".to_owned(),
                description: "CODE001\nLecture".to_owned(),
                location: "Unseen University".to_owned(),
                ..BASE_GOOGLE_EVENT.clone()
            }
        );
    }

    #[test]
    fn test_calclate_calendar_update() {
        // - the base event is unchanged
        // - "existing1" has been deleted
        // - "New Event" is created with a new id
        assert_eq!(
            calculate_calendar_update(CalendarUpdateRequest {
                new: vec![
                    BASE_KEATS_EVENT.clone(),
                    keats::Event {
                        title: Some("New Event".to_owned()),
                        ..BASE_KEATS_EVENT.clone()
                    },
                    keats::Event {
                        title: Some("Past Event (that should be filtered out".to_owned()),
                        date: "1993-11-12T00:00:00".to_owned(),
                        ..BASE_KEATS_EVENT.clone()
                    },
                ],
                existing: vec![BASE_GOOGLE_EVENT.id.clone(), "existing1".to_string(),],
                group: 253,
                time_min: DateTime::parse_from_rfc3339("2017-01-01T00:00:00+00:00").unwrap(),
            }),
            CalendarUpdateResponse {
                created: vec![google::Event {
                    id: "E0KO7T238TM42===".to_owned(),
                    summary: "New Event, 253-256".to_owned(),
                    ..BASE_GOOGLE_EVENT.clone()
                }],
                deleted: vec!["existing1".to_string()],
            }
        )
    }
}
