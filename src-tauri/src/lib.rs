mod commands;
mod storage;
mod ssh;
mod sftp;

use storage::database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    log::info!("Starting SSHGuard application");

    if let Err(e) = database::init_database() {
        log::error!("Failed to initialize database: {}", e);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::servers::get_servers,
            commands::servers::add_server,
            commands::servers::update_server,
            commands::servers::delete_server,
            commands::ssh::connect,
            commands::ssh::create_pty,
            commands::ssh::disconnect,
            commands::ssh::send_pty_data,
            commands::ssh::read_pty_data,
            commands::sftp::list_directory,
            commands::sftp::download_file,
            commands::sftp::upload_file,
            commands::update::check_for_updates,
            commands::update::download_and_install,
        ])
        .setup(|_app| {
            log::info!("Application setup complete");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
