
pub struct User {
    id: i32,
    first_name: String,
    last_name: String,
    pub phone_number: String,
    groups: Vec<Group>
}


enum Group {
    PreNovice,
    Novice,
    HighSchool,
    Collegiate,
    Masters,
    Casual,
    Private
}