use serde::{Deserialize, Serialize};
use semver::Version;

#[derive(Debug, Deserialize)]
pub struct GitHubAsset {
    pub name: String,
    pub browser_download_url: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub name: String,
    #[allow(dead_code)]
    pub body: Option<String>,
    pub assets: Option<Vec<GitHubAsset>>,
}

#[derive(Debug, Serialize)]
pub struct UpdateInfo {
    pub current_version: String,
    pub latest_version: String,
    pub has_update: bool,
    pub release_notes: Option<String>,
    pub download_url: Option<String>,
}

#[tauri::command]
pub async fn check_for_updates() -> Result<UpdateInfo, String> {
    let current_version = env!("CARGO_PKG_VERSION");
    
    let client = reqwest::Client::builder()
        .user_agent("SSHGuard")
        .build()
        .map_err(|e| e.to_string())?;
    
    let response = client
        .get("https://api.github.com/repos/yangzhen72/sshguard/releases/latest")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        return Err(format!("API returned status: {}", response.status()));
    }
    
    let release: GitHubRelease = response.json().await.map_err(|e| e.to_string())?;
    
    let latest_version = release.tag_name.trim_start_matches('v');
    let has_update = Version::parse(latest_version)
        .map(|v| v > Version::parse(current_version).unwrap_or_else(|_| Version::parse("0.0.0").unwrap()))
        .unwrap_or(false);
    
    let download_url = release.assets.and_then(|assets| {
        assets.into_iter().find(|a| a.name.contains("setup.exe")).map(|a| a.browser_download_url)
    });
    
    Ok(UpdateInfo {
        current_version: current_version.to_string(),
        latest_version: latest_version.to_string(),
        has_update,
        release_notes: release.body,
        download_url,
    })
}
