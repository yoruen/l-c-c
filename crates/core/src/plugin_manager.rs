//! Plugin Manager - Discovery, loading, and lifecycle management

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use lcc_plugin_api::{
    Plugin, PluginConfig, PluginError, PluginEvent, PluginHealth, 
    PluginMetadata, Capability, Permission, PLUGIN_API_VERSION
};
use tokio::sync::{mpsc, RwLock};
use tracing::{info, warn, error, debug};
use dashmap::DashMap;

use crate::event_bus::EventBus;
use crate::privilege_manager::PrivilegeManager;

/// Manages plugin lifecycle and routing
pub struct PluginManager {
    plugin_directory: PathBuf,
    plugins: DashMap<String, Box<dyn Plugin>>,
    metadata: DashMap<String, PluginMetadata>,
    event_bus: Arc<EventBus>,
    privilege_manager: Arc<PrivilegeManager>,
    event_handlers: HashMap<String, mpsc::Sender<PluginEvent>>,
}

impl PluginManager {
    pub async fn new(
        plugin_directory: PathBuf,
        event_bus: Arc<EventBus>,
        privilege_manager: Arc<PrivilegeManager>,
    ) -> anyhow::Result<Self> {
        tokio::fs::create_dir_all(&plugin_directory).await?;
        
        Ok(Self {
            plugin_directory,
            plugins: DashMap::new(),
            metadata: DashMap::new(),
            event_bus,
            privilege_manager,
            event_handlers: HashMap::new(),
        })
    }
    
    /// Discover and load all plugins
    pub async fn discover_and_load(&mut self) -> anyhow::Result<()> {
        info!("Discovering plugins in {:?}", self.plugin_directory);
        
        let mut entries = tokio::fs::read_dir(&self.plugin_directory).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "so" || ext == "dll") {
                match self.load_plugin(&path).await {
                    Ok(metadata) => {
                        info!("Loaded plugin: {} v{}", metadata.name, metadata.version);
                    }
                    Err(e) => {
                        warn!("Failed to load plugin {:?}: {}", path, e);
                    }
                }
            }
        }
        
        info!("Loaded {} plugins", self.plugins.len());
        Ok(())
    }
    
    /// Load a single plugin from file
    async fn load_plugin(&mut self, path: &Path) -> anyhow::Result<PluginMetadata> {
        debug!("Loading plugin from {:?}", path);
        
        // For now, use a trait object approach
        // In production, this would use libloading for dynamic libraries
        
        // Create plugin configuration
        let plugin_id = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
            
        let config = PluginConfig {
            plugin_id: plugin_id.clone(),
            data_directory: self.plugin_directory.join(&plugin_id).join("data"),
            cache_directory: self.plugin_directory.join(&plugin_id).join("cache"),
            settings: HashMap::new(),
            elevated: false,
        };
        
        // Initialize plugin
        // Note: This is where dynamic loading would happen
        // For now, we'll return an error indicating plugins need to be registered
        
        Err(anyhow::anyhow!("Dynamic plugin loading requires compiled plugins"))
    }
    
    /// Register a plugin directly (for built-in plugins)
    pub async fn register_plugin(
        &mut self,
        mut plugin: Box<dyn Plugin>,
    ) -> Result<(), PluginError> {
        let metadata = plugin.metadata().clone();
        
        // Check API version compatibility
        if metadata.api_version != PLUGIN_API_VERSION {
            return Err(PluginError::IncompatibleVersion(
                metadata.id.clone(),
                metadata.api_version,
            ));
        }
        
        let plugin_id = metadata.id.clone();
        
        // Initialize plugin
        let config = PluginConfig {
            plugin_id: plugin_id.clone(),
            data_directory: self.plugin_directory.join(&plugin_id).join("data"),
            cache_directory: self.plugin_directory.join(&plugin_id).join("cache"),
            settings: HashMap::new(),
            elevated: false,
        };
        
        plugin.initialize(config).await?;
        
        // Store plugin
        self.plugins.insert(plugin_id.clone(), plugin);
        self.metadata.insert(plugin_id.clone(), metadata.clone());
        
        // Start event listener
        self.start_event_listener(&plugin_id).await?;
        
        info!("Registered plugin: {} ({})", metadata.name, plugin_id);
        Ok(())
    }
    
    /// Start event listener for a plugin
    async fn start_event_listener(&mut self, plugin_id: &str) -> anyhow::Result<()> {
        if let Some(plugin) = self.plugins.get(plugin_id) {
            let mut rx = plugin.subscribe_events().await?;
            let bus = self.event_bus.clone();
            let id = plugin_id.to_string();
            
            tokio::spawn(async move {
                while let Some(event) = rx.recv().await {
                    bus.publish(&id, event).await;
                }
            });
        }
        
        Ok(())
    }
    
    /// Get a plugin by ID
    pub fn get_plugin(&self, id: &str) -> Option<dashmap::mapref::one::Ref<String, Box<dyn Plugin>>> {
        self.plugins.get(id)
    }
    
    /// Get all loaded plugins
    pub fn list_plugins(&self) -> Vec<PluginMetadata> {
        self.metadata.iter().map(|m| m.clone()).collect()
    }
    
    /// Get plugins supporting a specific capability
    pub fn get_plugins_by_capability(&self, capability: Capability) -> Vec<String> {
        self.metadata
            .iter()
            .filter(|m| m.capabilities.contains(&capability))
            .map(|m| m.id.clone())
            .collect()
    }
    
    /// Execute a command on a specific plugin
    pub async fn execute_command(
        &self,
        plugin_id: &str,
        command: &str,
        args: HashMap<String, serde_json::Value>,
    ) -> Result<serde_json::Value, PluginError> {
        let plugin = self.plugins.get(plugin_id)
            .ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))?;
            
        plugin.execute_command(command, args).await
    }
    
    /// Search across all plugins
    pub async fn search_all(
        &self,
        query: &str,
        limit: usize,
    ) -> Vec<(String, lcc_plugin_api::SearchResult)> {
        let mut results = Vec::new();
        
        for entry in self.plugins.iter() {
            let plugin_id = entry.key().clone();
            let plugin = entry.value();
            
            match plugin.search(query, limit).await {
                Ok(plugin_results) => {
                    for result in plugin_results {
                        results.push((plugin_id.clone(), result));
                    }
                }
                Err(e) => {
                    debug!("Search failed in plugin {}: {}", plugin_id, e);
                }
            }
        }
        
        // Sort by relevance score
        results.sort_by(|a, b| b.1.score.partial_cmp(&a.1.score).unwrap());
        results.truncate(limit);
        
        results
    }
    
    /// Get all widgets from all plugins
    pub async fn get_all_widgets(&self) -> Vec<(String, lcc_plugin_api::WidgetDefinition)> {
        let mut widgets = Vec::new();
        
        for entry in self.plugins.iter() {
            let plugin_id = entry.key().clone();
            let plugin = entry.value();
            
            match plugin.get_widgets().await {
                Ok(plugin_widgets) => {
                    for widget in plugin_widgets {
                        widgets.push((plugin_id.clone(), widget));
                    }
                }
                Err(e) => {
                    debug!("Failed to get widgets from {}: {}", plugin_id, e);
                }
            }
        }
        
        widgets
    }
    
    /// Shutdown all plugins gracefully
    pub async fn shutdown_all(&mut self) -> anyhow::Result<()> {
        info!("Shutting down all plugins");
        
        for mut entry in self.plugins.iter_mut() {
            let plugin_id = entry.key().clone();
            let plugin = entry.value_mut();
            
            if let Err(e) = plugin.shutdown().await {
                warn!("Error shutting down plugin {}: {}", plugin_id, e);
            }
        }
        
        self.plugins.clear();
        self.metadata.clear();
        
        Ok(())
    }
    
    /// Get plugin health status
    pub fn get_health(&self, plugin_id: &str) -> Option<PluginHealth> {
        self.plugins.get(plugin_id).map(|p| p.health())
    }
}
