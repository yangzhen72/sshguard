use serde::{Deserialize, Serialize};
use tauri::command;

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub id: String,
    pub name: String,
}

#[command]
pub fn get_servers() -> Result<Vec<Server>, String> {
    Ok(vec![])
}

#[command]
pub fn add_server(server: Server) -> Result<Server, String> {
    Ok(server)
}

#[command]
pub fn update_server(server: Server) -> Result<Server, String> {
    Ok(server)
}

#[command]
pub fn delete_server(_id: String) -> Result<(), String> {
    Ok(())
}
