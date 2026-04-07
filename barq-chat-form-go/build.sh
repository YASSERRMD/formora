#!/usr/bin/env bash
# Build the barq-chat-form-c Rust library required by this Go module.
# Run once before `go build` or `go test`.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BARQ_C_DIR="$SCRIPT_DIR/../barq-chat-form-c"

echo "Building barq-chat-form-c (release)..."
cargo build --release --manifest-path "$BARQ_C_DIR/Cargo.toml"

echo "Done. Library at: $BARQ_C_DIR/target/release/"
echo "Set LD_LIBRARY_PATH (Linux) or DYLD_LIBRARY_PATH (macOS) before running."
