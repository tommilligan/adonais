#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Event {
    pub start_datetime: String,
    pub end_datetime: String,
    pub summary: String,
    pub description: String,
    pub location: String,
}
