use state_machine::{SmState, tokenize_input, EventToken};
use user_store::MockUserStore;
use twilio_client_wrapper::SimpleTwimlMessage;

#[derive( Clone)]
enum Group {
    PreNovice,
    Novice,
    HighSchool,
    Collegiate,
    Masters,
    Casual,
    Private
}

#[derive( Clone)]
pub struct User {
    pub id: i32,
    first_name: String,
    last_name: String,
    pub phone_number: String, // TODO create a phone number type?
    groups: Vec<Group>,
    pub state: SmState
}

impl User {

    pub fn new(first_name: String, last_name: String, phone_number: String) -> User {
        User {
            id: 1, // ???
            first_name: first_name,
            last_name: last_name,
            phone_number: phone_number,
            groups: Vec::new(),
            state: SmState::StartState,
        }
    }

    ///TEMP, should probably be replaced by a builder.
    pub fn empty() -> User {
        User {
            id: 0,
            first_name: "".to_string(),
            last_name: "".to_string(),
            phone_number: "".to_string(),
            groups: Vec::new(),
            state: SmState::StartState
        }
    }

    pub fn set_state(&mut self, new_state: SmState) {
        self.state = new_state
    }




}



pub trait AdminUser {

}