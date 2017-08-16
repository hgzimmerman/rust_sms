use chrono::{NaiveDateTime};
use schema::events;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Clone, Debug, AsChangeset)]
pub struct Event {
    id: i32,
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


impl Event {
    fn db_update(&self, connection: &PgConnection) {
        use schema::events;

        diesel::update( events::table )
            .set(self)
            .execute(connection)
            .expect("Error updating event.");
    }
}