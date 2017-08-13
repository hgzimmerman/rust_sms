pub mod realized_new_user_builder;
pub mod user_builder_state;
pub use self::realized_new_user_builder::RealizedNewUserBuilder;
pub use self::user_builder_state::UserBuilderState;


use schema::new_user_builders;

///This uses the phone number as the primary key, I'm not sure how I feel about that...
/// The code is responsible for ensuring that this "key" isn't in the system
#[derive(Queryable, Identifiable, Clone, Insertable, AsChangeset )]
#[primary_key(phone_number)]
#[table_name="new_user_builders"]
pub struct NewUserBuilder {
    pub phone_number: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub builder_state: i32
}






