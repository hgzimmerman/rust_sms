
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate twilio;
extern crate dotenv;
extern crate regex;
extern crate percent_encoding;


use rocket::{Request, Data };
use rocket::data::{self};
use rocket::http::{Status};
use rocket::Outcome::*;
use std::io::Read;
use regex::Regex;


struct SimpleTwimlMessage {
    from: String,
    to: String,
    message: String,
}

fn create_client() -> twilio::Client {
    // Get the account ID
    let account_id_key = "TWILIO_ACCOUNT_SID";
    let account_id: String =  match dotenv::var(account_id_key) {
        Ok(val) => val,
        Err(_) => panic!("{} is not defined in the environment.", account_id_key)
    };
    // Get the auth token
    let auth_token_key = "TWILIO_AUTH_TOKEN";
    let auth_token: String = match dotenv::var(auth_token_key) {
        Ok(val) => val,
        Err(_) => panic!("{} is not defined in the environment.", auth_token_key)
    };
    //create the client
    twilio::Client::new(account_id.as_str(),auth_token.as_str())
}

fn send_message(client: twilio::Client, message: String, recepient: &str) {
    let phone_number_key = "TWILIO_PHONE_NUMBER";
    let phone_number: String = match dotenv::var(phone_number_key) {
        Ok(val) => val,
        Err(_) => panic!("{} is not defined in the environment.", phone_number_key)
    };
    let outbound_message: twilio::OutboundMessage = twilio::OutboundMessage {
        body : message.as_str(),
        from: phone_number.as_str(),
        to: recepient
    };
    let _ = client.send_message(outbound_message);
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/sms", data = "<input>" )]
fn sms(input: SimpleTwimlMessage) -> String {
    print!("/sms");
    input.message + input.from.as_str() + input.to.as_str()
}


impl rocket::data::FromData for SimpleTwimlMessage {
    type Error = String;

    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        // Read the data into a String.
        let mut string = String::new();
        if let Err(e) = data.open().read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }
        print!("{}", string);

        // grab the data from the request
        let to_regex: Regex = Regex::new(r"&To=(.*)&ToZip=").unwrap();
        let mut to = match to_regex.captures(string.as_str()) {
            Some(matches) => matches.get(1).map_or("", |m| m.as_str()).to_string(),
            None =>  "500 Internal error".to_string()
        };

        let from_regex: Regex = Regex::new(r"&From=(.*)&ApiVersion=").unwrap();
        let mut from = match from_regex.captures(string.as_str()) {
            Some(matches) => matches.get(1).map_or("", |m| m.as_str()).to_string(),
            None =>  "500 Internal error".to_string()
        };

        let body_regex: Regex = Regex::new(r"&Body=(.*)&FromCountry=").unwrap();
        let mut message = match body_regex.captures(string.as_str()) {
            Some(matches) => matches.get(1).map_or("", |m| m.as_str()).to_string(),
            None =>  "500 Internal error".to_string()
        };


        // convert to unicode
        from = convert_twilio_gsm7_to_utf8(from);
        to = convert_twilio_gsm7_to_utf8(to);
        message = convert_twilio_gsm7_to_utf8(message);


        Success(SimpleTwimlMessage {
            from: from,
            to: to,
            message: message
        })
    }
}

fn convert_twilio_gsm7_to_utf8(input: String) -> String {
    let mut input = input;
    let plus_to_space_regex: Regex = Regex::new(r"\+").unwrap();
    input = (*plus_to_space_regex.replace_all(input.as_str(), " ")).to_string();

    return (*percent_encoding::percent_decode(input.as_str().as_bytes()).decode_utf8_lossy()).to_string();
}




fn main() {

    let client = create_client();
//    send_message(client, "this is a test".to_string(), "+18472871920");
    rocket::ignite()
        .manage(client)
        .mount("/", routes![index, sms])
        .launch();
}
