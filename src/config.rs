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

    pub fn with_scan_config(mut self, config: ScanConfig) -> Self {
        self.scan_config = config;
        self
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
    /// Load configuration from btc_toolkit_config.json
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: AppConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to btc_toolkit_config.json
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Load configuration from default location (btc_toolkit_config.json in current directory)
    pub fn load() -> Self {
        Self::load_from_file("btc_toolkit_config.json").unwrap_or_else(|e| {
            eprintln!("Warning: Failed to load config file: {e}");

            // If loading fails, create default config and save it
            let config = Self::default();
            if let Err(e) = config.save_to_file("btc_toolkit_config.json") {
                eprintln!("Warning: Failed to save default config: {e}");
            }
            config
        })
    }

    /// Save configuration to default location
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.save_to_file("btc_toolkit_config.json")
    }

    /// Add a new scan group
    pub fn add_scan_group(&mut self, group: ScanGroup) {
        self.scan_groups.push(group);
    }

    /// Remove a scan group by name
    pub fn remove_scan_group(&mut self, name: &str) -> bool {
        let initial_len = self.scan_groups.len();
        self.scan_groups.retain(|group| group.name != name);
        self.scan_groups.len() != initial_len
    }

    /// Update an existing scan group
    pub fn update_scan_group(&mut self, name: &str, updated_group: ScanGroup) -> bool {
        if let Some(group) = self.scan_groups.iter_mut().find(|g| g.name == name) {
            *group = updated_group;
            true
        } else {
            false
        }
    }

    /// Get enabled scan groups
    pub fn get_enabled_groups(&self) -> Vec<&ScanGroup> {
        self.scan_groups.iter().filter(|g| g.enabled).collect()
    }

    /// Get scan group by name
    pub fn get_group(&self, name: &str) -> Option<&ScanGroup> {
        self.scan_groups.iter().find(|g| g.name == name)
    }

    /// Get mutable scan group by name
    pub fn get_group_mut(&mut self, name: &str) -> Option<&mut ScanGroup> {
        self.scan_groups.iter_mut().find(|g| g.name == name)
    }

    /// Store scan results for a group
    pub fn store_scan_results(&mut self, group_name: &str, miners: Vec<MinerData>) {
        self.last_scan_results
            .insert(group_name.to_string(), miners);
    }

    /// Get scan results for a group
    pub fn get_scan_results(&self, group_name: &str) -> Option<&Vec<MinerData>> {
        self.last_scan_results.get(group_name)
    }

    /// Get all scan results
    pub fn get_all_scan_results(&self) -> &HashMap<String, Vec<MinerData>> {
        &self.last_scan_results
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
