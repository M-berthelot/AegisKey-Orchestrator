#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════
#  AegisKey-Orchestrator — Generate Status Report
#  Usage: ./scripts/generate_report.sh [output_path]
# ═══════════════════════════════════════════════════════

set -euo pipefail

OUTPUT="${1:-report.json}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BIN="$PROJECT_ROOT/target/release/aegiskey"

if [ ! -x "$BIN" ]; then
    echo "Binary not found. Building first..."
    cd "$PROJECT_ROOT" && cargo build --release
fi

echo "Generating AegisKey status report..."
echo "Output: $OUTPUT"
echo ""

"$BIN" report --output "$OUTPUT"

echo ""
echo "✔ Report written to: $OUTPUT"
echo ""

# Pretty-print a summary
if command -v python3 &> /dev/null; then
    echo "── Summary ──"
    python3 -c "
import json, sys
with open('$OUTPUT') as f:
    r = json.load(f)
print(f\"  Generated : {r['generated_at']}\")
print(f\"  Version   : {r['version']}\")
print(f\"  Env       : {r['environment']}\")
print(f\"  Profiles  : {len(r['profiles'])}\")
print(f\"  Rotations : {len(r['recent_rotations'])}\")
print(f\"  Warnings  : {len(r['warnings'])}\")
"
fi
