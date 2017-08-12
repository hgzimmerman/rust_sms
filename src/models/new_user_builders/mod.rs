use models::users::NewUser;
use state_machine::SmState;


pub struct NewUserBuilder {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>
}


impl NewUserBuilder {
    pub fn build(self) -> Option<NewUser> {

        let first_name = self.first_name.unwrap_or(return None);
        let last_name = self.last_name.unwrap_or(return None);
        let phone_number = self.phone_number.unwrap_or(return None);

        Some(NewUser {
            first_name: first_name,
            last_name: last_name,
            phone_number: phone_number,
            state: SmState::StartState.into()
        })
    }
}