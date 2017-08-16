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
extern crate chrono;

#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::{Config, TermLogger, WriteLogger, CombinedLogger, LogLevelFilter};
use std::fs::File;


use rocket::{ State };
use std::sync::Mutex;

mod state_machine;
use state_machine::*;

mod twilio_client_wrapper;
use twilio_client_wrapper::*;



mod event;
mod resource;
mod db;

mod models;
mod schema; // The schema will auto-codegen the path for each table. This will create the module path: schema::users... for the users table. `diesel migration run` must me ran for this to take effect


use models::users;
use models::new_user_builders;
use models::events;

use diesel::pg::PgConnection;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/sms", data = "<input>" )]
fn sms(input: SimpleTwimlMessage, db_connection: State<Mutex<PgConnection>>) -> String {
    info!("Received message: \"{}\", from {}.", input.message, input.from);
    //The locks here will prevent other posts to /sms from being processed until this scope ends, dropping the lock. Consider adding pools if the underlying process begins to take too long.
    let message: String = state_machine::handle_input(input, &db_connection.lock().unwrap());
    info!("Sending response: {}", message);
    message
}




fn main() {

    //Set up Logging
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
    let db_connection: PgConnection = db::establish_connection();

    // For testing purposes, remove eventually.
    db::initialize_test_user(&db_connection, &client);

    db::get_users(); // print out users in the system

    // This must be mutexed in order for the handle_input fn to be able to use it mutably (in a multi-thread environment, you probably don't want simultaneous access to this "global" state).
    // Consider creating a connection pool that can be used by multiple threads. (pool owns multiple mutexed connections, calling fn can grab one, lock it for its use)
    let mutexed_pg_connection: Mutex<PgConnection> = Mutex::new(db_connection);

    rocket::ignite()
        .manage(client)
        .manage(mutexed_pg_connection)
        .mount("/", routes![index, sms])
        .launch();
}
