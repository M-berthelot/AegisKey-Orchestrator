//! Environment profiles — `development`, `staging`, `production`.
//!
//! Profiles govern key TTLs, rotation policies, and audit settings.
//! They are selected at runtime via the `--profile` CLI flag.

use crate::error::{AegisError, AegisResult};
use serde::{Deserialize, Serialize};

/// A security profile with environment-specific policies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// Profile identifier.
    pub name: String,
    /// Human-readable description.
    pub description: String,
    /// Key time-to-live in seconds.
    pub key_ttl: u64,
    /// Maximum allowed key age in seconds before forced rotation.
    pub max_key_age: u64,
    /// Whether automatic key rotation is enabled.
    pub rotation_enabled: bool,
    /// Whether operations are written to the audit log.
    pub audit_logging: bool,
}

impl Profile {
    /// Load a built-in profile by name.
    ///
    /// Accepted values (case-sensitive): `development` / `dev`, `staging`,
    /// `production` / `prod`.
    pub fn load(name: &str) -> AegisResult<Self> {
        match name {
            "development" | "dev" => Ok(Profile {
                name: "development".to_string(),
                description: "Local development — relaxed security, verbose logging".to_string(),
                key_ttl: 3600 * 24 * 30,    // 30 days
                max_key_age: 3600 * 24 * 90, // 90 days
                rotation_enabled: false,
                audit_logging: false,
            }),

            "staging" => Ok(Profile {
                name: "staging".to_string(),
                description: "Pre-production — mirrors prod constraints".to_string(),
                key_ttl: 3600 * 24 * 7,      // 7 days
                max_key_age: 3600 * 24 * 30,  // 30 days
                rotation_enabled: true,
                audit_logging: true,
            }),

            "production" | "prod" => Ok(Profile {
                name: "production".to_string(),
                description: "Production — strict rotation, full audit trail".to_string(),
                key_ttl: 3600 * 24,           // 24 hours
                max_key_age: 3600 * 24 * 7,   // 7 days
                rotation_enabled: true,
                audit_logging: true,
            }),

            other => Err(AegisError::Profile(format!(
                "Unknown profile '{}'. Available profiles: development, staging, production",
                other
            ))),
        }
    }

    /// Returns all built-in profiles. Used by the report generator.
    pub fn all() -> Vec<Profile> {
        vec![
            Profile::load("development").unwrap(),
            Profile::load("staging").unwrap(),
            Profile::load("production").unwrap(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_known_profiles() {
        assert!(Profile::load("dev").is_ok());
        assert!(Profile::load("development").is_ok());
        assert!(Profile::load("staging").is_ok());
        assert!(Profile::load("prod").is_ok());
        assert!(Profile::load("production").is_ok());
    }

    #[test]
    fn test_unknown_profile() {
        let err = Profile::load("narnia").unwrap_err();
        assert!(err.to_string().contains("narnia"));
    }

    #[test]
    fn test_prod_stricter_than_dev() {
        let dev = Profile::load("dev").unwrap();
        let prod = Profile::load("prod").unwrap();
        assert!(
            prod.key_ttl < dev.key_ttl,
            "Production key TTL should be shorter than development"
        );
        assert!(prod.rotation_enabled);
        assert!(!dev.rotation_enabled);
    }

    #[test]
    fn test_all_returns_three() {
        assert_eq!(Profile::all().len(), 3);
    }
}
