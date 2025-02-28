use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use lazy_static::lazy_static;
use crate::utils::generate_session_id;

#[derive(Clone)]
pub struct Session {
    pub id: String,
    pub created_at: Instant,
}

pub struct SessionStore {
    sessions: Arc<Mutex<HashMap<String, Session>>>,
    timeout: Duration,
}

impl SessionStore {
    pub fn new(timeout_minutes: u64) -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            timeout: Duration::from_secs(timeout_minutes * 60),
        }
    }

    pub fn create_session(&self) -> String {
        let session_id = generate_session_id();
        let session = Session {
            id: session_id.clone(),
            created_at: Instant::now(),
        };

        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(session_id.clone(), session);
        
        session_id
    }

    pub fn validate_session(&self, session_id: &str) -> bool {
        let sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get(session_id) {
            session.created_at.elapsed() < self.timeout
        } else {
            false
        }
    }

    pub fn cleanup_expired_sessions(&self) {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.retain(|_, session| session.created_at.elapsed() < self.timeout);
    }
}

// Singleton pour accéder au store globalement
lazy_static! {
    pub static ref SESSION_STORE: SessionStore = SessionStore::new(1); // 30 minutes timeout
}