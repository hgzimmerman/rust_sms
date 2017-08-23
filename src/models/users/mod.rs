mod user_state;
mod realized_user;

pub use self::user_state::UserState;
pub use self::realized_user::RealizedUser;
use schema::users;
use diesel::pg::PgConnection;
use diesel;
use diesel::prelude::*;

/// Db interfaceable user
#[derive(Queryable, Identifiable, Clone, Debug, AsChangeset)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String, // TODO create a phone number type?
//    groups: Vec<Group>,
    pub state: i32
}


#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub state: i32
}



impl NewUser {
    pub fn new(first_name: String, last_name: String, phone_number: String) -> NewUser {
        NewUser {
            first_name: first_name,
            last_name: last_name,
            phone_number: phone_number,
            //groups: Vec::new(),
            state: UserState::StartState.into(),
        }
    }

    pub fn db_insert(&self, connection: &PgConnection) {
        use schema::users;

        diesel::insert(self)
            .into(users::table)
            .execute(connection)
            .expect("Error saving user");
    }
}
