#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
source "$ROOT/scripts/release-contract.sh"

HOST_PLATFORM="$(rwc_host_platform)"
TARGET_PLATFORM="${RWC_TARGET_PLATFORM:-$HOST_PLATFORM}"
if [[ "$TARGET_PLATFORM" != "$HOST_PLATFORM" ]]; then
  echo "build-release.sh only builds the host platform binary" >&2
  echo "  host:   $HOST_PLATFORM" >&2
  echo "  target: $TARGET_PLATFORM" >&2
  echo "Use release-contract helpers or CI matrix checks for contract-only validation." >&2
  exit 1
fi

ASSET_NAME="$(rwc_asset_name_for_target "$HOST_PLATFORM")"
RELEASE_DIR="$ROOT/release"

rm -rf "$RELEASE_DIR/site"
mkdir -p "$RELEASE_DIR/site"
rm -f "$RELEASE_DIR/$ASSET_NAME" "$RELEASE_DIR/SHA256SUMS" "$RELEASE_DIR/release-manifest.json"

cargo run -p common_cli -- demo
cargo run -p common_cli -- docs
cargo build --release -p common_cli

cp "$ROOT/target/release/common" "$RELEASE_DIR/$ASSET_NAME"
chmod +x "$RELEASE_DIR/$ASSET_NAME"
cp -R "$ROOT/site/." "$RELEASE_DIR/site/"

{
  (
    cd "$RELEASE_DIR"
    rwc_sha_cmd "$ASSET_NAME"
    while IFS= read -r file; do
      rwc_sha_cmd "$file"
    done < <(find "site" -type f | sort)
  )
} > "$RELEASE_DIR/SHA256SUMS"
rwc_write_release_manifest "$RELEASE_DIR" "$ASSET_NAME"
rwc_validate_local_release_dir "$RELEASE_DIR" "$ASSET_NAME"

echo "Release artifacts written to $RELEASE_DIR"
echo "  binary: $RELEASE_DIR/$ASSET_NAME"
echo "  site:   $RELEASE_DIR/site"
echo "  sums:   $RELEASE_DIR/SHA256SUMS"
echo "  meta:   $RELEASE_DIR/release-manifest.json"
