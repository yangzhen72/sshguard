use parking_lot::RwLock;
use ssh2::Session;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::sync::Arc;
use thiserror::Error;

lazy_static::lazy_static! {
    pub static ref SESSION_MANAGER: RwLock<SessionManager> = RwLock::new(SessionManager::new());
}

pub struct SessionManager {
    sessions: HashMap<String, Arc<Session>>,
    pty_channels: HashMap<String, ssh2::Channel>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            pty_channels: HashMap::new(),
        }
    }

    pub fn get_session(&self, session_id: &str) -> Option<Arc<Session>> {
        self.sessions.get(session_id).cloned()
    }

    pub fn get_pty_channel(&self, session_id: &str) -> Option<&ssh2::Channel> {
        self.pty_channels.get(session_id)
    }

    pub fn get_pty_channel_mut(&mut self, session_id: &str) -> Option<&mut ssh2::Channel> {
        self.pty_channels.get_mut(session_id)
    }

    pub fn insert_session(&mut self, session_id: String, session: Arc<Session>) {
        self.sessions.insert(session_id, session);
    }

    pub fn insert_pty_channel(&mut self, session_id: String, channel: ssh2::Channel) {
        self.pty_channels.insert(session_id, channel);
    }

    pub fn remove_session(&mut self, session_id: &str) -> Option<Arc<Session>> {
        self.pty_channels.remove(session_id);
        self.sessions.remove(session_id)
    }
}

#[derive(Debug, Error)]
pub enum SshError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Authentication failed: {0}")]
    AuthFailed(String),
    #[error("Session not found: {0}")]
    SessionNotFound(String),
    #[error("PTY not initialized for session: {0}")]
    PtyNotInitialized(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("SSH error: {0}")]
    SshError(#[from] ssh2::Error),
}

pub type Result<T> = std::result::Result<T, SshError>;

#[derive(Debug, Clone)]
pub enum AuthMethod {
    Password(String),
    KeyFile {
        path: String,
        passphrase: Option<String>,
    },
    Agent,
}

pub fn connect(host: &str, port: u16, username: &str, auth_method: AuthMethod) -> Result<String> {
    let tcp = TcpStream::connect(format!("{}:{}", host, port))
        .map_err(|e| SshError::ConnectionFailed(e.to_string()))?;

    let mut session = Session::new().map_err(|e| SshError::SshError(e))?;
    session.set_tcp_stream(tcp);
    session.handshake().map_err(|e| SshError::SshError(e))?;

    match auth_method {
        AuthMethod::Password(password) => {
            session
                .userauth_password(username, &password)
                .map_err(|e| SshError::AuthFailed(e.to_string()))?;
        }
        AuthMethod::KeyFile { path, passphrase } => {
            session
                .userauth_pubkey_file(username, None, Path::new(&path), passphrase.as_deref())
                .map_err(|e| SshError::AuthFailed(e.to_string()))?;
        }
        AuthMethod::Agent => {
            session
                .userauth_agent(username)
                .map_err(|e| SshError::AuthFailed(e.to_string()))?;
        }
    }

    if !session.authenticated() {
        return Err(SshError::AuthFailed("Authentication failed".to_string()));
    }

    let session_id = uuid::Uuid::new_v4().to_string();
    SESSION_MANAGER
        .write()
        .insert_session(session_id.clone(), Arc::new(session));

    Ok(session_id)
}

pub fn create_pty(session_id: &str, term: &str, cols: u16, rows: u16) -> Result<String> {
    let mut manager = SESSION_MANAGER.write();
    let session = manager
        .get_session(session_id)
        .ok_or_else(|| SshError::SessionNotFound(session_id.to_string()))?;

    let mut channel = session
        .channel_session()
        .map_err(|e| SshError::SshError(e))?;

    channel
        .request_pty(term, None, Some((cols as u32, rows as u32, 0, 0)))
        .map_err(|e| SshError::SshError(e))?;

    channel.exec("bash").map_err(|e| SshError::SshError(e))?;

    manager.insert_pty_channel(session_id.to_string(), channel);

    Ok(session_id.to_string())
}

pub fn disconnect(session_id: &str) -> Result<()> {
    SESSION_MANAGER
        .write()
        .remove_session(session_id)
        .ok_or_else(|| SshError::SessionNotFound(session_id.to_string()))?;
    Ok(())
}

pub fn send_pty_data(session_id: &str, data: &[u8]) -> Result<()> {
    let mut manager = SESSION_MANAGER.write();
    let channel = manager
        .get_pty_channel_mut(session_id)
        .ok_or_else(|| SshError::PtyNotInitialized(session_id.to_string()))?;

    channel.write(data).map_err(|e| SshError::IoError(e))?;
    channel.flush().map_err(|e| SshError::IoError(e))?;

    Ok(())
}

pub fn read_pty_data(session_id: &str, _timeout_ms: u32) -> Result<Vec<u8>> {
    let mut manager = SESSION_MANAGER.write();
    let channel = manager
        .get_pty_channel_mut(session_id)
        .ok_or_else(|| SshError::PtyNotInitialized(session_id.to_string()))?;

    let mut buf = vec![0u8; 8192];

    match channel.read(&mut buf) {
        Ok(n) => {
            buf.truncate(n);
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            buf.clear();
        }
        Err(e) => {
            return Err(SshError::IoError(e));
        }
    }

    buf.retain(|&b| b != 0);
    Ok(buf)
}

pub fn resize_pty(session_id: &str, cols: u16, rows: u16) -> Result<()> {
    let mut manager = SESSION_MANAGER.write();
    let channel = manager
        .get_pty_channel_mut(session_id)
        .ok_or_else(|| SshError::PtyNotInitialized(session_id.to_string()))?;

    channel
        .request_pty_size(cols as u32, rows as u32, None, None)
        .map_err(|e| SshError::SshError(e))?;

    Ok(())
}
