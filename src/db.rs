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

pub fn get_users() {

    use schema::users::dsl::*;

    let connection = establish_connection();
    let results = users.load::<User>(&connection).expect("ERR loading users");

    for user in results {
        println!("{} {}", user.first_name, user.last_name);
        println!("{}\n", user.phone_number);
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

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
