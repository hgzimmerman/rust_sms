use user_store::MockUserStore;
use std::sync::Mutex;

use models::*;
use diesel::prelude::*;
use diesel;


use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use models::users::*;

use state_machine::SmState;

pub fn get_users() {

    use schema::users::dsl::*;

    let connection = establish_connection();
    let results = users.load::<User>(&connection).expect("ERR loading users");

    for user in results {
        println!("{}", user.id);
        println!("{} {}", user.first_name, user.last_name);
        println!("{}", user.phone_number);
        println!("stateNum: {}", user.state);
        println!("----------\n");
    }
}

pub fn get_user_by_phone_number(searched_phone_number: String) -> Option<RealizedUser> {
    use schema::users::dsl::*;

    let connection = establish_connection();
    let results = users.filter(phone_number.eq(searched_phone_number))
        .limit(1)
        .load::<User>(&connection)
        .expect("ERR loading users");

    // get the only element in the results
    match results.iter().last() {
        Some(user) => Some(RealizedUser::from(user.clone())),    // Clone the user to get ownership, the convert to the app based user
        None => None
    }
}

pub fn insert_user(new_user: NewUser) -> User {
    use schema::users;

    let connection = establish_connection();

    diesel::insert(&new_user).into(users::table)
        .get_result(&connection)
        .expect("Error saving user")
}

pub fn update_user_state(user_to_alter: &RealizedUser, new_state: SmState) {
    use schema::users::dsl::*;
    let connection = establish_connection();

    let db_user: User = user_to_alter.clone().into();
    let state_representation: i32 = new_state.into();
    diesel::update(&db_user)
        .set(state.eq(state_representation))
        .execute(&connection);
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
