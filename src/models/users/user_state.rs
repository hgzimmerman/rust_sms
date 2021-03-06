use state_machine::EventToken;


pub struct UserStateType;

#[derive(Debug, Clone, Copy, PartialEq)]
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

        match (self, event) {
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
                            (self, Some("You aren't attending an event with that number.".to_string()))
                        }
                    },
                    Err(_) => {
                        (self, Some("Please enter a valid number.".to_string()))
                    }
                }
            },
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

use diesel::pg::Pg;
use std::error::Error;
use std::io::Write;
use diesel::row::Row;
use diesel::expression::AsExpression;
use diesel::expression::bound::Bound;
use diesel::types::FromSqlRow;
use diesel::types::ToSqlOutput;
use diesel::types::ToSql;
use diesel::types::HasSqlType;
use diesel::types::IsNull;
use diesel::types::NotNull;


impl HasSqlType<UserStateType> for Pg {
    fn metadata(lookup: &Self::MetadataLookup) -> Self::TypeMetadata {
        lookup.lookup_type("user_state_type")
    }
}

impl NotNull for UserStateType {}

impl<'a> AsExpression<UserStateType> for &'a UserState {
    type Expression = Bound<UserStateType, &'a UserState>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl ToSql<UserStateType, Pg> for UserState {
    fn to_sql<W: Write>(&self, out: &mut ToSqlOutput<W, Pg>) -> Result<IsNull, Box<Error+Send+Sync>> {
        match *self {
            UserState::StartState => out.write_all(b"startState")?,
            UserState::AwaitingEventConfirmationState => out.write_all(b"awaitingEventConfirmationState")?,
            UserState::ConfirmingCancellationState => out.write_all(b"confirmingCancellationState")?
        }
        Ok(IsNull::No)
    }
}

impl FromSqlRow<UserStateType, Pg> for UserState {
    fn build_from_row<T: Row<Pg>>(row: &mut T) -> Result<Self, Box<Error+Send+Sync>> {
        match row.take() {
            Some(b"startState") => Ok(UserState::StartState),
            Some(b"awaitingEventConfirmationState") => Ok(UserState::AwaitingEventConfirmationState),
            Some(b"confirmingCancellationState") => Ok(UserState::ConfirmingCancellationState),
            Some(_) => Err("Unrecognized enum variant".into()),
            None => Err("Unexpected null for non-null column".into()),
        }
    }
}
