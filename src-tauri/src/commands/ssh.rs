use crate::ssh::{self, AuthMethod};
use serde::{Deserialize, Serialize};
use tauri::command;

#[derive(Debug, Deserialize)]
#[serde(tag = "authType")]
pub enum AuthInfo {
    #[serde(rename = "password")]
    Password { password: String },
    #[serde(rename = "keyfile")]
    KeyFile {
        keyFilePath: String,
        passphrase: Option<String>,
    },
    #[serde(rename = "agent")]
    Agent {},
}

impl From<AuthInfo> for AuthMethod {
    fn from(info: AuthInfo) -> Self {
        match info {
            AuthInfo::Password { password } => AuthMethod::Password(password),
            AuthInfo::KeyFile {
                keyFilePath,
                passphrase,
            } => AuthMethod::KeyFile {
                path: keyFilePath,
                passphrase,
            },
            AuthInfo::Agent {} => AuthMethod::Agent,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ConnectResult {
    pub session_id: String,
}

#[command]
pub fn connect(
    host: String,
    port: u16,
    username: String,
    auth_info: AuthInfo,
) -> Result<ConnectResult, String> {
    let auth_method: AuthMethod = auth_info.into();
    ssh::connect(&host, port, &username, auth_method)
        .map(|session_id| ConnectResult { session_id })
        .map_err(|e| e.to_string())
}

#[command]
pub fn create_pty(
    session_id: String,
    term: String,
    cols: u16,
    rows: u16,
) -> Result<String, String> {
    ssh::create_pty(&session_id, &term, cols, rows).map_err(|e| e.to_string())
}

#[command]
pub fn disconnect(session_id: String) -> Result<(), String> {
    ssh::disconnect(&session_id).map_err(|e| e.to_string())
}

#[command]
pub fn send_pty_data(session_id: String, data: Vec<u8>) -> Result<(), String> {
    ssh::send_pty_data(&session_id, &data).map_err(|e| e.to_string())
}

#[command]
pub fn read_pty_data(session_id: String, timeout_ms: u32) -> Result<Vec<u8>, String> {
    ssh::read_pty_data(&session_id, timeout_ms).map_err(|e| e.to_string())
}

#[command]
pub fn resize_pty(session_id: String, cols: u16, rows: u16) -> Result<(), String> {
    ssh::resize_pty(&session_id, cols, rows).map_err(|e| e.to_string())
}
