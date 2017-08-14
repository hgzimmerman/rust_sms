
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use models::users::*;
use twilio_client_wrapper::*;
use twilio::Client;


pub fn get_users() {
    use schema::users::dsl::*;

    let connection = establish_connection();
    let results = users.load::<User>(&connection).expect("ERR loading users");

    info!("Listing all users:");
    for user in results {
        info!("{:?}\n                ---------\n", user);
    }
}


pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


/// Optionally add a user to the db if it doesn't exist yet, otherwise, set its state to the start state.
/// Then get the user, and send them a message, and change their state.
pub fn initialize_test_user(db_connection: &PgConnection, client: &Client ) {
    use state_machine::EventToken;
    use models::users;

    const HENRY_PHONE : &'static str = "+18472871920";
    match users::RealizedUser::get_user_by_phone_number(&HENRY_PHONE.to_string(), db_connection) {
        None => {
            let henry: users::NewUser = users::NewUser::new("Henry".to_string(), "Zimmerman".to_string(), HENRY_PHONE.to_string());
            henry.db_insert(db_connection);
        }
        Some(hen) => hen.db_update_state(UserState::StartState, db_connection)
    }


    let realized_henry: users::RealizedUser = users::RealizedUser::get_user_by_phone_number(&HENRY_PHONE.to_string(), db_connection).unwrap();
    let cloned_henry = realized_henry.clone();

    let (new_state, message) = realized_henry.state.next(EventToken::BoatAttendanceInternalRequest { message: &"Do you want to do event at time?".to_string() });
    cloned_henry.db_update_state(new_state, db_connection);


    send_message_to_user(client, message.unwrap(), &cloned_henry);

}