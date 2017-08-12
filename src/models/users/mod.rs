use schema::users;
use state_machine::SmState;

/// Db interfaceable user
#[derive(Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String, // TODO create a phone number type?
//    groups: Vec<Group>,
    pub state: i32
}


#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub state: i32
}




impl NewUser {
    pub fn new(first_name: String, last_name: String, phone_number: String) -> NewUser {
        NewUser {
            first_name: first_name,
            last_name: last_name,
            phone_number: phone_number,
            //groups: Vec::new(),
            state: SmState::StartState.into(),
        }
    }
}

/// User
#[derive(Clone)]
pub struct RealizedUser {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String, // TODO create a phone number type?
    //    pub groups: Vec<Group>,
    pub state: SmState
}

impl From<User> for RealizedUser {
    fn from(user: User) -> Self {
        RealizedUser {
            id : user.id,
            first_name : user.first_name.clone(),
            last_name : user.last_name.clone(),
            phone_number : user.phone_number.clone(),
            state : user.state.into()
        }
    }
}