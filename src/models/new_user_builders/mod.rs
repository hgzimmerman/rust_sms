mod realized_new_user_builder;
use self::realized_new_user_builder::RealizedNewUserBuilder;

use models::users::NewUser;
use state_machine::SmState;
use schema::new_user_builders;
use std::borrow::ToOwned;


///This uses the phone number as the primary key, I'm not sure how I feel about that...
/// The code is responsible for ensuring that this "key" isn't in the system
#[derive(Queryable, Identifiable, Clone, Insertable )]
#[primary_key(phone_number)]
#[table_name="new_user_builders"]
pub struct NewUserBuilder {
    pub phone_number: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub builder_state: i32
}


impl NewUserBuilder {
    fn new(phone_number: String) -> NewUserBuilder {
        NewUserBuilder {
            first_name: None,
            last_name: None,
            phone_number: phone_number,
            builder_state: 0
        }
    }


    pub fn build(self) -> Option<NewUser> {

        let first_name = self.first_name.unwrap_or(return None);
        let last_name = self.last_name.unwrap_or(return None);

        Some(NewUser {
            first_name: first_name,
            last_name: last_name,
            phone_number: self.phone_number.clone(),
            state: SmState::StartState.into()
        })
    }


    fn add_first_name(&mut self, first_name: String) {
        self.first_name = Some(first_name);
    }
    fn add_last_name(&mut self, last_name: String) {
        self.last_name = Some(last_name);
    }
    fn add_phone_number(&mut self, phone_number: String) {
        self.phone_number = phone_number;
    }
}





