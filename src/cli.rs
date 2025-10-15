use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// AegisKey-Orchestrator — Internal key management & file encryption CLI.
///
/// Manages encryption keys, rotates secrets, and encrypts/decrypts
/// sensitive files across Malwarius development environments.
#[derive(Parser, Debug)]
#[command(
    name = "aegiskey",
    version,
    author = "Maurice Berthelot <m.berthelot@malwarius.io>",
    about = "AegisKey-Orchestrator — Internal key management & file encryption CLI",
    long_about = "\
AegisKey-Orchestrator is Malwarius' internal tooling for managing \
encryption keys, rotating secrets, and encrypting/decrypting \
sensitive files across development environments.\n\n\
See https://github.com/malwarius/AegisKey-Orchestrator for documentation."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose / debug logging
    #[arg(long, short, global = true)]
    pub verbose: bool,

    /// Simulate actions without writing any files
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Active security profile (development, staging, production)
    #[arg(long, global = true, default_value = "production")]
    pub profile: String,

    /// Path to the environment file (defaults to env/.env)
    #[arg(long, global = true)]
    pub env_file: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Encrypt a file using AES-256-GCM with the admin password
    Encrypt {
        /// Path to the input (plaintext) file
        #[arg(value_name = "INPUT")]
        input: PathBuf,

        /// Path for the encrypted output file
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,
    },

    /// Decrypt a file previously encrypted with AegisKey
    Decrypt {
        /// Path to the encrypted input file
        #[arg(value_name = "INPUT")]
        input: PathBuf,

        /// Path for the decrypted output file
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,
    },

    /// Simulate key rotation for the active profile
    #[command(name = "rotate-keys")]
    RotateKeys {
        /// Rotation scope: "all", "primary", or "hmac"
        #[arg(long, default_value = "all")]
        scope: String,
    },

    /// Generate a JSON status report
    Report {
        /// Output path for the report file
        #[arg(long, short, default_value = "report.json")]
        output: PathBuf,
    },

    /// Show current orchestrator status and configuration
    Status,
}
