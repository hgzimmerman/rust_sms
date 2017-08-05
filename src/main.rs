#![feature(const_fn)]
#![feature(drop_types_in_const)] //Needed to have a statically accessible db connection
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate twilio;
extern crate dotenv;
extern crate regex;
extern crate percent_encoding;
#[macro_use]
extern crate lazy_static;


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
mod db_handle;
mod user_store;
use db_handle::{DB_HANDLE, DbHandle};
use user_store::MockUserStore;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/sms", data = "<input>" )]
fn sms(input: SimpleTwimlMessage, mut user_store: State<Mutex<MockUserStore>>) -> String {
    print!("/sms");

    state_machine::SmState::handle_input(input, &mut user_store.lock().unwrap())
}




fn main() {
    let client = create_client();
    let mut user_store = MockUserStore::init();
    {
        let mut new_henry_user = User::new("".to_string(), "".to_string(), "".to_string()); //Initialize to empty user.
        {
            let mut henry_user = user_store.clone().get_user_by_phone_number("+18472871920").unwrap().clone(); // This is _really_ ugly (2 clones to avoid the borrow checker, one of which should be expensive)

            let (new_state, message) = user_store
                .get_user_by_phone_number("+18472871920").unwrap()
                .state
                .next(EventToken::BoatAttendanceInternalRequest { message: &"do you want to do event at time?".to_string() }, &mut user_store);

            send_message_to_user(&client, message.unwrap(), &henry_user);

            new_henry_user = henry_user.clone();
            new_henry_user.set_state(new_state);
        }

        user_store.update_user(&new_henry_user);


        //    //check what state it has
        let mut user_henry = user_store.get_user_by_phone_number("+18472871920").unwrap();
        println!("{:?}", user_henry.state);
    }


    // The user store must be mutexed in order for the handle_input fn to be able to use it mutably (in a multi-thread env, you probably don't want simultaneous access to this global state)
    // Mitigate this restriction when using the DB, by getting a connection pool, so the pool members can each be borrowed mutably, while the container doesn't have to be (not even sure if the db connections will be mutable in the first place)
    let mutexed_user_store: Mutex<MockUserStore> = Mutex::new(user_store);


    rocket::ignite()
        .manage(client)
        .manage(mutexed_user_store)
        .mount("/", routes![index, sms])
        .launch();
}
