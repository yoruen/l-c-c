//! Distribution detection and compatibility

use lcc_plugin_api::Distribution;
use std::fs;
use std::path::Path;
use tracing::{info, debug};

/// Information about the current distribution
#[derive(Debug, Clone)]
pub struct DistributionInfo {
    pub distribution: Distribution,
    pub version: String,
    pub codename: Option<String>,
    pub id_like: Vec<String>,
}

pub struct DistributionDetector;

impl DistributionDetector {
    /// Detect the current Linux distribution
    pub async fn detect() -> anyhow::Result<DistributionInfo> {
        // Try /etc/os-release first (standard)
        if let Ok(info) = Self::from_os_release().await {
            info!("Detected distribution from /etc/os-release: {:?}", info.distribution);
            return Ok(info);
        }
        
        // Fallback to legacy methods
        Self::fallback_detection().await
    }
    
    async fn from_os_release() -> anyhow::Result<DistributionInfo> {
        let content = fs::read_to_string("/etc/os-release")?;
        
        let mut id = None;
        let mut version_id = None;
        let mut codename = None;
        let mut id_like = Vec::new();
        
        for line in content.lines() {
            if let Some((key, value)) = line.split_once('=') {
                let value = value.trim_matches('"').to_string();
                match key {
                    "ID" => id = Some(value),
                    "VERSION_ID" => version_id = Some(value),
                    "VERSION_CODENAME" => codename = Some(value),
                    "ID_LIKE" => {
                        id_like = value.split_whitespace().map(String::from).collect();
                    }
                    _ => {}
                }
            }
        }
        
        let distribution = match id.as_deref() {
            Some("ubuntu") => Distribution::Ubuntu,
            Some("debian") => Distribution::Debian,
            Some("linuxmint") => Distribution::LinuxMint,
            Some("pop") => Distribution::PopOS,
            Some("fedora") => Distribution::Fedora,
            Some("arch") => Distribution::Arch,
            Some("endeavouros") => Distribution::EndeavourOS,
            Some("manjaro") => Distribution::Manjaro,
            Some("opensuse") | Some("opensuse-leap") | Some("opensuse-tumbleweed") => {
                Distribution::OpenSUSE
            }
            Some("nixos") => Distribution::NixOS,
            Some("gentoo") => Distribution::Gentoo,
            Some("alpine") => Distribution::Alpine,
            Some("void") => Distribution::Void,
            _ => Distribution::Generic,
        };
        
        Ok(DistributionInfo {
            distribution,
            version: version_id.unwrap_or_else(|| "unknown".to_string()),
            codename,
            id_like,
        })
    }
    
    async fn fallback_detection() -> anyhow::Result<DistributionInfo> {
        // Check for distribution-specific files
        if Path::new("/etc/arch-release").exists() {
            return Ok(DistributionInfo {
                distribution: Distribution::Arch,
                version: "rolling".to_string(),
                codename: None,
                id_like: vec![],
            });
        }
        
        if Path::new("/etc/debian_version").exists() {
            let version = fs::read_to_string("/etc/debian_version").unwrap_or_default();
            return Ok(DistributionInfo {
                distribution: Distribution::Debian,
                version: version.trim().to_string(),
                codename: None,
                id_like: vec![],
            });
        }
        
        if Path::new("/etc/fedora-release").exists() {
            return Ok(DistributionInfo {
                distribution: Distribution::Fedora,
                version: "unknown".to_string(),
                codename: None,
                id_like: vec![],
            });
        }
        
        Ok(DistributionInfo {
            distribution: Distribution::Generic,
            version: "unknown".to_string(),
            codename: None,
            id_like: vec![],
        })
    }
    
    /// Check if a distribution is supported in Phase 1
    pub fn is_phase1_supported(info: &DistributionInfo) -> bool {
        matches!(
            info.distribution,
            Distribution::Ubuntu
                | Distribution::Debian
                | Distribution::LinuxMint
                | Distribution::PopOS
                | Distribution::Fedora
                | Distribution::Arch
                | Distribution::EndeavourOS
                | Distribution::Manjaro
                | Distribution::OpenSUSE
        )
    }
}
