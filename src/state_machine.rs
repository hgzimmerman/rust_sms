extern crate twilio;

use user_store::MockUserStore;
use twilio_client_wrapper::SimpleTwimlMessage;
use diesel::pg::PgConnection;
use models::users::RealizedUser;
use models::new_user_builders::realized_new_user_builder::RealizedNewUserBuilder;

#[derive(Debug, Clone)]
pub enum EventToken<'a> {
    RawInput { raw_input: String },
    Confirmation,
    Declination,
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
    let input = input.to_lowercase();
    match input.as_str() {
        "yes" |
        "yup" |
        "yeah" |
        "accept" |
        "confirmed" => Confirmation,
        "no"|
        "nah" |
        "nope"|
        "no thank you" |
        "no thanks"|
        "no thankyou" |
        "decline"=> Declination,
        "h" |
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


/// SmState is shorthand for State-Machine State, distinguishing it from Rocket's 'State'
#[derive(Debug, Clone)]
pub enum SmState {
    StartState,
    AwaitingEventConfirmationState,
    ConfirmingCancellationState,
}

impl SmState {
    //Consider making this take the SimpleTwimlMessage, and extracting the token from that
    pub fn next(self, event: EventToken) -> (SmState, Option<String>) {
        use EventToken::*;
        use SmState::*;

        info!("Transitioning to new state with current state: {:?}, and Token: {:?}", self, event);

        match (self.clone(), event) {
            (StartState, BoatAttendanceInternalRequest {message: m}) => {
                (AwaitingEventConfirmationState, Some(m.clone()))
            },
            (AwaitingEventConfirmationState, Confirmation) => {
                //Add user to event
                (StartState, Some("You have confirmed for $event".to_string()) )
            },
            (AwaitingEventConfirmationState, Declination) => {
                //Let user know they have declined
                (StartState, Some("You have declined the invitation for $event".to_string()) )
            },
            (StartState, CancellationRequestInitiation) => {
                //Get events the user is a participant within
                (ConfirmingCancellationState, Some("You are subscribed to the following events: 1. $event 2. $event. Text \"List events\" to get more specific info.".to_string()) )
            },
            (ConfirmingCancellationState, RawInput{raw_input: raw_input}) => {
                //Parse input to number
                match raw_input.parse::<i32>() {
                    Ok(parsed_number) => {
                        // get valid numbers
                        if parsed_number < 5 { // TODO check against a non-magic number, this should check the number of
                            // Get the event based on the number
                            // Cancel the event
                            (StartState, Some("You have canceled $event.".to_string()))
                        } else {
                            (self.clone(), Some("You aren't attending an event with that number.".to_string()))
                        }
                    },
                    Err(_) => {
                        (self.clone(), Some("Please enter a valid number.".to_string()))
                    }
                }
            }
            (_, HelpRequest) => {
                // send help message.
                (self.clone(), Some("help message".to_string()))
            },
            _ => {
                // Let user know they had invalid input.
                (self.clone(), Some("Invalid input".to_string()))
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
            }
            None => {
                // current user doesn't exist in a fully realized state
                match RealizedNewUserBuilder::get_by_phone_number(&twim.from, db_connection) {
                    Some(provisional_user) => {
                        let (new_state, message) = provisional_user.clone().builder_state.next(token);
                        provisional_user.db_update(db_connection);
                        message.unwrap()
                    },
                    None => {
                        let new_user = RealizedNewUserBuilder::new(twim.from);
                        new_user.db_insert(&db_connection);
                        "You don't have an account yet, you can start by entering your first name".to_string()
                    }
                }

            }
        }
    }

    pub fn alt_next(mut self, event: EventToken) -> Option<String> {
        self = SmState::AwaitingEventConfirmationState;
        Some("You have confirmed".to_string())
    }
}


impl Into<i32> for SmState {
    fn into(self) -> i32{
        use SmState::*;
        match self {
            StartState => 0,
            AwaitingEventConfirmationState => 1,
            ConfirmingCancellationState => 2,
//            NewUserState => 6
        }
    }
}

impl From<i32> for SmState {
    fn from(number: i32) -> SmState {
        use SmState::*;
        match number {
            0 => StartState,
            1 => AwaitingEventConfirmationState,
            2 => ConfirmingCancellationState,

//            6 => NewUserState{}
            _ => panic!("Tried to convert number {} to state", number)
        }
    }
}