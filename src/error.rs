use thiserror::Error;

/// Central error type for AegisKey-Orchestrator.
///
/// All modules propagate errors through this enum, ensuring consistent
/// error messages and exit behaviour across the CLI.
#[derive(Error, Debug)]
pub enum AegisError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Cryptographic error: {0}")]
    Crypto(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Key rotation error: {0}")]
    KeyRotation(String),

    #[error("Profile error: {0}")]
    Profile(String),

    #[error("Environment variable not found: {0}")]
    EnvVar(String),
}

/// Convenience alias used throughout the crate.
pub type AegisResult<T> = Result<T, AegisError>;
