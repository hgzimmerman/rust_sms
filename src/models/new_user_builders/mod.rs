pub mod realized_new_user_builder;
pub mod user_builder_state;
use self::realized_new_user_builder::RealizedNewUserBuilder;

use models::users::NewUser;

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






