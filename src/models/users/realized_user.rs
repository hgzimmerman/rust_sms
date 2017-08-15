use schema::users;
use diesel::pg::PgConnection;
use diesel;
use diesel::prelude::*;
use users::UserState;
use users::User;


#[derive(Clone, Debug)]
pub struct RealizedUser {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String, // TODO create a phone number type?
    //    pub groups: Vec<Group>,
//    pub state: UserState
}

impl RealizedUser {

//    pub fn db_update_state(&self, new_state: UserState, connection: &PgConnection) {
//        use schema::users::dsl::*;
//
//        let db_user: User = self.clone().into();
//        let state_representation: i32 = new_state.into();
//        diesel::update(&db_user)
//            .set(state.eq(state_representation))
//            .execute(connection);
//    }
//
//    /// Given a string representing a phone number, search the db for the corresponding user
//    pub fn get_user_by_phone_number(searched_phone_number: &String, connection: &PgConnection) -> Option<RealizedUser> {
//        use schema::users::dsl::*;
//
//        let phone_num: String = searched_phone_number.clone();
//        let results = users.filter(phone_number.eq(phone_num))
//            .limit(1)
//            .load::<User>(connection)
//            .expect("ERR loading users");
//
//        // get the only element in the results
//        match results.iter().last() {
//            Some(user) => Some(RealizedUser::from(user.clone())),    // Clone the user to get ownership, the convert to the app based user
//            None => None
//        }
//    }
}

//impl From<User> for RealizedUser {
//    fn from(user: User) -> Self {
//        RealizedUser {
//            id : user.id,
//            first_name : user.first_name.clone(),
//            last_name : user.last_name.clone(),
//            phone_number : user.phone_number.clone(),
//            state : user.state.into()
//        }
//    }
//}
//
//impl Into<User> for RealizedUser {
//    fn into(self) -> User {
//        User {
//            id: self.id,
//            first_name: self.first_name.clone(),
//            last_name: self.last_name.clone(),
//            phone_number: self.phone_number.clone(),
//            state : self.state.into()
//        }
//    }
//}