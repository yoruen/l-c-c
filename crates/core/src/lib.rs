//! Linux Control Center - Core Engine
//! 
//! The core engine manages plugin lifecycle, event routing,
//! privilege escalation, and system integration.

pub mod plugin_manager;
pub mod privilege_manager;
pub mod event_bus;
pub mod database;
pub mod distribution;
pub mod command_executor;
pub mod settings;

use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error, warn};

pub use plugin_manager::PluginManager;
pub use privilege_manager::PrivilegeManager;
pub use event_bus::EventBus;
pub use database::Database;
pub use distribution::DistributionDetector;
pub use settings::Settings;

/// Core application state
pub struct Core {
    plugin_manager: Arc<RwLock<PluginManager>>,
    privilege_manager: Arc<PrivilegeManager>,
    event_bus: Arc<EventBus>,
    database: Arc<Database>,
    settings: Arc<RwLock<Settings>>,
    distribution: distribution::DistributionInfo,
}

impl Core {
    /// Initialize the core application
    pub async fn new(config: CoreConfig) -> anyhow::Result<Self> {
        info!("Initializing Linux Control Center Core");
        
        // Detect distribution
        let distribution = DistributionDetector::detect().await?;
        info!("Detected distribution: {:?}", distribution);
        
        // Initialize database
        let database = Arc::new(Database::new(&config.database_path).await?);
        
        // Initialize settings
        let settings = Arc::new(RwLock::new(
            Settings::load(&config.config_path).await?
        ));
        
        // Initialize event bus
        let event_bus = Arc::new(EventBus::new());
        
        // Initialize privilege manager
        let privilege_manager = Arc::new(
            PrivilegeManager::new().await?
        );
        
        // Initialize plugin manager
        let plugin_manager = Arc::new(RwLock::new(
            PluginManager::new(
                config.plugin_directory.clone(),
                event_bus.clone(),
                privilege_manager.clone(),
            ).await?
        ));
        
        let core = Self {
            plugin_manager,
            privilege_manager,
            event_bus,
            database,
            settings,
            distribution,
        };
        
        // Load plugins
        core.load_plugins().await?;
        
        Ok(core)
    }
    
    /// Load all available plugins
    async fn load_plugins(&self) -> anyhow::Result<()> {
        let mut manager = self.plugin_manager.write().await;
        manager.discover_and_load().await?;
        Ok(())
    }
    
    /// Get plugin manager reference
    pub fn plugin_manager(&self) -> Arc<RwLock<PluginManager>> {
        self.plugin_manager.clone()
    }
    
    /// Get event bus reference
    pub fn event_bus(&self) -> Arc<EventBus> {
        self.event_bus.clone()
    }
    
    /// Get privilege manager reference
    pub fn privilege_manager(&self) -> Arc<PrivilegeManager> {
        self.privilege_manager.clone()
    }
    
    /// Get database reference
    pub fn database(&self) -> Arc<Database> {
        self.database.clone()
    }
    
    /// Get current distribution info
    pub fn distribution(&self) -> &distribution::DistributionInfo {
        &self.distribution
    }
    
    /// Graceful shutdown
    pub async fn shutdown(&self) -> anyhow::Result<()> {
        info!("Shutting down Linux Control Center Core");
        
        let mut manager = self.plugin_manager.write().await;
        manager.shutdown_all().await?;
        
        // Save settings
        let settings = self.settings.read().await;
        settings.save().await?;
        
        Ok(())
    }
}

/// Core configuration
#[derive(Debug, Clone)]
pub struct CoreConfig {
    pub plugin_directory: PathBuf,
    pub database_path: PathBuf,
    pub config_path: PathBuf,
    pub cache_directory: PathBuf,
    pub log_level: String,
}

impl Default for CoreConfig {
    fn default() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("linux-control-center");
            
        Self {
            plugin_directory: config_dir.join("plugins"),
            database_path: config_dir.join("lcc.db"),
            config_path: config_dir.join("settings.toml"),
            cache_directory: dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from("~/.cache"))
                .join("linux-control-center"),
            log_level: "info".to_string(),
        }
    }
}
