use crate::errors::{ConfigError, ConfigResult};
use crate::network::scanner::ScanConfig;
use asic_rs::data::miner::MinerData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Represents a scan group with name, network range, and scan configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanGroup {
    pub name: String,
    pub network_range: String, // CIDR or range notation
    pub scan_config: ScanConfig,
    pub enabled: bool,
}

impl ScanGroup {
    pub fn new(name: String, network_range: String) -> Self {
        Self {
            name,
            network_range,
            scan_config: ScanConfig::default(),
            enabled: true,
        }
    }
}

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: String,
    pub scan_groups: Vec<ScanGroup>,
    pub last_scan_results: HashMap<String, Vec<MinerData>>, // Group name -> miners
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            scan_groups: vec![ScanGroup::new(
                "Default".to_string(),
                "192.168.1.0/24".to_string(),
            )],
            last_scan_results: HashMap::new(),
        }
    }
}

impl AppConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> ConfigResult<Self> {
        let path_ref = path.as_ref();
        let content = fs::read_to_string(path_ref)
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    ConfigError::FileNotFound(path_ref.display().to_string())
                } else {
                    ConfigError::Io(format!("{}: {}", path_ref.display(), e))
                }
            })?;

        serde_json::from_str(&content)
            .map_err(|e| ConfigError::Serialization(e.to_string()))
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> ConfigResult<()> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| ConfigError::Serialization(e.to_string()))?;

        fs::write(path.as_ref(), content)
            .map_err(|e| ConfigError::Io(format!("{}: {}", path.as_ref().display(), e)))
    }

    pub fn load() -> Self {
        // Load config or create default if file missing/invalid
        Self::load_from_file("btc_toolkit_config.json").unwrap_or_else(|e| {
            eprintln!("Warning: Failed to load config file: {e}");

            let config = Self::default();
            if let Err(e) = config.save_to_file("btc_toolkit_config.json") {
                eprintln!("Warning: Failed to save default config: {e}");
            }
            config
        })
    }

    pub fn save(&self) -> ConfigResult<()> {
        self.save_to_file("btc_toolkit_config.json")
    }

    pub fn add_scan_group(&mut self, group: ScanGroup) {
        self.scan_groups.push(group);
    }

    pub fn remove_scan_group(&mut self, name: &str) -> bool {
        let initial_len = self.scan_groups.len();
        self.scan_groups.retain(|group| group.name != name);
        self.scan_groups.len() < initial_len
    }

    pub fn update_scan_group(&mut self, name: &str, updated_group: ScanGroup) -> bool {
        self.scan_groups
            .iter_mut()
            .find(|g| g.name == name)
            .map(|group| {
                *group = updated_group;
                true
            })
            .unwrap_or(false)
    }

    pub fn get_enabled_groups(&self) -> Vec<&ScanGroup> {
        self.scan_groups.iter().filter(|g| g.enabled).collect()
    }

    pub fn get_group(&self, name: &str) -> Option<&ScanGroup> {
        self.scan_groups.iter().find(|g| g.name == name)
    }

    pub fn get_group_mut(&mut self, name: &str) -> Option<&mut ScanGroup> {
        self.scan_groups.iter_mut().find(|g| g.name == name)
    }

    pub fn store_scan_results(&mut self, group_name: &str, miners: Vec<MinerData>) {
        self.last_scan_results
            .insert(group_name.to_string(), miners);
    }

    pub fn get_all_scan_results(&self) -> &HashMap<String, Vec<MinerData>> {
        &self.last_scan_results
    }

    pub fn clear_scan_results(&mut self) {
        self.last_scan_results.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_serialization() {
        let config = AppConfig::default();
        let json = serde_json::to_string_pretty(&config).unwrap();
        let parsed: AppConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.version, parsed.version);
        assert_eq!(config.scan_groups.len(), parsed.scan_groups.len());
    }

    #[test]
    fn test_scan_group_management() {
        let mut config = AppConfig::default();

        // Add a new group
        let new_group = ScanGroup::new("Farm A".to_string(), "10.0.1.0/24".to_string());
        config.add_scan_group(new_group);
        assert_eq!(config.scan_groups.len(), 2);

        // Remove a group
        assert!(config.remove_scan_group("Farm A"));
        assert_eq!(config.scan_groups.len(), 1);
        assert!(!config.remove_scan_group("Non-existent"));
    }
}
