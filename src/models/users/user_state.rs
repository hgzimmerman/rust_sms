use state_machine::EventToken;

/// SmState is shorthand for State-Machine State, distinguishing it from Rocket's 'State'
#[derive(Debug, Clone)]
pub enum UserState {
    StartState,
    AwaitingEventConfirmationState,
    ConfirmingCancellationState,
}

impl UserState {
    //Consider making this take the SimpleTwimlMessage, and extracting the token from that
    pub fn next(self, event: EventToken) -> (UserState, Option<String>) {
        use EventToken::*;
        use self::UserState::*;

        info!("Transitioning to a new user state with current state: {:?}, and Token: {:?}", self, event);

        match (self.clone(), event) {
            (StartState, BoatAttendanceInternalRequest { message: m }) => {
                (AwaitingEventConfirmationState, Some(m.clone()))
            },
            (AwaitingEventConfirmationState, Confirmation) => {
                //Add user to event
                (StartState, Some("You have confirmed for $event".to_string()))
            },
            (AwaitingEventConfirmationState, Declination) => {
                //Let user know they have declined
                (StartState, Some("You have declined the invitation for $event".to_string()))
            },
            (StartState, CancellationRequestInitiation) => {
                //Get events the user is a participant within
                (ConfirmingCancellationState, Some("You are subscribed to the following events: 1. $event 2. $event. Text \"List events\" to get more specific info.".to_string()))
            },
            (ConfirmingCancellationState, RawInput { raw_input: raw_input }) => {
                //Parse input to number
                match raw_input.parse::<i32>() {
                    Ok(parsed_number) => {
                        // get valid numbers
                        if parsed_number < 5 {
                            // TODO check against a non-magic number, this should check the number of
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

}


impl Into<i32> for UserState {
    fn into(self) -> i32{
        use self::UserState::*;
        match self {
            StartState => 0,
            AwaitingEventConfirmationState => 1,
            ConfirmingCancellationState => 2,
        }
    }
}

impl From<i32> for UserState {
    fn from(number: i32) -> UserState {
        use self::UserState::*;
        match number {
            0 => StartState,
            1 => AwaitingEventConfirmationState,
            2 => ConfirmingCancellationState,

            _ => panic!("Tried to convert number {} to state", number)
        }
    }
}
