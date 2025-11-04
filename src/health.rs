use asic_rs::data::miner::MinerData;
use iced::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

impl HealthStatus {
    /// Calculate health status from miner data
    pub fn from_miner_data(miner: &MinerData) -> Self {
        // Check if miner is actively mining
        if !miner.is_mining {
            return HealthStatus::Critical;
        }

        let mut critical_count = 0;
        let mut warning_count = 0;

        // Chip health check
        if let (Some(total), Some(expected)) = (miner.total_chips, miner.expected_chips) {
            let chip_health_ratio = total as f64 / expected as f64;
            if chip_health_ratio < 0.90 {
                critical_count += 1;
            } else if chip_health_ratio < 0.95 {
                warning_count += 1;
            }
        }

        // Hashrate health check
        if let (Some(current), Some(expected)) = (&miner.hashrate, &miner.expected_hashrate) {
            let hashrate_ratio = current.value / expected.value;
            if hashrate_ratio < 0.50 {
                critical_count += 1;
            } else if hashrate_ratio < 0.80 {
                warning_count += 1;
            }
        }

        // Temperature check
        if let Some(temp) = miner.average_temperature {
            let temp_c = temp.as_celsius();
            if temp_c > 85.0 {
                critical_count += 1;
            } else if temp_c > 75.0 {
                warning_count += 1;
            }
        }

        // Fan check - any fan at 0 RPM is critical
        for fan in &miner.fans {
            if let Some(rpm) = fan.rpm {
                if rpm.as_rpm() == 0.0 {
                    critical_count += 1;
                    break;
                }
            }
        }

        // Board health check - any board with 0 chips is critical
        for board in &miner.hashboards {
            if board.working_chips == Some(0) || board.working_chips.is_none() {
                critical_count += 1;
                break;
            }
        }

        // Check for critical error messages
        if !miner.messages.is_empty() {
            warning_count += 1;
        }

        // Determine overall health
        if critical_count > 0 {
            HealthStatus::Critical
        } else if warning_count > 0 {
            HealthStatus::Warning
        } else {
            HealthStatus::Healthy
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            HealthStatus::Healthy => "Healthy",
            HealthStatus::Warning => "Warning",
            HealthStatus::Critical => "Critical",
            HealthStatus::Unknown => "Unknown",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            HealthStatus::Healthy => "OK",
            HealthStatus::Warning => "!",
            HealthStatus::Critical => "X",
            HealthStatus::Unknown => "?",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            HealthStatus::Healthy => Color::from_rgb(0.2, 0.8, 0.2), // Green
            HealthStatus::Warning => Color::from_rgb(1.0, 0.7, 0.0), // Orange
            HealthStatus::Critical => Color::from_rgb(0.9, 0.2, 0.2), // Red
            HealthStatus::Unknown => Color::from_rgb(0.5, 0.5, 0.5), // Gray
        }
    }

    pub fn sort_priority(&self) -> u8 {
        match self {
            HealthStatus::Critical => 0,
            HealthStatus::Warning => 1,
            HealthStatus::Healthy => 2,
            HealthStatus::Unknown => 3,
        }
    }
}

/// Detailed health issues for a miner
#[derive(Debug, Clone)]
pub struct HealthReport {
    pub status: HealthStatus,
    pub issues: Vec<HealthIssue>,
}

#[derive(Debug, Clone)]
pub struct HealthIssue {
    pub severity: HealthStatus,
    pub category: IssueCategory,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IssueCategory {
    Chips,
    Hashrate,
    Temperature,
    Fans,
    Boards,
    Power,
    Network,
    Other,
}

impl HealthReport {
    pub fn from_miner_data(miner: &MinerData) -> Self {
        let mut issues = Vec::new();

        // Not mining check
        if !miner.is_mining {
            issues.push(HealthIssue {
                severity: HealthStatus::Critical,
                category: IssueCategory::Other,
                description: "Miner is not actively mining".to_string(),
            });
        }

        // Chip issues
        if let (Some(total), Some(expected)) = (miner.total_chips, miner.expected_chips) {
            let missing = expected - total;
            if missing > 0 {
                let ratio = total as f64 / expected as f64;
                let severity = if ratio < 0.90 {
                    HealthStatus::Critical
                } else {
                    HealthStatus::Warning
                };
                issues.push(HealthIssue {
                    severity,
                    category: IssueCategory::Chips,
                    description: format!(
                        "Missing {} chips ({}/{})",
                        missing, total, expected
                    ),
                });
            }
        }

        // Hashrate issues
        if let (Some(current), Some(expected)) = (&miner.hashrate, &miner.expected_hashrate) {
            let ratio = current.value / expected.value;
            if ratio < 0.80 {
                let severity = if ratio < 0.50 {
                    HealthStatus::Critical
                } else {
                    HealthStatus::Warning
                };
                let percentage = (ratio * 100.0) as u32;
                issues.push(HealthIssue {
                    severity,
                    category: IssueCategory::Hashrate,
                    description: format!(
                        "Low hashrate ({}% of expected)",
                        percentage
                    ),
                });
            }
        }

        // Temperature issues
        if let Some(temp) = miner.average_temperature {
            let temp_c = temp.as_celsius();
            if temp_c > 75.0 {
                let severity = if temp_c > 85.0 {
                    HealthStatus::Critical
                } else {
                    HealthStatus::Warning
                };
                issues.push(HealthIssue {
                    severity,
                    category: IssueCategory::Temperature,
                    description: format!("High temperature ({:.1}°C)", temp_c),
                });
            }
        }

        // Fan issues
        let dead_fans = miner.fans.iter().filter(|f| {
            f.rpm.map(|r| r.as_rpm() == 0.0).unwrap_or(false)
        }).count();
        if dead_fans > 0 {
            issues.push(HealthIssue {
                severity: HealthStatus::Critical,
                category: IssueCategory::Fans,
                description: format!("{} fan(s) not spinning", dead_fans),
            });
        }

        // Board issues
        let dead_boards = miner.hashboards.iter().filter(|b| b.working_chips == Some(0) || b.working_chips.is_none()).count();
        if dead_boards > 0 {
            issues.push(HealthIssue {
                severity: HealthStatus::Critical,
                category: IssueCategory::Boards,
                description: format!("{} board(s) with no working chips", dead_boards),
            });
        }

        // Efficiency issues (optional)
        if let Some(efficiency) = miner.efficiency {
            // Flag inefficient miners (>50 W/TH for modern miners)
            if efficiency > 50.0 {
                issues.push(HealthIssue {
                    severity: HealthStatus::Warning,
                    category: IssueCategory::Power,
                    description: format!("Poor efficiency ({:.1} W/TH)", efficiency),
                });
            }
        }

        // Parse error messages
        for msg in &miner.messages {
            if !msg.message.is_empty() {
                issues.push(HealthIssue {
                    severity: HealthStatus::Warning,
                    category: IssueCategory::Other,
                    description: msg.message.clone(),
                });
            }
        }

        let status = HealthStatus::from_miner_data(miner);

        HealthReport { status, issues }
    }

    pub fn critical_issues(&self) -> Vec<&HealthIssue> {
        self.issues
            .iter()
            .filter(|i| i.severity == HealthStatus::Critical)
            .collect()
    }

    pub fn warning_issues(&self) -> Vec<&HealthIssue> {
        self.issues
            .iter()
            .filter(|i| i.severity == HealthStatus::Warning)
            .collect()
    }
}
