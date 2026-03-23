use ssh2::Session;
use std::collections::HashMap;
use parking_lot::RwLock;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;

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
    
    let mut session = Session::new().map_err(|e| SshError::SshError(e))?;
    session.set_tcp_stream(tcp);
    session.handshake().map_err(|e| SshError::SshError(e))?;
    session.userauth_password(username, password).map_err(|e| SshError::AuthFailed(e.to_string()))?;
    
    let session_id = uuid::Uuid::new_v4().to_string();
    SESSION_MANAGER.write().sessions.insert(session_id.clone(), session);
    
    Ok(session_id)
}

pub fn create_pty(session_id: &str) -> Result<(ssh2::Session, ssh2::Pty)> {
    let manager = SESSION_MANAGER.read();
    let session = manager.sessions.get(session_id)
        .ok_or_else(|| SshError::SessionNotFound(session_id.to_string()))?;
    
    let pty = session.pty_req("xterm", 80, 24, 0, 0)
        .map_err(|e| SshError::SshError(e))?;
    
    Ok((session.clone(), pty))
}

pub fn disconnect(session_id: &str) -> Result<()> {
    let mut manager = SESSION_MANAGER.write();
    manager.sessions.remove(session_id)
        .ok_or_else(|| SshError::SessionNotFound(session_id.to_string()))?;
    Ok(())
}

pub fn send_pty_data(session_id: &str, data: &[u8]) -> Result<()> {
    let manager = SESSION_MANAGER.read();
    let session = manager.sessions.get(session_id)
        .ok_or_else(|| SshError::SessionNotFound(session_id.to_string()))?;
    
    session.channel_session()
        .map_err(|e| SshError::SshError(e))?
        .write(data)
        .map_err(|e| SshError::IoError(e))?;
    
    Ok(())
}

pub fn read_pty_data(session_id: &str, timeout_ms: u32) -> Result<Vec<u8>> {
    let manager = SESSION_MANAGER.read();
    let session = manager.sessions.get(session_id)
        .ok_or_else(|| SshError::SessionNotFound(session_id.to_string()))?;
    
    let mut buf = vec![0u8; 8192];
    let channel = session.channel_session()
        .map_err(|e| SshError::SshError(e))?;
    
    channel.read_timeout(timeout_ms as i32)
        .map_err(|e| SshError::IoError(e))
}

impl Clone for Session {
    fn clone(&self) -> Self {
        unsafe { std::mem::transmute(self) }
    }
}