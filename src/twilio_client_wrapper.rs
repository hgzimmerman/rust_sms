extern crate dotenv;
extern crate twilio;
use twilio::Client;

use user::User;


pub fn create_client() -> twilio::Client {
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

fn send_message(client: &twilio::Client, message: String, recepient: &str) {
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

pub fn send_message_to_user(client: &twilio::Client, message: String, user: &User) {
    send_message(client, message, user.phone_number.as_str());
    println!("sent message");
}