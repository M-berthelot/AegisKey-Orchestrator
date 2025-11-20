//! JSON report generation for audit and monitoring purposes.

use crate::config::Config;
use crate::keys::KeyMetadata;
use crate::profiles::Profile;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Top-level audit report structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct AegisReport {
    pub generated_at: String,
    pub generator: String,
    pub version: String,
    pub environment: String,
    pub profiles: Vec<ProfileSummary>,
    pub recent_rotations: Vec<KeyMetadata>,
    pub warnings: Vec<String>,
}

/// Condensed profile information for reports.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileSummary {
    pub name: String,
    pub description: String,
    pub rotation_enabled: bool,
    pub key_ttl_hours: f64,
}

impl AegisReport {
    /// Generate a report snapshot from the current configuration and recent
    /// key rotations.
    pub fn generate(config: &Config, rotations: &[KeyMetadata]) -> Self {
        let profiles: Vec<ProfileSummary> = Profile::all()
            .into_iter()
            .map(|p| ProfileSummary {
                name: p.name,
                description: p.description,
                rotation_enabled: p.rotation_enabled,
                key_ttl_hours: p.key_ttl as f64 / 3600.0,
            })
            .collect();

        let mut warnings = Vec::new();

        if !config.metrics_enabled {
            warnings.push("Metrics collection is disabled".to_string());
        }
        if config.environment == "development" {
            warnings.push(
                "Running in development mode — not suitable for production use".to_string(),
            );
        }
        if rotations.is_empty() {
            warnings.push("No recent key rotations recorded".to_string());
        }

        AegisReport {
            generated_at: Utc::now().to_rfc3339(),
            generator: "aegiskey-orchestrator".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: config.environment.clone(),
            profiles,
            recent_rotations: rotations.to_vec(),
            warnings,
        }
    }

    /// Serialise the report as pretty-printed JSON and write it to `path`.
    pub fn write_to_file(&self, path: &Path) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)
            .expect("Report serialisation failed — this should never happen");
        fs::write(path, json)
    }
}
