#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════
#  AegisKey-Orchestrator — Automated Key Rotation
#  Typically invoked from a cron job or CI pipeline.
#  Usage: ./scripts/rotate_keys.sh [profile]
# ═══════════════════════════════════════════════════════

set -euo pipefail

PROFILE="${1:-production}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BIN="$PROJECT_ROOT/target/release/aegiskey"

if [ ! -x "$BIN" ]; then
    echo "✗ Binary not found at $BIN"
    echo "  Run ./scripts/setup.sh first."
    exit 1
fi

echo "──── Key Rotation ────"
echo "Profile : $PROFILE"
echo "Time    : $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo ""

"$BIN" --profile "$PROFILE" rotate-keys --scope all

echo ""
echo "──── Post-Rotation Report ────"

REPORT_DIR="$PROJECT_ROOT/reports"
mkdir -p "$REPORT_DIR"

REPORT_FILE="$REPORT_DIR/rotation-$(date +%Y%m%d-%H%M%S).json"
"$BIN" --profile "$PROFILE" report --output "$REPORT_FILE"

echo "Report saved: $REPORT_FILE"
echo ""
echo "Done."
