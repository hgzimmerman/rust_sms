use super::EventBuilderState;
use chrono::{NaiveDateTime};

pub struct RealizedEventBuilder {
    id: i32,
    pub title: Option<String>,
    pub location: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub state: EventBuilderState
}