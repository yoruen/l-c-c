//! Event Bus - Central event routing system

use lcc_plugin_api::PluginEvent;
use std::collections::HashMap;
use tokio::sync::{broadcast, RwLock};
use tracing::{debug, trace};

/// Central event bus for plugin communication
pub struct EventBus {
    sender: broadcast::Sender<(String, PluginEvent)>,
    subscribers: RwLock<HashMap<String, Vec<broadcast::Sender<PluginEvent>>>>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(1000);
        
        Self {
            sender,
            subscribers: RwLock::new(HashMap::new()),
        }
    }
    
    /// Publish an event to all subscribers
    pub async fn publish(&self, source: &str, event: PluginEvent) {
        let msg = (source.to_string(), event);
        let _ = self.sender.send(msg);
        trace!("Event published from {}", source);
    }
    
    /// Subscribe to all events
    pub fn subscribe(&self) -> broadcast::Receiver<(String, PluginEvent)> {
        self.sender.subscribe()
    }
    
    /// Subscribe to events from a specific source
    pub async fn subscribe_to(
        &self,
        source: &str,
    ) -> broadcast::Receiver<PluginEvent> {
        let (tx, rx) = broadcast::channel(100);
        
        let mut subs = self.subscribers.write().await;
        subs.entry(source.to_string())
            .or_insert_with(Vec::new)
            .push(tx);
            
        rx
    }
}
