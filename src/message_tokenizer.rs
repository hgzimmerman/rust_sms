extern crate twilio;
use twilio::Client;
use twilio_client_wrapper;
use std::boxed::Box;

#[derive( Clone)]
pub enum Event<'a> {
    RawInput { raw_input: String },
    Confirmation,
    Declination,
    HelpRequest,
    CancellationRequestInitiation,
    ChangeGroupsRequest,
    BoatAttendanceInternalRequest {message: &'a String}, // sent by the system, do not parse input to this event.
    Reminder {message: &'a String}
}

pub fn tokenize_input<'a>(input: String) -> Event<'a> {
    use Event::*;
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
        _ => {
            RawInput { raw_input : input }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    StartState,
    AwaitingEventConfirmationState,
    ConfirmingCancellationState,
    AwaitingFirstNameState,
    AwaitingLastNameState,
    ConfirmingNameState
}

impl State {
    pub fn next(self, event: Event) -> (State, Option<String>) {
        use Event::*;
        use State::*;
        match (self, event) {
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

            (_, HelpRequest) => {
                // send help message.
                (self, Some("help message".to_string()))
            }
            _ => {
                // Let user know they had invalid input.
                (self, Some("Invalid input".to_string()))
            }
        }
    }
}