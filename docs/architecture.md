# Architecture — AegisKey-Orchestrator

> **Version:** 1.4.2
> **Author:** Maurice Berthelot — Malwarius Infrastructure Team
> **Last updated:** 2026-02-10

---

## 1. High-Level Overview

```
┌──────────────────────────────────────────────────────────────┐
│                      CLI (clap)                              │
│   encrypt │ decrypt │ rotate-keys │ report │ status          │
├──────────────────────────────────────────────────────────────┤
│                   Application Layer                          │
│  ┌──────────┐  ┌───────────┐  ┌──────────┐  ┌────────────┐ │
│  │  Config   │  │  Profiles │  │   Keys   │  │   Report   │ │
│  │ (dotenv)  │  │ dev/stg/  │  │ rotation │  │   (JSON)   │ │
│  │          │  │   prod    │  │ metadata │  │            │ │
│  └────┬─────┘  └─────┬─────┘  └────┬─────┘  └─────┬──────┘ │
│       │               │             │              │         │
├───────┴───────────────┴─────────────┴──────────────┴─────────┤
│                    Crypto Engine                             │
│          AES-256-GCM  +  PBKDF2-HMAC-SHA256                 │
│          Salt: 16 bytes │ Nonce: 12 bytes                    │
│          PBKDF2 rounds: 600,000                              │
├──────────────────────────────────────────────────────────────┤
│                    Logging (env_logger)                       │
│              Structured, timestamped output                  │
└──────────────────────────────────────────────────────────────┘
```

## 2. Modules

| Module       | File              | Responsibility                          |
| ------------ | ----------------- | --------------------------------------- |
| `cli`        | `src/cli.rs`      | Argument parsing (clap derive)          |
| `config`     | `src/config.rs`   | Load and validate `env/.env`            |
| `crypto`     | `src/crypto.rs`   | AES-256-GCM encrypt/decrypt, PBKDF2    |
| `error`      | `src/error.rs`    | Typed error hierarchy (`thiserror`)     |
| `keys`       | `src/keys.rs`     | Key metadata, simulated rotation        |
| `logging`    | `src/logging.rs`  | Logger initialisation, format           |
| `profiles`   | `src/profiles.rs` | Environment profiles and policies       |
| `report`     | `src/report.rs`   | JSON audit report generation            |

## 3. Cryptographic Design

### 3.1 Password Derivation

The admin password (`ADMIN_PASSWORD`) is stored as a base64-encoded value in
`env/.env` under `ADMIN_PASSWORD_B64`. At runtime:

1. The base64 value is decoded to obtain the raw password string.
2. A random 16-byte **salt** is generated (per encryption operation).
3. PBKDF2-HMAC-SHA256 derives a 256-bit key using **600,000 iterations**.

### 3.2 Authenticated Encryption

- **Algorithm:** AES-256-GCM (Galois/Counter Mode)
- **Nonce:** 12 bytes, randomly generated per operation
- **Tag:** 16 bytes (appended by GCM automatically)

### 3.3 Encrypted File Format

```
┌──────────┬───────────┬──────────────────────────┐
│  Salt    │  Nonce    │  Ciphertext + Auth Tag   │
│ 16 bytes │ 12 bytes  │  variable length         │
└──────────┴───────────┴──────────────────────────┘
```

The first 28 bytes of any `.aegis` file are the salt and nonce. The rest is
the GCM-encrypted payload including the 16-byte authentication tag.

### 3.4 Security Properties

- **Confidentiality:** AES-256-GCM
- **Integrity:** GCM authentication tag
- **Key stretching:** PBKDF2 with 600k rounds mitigates brute-force
- **Nonce uniqueness:** Random 96-bit nonce (collision probability negligible
  for typical usage volumes)

## 4. Profile System

Three built-in profiles govern operational parameters:

| Profile       | Key TTL  | Max Key Age | Rotation | Audit |
| ------------- | -------- | ----------- | -------- | ----- |
| `development` | 30 days  | 90 days     | Off      | Off   |
| `staging`     | 7 days   | 30 days     | On       | On    |
| `production`  | 24 hours | 7 days      | On       | On    |

Profiles are selected via `--profile <name>` and affect key rotation behaviour
and report warnings.

## 5. Data Flow — Encrypt

```
User invokes: aegiskey encrypt <in> <out>
         │
         ▼
  ┌─────────────┐
  │ Load Config  │ ← env/.env
  │ (dotenvy)    │
  └──────┬──────┘
         │  ADMIN_PASSWORD (decoded from B64)
         ▼
  ┌─────────────┐
  │ Read Input   │ ← plaintext file
  └──────┬──────┘
         │
         ▼
  ┌─────────────┐
  │ Generate     │  salt (16 B) + nonce (12 B)
  │ Random IV    │
  └──────┬──────┘
         │
         ▼
  ┌─────────────┐
  │ PBKDF2       │  password + salt → 256-bit key
  └──────┬──────┘
         │
         ▼
  ┌─────────────┐
  │ AES-256-GCM  │  key + nonce + plaintext → ciphertext
  │ Encrypt      │
  └──────┬──────┘
         │
         ▼
  ┌─────────────┐
  │ Write Output │  salt ‖ nonce ‖ ciphertext → file
  └─────────────┘
```

## 6. Threat Model

| Threat                         | Mitigation                          |
| ------------------------------ | ----------------------------------- |
| Brute-force password           | PBKDF2 600k rounds                 |
| Nonce reuse                    | Random nonce per operation          |
| Ciphertext tampering           | GCM authentication tag              |
| Env file exposure              | `.gitignore` + warnings             |
| Weak password                  | Organisational policy (not enforced)|

## 7. Future Considerations

- Hardware security module (HSM) integration for key storage
- Envelope encryption with master key wrapping
- Multi-user key access via Shamir's Secret Sharing
- Prometheus metrics endpoint for rotation monitoring
- gRPC/REST API mode for service-to-service encryption

---

*This document is maintained alongside the codebase. For questions, contact
`infra@malwarius.io`.*
