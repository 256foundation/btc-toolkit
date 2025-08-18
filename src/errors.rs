use std::fmt;


#[derive(Debug, Clone)]
pub enum ConfigError {
    FileNotFound(String),
    Serialization(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileNotFound(path) => write!(f, "Config file not found: {}", path),
            ConfigError::Serialization(msg) => write!(f, "Config serialization error: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug, Clone)]
pub enum ScannerError {
    NetworkRangeInvalid(String),
    ChannelClosed,
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScannerError::NetworkRangeInvalid(range) => {
                write!(f, "Invalid network range: {}", range)
            }
            ScannerError::ChannelClosed => write!(f, "Communication channel closed"),
        }
    }
}

impl std::error::Error for ScannerError {}


pub type ConfigResult<T> = Result<T, ConfigError>;
pub type ScannerResult<T> = Result<T, ScannerError>;
