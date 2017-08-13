
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
        println!("{}", user.id);
        println!("{} {}", user.first_name, user.last_name);
        println!("{}", user.phone_number);
        println!("stateNum: {}", user.state);
        println!("----------\n");
    }
}


pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
