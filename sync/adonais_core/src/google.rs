#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Time {
    #[serde(rename(serialize = "dateTime"))]
    pub datetime: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Event {
    pub id: String,
    pub start: Time,
    pub end: Time,
    pub summary: String,
    pub description: String,
    pub location: String,
}
