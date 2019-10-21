pub mod groups_parser;

/// An event as returned from the KEATS API.
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Event {
    #[serde(rename(deserialize = "C"))]
    pub code: String,
    #[serde(rename(deserialize = "Date"))]
    pub date: String,
    #[serde(rename(deserialize = "N"))]
    pub title: Option<String>,
    #[serde(rename(deserialize = "T"))]
    pub type_: Option<String>,
    #[serde(rename(deserialize = "ST"))]
    pub start_time: String,
    #[serde(rename(deserialize = "ET"))]
    pub end_time: String,
    #[serde(rename(deserialize = "G"))]
    pub groups: Option<String>,
    #[serde(rename(deserialize = "S"))]
    pub staff: Option<String>,
    #[serde(rename(deserialize = "R"))]
    pub room: Option<String>,
    #[serde(rename(deserialize = "CP"))]
    pub campus: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_from_json() {
        assert_eq!(
            serde_json::from_str::<Event>(
                r#"{
                    "M": "5MBBS201-OW",
                    "C": "5MBBS201-OW",
                    "DW": "Mon",
                    "Date": "2019-09-09T00:00:00",
                    "D": "09 Sep 2019",
                    "N": "Year 2 Everything you need to know",
                    "T": "Lecture",
                    "ST": "09:00",
                    "ET": "12:30",
                    "G": "201-289",
                    "S": null,
                    "R": "Guy's Greenwood Theatre",
                    "CP": "Guy's"
                }"#
            )
            .unwrap(),
            Event {
                date: "2019-09-09T00:00:00".to_owned(),
                start_time: "09:00".to_owned(),
                end_time: "12:30".to_owned(),
                code: "5MBBS201-OW".to_owned(),
                groups: Some("201-289".to_owned()),
                title: Some("Year 2 Everything you need to know".to_owned()),
                type_: Some("Lecture".to_owned()),
                staff: None,
                room: Some("Guy's Greenwood Theatre".to_owned()),
                campus: Some("Guy's".to_owned()),
            }
        )
    }
}
