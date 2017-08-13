extern crate twilio;

use twilio_client_wrapper::SimpleTwimlMessage;
use diesel::pg::PgConnection;
use models::users::RealizedUser;
use models::new_user_builders::RealizedNewUserBuilder;

#[derive(Debug, Clone)]
pub enum EventToken<'a> {
    RawInput { raw_input: String },
    Confirmation,
    Declination,
    Restart,
    HelpRequest,
    CancellationRequestInitiation,
    ChangeGroupsRequest,
    ListGroupsRequest,
    ListEventsRequest,
    BoatAttendanceInternalRequest {message: &'a String}, // sent by the system, do not parse input to this event.
    Reminder {message: &'a String}
}

pub fn tokenize_input<'a>(input: String) -> EventToken<'a> {
    use EventToken::*;
    let sanitized_input = input.to_lowercase();
    match sanitized_input.as_str() {
        "yes" |
        "yup" |
        "yeah" |
        "accept" |
        "confirm" |
        "!confirm" |
        "confirmed" => Confirmation,
        "no"|
        "nah" |
        "nope"|
        "no thank you" |
        "no thanks"|
        "no thankyou" |
        "decline"=> Declination,
        "!restart" => Restart,
        "h" |
        "!help" |
        "commands" |
        "options" => HelpRequest,
        "cancel boat" => CancellationRequestInitiation,
        "change group" |
        "change groups" => ChangeGroupsRequest,
        "list groups" => ListGroupsRequest,
        "list boats" => ListEventsRequest,
        _ => {
            RawInput { raw_input : input }
        }
    }
}



pub fn handle_input(twim: SimpleTwimlMessage, db_connection: &PgConnection) -> String {
    let token: EventToken = tokenize_input(twim.message);

    match RealizedUser::get_user_by_phone_number(&twim.from, db_connection) {
        Some(user) => {
            let (new_state, message) = user.clone().state.next(token); // Consider moving this into a fn in User
            let mut user = user;
            user.db_update_state(new_state, db_connection);
            message.unwrap()
        },
        None => {
            // current user doesn't exist in a fully realized state
            match RealizedNewUserBuilder::get_by_phone_number(&twim.from, db_connection) {
                Some(provisional_user) => {
                    info!("Found an existing user builder: {:?}", provisional_user);
                    let (new_state, message) = provisional_user.clone().builder_state.next(token, &provisional_user, db_connection);
                    info!("New state: {:?}", new_state);
                    let mut mutable_provisional_user = provisional_user;
                    mutable_provisional_user.builder_state = new_state;
                    mutable_provisional_user.db_update(db_connection);
                    message.unwrap()
                },
                None => {
                    info!("Didn't find user, creating a user builder");
                    let new_user = RealizedNewUserBuilder::new(twim.from);
                    new_user.db_insert(&db_connection);
                    "You don't have an account yet, you can start by entering your first name.".to_string()
                }
            }
        }
    }
}
