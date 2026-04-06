#!/usr/bin/env bash
# Build the formora-c Rust library required by this C# project.
# Run once before dotnet build.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FORMORA_C_DIR="$SCRIPT_DIR/../formora-c"

echo "Building formora-c (release)..."
cargo build --release --manifest-path "$FORMORA_C_DIR/Cargo.toml"

LIB_DIR="$FORMORA_C_DIR/target/release"
echo "Done. Library at: $LIB_DIR"
echo ""
echo "Set the native library directory so dotnet can find it:"
echo "  Linux:  export LD_LIBRARY_PATH=\"$LIB_DIR:\$LD_LIBRARY_PATH\""
echo "  macOS:  export DYLD_LIBRARY_PATH=\"$LIB_DIR:\$DYLD_LIBRARY_PATH\""
echo "  Windows: copy $LIB_DIR/formora_c.dll next to your executable"
