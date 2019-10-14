pub static URI: &str =
    "https://lsm-education.kcl.ac.uk/apicommonstring/api/values/Mod-Module.5MBBSStage2";

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

/// Parse a collection of ints of the form `1-3, 5`.
/// Each item consists of a range or an int. These are parsed and concatenated.
/// If no range is given or half the range is missing, default to 200-300.
pub fn parse_group_range(range: &str) -> Vec<u32> {
    let mut items: Vec<u32> = vec![];
    for part in range.split(',') {
        let x: Vec<&str> = part.split('-').map(|s| s.trim()).collect();
        // unwrappping here is fine, we're guaranteed to have one elemnt
        // from the split above
        let range_start: u32 = x.first().unwrap().parse().unwrap_or(200);
        let range_end: u32 = x.last().unwrap().parse().unwrap_or(299) + 1;
        items.extend(range_start..range_end);
    }
    items.sort();
    items.dedup();
    items
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

    #[test]
    fn test_parse_group_range() {
        let all_groups: Vec<u32> = (200..300).collect();

        // Single item
        assert_eq!(parse_group_range("0"), vec![0]);
        // Range of items, inclusive
        assert_eq!(parse_group_range("0-2"), vec![0, 1, 2]);
        // Multiple spec, separted by comma
        assert_eq!(parse_group_range("0, 7"), vec![0, 7]);
        assert_eq!(parse_group_range("0, 7-10"), vec![0, 7, 8, 9, 10]);

        // Invalid spec defaults to 200 (low) and 299 (high)
        assert_eq!(parse_group_range("0, 297-spam"), vec![0, 297, 298, 299]);
        assert_eq!(parse_group_range("0, spam-201"), vec![0, 200, 201]);

        // When in doubt, default to everyone
        assert_eq!(parse_group_range(""), all_groups);
        assert_eq!(parse_group_range("250, spam"), all_groups);
    }
}
