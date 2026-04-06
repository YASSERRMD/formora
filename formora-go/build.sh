#!/usr/bin/env bash
# Build the formora-c Rust library required by this Go module.
# Run once before `go build` or `go test`.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FORMORA_C_DIR="$SCRIPT_DIR/../formora-c"

echo "Building formora-c (release)..."
cargo build --release --manifest-path "$FORMORA_C_DIR/Cargo.toml"

echo "Done. Library at: $FORMORA_C_DIR/target/release/"
echo "Set LD_LIBRARY_PATH (Linux) or DYLD_LIBRARY_PATH (macOS) before running."
