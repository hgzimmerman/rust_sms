
use schema::users;

#[derive(Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String, // TODO create a phone number type?
//    groups: Vec<Group>,
//    pub state: SmState
}


#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String
}