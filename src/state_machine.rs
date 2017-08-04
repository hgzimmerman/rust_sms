extern crate twilio;

#[derive( Clone)]
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
    pub fn next(self, event: EventToken) -> (State, Option<String>) {
        use EventToken::*;
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
                            (self, Some("You aren't attending an event with that number.".to_string()))
                        }
                    },
                    Err(_) => {
                        (self, Some("Please enter a valid number.".to_string()))
                    }
                }
            }
            (_, HelpRequest) => {
                // send help message.
                (self, Some("help message".to_string()))
            },
            _ => {
                // Let user know they had invalid input.
                (self, Some("Invalid input".to_string()))
            }
        }
    }

    pub fn alt_next(mut self, event: EventToken) -> Option<String> {
        self = State::AwaitingEventConfirmationState;
        Some("You have confirmed".to_string())
    }
}