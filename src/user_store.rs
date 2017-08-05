
use user::User;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Mutex, MutexGuard};

#[derive( Clone)]
pub struct MockUserStore {
    users: Vec<User>
}

impl <'a> MockUserStore{
    pub fn new() -> MockUserStore {
        MockUserStore {
            users: Vec::new()
        }
    }

    pub fn init() -> MockUserStore {
        let mut store: MockUserStore = MockUserStore::new();
        let user_henry: User = User::new("Henry".to_string(), "Zimmerman".to_string(), "+18472871920".to_string()); // insert some test data
        store.insert(user_henry);
        store
    }

    pub fn get_user_by_id(self, id: i32) -> Option<User>{
        for user in self.users {
            if user.id == id {
                return Some(user);
            }
        }
        return None;
    }

    pub fn get_user_by_phone_number(&self, phone_number: &str) -> Option<&User> {
        for user in &self.users {
            if user.phone_number.as_str() == phone_number {
                return Some(user);
            }
        }
        return None;
    }

    pub fn update_user(&mut self, user: &User) {
        println!("updating user");
        self.users.retain(|u| u.id != user.id); // retain all who have a different id
        self.insert(user.clone());
    }

    pub fn insert(&mut self, user: User) {
        self.users.push(user);
    }

}


//Forget this, just fucking pass the state around :/
lazy_static! {
    pub static ref USER_STORE: Mutex<MockUserStore> =  {
        let mut s = MockUserStore::new();
        let user_henry: User = User::new("Henry".to_string(), "Zimmerman".to_string(), "+18472871920".to_string()); // insert some test data
        s.insert(user_henry);
        return Mutex::new(s);
    };
}

pub fn get_user_store<'a>() -> MutexGuard<'a, MockUserStore> {
    println!("getting user store");
    USER_STORE.try_lock().unwrap()
}

