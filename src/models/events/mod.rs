use chrono::{NaiveDateTime};
use schema::{events, users};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;


#[derive(Queryable, Identifiable, Associations, AsChangeset, Clone, Debug )]
#[belongs_to(User)]
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

//    fn db_associate_user(&self, user: &User, connection: &PgConnection) {
//        use schema::events;
//
//        let _ = diesel::insert(user)
//            .into(events::table)
//            .execute(connection)
//            .expect("Failed to associate user with event.");
//    }

//    fn get_associated_users(&self, connection: &PgConnection) -> Option<Vec<User>> {
//        use schema::events;
//
//        match User::belonging_to(self)
//            .load::<User>(connection) {
//            Ok(users) => Some(users),
//            Err(_) => None
//        }
//    }
}