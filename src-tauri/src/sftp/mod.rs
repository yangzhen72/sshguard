use ssh2::{Sftp, File};
use std::collections::HashMap;
use parking_lot::RwLock;
use std::path::Path;
use std::io::{Read, Write};

lazy_static::lazy_static! {
    pub static ref SFTP_MANAGER: RwLock<SftpManager> = RwLock::new(SftpManager::new());
}

pub struct SftpManager {
    sftp_sessions: HashMap<String, Sftp>,
}

impl SftpManager {
    pub fn new() -> Self {
        Self {
            sftp_sessions: HashMap::new(),
        }
    }
    
    pub fn create_sftp(&mut self, session_id: &str, sftp: Sftp) {
        self.sftp_sessions.insert(session_id.to_string(), sftp);
    }
    
    pub fn get_sftp(&self, session_id: &str) -> Option<&Sftp> {
        self.sftp_sessions.get(session_id)
    }
    
    pub fn remove_sftp(&mut self, session_id: &str) {
        self.sftp_sessions.remove(session_id);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SftpError {
    #[error("SFTP session not found: {0}")]
    SessionNotFound(String),
    #[error("Path error: {0}")]
    PathError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("SSH error: {0}")]
    SshError(#[from] ssh2::Error),
}

pub type Result<T> = std::result::Result<T, SftpError>;

pub fn list_directory(sftp: &Sftp, path: &str) -> Result<Vec<String>> {
    let mut entries = Vec::new();
    let dir = sftp.opendir(Path::new(path))
        .map_err(|e| SftpError::SshError(e))?;
    
    loop {
        match dir.readdir() {
            Ok((filename, _stat)) => {
                entries.push(filename.to_string_lossy().to_string());
            }
            Err(e) => {
                if e.code() == ssh2::ErrorCode::SFTP(1) {
                    break;
                }
                return Err(SftpError::SshError(e));
            }
        }
    }
    
    Ok(entries)
}

pub fn download_file(sftp: &Sftp, remote_path: &str, local_path: &str) -> Result<()> {
    let mut remote_file: File = sftp.open(Path::new(remote_path))
        .map_err(|e| SftpError::SshError(e))?;
    
    let mut contents = Vec::new();
    remote_file.read_to_end(&mut contents).map_err(|e| SftpError::IoError(e))?;
    
    std::fs::write(local_path, contents).map_err(|e| SftpError::IoError(e))?;
    
    Ok(())
}

pub fn upload_file(sftp: &Sftp, local_path: &str, remote_path: &str) -> Result<()> {
    let contents = std::fs::read(local_path).map_err(|e| SftpError::IoError(e))?;
    
    let mut remote_file: File = sftp.create(Path::new(remote_path))
        .map_err(|e| SftpError::SshError(e))?;
    
    remote_file.write_all(&contents).map_err(|e| SftpError::IoError(e))?;
    
    Ok(())
}