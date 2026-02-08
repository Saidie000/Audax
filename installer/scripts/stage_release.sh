#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
STAGING_DIR="$ROOT_DIR/installer/staging"
ASSETS_DIR="$STAGING_DIR/assets"

mkdir -p "$ASSETS_DIR"

if [[ ! -f "$ROOT_DIR/target/release/ncom_audax.exe" ]]; then
  echo "Release executable not found. Run: cargo build --release" >&2
  exit 1
fi

cp "$ROOT_DIR/target/release/ncom_audax.exe" "$STAGING_DIR/NCOM_Audax.exe"

if [[ -d "$ROOT_DIR/assets" ]]; then
  rsync -a --delete "$ROOT_DIR/assets/" "$ASSETS_DIR/"
fi

echo "Staged release output to $STAGING_DIR"
