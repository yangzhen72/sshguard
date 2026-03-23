use tauri::command;
use crate::ssh;
use crate::sftp;

#[command]
pub fn list_directory(session_id: String, path: String) -> Result<Vec<String>, String> {
    let manager = ssh::SESSION_MANAGER.read();
    let session = manager.sessions.get(&session_id)
        .ok_or_else(|| format!("Session not found: {}", session_id))?;
    
    let sftp = session.sftp().map_err(|e| e.to_string())?;
    
    sftp::list_directory(&sftp, &path)
        .map_err(|e| e.to_string())
}

#[command]
pub fn download_file(session_id: String, remote_path: String, local_path: String) -> Result<(), String> {
    let manager = ssh::SESSION_MANAGER.read();
    let session = manager.sessions.get(&session_id)
        .ok_or_else(|| format!("Session not found: {}", session_id))?;
    
    let sftp = session.sftp().map_err(|e| e.to_string())?;
    
    sftp::download_file(&sftp, &remote_path, &local_path)
        .map_err(|e| e.to_string())
}

#[command]
pub fn upload_file(session_id: String, local_path: String, remote_path: String) -> Result<(), String> {
    let manager = ssh::SESSION_MANAGER.read();
    let session = manager.sessions.get(&session_id)
        .ok_or_else(|| format!("Session not found: {}", session_id))?;
    
    let sftp = session.sftp().map_err(|e| e.to_string())?;
    
    sftp::upload_file(&sftp, &local_path, &remote_path)
        .map_err(|e| e.to_string())
}