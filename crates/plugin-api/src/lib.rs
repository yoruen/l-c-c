//! Linux Control Center - Plugin API
//! 
//! This crate defines the contract between the core application and plugins.
//! All plugins must implement these traits to be loaded by the system.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

/// Plugin API version for compatibility checking
pub const PLUGIN_API_VERSION: u32 = 1;

/// Errors that can occur in plugin operations
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum PluginError {
    #[error("Plugin initialization failed: {0}")]
    InitializationFailed(String),
    
    #[error("Plugin capability not supported: {0}")]
    CapabilityNotSupported(String),
    
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Plugin {0} is not compatible with API version {1}")]
    IncompatibleVersion(String, u32),
    
    #[error("Plugin not found: {0}")]
    NotFound(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
}

/// Result type for plugin operations
pub type PluginResult<T> = Result<T, PluginError>;

/// Distribution types supported by plugins
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Distribution {
    Ubuntu,
    Debian,
    LinuxMint,
    PopOS,
    Fedora,
    Arch,
    EndeavourOS,
    Manjaro,
    OpenSUSE,
    NixOS,
    Gentoo,
    Alpine,
    Void,
    Generic,
}

/// Plugin metadata exposed to the core
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub api_version: u32,
    pub supported_distributions: Vec<Distribution>,
    pub capabilities: Vec<Capability>,
    pub permissions: Vec<Permission>,
    pub dependencies: Vec<String>,
}

/// Capabilities a plugin can provide
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Capability {
    PackageManagement,
    ServiceManagement,
    HardwareInfo,
    StorageManagement,
    NetworkManagement,
    SecurityManagement,
    LogManagement,
    BackupManagement,
    ContainerManagement,
    VirtualMachineManagement,
    UpdateManagement,
    RepositoryManagement,
    ProcessManagement,
    SystemCleanup,
    SearchProvider,
    WidgetProvider,
    SettingsProvider,
    RouteProvider,
    EventPublisher,
}

/// Required permissions for plugin operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    ReadSystemInfo,
    ReadPackages,
    ModifyPackages,
    ReadServices,
    ModifyServices,
    ReadProcesses,
    ModifyProcesses,
    ReadStorage,
    ModifyStorage,
    ReadNetwork,
    ModifyNetwork,
    ReadSecurity,
    ModifySecurity,
    ReadLogs,
    ElevatedPrivileges,
}

/// Plugin health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginHealth {
    Healthy,
    Degraded(String),
    Failed(String),
    Initializing,
    ShuttingDown,
}

/// Event types for plugin communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginEvent {
    SystemInfoUpdated(SystemInfo),
    PackageInstalled { package_id: String, version: String },
    PackageRemoved { package_id: String },
    ServiceStatusChanged { service_id: String, active: bool },
    StorageChanged { mount_point: String, usage_percent: f32 },
    NetworkStatusChanged { interface: String, connected: bool },
    SecurityAlert { severity: AlertSeverity, message: String },
    LogEntry { source: String, level: LogLevel, message: String },
    Custom { event_type: String, payload: serde_json::Value },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

/// System information snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub timestamp: DateTime<Utc>,
    pub hostname: String,
    pub distribution: Distribution,
    pub distribution_version: String,
    pub kernel_version: String,
    pub uptime_seconds: u64,
    pub load_average: [f64; 3],
    pub cpu_percent: f32,
    pub memory_used_percent: f32,
    pub swap_used_percent: f32,
    pub disk_usage_percent: HashMap<String, f32>,
}

/// Core plugin trait - all plugins must implement this
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Initialize the plugin with configuration
    async fn initialize(&mut self, config: PluginConfig) -> PluginResult<()>;
    
    /// Gracefully shutdown the plugin
    async fn shutdown(&mut self) -> PluginResult<()>;
    
    /// Get plugin metadata
    fn metadata(&self) -> &PluginMetadata;
    
    /// Get current health status
    fn health(&self) -> PluginHealth;
    
    /// Check if plugin supports a specific capability
    fn has_capability(&self, capability: Capability) -> bool;
    
    /// Check if plugin has required permission
    fn has_permission(&self, permission: Permission) -> bool;
    
    /// Subscribe to events from this plugin
    async fn subscribe_events(&self) -> PluginResult<tokio::sync::mpsc::Receiver<PluginEvent>>;
    
    /// Execute a command provided by this plugin
    async fn execute_command(
        &self,
        command: &str,
        args: HashMap<String, serde_json::Value>,
    ) -> PluginResult<serde_json::Value>;
    
    /// Get search results from this plugin
    async fn search(&self, query: &str, limit: usize) -> PluginResult<Vec<SearchResult>>;
    
    /// Get widgets provided by this plugin
    async fn get_widgets(&self) -> PluginResult<Vec<WidgetDefinition>>;
    
    /// Get settings schema
    async fn get_settings_schema(&self) -> PluginResult<SettingsSchema>;
    
    /// Update settings
    async fn update_settings(&mut self, settings: serde_json::Value) -> PluginResult<()>;
}

/// Plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub plugin_id: String,
    pub data_directory: std::path::PathBuf,
    pub cache_directory: std::path::PathBuf,
    pub settings: HashMap<String, serde_json::Value>,
    pub elevated: bool,
}

/// Search result from a plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: SearchCategory,
    pub icon: Option<String>,
    pub action: SearchAction,
    pub score: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchCategory {
    Package,
    Service,
    Process,
    Setting,
    Log,
    File,
    Command,
    Documentation,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchAction {
    Open { route: String },
    Execute { command: String, args: Vec<String> },
    ShowDetails { entity_id: String },
}

/// Widget definition for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub widget_type: WidgetType,
    pub default_size: WidgetSize,
    pub refresh_interval_ms: Option<u64>,
    pub requires_elevation: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WidgetType {
    Chart,
    Gauge,
    List,
    Table,
    Status,
    Progress,
    Custom,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WidgetSize {
    pub width: u32,
    pub height: u32,
}

/// Settings schema for plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsSchema {
    pub fields: Vec<SettingField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingField {
    pub key: String,
    pub name: String,
    pub description: String,
    pub field_type: FieldType,
    pub default_value: Option<serde_json::Value>,
    pub required: bool,
    pub validation: Option<ValidationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    String { max_length: Option<usize> },
    Integer { min: Option<i64>, max: Option<i64> },
    Float { min: Option<f64>, max: Option<f64> },
    Boolean,
    Select { options: Vec<String> },
    MultiSelect { options: Vec<String> },
    Path { must_exist: bool },
    Secret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub pattern: Option<String>,
    pub custom_validator: Option<String>,
}

/// FFI-safe plugin factory function signature
pub type PluginCreateFn = unsafe fn() -> *mut dyn Plugin;

/// Trait for plugin factories
pub trait PluginFactory: Send + Sync {
    fn create(&self) -> Box<dyn Plugin>;
    fn metadata(&self) -> PluginMetadata;
}

/// Macro for exporting plugin factory
#[macro_export]
macro_rules! export_plugin {
    ($plugin_type:ty) => {
        #[no_mangle]
        pub extern "C" fn _lcc_create_plugin() -> *mut dyn $crate::Plugin {
            let plugin: $plugin_type = Default::default();
            Box::into_raw(Box::new(plugin))
        }
        
        #[no_mangle]
        pub extern "C" fn _lcc_plugin_metadata() -> *mut $crate::PluginMetadata {
            let metadata = <$plugin_type as $crate::PluginMetadataProvider>::metadata();
            Box::into_raw(Box::new(metadata))
        }
        
        #[no_mangle]
        pub extern "C" fn _lcc_api_version() -> u32 {
            $crate::PLUGIN_API_VERSION
        }
    };
}

/// Trait for plugins that provide metadata statically
pub trait PluginMetadataProvider {
    fn metadata() -> PluginMetadata;
}

// Re-export commonly used types
pub use async_trait::async_trait;
