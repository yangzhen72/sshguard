use ssh2::Session;
use std::collections::HashMap;
use parking_lot::RwLock;
use std::net::TcpStream;
use std::sync::Arc;

lazy_static::lazy_static! {
    pub static ref SESSION_MANAGER: RwLock<SessionManager> = RwLock::new(SessionManager::new());
}

pub struct SessionManager {
    sessions: HashMap<String, Arc<Session>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }
    
    pub fn get_session(&self, session_id: &str) -> Option<Arc<Session>> {
        self.sessions.get(session_id).cloned()
    }
    
    pub fn insert_session(&mut self, session_id: String, session: Arc<Session>) {
        self.sessions.insert(session_id, session);
    }
    
    pub fn remove_session(&mut self, session_id: &str) -> Option<Arc<Session>> {
        self.sessions.remove(session_id)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SshError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Authentication failed: {0}")]
    AuthFailed(String),
    #[error("Session not found: {0}")]
    SessionNotFound(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("SSH error: {0}")]
    SshError(#[from] ssh2::Error),
}

pub type Result<T> = std::result::Result<T, SshError>;

pub fn connect(host: &str, port: u16, username: &str, password: &str) -> Result<String> {
    let tcp = TcpStream::connect(format!("{}:{}", host, port))
        .map_err(|e| SshError::ConnectionFailed(e.to_string()))?;
    
    let session = Session::new().map_err(|e| SshError::SshError(e))?;
    session.set_tcp_stream(tcp);
    session.handshake().map_err(|e| SshError::SshError(e))?;
    session.userauth_password(username, password)
        .map_err(|e| SshError::AuthFailed(e.to_string()))?;
    
    let session_id = uuid::Uuid::new_v4().to_string();
    SESSION_MANAGER.write().insert_session(session_id.clone(), Arc::new(session));
    
    Ok(session_id)
}

pub fn create_pty(session_id: &str) -> Result<String> {
    let manager = SESSION_MANAGER.read();
    let _session = manager.get_session(session_id)
        .ok_or_else(|| SshError::SessionNotFound(session_id.to_string()))?;
    Ok(session_id.to_string())
}

pub fn disconnect(session_id: &str) -> Result<()> {
    SESSION_MANAGER.write().remove_session(session_id)
        .ok_or_else(|| SshError::SessionNotFound(session_id.to_string()))?;
    Ok(())
}

pub fn send_pty_data(session_id: &str, data: &[u8]) -> Result<()> {
    let manager = SESSION_MANAGER.read();
    let session = manager.get_session(session_id)
        .ok_or_else(|| SshError::SessionNotFound(session_id.to_string()))?;
    
    let mut channel = session.channel_session()
        .map_err(|e| SshError::SshError(e))?;
    channel.write(data).map_err(|e| SshError::IoError(e))?;
    channel.send_eof().map_err(|e| SshError::SshError(e))?;
    
    Ok(())
}

pub fn read_pty_data(session_id: &str, _timeout_ms: u32) -> Result<Vec<u8>> {
    let manager = SESSION_MANAGER.read();
    let session = manager.get_session(session_id)
        .ok_or_else(|| SshError::SessionNotFound(session_id.to_string()))?;
    
    let mut channel = session.channel_session()
        .map_err(|e| SshError::SshError(e))?;
    channel.exec("").map_err(|e| SshError::SshError(e))?;
    
    let mut buf = vec![0u8; 8192];
    let n = channel.read(&mut buf).map_err(|e| SshError::IoError(e))?;
    buf.truncate(n);
    
    Ok(buf)
}