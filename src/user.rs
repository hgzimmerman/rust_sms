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


#[derive(Clone, Debug)]
pub struct UserBuilder {
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>
}


impl UserBuilder {
    fn new() -> UserBuilder {
        UserBuilder {
            first_name: None,
            last_name: None,
            phone_number: None
        }
    }

    fn build(&self) -> Option<User> {
        if  self.first_name.is_none()
            || self.last_name.is_none()
            || self.phone_number.is_none() {
            println!("user creation failed");
            None
        } else {
            Some(User {
                id: 0,
                first_name: self.clone().first_name.unwrap(),
                last_name: self.clone().last_name.unwrap(),
                phone_number: self.clone().phone_number.unwrap(),
                groups: Vec::new(),
                state: SmState::StartState
            })
        }


    }

    fn add_first_name(&mut self, first_name: String) {
        self.first_name = Some(first_name);
    }
    fn add_last_name(&mut self, last_name: String) {
        self.last_name = Some(last_name);
    }
    fn add_phone_number(&mut self, phone_number: String) {
        self.phone_number = Some(phone_number);
    }
}



pub trait AdminUser {

}