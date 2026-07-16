//! Systemd Plugin for Linux Control Center
//! 
//! Manages systemd services through D-Bus

use async_trait::async_trait;
use lcc_plugin_api::{
    export_plugin, Capability, FieldType, Permission, Plugin, PluginConfig,
    PluginError, PluginEvent, PluginHealth, PluginMetadata, PluginMetadataProvider,
    SearchAction, SearchCategory, SearchResult, SettingsSchema, WidgetDefinition,
    WidgetSize, WidgetType,
};
use std::collections::HashMap;
use tokio::sync::mpsc;

pub struct SystemdPlugin {
    metadata: PluginMetadata,
    health: PluginHealth,
    config: Option<PluginConfig>,
}

impl Default for SystemdPlugin {
    fn default() -> Self {
        Self {
            metadata: PluginMetadata {
                id: "systemd".to_string(),
                name: "Systemd Manager".to_string(),
                version: "0.1.0".to_string(),
                description: "Manage systemd services and units".to_string(),
                author: "LCC Team".to_string(),
                api_version: 1,
                supported_distributions: vec![
                    lcc_plugin_api::Distribution::Ubuntu,
                    lcc_plugin_api::Distribution::Debian,
                    lcc_plugin_api::Distribution::Fedora,
                    lcc_plugin_api::Distribution::Arch,
                ],
                capabilities: vec![
                    Capability::ServiceManagement,
                    Capability::SearchProvider,
                    Capability::WidgetProvider,
                ],
                permissions: vec![
                    Permission::ReadServices,
                    Permission::ModifyServices,
                ],
                dependencies: vec![],
            },
            health: PluginHealth::Initializing,
            config: None,
        }
    }
}

#[async_trait]
impl Plugin for SystemdPlugin {
    async fn initialize(&mut self, config: PluginConfig) -> Result<(), PluginError> {
        // Verify systemd is available
        if !self.check_systemd_available().await {
            return Err(PluginError::InitializationFailed(
                "systemd is not available on this system".to_string()
            ));
        }
        
        self.config = Some(config);
        self.health = PluginHealth::Healthy;
        
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<(), PluginError> {
        self.health = PluginHealth::ShuttingDown;
        Ok(())
    }
    
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }
    
    fn health(&self) -> PluginHealth {
        self.health.clone()
    }
    
    fn has_capability(&self, capability: Capability) -> bool {
        self.metadata.capabilities.contains(&capability)
    }
    
    fn has_permission(&self, permission: Permission) -> bool {
        self.metadata.permissions.contains(&permission)
    }
    
    async fn subscribe_events(&self) -> Result<mpsc::Receiver<PluginEvent>, PluginError> {
        let (tx, rx) = mpsc::channel(100);
        
        // Start monitoring systemd events
        tokio::spawn(async move {
            // Monitor service changes via D-Bus
            // Implementation would connect to systemd D-Bus interface
        });
        
        Ok(rx)
    }
    
    async fn execute_command(
        &self,
        command: &str,
        args: HashMap<String, serde_json::Value>,
    ) -> Result<serde_json::Value, PluginError> {
        match command {
            "list_services" => {
                self.list_services().await
            }
            "start_service" => {
                let service = args.get("service")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| PluginError::InvalidConfiguration(
                        "service name required".to_string()
                    ))?;
                self.start_service(service).await
            }
            "stop_service" => {
                let service = args.get("service")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| PluginError::InvalidConfiguration(
                        "service name required".to_string()
                    ))?;
                self.stop_service(service).await
            }
            "get_service_status" => {
                let service = args.get("service")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| PluginError::InvalidConfiguration(
                        "service name required".to_string()
                    ))?;
                self.get_service_status(service).await
            }
            _ => Err(PluginError::CapabilityNotSupported(command.to_string())),
        }
    }
    
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>, PluginError> {
        // Search systemd units
        let services = self.list_services().await?;
        let services: Vec<serde_json::Value> = serde_json::from_value(services)
            .map_err(|e| PluginError::ExecutionFailed(e.to_string()))?;
        
        let mut results = Vec::new();
        
        for service in services.iter().take(limit) {
            let name = service.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let description = service.get("description").and_then(|v| v.as_str()).unwrap_or("");
            
            if name.to_lowercase().contains(&query.to_lowercase())
                || description.to_lowercase().contains(&query.to_lowercase())
            {
                results.push(SearchResult {
                    id: format!("systemd:{}", name),
                    title: name.to_string(),
                    description: description.to_string(),
                    category: SearchCategory::Other,
                    icon: Some("settings".to_string()),
                    action: SearchAction::Open {
                        route: format!("/services?service={}", name),
                    },
                    score: 1.0,
                });
            }
        }
        
        Ok(results)
    }
    
    async fn get_widgets(&self) -> Result<Vec<WidgetDefinition>, PluginError> {
        Ok(vec![
            WidgetDefinition {
                id: "failed_services".to_string(),
                name: "Failed Services".to_string(),
                description: "Shows count of failed systemd units".to_string(),
                widget_type: WidgetType::Status,
                default_size: WidgetSize { width: 1, height: 1 },
                refresh_interval_ms: Some(5000),
                requires_elevation: false,
            },
            WidgetDefinition {
                id: "service_list".to_string(),
                name: "Active Services".to_string(),
                description: "List of currently active services".to_string(),
                widget_type: WidgetType::List,
                default_size: WidgetSize { width: 2, height: 2 },
                refresh_interval_ms: Some(10000),
                requires_elevation: false,
            },
        ])
    }
    
    async fn get_settings_schema(&self) -> Result<SettingsSchema, PluginError> {
        Ok(SettingsSchema {
            fields: vec![],
        })
    }
    
    async fn update_settings(&mut self, _settings: serde_json::Value) -> Result<(), PluginError> {
        Ok(())
    }
}

impl SystemdPlugin {
    async fn check_systemd_available(&self) -> bool {
        // Check if systemctl exists and systemd is running
        tokio::process::Command::new("systemctl")
            .arg("--version")
            .output()
            .await
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    
    async fn list_services(&self) -> Result<serde_json::Value, PluginError> {
        // Use systemctl to list services
        let output = tokio::process::Command::new("systemctl")
            .args(&["list-units", "--type=service", "--all", "--no-pager", "--output=json"])
            .output()
            .await
            .map_err(|e| PluginError::ExecutionFailed(e.to_string()))?;
            
        if !output.status.success() {
            // Fallback to parsing text output
            return self.parse_service_list().await;
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let services: Vec<serde_json::Value> = serde_json::from_str(&stdout)
            .map_err(|e| PluginError::ExecutionFailed(e.to_string()))?;
            
        Ok(serde_json::json!(services))
    }
    
    async fn parse_service_list(&self) -> Result<serde_json::Value, PluginError> {
        let output = tokio::process::Command::new("systemctl")
            .args(&["list-units", "--type=service", "--no-pager", "--no-legend"])
            .output()
            .await
            .map_err(|e| PluginError::ExecutionFailed(e.to_string()))?;
            
        let stdout = String::from_utf8_lossy(&output.stdout);
        let services: Vec<HashMap<String, String>> = stdout
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let mut map = HashMap::new();
                    map.insert("name".to_string(), parts[0].to_string());
                    map.insert("load".to_string(), parts[1].to_string());
                    map.insert("active".to_string(), parts[2].to_string());
                    map.insert("sub".to_string(), parts[3].to_string());
                    Some(map)
                } else {
                    None
                }
            })
            .collect();
            
        Ok(serde_json::json!(services))
    }
    
    async fn start_service(&self, service: &str) -> Result<serde_json::Value, PluginError> {
        let output = tokio::process::Command::new("systemctl")
            .args(&["start", service])
            .output()
            .await
            .map_err(|e| PluginError::ExecutionFailed(e.to_string()))?;
            
        if output.status.success() {
            Ok(serde_json::json!({ "success": true }))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(PluginError::ExecutionFailed(stderr.to_string()))
        }
    }
    
    async fn stop_service(&self, service: &str) -> Result<serde_json::Value, PluginError> {
        let output = tokio::process::Command::new("systemctl")
            .args(&["stop", service])
            .output()
            .await
            .map_err(|e| PluginError::ExecutionFailed(e.to_string()))?;
            
        if output.status.success() {
            Ok(serde_json::json!({ "success": true }))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(PluginError::ExecutionFailed(stderr.to_string()))
        }
    }
    
    async fn get_service_status(&self, service: &str) -> Result<serde_json::Value, PluginError> {
        let output = tokio::process::Command::new("systemctl")
            .args(&["show", service, "--property=Id,Description,LoadState,ActiveState,SubState,MainPID,MemoryCurrent,CPUUsageNSec"])
            .output()
            .await
            .map_err(|e| PluginError::ExecutionFailed(e.to_string()))?;
            
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut status = HashMap::new();
        
        for line in stdout.lines() {
            if let Some((key, value)) = line.split_once('=') {
                status.insert(key.to_string(), value.to_string());
            }
        }
        
        Ok(serde_json::json!(status))
    }
}

impl PluginMetadataProvider for SystemdPlugin {
    fn metadata() -> PluginMetadata {
        PluginMetadata {
            id: "systemd".to_string(),
            name: "Systemd Manager".to_string(),
            version: "0.1.0".to_string(),
            description: "Manage systemd services and units".to_string(),
            author: "LCC Team".to_string(),
            api_version: 1,
            supported_distributions: vec![
                lcc_plugin_api::Distribution::Ubuntu,
                lcc_plugin_api::Distribution::Debian,
                lcc_plugin_api::Distribution::Fedora,
                lcc_plugin_api::Distribution::Arch,
            ],
            capabilities: vec![
                Capability::ServiceManagement,
                Capability::SearchProvider,
                Capability::WidgetProvider,
            ],
            permissions: vec![
                Permission::ReadServices,
                Permission::ModifyServices,
            ],
            dependencies: vec![],
        }
    }
}

export_plugin!(SystemdPlugin);
