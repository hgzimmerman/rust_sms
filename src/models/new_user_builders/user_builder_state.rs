use state_machine::EventToken;
use new_user_builders::realized_new_user_builder::RealizedNewUserBuilder;
use diesel::pg::PgConnection;

#[derive(Clone, Copy, Debug)]
pub enum UserBuilderState {
    AwaitingFirstName, // start state
    AwaitingLastName,
    Confirming,
    Done
}


impl From<i32> for UserBuilderState {
    fn from(number: i32) -> Self {
        match number {
            1 => UserBuilderState::AwaitingFirstName,
            2 => UserBuilderState::AwaitingLastName,
            3 => UserBuilderState::Confirming,
            4 => UserBuilderState::Done,
            _ => panic!("Tried to convert number: {}", number)
        }
    }
}

impl Into<i32> for UserBuilderState {
    fn into(self) -> i32 {
        match self {
            UserBuilderState::AwaitingFirstName => 1,
            UserBuilderState::AwaitingLastName => 2,
            UserBuilderState::Confirming => 3,
            UserBuilderState::Done => 4
        }
    }
}


impl UserBuilderState {
    // Consider removing the builder and making each state grab it from the DB.
    pub fn next(self, event: EventToken, builder: &RealizedNewUserBuilder, db_connection: &PgConnection) -> (UserBuilderState, Option<String>) {
        use EventToken::*;
        use self::UserBuilderState::*;
        info!("Transitioning to a new user builder state with current builder state: {:?}, and Token: {:?}", self, event);

        match (self.clone(), event) {
            (AwaitingFirstName, RawInput{raw_input: message}) => {
                let mut mutable_builder: RealizedNewUserBuilder = builder.clone();
                mutable_builder.add_first_name(message);
                mutable_builder.db_update(db_connection);
                (AwaitingLastName, Some("What is your last name".to_string()))
            },
            (AwaitingLastName, RawInput {raw_input: message}) => {
                let mut mutable_builder: RealizedNewUserBuilder = builder.clone();
                mutable_builder.add_last_name(message);
                mutable_builder.db_update(db_connection);
                (Confirming, Some(format!("Your name is {}. Text !confirm to create your account or !restart to change your name.", mutable_builder.get_printable_name()).to_string()))
            },
            (Confirming, Confirmation) => {
                builder.clone().build().unwrap().db_insert(db_connection); // Because this state block is only reachable after the builder has had its first and last names set, unwrapping this is safe.
                // TODO, remove this entry from the db.
                (Done, Some("Account created. You must wait for an administrator to approve your account to receive alerts. Nonetheless, type !help to access help".to_string()))
            },
            (Confirming, Restart) => {
                (AwaitingFirstName, Some("You have restarted the signup process, please enter your first name.".to_string()))
            },
            (Done, _) => {
                error!("Unreachable state encountered.");
                (self, None)
            },
            _ => {
                (self, Some("Invalid input".to_string()))
            }
        }

    }


}