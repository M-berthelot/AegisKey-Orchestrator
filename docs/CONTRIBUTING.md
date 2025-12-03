# Contributing to AegisKey-Orchestrator

Thank you for considering contributing to AegisKey-Orchestrator. This document
outlines the process and conventions.

## Getting Started

1. Fork the repository (internal GitLab mirror preferred for Malwarius staff).
2. Create a feature branch from `develop`:
   ```bash
   git checkout -b feat/my-feature develop
   ```
3. Make your changes and ensure all checks pass:
   ```bash
   cargo fmt --all -- --check
   cargo clippy --all-targets -- -D warnings
   cargo test
   ```
4. Open a pull request against `develop`.

## Branch Naming

| Prefix     | Purpose                    |
| ---------- | -------------------------- |
| `feat/`    | New features               |
| `fix/`     | Bug fixes                  |
| `refactor/`| Code restructuring         |
| `docs/`    | Documentation only         |
| `chore/`   | Tooling, CI, dependencies  |

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(crypto): add Argon2id support for key derivation
fix(config): handle missing AEGIS_ENV gracefully
docs(readme): update demo walkthrough
```

## Code Style

- Run `cargo fmt` before committing.
- All public items must have doc comments (`///`).
- Avoid `unwrap()` in library code — use the `AegisError` type.
- `unwrap()` is acceptable in tests (with a comment explaining why).

## Testing

- Every new feature should include at least one test.
- Integration tests go in `tests/`.
- Unit tests go in the module file under `#[cfg(test)]`.
- Name tests descriptively: `test_<what>_<expected_behaviour>`.

## Security

If your change touches cryptographic code:

1. Document the rationale in the PR description.
2. Reference the relevant section of `docs/architecture.md`.
3. Request review from at least two maintainers.

See [SECURITY.md](../SECURITY.md) for vulnerability reporting.

---

*Questions? Reach out to `infra@malwarius.io`.*
