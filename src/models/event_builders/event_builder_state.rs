use diesel::pg::PgConnection;
use state_machine::EventToken;
use state_machine::State;

use super::RealizedEventBuilder;


#[derive(Clone, Copy, Debug)]
pub enum EventBuilderState {
    StartState,
    AwaitingTitle,
    AwaitingLocation,
    AwaitingStartTime,
    AwaitingEndTime,
    AwaitingRequestedParticipants,
    AwaitingGroups,
    Confirming,
    Done
}

impl From<i32> for EventBuilderState {
    fn from(number: i32) -> Self {
        match number {
            1 => EventBuilderState::StartState,
            2 => EventBuilderState::AwaitingTitle,
            3 => EventBuilderState::AwaitingLocation,
            4 => EventBuilderState::AwaitingStartTime,
            5 => EventBuilderState::AwaitingEndTime,
            6 => EventBuilderState::AwaitingRequestedParticipants,
            7 => EventBuilderState::AwaitingGroups,
            8 => EventBuilderState::Confirming,
            9 => EventBuilderState::Done,
            _ => panic!("Tried to convert number: {}", number)
        }
    }
}

impl Into<i32> for EventBuilderState {
    fn into(self) -> i32 {
        match self {
            EventBuilderState::StartState => 1,
            EventBuilderState::AwaitingTitle => 2,
            EventBuilderState::AwaitingLocation => 3,
            EventBuilderState::AwaitingStartTime => 4,
            EventBuilderState::AwaitingEndTime => 5,
            EventBuilderState::AwaitingRequestedParticipants => 6,
            EventBuilderState::AwaitingGroups => 7,
            EventBuilderState::Confirming => 8,
            EventBuilderState::Done => 9,
        }
    }
}

impl State<RealizedEventBuilder> for EventBuilderState {
    fn next(self, event: EventToken, builder: &RealizedEventBuilder, db_connection: PgConnection) -> (EventBuilderState, Option<String>) {
        (self, Some("Not implemented".to_string()))
    }
}
