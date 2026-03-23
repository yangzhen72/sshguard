use ssh2::Session;
use std::collections::HashMap;
use parking_lot::RwLock;

lazy_static::lazy_static! {
    pub static ref SESSION_MANAGER: RwLock<SessionManager> = RwLock::new(SessionManager::new());
}

pub struct SessionManager {
    sessions: HashMap<String, Session>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }
}
