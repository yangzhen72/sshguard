use tauri::command;
use crate::ssh;

#[command]
pub fn connect(host: String, port: u16, username: String, password: String) -> Result<String, String> {
    ssh::connect(&host, port, &username, &password)
        .map_err(|e| e.to_string())
}

#[command]
pub fn create_pty(session_id: String) -> Result<String, String> {
    ssh::create_pty(&session_id)
        .map_err(|e| e.to_string())
}

#[command]
pub fn disconnect(session_id: String) -> Result<(), String> {
    ssh::disconnect(&session_id)
        .map_err(|e| e.to_string())
}

#[command]
pub fn send_pty_data(session_id: String, data: Vec<u8>) -> Result<(), String> {
    ssh::send_pty_data(&session_id, &data)
        .map_err(|e| e.to_string())
}

#[command]
pub fn read_pty_data(session_id: String, timeout_ms: u32) -> Result<Vec<u8>, String> {
    ssh::read_pty_data(&session_id, timeout_ms)
        .map_err(|e| e.to_string())
}