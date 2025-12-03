# AegisKey-Orchestrator

**Internal key management & file encryption CLI for Malwarius infrastructure.**

![CI](https://github.com/malwarius/AegisKey-Orchestrator/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)

---

## Overview

AegisKey-Orchestrator is Malwarius' internal tool for:

- **Encrypting / decrypting** sensitive files (AES-256-GCM + PBKDF2-HMAC-SHA256)
- **Simulating key rotation** across environments (development / staging / production)
- **Generating audit reports** in structured JSON
- **Managing security profiles** with environment-specific policies

> **Note:** This tool expects an `ADMIN_PASSWORD` derived from the `ADMIN_PASSWORD_B64`
> variable in your `env/.env` file. See [Configuration](#configuration) below.

---

## Prerequisites

- [Rust](https://rustup.rs/) 1.75 or later
- A Unix-like shell (Linux / macOS / WSL)

## Quick Start

```bash
# 1. Clone the repository
git clone https://github.com/malwarius/AegisKey-Orchestrator.git
cd AegisKey-Orchestrator

# 2. Set up the environment
cp env/.env.example env/.env
# → Edit env/.env and set ADMIN_PASSWORD_B64 (base64-encoded password)

# 3. Build
cargo build --release

# 4. Run
./target/release/aegiskey --help
```

Or use the setup script:

```bash
chmod +x scripts/setup.sh
./scripts/setup.sh
```

---

## Configuration

AegisKey reads its configuration from `env/.env`. Copy the example file and
populate the required values:

```bash
cp env/.env.example env/.env
```

| Variable                       | Required | Description                        |
| ------------------------------ | -------- | ---------------------------------- |
| `ADMIN_PASSWORD_B64`           | **Yes**  | Base64-encoded admin password      |
| `AEGIS_ENV`                    | No       | Environment name (default: `production`) |
| `AEGIS_LOG_LEVEL`              | No       | Log level: `debug`, `info`, `warn` |
| `AEGIS_KEY_ROTATION_INTERVAL`  | No       | Rotation interval in seconds       |
| `AEGIS_INTERNAL_API`           | No       | Internal API endpoint              |
| `AEGIS_METRICS_ENABLED`        | No       | Enable metrics (`true` / `false`)  |

> ⚠️ **WARNING:** Never commit `env/.env` to version control. It contains
> secrets. Use `env/.env.example` as a template.

---

## Usage

### Encrypt a file

```bash
aegiskey encrypt <INPUT> <OUTPUT>

# Example:
aegiskey encrypt examples/sample.txt secret.enc
```

### Decrypt a file

```bash
aegiskey decrypt <INPUT> <OUTPUT>

# Example:
aegiskey decrypt secret.enc recovered.txt
```

### Rotate keys (simulation)

```bash
aegiskey --profile production rotate-keys --scope all
```

### Generate status report

```bash
aegiskey report --output report.json
```

### Show orchestrator status

```bash
aegiskey status
```

### Global flags

| Flag              | Description                                    |
| ----------------- | ---------------------------------------------- |
| `--verbose`, `-v` | Enable debug logging                           |
| `--dry-run`       | Simulate without modifying files               |
| `--profile <P>`   | Active profile (`development`, `staging`, `production`) |
| `--env-file <F>`  | Path to a custom env file                      |

---

## Demo Walkthrough

```bash
# Build
cargo build --release

# Make sure env/.env exists and has ADMIN_PASSWORD_B64 set
# Encrypt the sample file
./target/release/aegiskey encrypt examples/sample.txt demo_encrypted.aegis

# Decrypt it back
./target/release/aegiskey decrypt demo_encrypted.aegis demo_decrypted.txt

# Verify
diff examples/sample.txt demo_decrypted.txt  # should produce no output

# Check status
./target/release/aegiskey status

# Generate a report
./target/release/aegiskey report --output demo_report.json
cat demo_report.json

# Try dry-run mode
./target/release/aegiskey --dry-run encrypt examples/sample.txt nope.enc

# Clean up
rm -f demo_encrypted.aegis demo_decrypted.txt demo_report.json
```

You can also run the library example directly:

```bash
cargo run --example demo_workflow
```

---

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture
```

---

## Project Structure

```
AegisKey-Orchestrator/
├── Cargo.toml              # Rust project manifest
├── README.md
├── LICENSE
├── SECURITY.md
├── CHANGELOG.md
├── .gitignore
├── docs/
│   ├── architecture.md     # System architecture & threat model
│   └── CONTRIBUTING.md     # Contribution guidelines
├── env/
│   ├── .env                # Local config (DO NOT COMMIT)
│   └── .env.example        # Template
├── scripts/
│   ├── setup.sh            # Initial setup
│   ├── rotate_keys.sh      # Automated rotation wrapper
│   └── generate_report.sh  # Report generation shortcut
├── src/
│   ├── main.rs             # CLI entry point
│   ├── lib.rs              # Library root
│   ├── cli.rs              # Clap CLI definitions
│   ├── config.rs           # Environment loading
│   ├── crypto.rs           # AES-256-GCM + PBKDF2
│   ├── error.rs            # Custom error types
│   ├── keys.rs             # Key metadata & rotation
│   ├── logging.rs          # Structured logging
│   ├── profiles.rs         # Environment profiles
│   └── report.rs           # JSON report generation
├── tests/
│   └── integration_test.rs # Integration tests
├── examples/
│   ├── sample.txt          # Sample plaintext file
│   └── demo_workflow.rs    # Programmatic usage example
└── .github/
    └── workflows/
        └── ci.yml          # GitHub Actions CI
```

---

## Architecture

See [docs/architecture.md](docs/architecture.md) for the full system design,
cryptographic choices, and threat model.

---

## Contributing

See [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md).

---

## License

MIT — see [LICENSE](LICENSE).

---

*Maintained by the Malwarius Infrastructure Team.*
*For internal use. Questions → `infra@malwarius.io`.*
