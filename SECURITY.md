# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.4.x   | :white_check_mark: |
| 1.3.x   | :white_check_mark: |
| < 1.3   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability within AegisKey-Orchestrator, please
report it responsibly.

**DO NOT** open a public GitHub issue for security vulnerabilities.

Instead, email the security team directly:

- **Contact:** `security@malwarius.io`
- **PGP Key:** Available on request

We aim to acknowledge reports within **48 hours** and provide a fix within
**7 business days** for critical issues.

## Security Best Practices

- **Never** commit `env/.env` to version control. Use `env/.env.example` as a
  template and populate secrets locally.
- Rotate `ADMIN_PASSWORD` regularly (at least every 90 days).
- Use the `rotate-keys` command to cycle encryption keys on schedule.
- Enable audit logging in production profiles.
- Review the [architecture documentation](docs/architecture.md) for threat
  model details.

## Dependency Auditing

We run `cargo audit` in CI on every push. You can run it locally:

```bash
cargo install cargo-audit
cargo audit
```
