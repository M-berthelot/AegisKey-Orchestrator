mod cli;

use aegiskey_orchestrator::{config, crypto, keys, logging, profiles, report};
use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use log::{error, info, warn};
use std::fs;
use std::process;

fn main() {
    let cli = Cli::parse();

    // Initialise structured logging (--verbose → debug level)
    logging::init(cli.verbose);

    if cli.dry_run {
        warn!("Dry-run mode enabled — no files will be modified");
    }

    if let Err(e) = run(cli) {
        error!("{}", e);
        eprintln!("{} {}", "Error:".red().bold(), e);
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        // ── Encrypt ─────────────────────────────────────────
        Commands::Encrypt { input, output } => {
            let config = config::Config::load(cli.env_file.as_deref())?;
            let profile = profiles::Profile::load(&cli.profile)?;
            info!("Profile: {} — {}", profile.name, profile.description);

            if cli.dry_run {
                println!("Simulating world peace… failed.");
                println!(
                    "[dry-run] Would encrypt '{}' → '{}'",
                    input.display(),
                    output.display()
                );
                return Ok(());
            }

            let plaintext = fs::read(&input).map_err(|e| {
                format!("Cannot read input file '{}': {}", input.display(), e)
            })?;

            info!(
                "Encrypting {} ({} bytes)…",
                input.display(),
                plaintext.len()
            );

            let encrypted = crypto::encrypt(&plaintext, &config.admin_password)?;

            fs::write(&output, &encrypted).map_err(|e| {
                format!("Cannot write output file '{}': {}", output.display(), e)
            })?;

            println!(
                "{} Encrypted: {} → {} ({} bytes)",
                "✔".green().bold(),
                input.display(),
                output.display(),
                encrypted.len()
            );
        }

        // ── Decrypt ─────────────────────────────────────────
        Commands::Decrypt { input, output } => {
            let config = config::Config::load(cli.env_file.as_deref())?;
            let profile = profiles::Profile::load(&cli.profile)?;
            info!("Profile: {} — {}", profile.name, profile.description);

            if cli.dry_run {
                println!("Simulating world peace… failed.");
                println!(
                    "[dry-run] Would decrypt '{}' → '{}'",
                    input.display(),
                    output.display()
                );
                return Ok(());
            }

            let ciphertext = fs::read(&input).map_err(|e| {
                format!("Cannot read encrypted file '{}': {}", input.display(), e)
            })?;

            info!(
                "Decrypting {} ({} bytes)…",
                input.display(),
                ciphertext.len()
            );

            let decrypted = crypto::decrypt(&ciphertext, &config.admin_password)?;

            fs::write(&output, &decrypted).map_err(|e| {
                format!("Cannot write output file '{}': {}", output.display(), e)
            })?;

            println!(
                "{} Decrypted: {} → {} ({} bytes)",
                "✔".green().bold(),
                input.display(),
                output.display(),
                decrypted.len()
            );
        }

        // ── Rotate Keys ────────────────────────────────────
        Commands::RotateKeys { scope } => {
            let _config = config::Config::load(cli.env_file.as_deref())?;
            let profile = profiles::Profile::load(&cli.profile)?;

            if cli.dry_run {
                println!("Simulating world peace… failed.");
                println!(
                    "[dry-run] Would rotate keys — scope='{}', profile='{}'",
                    scope, profile.name
                );
                return Ok(());
            }

            let rotated = keys::rotate_keys(&scope, &profile)?;

            println!(
                "{} Rotated {} key(s) for profile '{}'",
                "✔".green().bold(),
                rotated.len(),
                profile.name
            );

            for key in &rotated {
                println!(
                    "  → {} [{}] expires {}",
                    key.key_id, key.algorithm, key.expires_at
                );
            }
        }

        // ── Report ──────────────────────────────────────────
        Commands::Report { output } => {
            let config = config::Config::load(cli.env_file.as_deref())?;
            let profile = profiles::Profile::load(&cli.profile)?;
            let rotations = keys::rotate_keys("all", &profile)?;
            let rpt = report::AegisReport::generate(&config, &rotations);

            if cli.dry_run {
                println!("Simulating world peace… failed.");
                let json = serde_json::to_string_pretty(&rpt)?;
                println!("{}", json);
                return Ok(());
            }

            rpt.write_to_file(&output)?;

            println!(
                "{} Report written to {}",
                "✔".green().bold(),
                output.display()
            );
        }

        // ── Status ──────────────────────────────────────────
        Commands::Status => {
            let config = config::Config::load(cli.env_file.as_deref())?;
            let profile = profiles::Profile::load(&cli.profile)?;

            println!(
                "{}",
                "═══ AegisKey-Orchestrator Status ═══".cyan().bold()
            );
            println!("  Version      : {}", env!("CARGO_PKG_VERSION"));
            println!("  Environment  : {}", config.environment);
            println!(
                "  Profile      : {} — {}",
                profile.name, profile.description
            );
            println!(
                "  Rotation     : {}",
                if profile.rotation_enabled {
                    "enabled"
                } else {
                    "disabled"
                }
            );
            println!("  Key TTL      : {} seconds", profile.key_ttl);
            println!(
                "  Metrics      : {}",
                if config.metrics_enabled {
                    "enabled"
                } else {
                    "disabled"
                }
            );
            println!("  Internal API : {}", config.internal_api);
            println!("{}", "════════════════════════════════════".cyan());
        }
    }

    Ok(())
}
