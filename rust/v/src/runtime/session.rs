use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Clone)]
pub enum LoginSessionState {
    BeforeLogin,
    AfterLogin,
    Transition,
    InGame,
}

#[derive(Clone)]
pub struct LoginSession {
    pub id: u32,
    pub account_id: Option<i64>,
    pub hwid: Option<String>,
    pub selected_world_id: Option<u8>,
    pub selected_channel_id: Option<u8>,
    pub login_state: LoginSessionState,
}

pub struct SessionStore {
    sessions: RwLock<HashMap<u32, LoginSession>>,
    next_id: AtomicU32,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            next_id: AtomicU32::new(1),
        }
    }

    pub fn insert(&self, mut session: LoginSession) -> u32 {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        session.id = id;
        self.sessions
            .write()
            .expect("session store write lock poisoned")
            .insert(id, session);
        id as u32
    }

    pub fn get(&self, id: u32) -> Option<LoginSession> {
        self.sessions
            .read()
            .expect("session store read lock poisoned")
            .get(&id)
            .cloned()
    }

    pub fn update(&self, id: u32, f: impl FnOnce(&mut LoginSession)) {
        let mut guard = self
            .sessions
            .write()
            .expect("session store write lock poisoned");
        if let Some(session) = guard.get_mut(&id) {
            f(session);
        }
    }

    pub fn remove(&self, id: u32) {
        self.sessions
            .write()
            .expect("session store write lock poisoned")
            .remove(&id);
    }
}

impl Default for SessionStore {
    fn default() -> Self {
        Self::new()
    }
}
