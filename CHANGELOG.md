# Changelog

All notable changes to AegisKey-Orchestrator are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [1.4.2] — 2026-02-10

### Fixed
- Fixed key rotation timestamp precision on Windows targets
- Corrected PBKDF2 round count documentation (600,000 rounds, not 100,000)

### Changed
- Improved error messages when `env/.env` is missing
- Updated `chrono` to 0.4.39

## [1.4.1] — 2026-01-22

### Fixed
- Decrypt command no longer panics on zero-length files
- Profile name validation now case-insensitive

## [1.4.0] — 2025-12-15

### Added
- `--dry-run` global flag for all subcommands
- JSON report generation (`report` subcommand)
- `status` subcommand for quick health checks
- Structured logging with timestamps

### Changed
- Migrated from `dotenv` to `dotenvy` (maintained fork)
- PBKDF2 rounds increased from 100,000 to 600,000

## [1.3.0] — 2025-10-03

### Added
- Profile system (development, staging, production)
- Key rotation simulation with `rotate-keys` subcommand
- CI pipeline with `cargo audit`

### Changed
- Replaced raw AES-CBC with AES-256-GCM (authenticated encryption)
- Password derivation switched from SHA-256 to PBKDF2-HMAC-SHA256

## [1.2.0] — 2025-07-19

### Added
- `--verbose` flag with debug-level logging
- Basic encrypt/decrypt CLI commands

### Fixed
- Salt generation now uses OS-level CSPRNG

## [1.1.0] — 2025-05-01

### Added
- Initial env file loading from `env/.env`
- MIT license

## [1.0.0] — 2025-03-14

### Added
- Project scaffolding
- README and architecture documentation
- "It compiles, ship it." — M. Berthelot
