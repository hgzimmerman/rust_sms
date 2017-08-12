
use super::NewUserBuilder;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct RealizedNewUserBuilder {
    pub phone_number: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub builder_state: BuilderState
}




impl RealizedNewUserBuilder {

    pub fn db_insert(self, connection: &PgConnection) {
        use schema::new_user_builders;



        let u: NewUserBuilder = self.into();

        diesel::insert(&u)
            .into(new_user_builders::table)
            .execute(connection)
            .expect("Error saving provisional user");

    }

    pub fn db_update(&self, connection:&PgConnection) {
        unimplemented!()
    }

    pub fn get_by_phone_number(searched_phone_number: String, connection: &PgConnection) -> RealizedNewUserBuilder {
        unimplemented!()
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





#[derive(Clone)]
pub enum BuilderState {
    AwaitingFirstName, // start state
    AwaitingLastName,
    Confirming,
    Done
}


impl From<i32> for BuilderState {
    fn from(number: i32) -> Self {
        match number {
            1 => BuilderState::AwaitingFirstName,
            2 => BuilderState::AwaitingLastName,
            3 => BuilderState::Confirming,
            4 => BuilderState::Done,
            _ => panic!("Tried to convert number: {}", number)
        }
    }
}

impl Into<i32> for BuilderState {
    fn into(self) -> i32 {
        match self {
            BuilderState::AwaitingFirstName => 1,
            BuilderState::AwaitingLastName => 2,
            BuilderState::Confirming => 3,
            BuilderState::Done => 4
        }
    }
}