use state_machine::EventToken;


#[derive(Clone, Debug)]
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
    pub fn next(self, event: EventToken) -> (Self, Option<String>) {
        unimplemented!()
    }


}