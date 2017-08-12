#![feature(const_fn)]
#![feature(drop_types_in_const)] //Needed to have a statically accessible db connection //lazy static
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

extern crate rocket;
extern crate twilio;
extern crate dotenv;
extern crate regex;
extern crate percent_encoding;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::{Config, TermLogger, WriteLogger, CombinedLogger, LogLevelFilter};
use std::fs::File;


use rocket::{Request, Data, State };
use rocket::data::{self};
use rocket::http::{Status};
use std::io::Read;
use std::sync::Mutex;

mod state_machine;
use state_machine::*;

mod twilio_client_wrapper;
use twilio_client_wrapper::*;

mod user;
use user::User;

mod event;
mod resource;
mod db;
mod user_store;
use user_store::MockUserStore;

mod models;
mod schema; // The schema will auto-codegen the path for each table. This will create the module path: schema::users... for the users table. `diesel migration run` must me ran for this to take effect


use models::users;

use diesel::pg::PgConnection;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/sms", data = "<input>" )]
fn sms(input: SimpleTwimlMessage, mut user_store: State<Mutex<MockUserStore>>, db_connection: State<Mutex<PgConnection>>) -> String {
    print!("/sms");

    //The locks here will prevent other posts to /sms from being processed until this action ends, dropping the lock. Consider adding pools if the underlying process begins to take too long.
    state_machine::SmState::handle_input(input, &mut user_store.lock().unwrap(), &db_connection.lock().unwrap())
}




fn main() {

    const LOGFILE_NAME: &'static str = "sms.log";
    CombinedLogger::init(
        vec!
        [

            TermLogger::new(LogLevelFilter::Info, Config::default()).unwrap(),
            TermLogger::new(LogLevelFilter::Warn, Config::default()).unwrap(),
            TermLogger::new(LogLevelFilter::Error, Config::default()).unwrap(),

            WriteLogger::new(LogLevelFilter::Trace, Config::default(), File::create(LOGFILE_NAME).unwrap()),
            WriteLogger::new(LogLevelFilter::Debug, Config::default(), File::create(LOGFILE_NAME).unwrap()),
            WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create(LOGFILE_NAME).unwrap()),
            WriteLogger::new(LogLevelFilter::Warn, Config::default(), File::create(LOGFILE_NAME).unwrap()),
            WriteLogger::new(LogLevelFilter::Error, Config::default(), File::create(LOGFILE_NAME).unwrap()),
        ]
    ).unwrap();

    let client = create_client();
    let mut user_store = MockUserStore::init();
    let db_connection: PgConnection = db::establish_connection();

    const HENRY_PHONE : &'static str = "+18472871920";
    match db::get_user_by_phone_number(HENRY_PHONE.to_string()) {
        None => {
            let henry: users::NewUser = users::NewUser::new("Henry".to_string(), "Zimmerman".to_string(), HENRY_PHONE.to_string());
            db::insert_user(henry);
        }
        Some(hen) => db::update_user_state(&hen, SmState::StartState)
    }


    let realized_henry: users::RealizedUser = db::get_user_by_phone_number(HENRY_PHONE.to_string()).unwrap();
    let cloned_henry = realized_henry.clone();

    let (new_state, message) = realized_henry.state.next(EventToken::BoatAttendanceInternalRequest { message: &"do you want to do event at time?".to_string() });
    db::update_user_state(&cloned_henry, new_state);

    send_message_to_user(&client, message.unwrap(), &cloned_henry);

    db::get_users();// print out users in the system


    // The user store must be mutexed in order for the handle_input fn to be able to use it mutably (in a multi-thread env, you probably don't want simultaneous access to this global state)
    // Mitigate this restriction when using the DB, by getting a connection pool, so the pool members can each be borrowed mutably, while the container doesn't have to be (not even sure if the db connections will be mutable in the first place)
    let mutexed_user_store: Mutex<MockUserStore> = Mutex::new(user_store);
    let mutexed_pg_connection: Mutex<PgConnection> = Mutex::new(db_connection);

    rocket::ignite()
        .manage(client)
        .manage(mutexed_user_store)
        .manage(mutexed_pg_connection)
        .mount("/", routes![index, sms])
        .launch();
}
