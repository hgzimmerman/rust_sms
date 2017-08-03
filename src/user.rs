use state_machine::State;

enum Group {
    PreNovice,
    Novice,
    HighSchool,
    Collegiate,
    Masters,
    Casual,
    Private
}

pub struct User {
    id: i32,
    first_name: String,
    last_name: String,
    pub phone_number: String, // TODO create a phone number type?
    groups: Vec<Group>,
    pub state: State
}

impl User {

    pub fn new(first_name: String, last_name: String, phone_number: String) -> User {
        User {
            id: 1, // ???
            first_name: first_name,
            last_name: last_name,
            phone_number: phone_number,
            groups: Vec::new(),
            state: State::StartState,
        }
    }

}


pub trait AdminUser {

}