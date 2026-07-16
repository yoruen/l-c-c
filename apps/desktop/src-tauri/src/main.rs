//! Linux Control Center - Desktop Application
//! 
//! Tauri 2 backend with Rust core integration

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lcc_core::{Core, CoreConfig};
use std::sync::Arc;
use tauri::{Manager, State};
use tokio::sync::RwLock;

/// Application state shared with Tauri commands
struct AppState {
    core: Arc<RwLock<Core>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    // Initialize core
    let config = CoreConfig::default();
    let core = Core::new(config).await?;
    
    let state = AppState {
        core: Arc::new(RwLock::new(core)),
    };
    
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            list_plugins,
            search,
            get_widgets,
            execute_command,
            get_dashboard_data,
        ])
        .setup(|app| {
            // Additional setup if needed
            Ok(())
        })
        .run(tauri::generate_context!())?;
        
    Ok(())
}

// Tauri Commands

#[tauri::command]
async fn get_system_info(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let core = state.core.read().await;
    let dist = core.distribution();
    
    Ok(serde_json::json!({
        "distribution": format!("{:?}", dist.distribution),
        "version": dist.version,
        "codename": dist.codename,
    }))
}

#[tauri::command]
async fn list_plugins(state: State<'_, AppState>) -> Result<Vec<lcc_plugin_api::PluginMetadata>, String> {
    let core = state.core.read().await;
    let manager = core.plugin_manager().read().await;
    
    Ok(manager.list_plugins())
}

#[tauri::command]
async fn search(
    state: State<'_, AppState>,
    query: String,
    limit: usize,
) -> Result<Vec<(String, lcc_plugin_api::SearchResult)>, String> {
    let core = state.core.read().await;
    let manager = core.plugin_manager().read().await;
    
    manager.search_all(&query, limit).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_widgets(state: State<'_, AppState>) -> Result<Vec<(String, lcc_plugin_api::WidgetDefinition)>, String> {
    let core = state.core.read().await;
    let manager = core.plugin_manager().read().await;
    
    manager.get_all_widgets().await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn execute_command(
    state: State<'_, AppState>,
    plugin_id: String,
    command: String,
    args: std::collections::HashMap<String, serde_json::Value>,
) -> Result<serde_json::Value, String> {
    let core = state.core.read().await;
    let manager = core.plugin_manager().read().await;
    
    manager.execute_command(&plugin_id, &command, args).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_dashboard_data(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    // Aggregate data from all plugins for dashboard
    let core = state.core.read().await;
    
    // This would aggregate real data from plugins
    Ok(serde_json::json!({
        "cpu_percent": 45.2,
        "memory_percent": 62.5,
        "disk_usage": {
            "/": 78.3,
            "/home": 45.1,
        },
        "network": {
            "download_mbps": 12.5,
            "upload_mbps": 3.2,
        },
        "updates_available": 15,
        "active_services": 142,
        "security_alerts": 0,
    }))
}
