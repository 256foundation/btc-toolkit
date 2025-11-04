use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ConfigError {
    #[error("Config file not found: {0}")]
    FileNotFound(String),

    #[error("Config serialization error: {0}")]
    Serialization(String),

    #[error("IO error: {0}")]
    Io(String),
}

#[derive(Debug, Clone, Error)]
pub enum ScannerError {
    #[error("Invalid network range: {0}")]
    NetworkRangeInvalid(String),

    #[error("Communication channel closed")]
    ChannelClosed,

    #[error("Thread execution error: {0}")]
    ThreadError(String),

    #[error("Runtime creation failed: {0}")]
    RuntimeError(String),
}

#[derive(Debug, Clone, Error)]
pub enum FetchError {
    #[error("Failed to create Tokio runtime: {0}")]
    RuntimeCreation(String),

    #[error("Failed to create miner factory: {0}")]
    FactoryCreation(String),

    #[error("No miner found at {0}")]
    MinerNotFound(String),

    #[error("Failed to get miner data: {0}")]
    MinerDataError(String),
}

pub type ConfigResult<T> = Result<T, ConfigError>;
pub type ScannerResult<T> = Result<T, ScannerError>;
pub type FetchResult<T> = Result<T, FetchError>;
