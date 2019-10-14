#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Time {
    #[serde(rename(serialize = "dateTime"))]
    pub datetime: String,
}

/// A Google Event resource for insertion, [as specified in the Calendar API](https://developers.google.com/calendar/v3/reference/events/insert)
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Event {
    pub id: String,
    pub start: Time,
    pub end: Time,
    pub summary: String,
    pub description: String,
    pub location: String,
}
