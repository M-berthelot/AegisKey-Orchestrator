#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════
#  AegisKey-Orchestrator — Initial Setup
#  Usage: ./scripts/setup.sh
# ═══════════════════════════════════════════════════════

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "═══ AegisKey-Orchestrator Setup ═══"
echo ""

# ── Check Rust toolchain ──
if ! command -v cargo &> /dev/null; then
    echo "✗ Rust toolchain not found."
    echo "  Install from https://rustup.rs/"
    exit 1
fi

echo "✔ Rust toolchain: $(rustc --version)"
echo "  Cargo: $(cargo --version)"

# ── Environment file ──
if [ ! -f "$PROJECT_ROOT/env/.env" ]; then
    echo ""
    echo "⚠ env/.env not found — creating from template"
    cp "$PROJECT_ROOT/env/.env.example" "$PROJECT_ROOT/env/.env"
    echo "  → Edit env/.env and set ADMIN_PASSWORD_B64 before use."
    echo "  → Generate a value:  echo -n 'your_password' | base64"
else
    echo "✔ env/.env found"
fi

# ── Build ──
echo ""
echo "Building in release mode..."
cd "$PROJECT_ROOT"
cargo build --release 2>&1

BINARY="$PROJECT_ROOT/target/release/aegiskey"

if [ -x "$BINARY" ]; then
    echo ""
    echo "✔ Build successful: $BINARY"
    echo "  Version: $($BINARY --version)"
else
    echo ""
    echo "✗ Build failed — check output above."
    exit 1
fi

# ── Run tests ──
echo ""
echo "Running test suite..."
cargo test 2>&1

echo ""
echo "═══ Setup complete ═══"
echo ""
echo "Next steps:"
echo "  1. Verify env/.env has ADMIN_PASSWORD_B64 set"
echo "  2. Run:  ./target/release/aegiskey status"
echo "  3. Try:  ./target/release/aegiskey encrypt examples/sample.txt test.enc"
echo ""
