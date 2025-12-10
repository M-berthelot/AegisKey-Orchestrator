//! Integration tests for AegisKey-Orchestrator.
//!
//! These tests exercise the public library API end-to-end.
//! For CLI binary tests, see the `assert_cmd` section at the bottom.

use aegiskey_orchestrator::crypto;
use aegiskey_orchestrator::profiles::Profile;

// ── Crypto round-trips ────────────────────────────────────

#[test]
fn test_encrypt_decrypt_roundtrip_small() {
    let password = "integration_test_password_42";
    let content = b"Short message.";

    let encrypted = crypto::encrypt(content, password).unwrap();
    let decrypted = crypto::decrypt(&encrypted, password).unwrap();

    assert_eq!(content.to_vec(), decrypted);
}

#[test]
fn test_encrypt_decrypt_roundtrip_multiline() {
    let password = "multiline_test";
    let content = b"Line 1: AegisKey integration test.\n\
                    Line 2: Multiple lines of text.\n\
                    Line 3: End of transmission.\n";

    let encrypted = crypto::encrypt(content, password).unwrap();
    let decrypted = crypto::decrypt(&encrypted, password).unwrap();

    assert_eq!(content.to_vec(), decrypted);
}

#[test]
fn test_encrypt_decrypt_binary_data() {
    let password = "binary_blob";
    let content: Vec<u8> = (0u8..=255).collect(); // all byte values

    let encrypted = crypto::encrypt(&content, password).unwrap();
    let decrypted = crypto::decrypt(&encrypted, password).unwrap();

    assert_eq!(content, decrypted);
}

#[test]
fn test_large_payload() {
    let password = "big_data_energy";
    let content = vec![0xABu8; 1_000_000]; // 1 MB of 0xAB

    let encrypted = crypto::encrypt(&content, password).unwrap();
    let decrypted = crypto::decrypt(&encrypted, password).unwrap();

    assert_eq!(content, decrypted);
}

// ── Error cases ───────────────────────────────────────────

#[test]
fn test_empty_ciphertext_rejected() {
    let result = crypto::decrypt(&[], "password");
    assert!(result.is_err());
}

#[test]
fn test_garbage_ciphertext_rejected() {
    let garbage = vec![0xFFu8; 128];
    let result = crypto::decrypt(&garbage, "password");
    assert!(result.is_err(), "random bytes should not decrypt successfully");
}

// ── Profile system ────────────────────────────────────────

#[test]
fn test_profile_dev_alias() {
    let p = Profile::load("dev").unwrap();
    assert_eq!(p.name, "development");
    assert!(!p.rotation_enabled);
    assert!(!p.audit_logging);
}

#[test]
fn test_profile_prod_alias() {
    let p = Profile::load("prod").unwrap();
    assert_eq!(p.name, "production");
    assert!(p.rotation_enabled);
    assert!(p.audit_logging);
}

#[test]
fn test_profile_staging() {
    let p = Profile::load("staging").unwrap();
    assert_eq!(p.name, "staging");
    assert!(p.rotation_enabled);
}

#[test]
fn test_profile_unknown_fails() {
    let result = Profile::load("hogwarts");
    assert!(result.is_err());
    assert!(
        result.unwrap_err().to_string().contains("hogwarts"),
        "error message should mention the invalid profile name"
    );
}

// ── Troll tests (mandatory) ──────────────────────────────

#[test]
fn test_certified_works_on_ci_probably() {
    // "Worked in staging" — every SRE's epitaph
    let data = vec![42u8; 1024]; // The answer to everything
    let enc = crypto::encrypt(&data, "the_answer").unwrap();
    let dec = crypto::decrypt(&enc, "the_answer").unwrap();
    assert_eq!(data, dec, "The universe is broken. Again.");
}

// ── CLI binary tests (assert_cmd) ─────────────────────────

#[cfg(feature = "cli_tests")]
mod cli_tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn test_missing_env_file_error() {
        let mut cmd = Command::cargo_bin("aegiskey").unwrap();
        cmd.args(["--env-file", "/tmp/_aegiskey_nonexistent_env_", "status"]);
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("not found"));
    }

    #[test]
    fn test_help_flag() {
        let mut cmd = Command::cargo_bin("aegiskey").unwrap();
        cmd.arg("--help");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("AegisKey-Orchestrator"));
    }
}
