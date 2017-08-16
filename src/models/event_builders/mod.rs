mod event_builder_state;
mod realized_event_builder;
pub use self::event_builder_state::EventBuilderState;
pub use self::realized_event_builder::RealizedEventBuilder;


use chrono::{NaiveDateTime};

#[derive(Clone, Debug)]
pub struct EventBuilder {
    id: i32,
    pub title: Option<String>,
    pub location: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub state: i32
}


