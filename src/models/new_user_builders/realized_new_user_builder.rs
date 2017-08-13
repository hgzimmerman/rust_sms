
use super::NewUserBuilder;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::users::{NewUser, UserState};

use super::user_builder_state::UserBuilderState;


#[derive(Clone, Debug)]
pub struct RealizedNewUserBuilder {
    pub phone_number: String,
    first_name: Option<String>,
    last_name: Option<String>,
    pub builder_state: UserBuilderState
}




impl RealizedNewUserBuilder {

    pub fn new(phone_number: String) -> RealizedNewUserBuilder {
        RealizedNewUserBuilder {
            first_name: None,
            last_name: None,
            phone_number: phone_number,
            builder_state: UserBuilderState::AwaitingFirstName
        }
    }


    pub fn build(self) -> Option<NewUser> {

        let first_name = match self.first_name {
            Some(first_name) => first_name,
            None => {
                error!("Tried to call build without setting the first name.");
                return None
            }
        };
        let last_name = match self.last_name {
            Some(last_name) => last_name,
            None => {
                error!("Tried to call build without setting the last name.");
                return None
            }
        };

        // Actually build the new user
        Some( NewUser {
            first_name: first_name,
            last_name: last_name,
            phone_number: self.phone_number,
            state: UserState::StartState.into()
        })
    }

    pub fn add_first_name(&mut self, first_name: String) {
        self.first_name = Some(first_name);
    }
    pub fn add_last_name(&mut self, last_name: String) {
        self.last_name = Some(last_name);
    }
    pub fn add_phone_number(&mut self, phone_number: String) {
        self.phone_number = phone_number;
    }

    pub fn get_printable_name(&self) -> String {
        let self_clone: RealizedNewUserBuilder = self.clone();
        let first = self_clone.first_name.clone().unwrap();
        let last = self_clone.last_name.clone().unwrap();
        format!("{} {}", first, last)
    }


    pub fn db_insert(&self, connection: &PgConnection) {
        use schema::new_user_builders;

        let u: NewUserBuilder = self.clone().into();

        diesel::insert(&u)
            .into(new_user_builders::table)
            .execute(connection)
            .expect("Error saving provisional user");
    }

    pub fn db_update(&self, connection:&PgConnection) {
        use schema::new_user_builders;

        let u: NewUserBuilder = self.clone().into();
        diesel::update(new_user_builders::table)
           .set(&u)
           .execute(connection)
           .expect("Error updating");
    }

    pub fn get_by_phone_number(searched_phone_number: &String, connection: &PgConnection) -> Option<RealizedNewUserBuilder> {
        use schema::new_user_builders::dsl::*;

        let phone_num: String = searched_phone_number.clone();
        let results = new_user_builders.filter(phone_number.eq(phone_num))
            .limit(1)
            .load::<NewUserBuilder>(connection)
            .expect("ERR loading users");

        // get the only element in the results
        match results.iter().last() {
            Some(user_builder) => Some(RealizedNewUserBuilder::from(user_builder.clone())),    // Clone to get ownership, then convert.
            None => None
        }
    }


}



impl Into<NewUserBuilder> for RealizedNewUserBuilder {
    fn into(self) -> NewUserBuilder {
        NewUserBuilder {
            phone_number: self.phone_number,
            first_name: self.first_name,
            last_name: self.last_name,
            builder_state: self.builder_state.into(),
        }
    }
}

impl From<NewUserBuilder> for RealizedNewUserBuilder {
    fn from(new_user_builder: NewUserBuilder) -> Self {
        RealizedNewUserBuilder {
            phone_number: new_user_builder.phone_number,
            first_name: new_user_builder.first_name,
            last_name: new_user_builder.last_name,
            builder_state: new_user_builder.builder_state.into(),
        }
    }
}
