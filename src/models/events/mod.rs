use chrono::{NaiveDateTime};
use schema::events;

#[derive(Queryable, Identifiable, Clone, Debug)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub location: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>
}

#[derive(Insertable)]
#[table_name="events"]
pub struct NewEvent {
    pub title: String,
    pub location: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>
}