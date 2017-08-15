pub mod user_state;
pub mod realized_user;

use state_machine::EventToken;

pub use self::realized_user::RealizedUser;

use diesel::pg::PgConnection;
use diesel;
use diesel::*;
use diesel::prelude::*;


use schema::*;

#[derive(Queryable, Identifiable, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String, // TODO create a phone number type?
//    groups: Vec<Group>,
    pub state: UserState
}
//
//expression_impls!( UserState -> UserState,);
//table! {
//    articles {
//        id -> Integer,
//        first_name -> VarChar,
//        last_name -> VarChar,
//        phone_number -> VarChar,
//        state -> ::types::UserState,
//    }
//}



#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub state: UserState
}



impl NewUser {
    pub fn new(first_name: String, last_name: String, phone_number: String) -> NewUser {
        NewUser {
            first_name: first_name,
            last_name: last_name,
            phone_number: phone_number,
            //groups: Vec::new(),
            state: UserState::StartState.into(),
        }
    }

    pub fn db_insert(&self, connection: &PgConnection) {
        use schema::users;

        diesel::insert(self)
            .into(users::table)
            .execute(connection)
            .expect("Error saving user");
    }
}




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

pub mod types {
    use diesel::pg::Pg;
    use std::error::Error;
    use std::io::Write;
    use diesel::row::Row;
    use diesel::expression::AsExpression;
    use diesel::expression::bound::Bound;
    use diesel::types::*;

    #[derive(Debug)]
    pub struct UserState;


    impl HasSqlType<UserState> for Pg {
        fn metadata(lookup: &Self::MetadataLookup) -> Self::TypeMetadata {
            lookup.lookup_type("Ustate")
        }
    }

    impl NotNull for UserState {}

    impl<'a> AsExpression<UserState> for &'a super::UserState {
        type Expression = Bound<UserState, &'a super::UserState>;

        fn as_expression(self) -> Self::Expression {
            Bound::new(self)
        }
    }

    impl ToSql<UserState, Pg> for super::UserState {
        fn to_sql<W: Write>(&self, out: &mut ToSqlOutput<W, Pg>) -> Result<IsNull, Box<Error + Send + Sync>> {
            match *self {
                super::UserState::StartState => out.write_all(b"startState")?,
                super::UserState::AwaitingEventConfirmationState => out.write_all(b"awaitingEventConfirmationState")?,
                super::UserState::ConfirmingCancellationState => out.write_all(b"confirmingCancellationState")?
            }
            Ok(IsNull::No)
        }
    }

    impl FromSqlRow<UserState, Pg> for super::UserState {
        fn build_from_row<T: Row<Pg>>(row: &mut T) -> Result<Self, Box<Error + Send + Sync>> {
            match row.take() {
                Some(b"startState") => Ok(super::UserState::StartState),
                Some(b"awaitingEventConfirmationState") => Ok(super::UserState::AwaitingEventConfirmationState),
                Some(b"confirmingCancellationState") => Ok(super::UserState::ConfirmingCancellationState),
                Some(_) => Err("Unrecognized enum variant".into()),
                None => Err("Unexpected null for non-null column".into()),
            }
        }
    }
}

