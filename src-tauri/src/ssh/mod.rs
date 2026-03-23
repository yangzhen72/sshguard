use parking_lot::RwLock;
use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref SESSION_MANAGER: RwLock<SessionManager> = RwLock::new(SessionManager::new());
}

pub struct SessionManager {
    sessions: HashMap<String, ()>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }
}
