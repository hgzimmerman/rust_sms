use user_store::MockUserStore;
use user::User;
use std::sync::Mutex;


pub struct DbHandle {
    pub user_store: MockUserStore
}

impl DbHandle{
    pub fn new() -> DbHandle {
        DbHandle {
            user_store:MockUserStore::new()
        }
    }

}


lazy_static! {
    pub static ref DB_HANDLE: Mutex<DbHandle> =  {
        let mut h = DbHandle::new();
        let user_henry: User = User::new("Henry".to_string(), "Zimmerman".to_string(), "+18472871920".to_string());
        h.user_store.insert(user_henry);
        Mutex::new(h)
    };
}