//! Key metadata and simulated rotation logic.
//!
//! In a production system this module would interface with a KMS (AWS KMS,
//! Vault, etc.). For now it generates metadata locally to demonstrate the
//! orchestration workflow.

use crate::error::AegisResult;
use crate::profiles::Profile;
use chrono::Utc;
use log::{info, warn};
use serde::{Deserialize, Serialize};

/// Metadata describing a managed encryption key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    pub key_id: String,
    pub profile: String,
    pub created_at: String,
    pub expires_at: String,
    pub algorithm: String,
    pub status: KeyStatus,
}

/// Lifecycle state of a managed key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyStatus {
    Active,
    Rotating,
    Expired,
    Revoked,
}

impl std::fmt::Display for KeyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyStatus::Active => write!(f, "active"),
            KeyStatus::Rotating => write!(f, "rotating"),
            KeyStatus::Expired => write!(f, "expired"),
            KeyStatus::Revoked => write!(f, "revoked"),
        }
    }
}

/// Simulates key rotation for the given `scope` under `profile`.
///
/// Returns freshly generated key metadata. In a real deployment this would:
/// 1. Contact the KMS to create new key material.
/// 2. Re-encrypt active data envelopes with the new key.
/// 3. Mark old keys as `Expired` after a grace period.
pub fn rotate_keys(scope: &str, profile: &Profile) -> AegisResult<Vec<KeyMetadata>> {
    info!(
        "Initiating key rotation — scope='{}', profile='{}'",
        scope, profile.name
    );

    let now = Utc::now();
    let expiry = now + chrono::Duration::seconds(profile.key_ttl as i64);

    let mut keys = Vec::new();

    // Primary encryption key
    if scope == "all" || scope == "primary" {
        keys.push(KeyMetadata {
            key_id: format!("aegis-{}-{}", profile.name, now.timestamp()),
            profile: profile.name.clone(),
            created_at: now.to_rfc3339(),
            expires_at: expiry.to_rfc3339(),
            algorithm: "AES-256-GCM".to_string(),
            status: KeyStatus::Active,
        });
    }

    // HMAC signing key
    if scope == "all" || scope == "hmac" {
        keys.push(KeyMetadata {
            key_id: format!("aegis-{}-{}-hmac", profile.name, now.timestamp()),
            profile: profile.name.clone(),
            created_at: now.to_rfc3339(),
            expires_at: expiry.to_rfc3339(),
            algorithm: "HMAC-SHA256".to_string(),
            status: KeyStatus::Active,
        });
    }

    if keys.is_empty() {
        warn!(
            "Unknown rotation scope '{}' — no keys rotated. Valid: all, primary, hmac",
            scope
        );
    } else {
        // In production: invalidate old keys, distribute new ones, update audit log…
        info!(
            "Key rotation complete — {} new key(s) for profile '{}'",
            keys.len(),
            profile.name
        );
    }

    Ok(keys)
}
