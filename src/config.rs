use crate::error::{AegisError, AegisResult};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use log::{debug, info, warn};
use std::path::{Path, PathBuf};

/// Runtime configuration loaded from `env/.env`.
#[derive(Debug, Clone)]
pub struct Config {
    /// Current environment name (development, staging, production).
    pub environment: String,
    /// Log level string.
    pub log_level: String,
    /// Decoded admin password (from base64).
    pub admin_password: String,
    /// Key rotation interval in seconds.
    pub key_rotation_interval: u64,
    /// Internal API base URL.
    pub internal_api: String,
    /// Whether Prometheus-style metrics are enabled.
    pub metrics_enabled: bool,
}

impl Config {
    /// Load configuration from the given env file (or `env/.env` by default).
    ///
    /// The file is parsed with `dotenvy` and the `ADMIN_PASSWORD_B64` value
    /// is decoded from base64 to obtain the raw password.
    pub fn load(env_path: Option<&Path>) -> AegisResult<Self> {
        let path = env_path
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("env/.env"));

        if !path.exists() {
            return Err(AegisError::Config(format!(
                "Environment file not found at '{}'. \
                 Did you forget to create it? Copy env/.env.example → env/.env",
                path.display()
            )));
        }

        dotenvy::from_path(&path).map_err(|e| {
            AegisError::Config(format!(
                "Failed to parse env file '{}': {}",
                path.display(),
                e
            ))
        })?;

        // Decode the admin password from base64
        let password_b64 = std::env::var("ADMIN_PASSWORD_B64").map_err(|_| {
            AegisError::EnvVar(
                "ADMIN_PASSWORD_B64 is not set. Check your env/.env file.".to_string(),
            )
        })?;

        if password_b64.is_empty() {
            return Err(AegisError::Config(
                "ADMIN_PASSWORD_B64 is set but empty. Please provide a base64-encoded password."
                    .to_string(),
            ));
        }

        let password_bytes = STANDARD.decode(&password_b64).map_err(|e| {
            AegisError::Config(format!(
                "Invalid base64 in ADMIN_PASSWORD_B64: {}. \
                 Generate a valid value with: echo -n 'password' | base64",
                e
            ))
        })?;

        let admin_password = String::from_utf8(password_bytes).map_err(|e| {
            AegisError::Config(format!("ADMIN_PASSWORD_B64 decoded to invalid UTF-8: {}", e))
        })?;

        debug!("Admin password decoded ({} chars)", admin_password.len());
        info!("Configuration loaded from {}", path.display());

        let environment =
            std::env::var("AEGIS_ENV").unwrap_or_else(|_| "development".to_string());

        if environment == "development" {
            warn!("Running in development mode — not recommended for production workloads");
        }

        Ok(Config {
            environment,
            log_level: std::env::var("AEGIS_LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            admin_password,
            key_rotation_interval: std::env::var("AEGIS_KEY_ROTATION_INTERVAL")
                .unwrap_or_else(|_| "86400".to_string())
                .parse()
                .unwrap_or(86400),
            internal_api: std::env::var("AEGIS_INTERNAL_API")
                .unwrap_or_else(|_| "http://127.0.0.1:9443".to_string()),
            metrics_enabled: std::env::var("AEGIS_METRICS_ENABLED")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
        })
    }
}
