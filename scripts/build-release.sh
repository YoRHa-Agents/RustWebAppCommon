#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
source "$ROOT/scripts/release-contract.sh"

# The release directory keeps both a generic `release-manifest.json`
# and a platform-specific `release-manifest-<platform>.json`.
HOST_PLATFORM="$(rwc_host_platform)"
TARGET_PLATFORM="${RWC_TARGET_PLATFORM:-$HOST_PLATFORM}"
ASSET_NAME="$(rwc_asset_name_for_target "$TARGET_PLATFORM")"
PLATFORM_MANIFEST="$(rwc_manifest_name_for_target "$TARGET_PLATFORM")"
RELEASE_DIR="$ROOT/release"

rm -rf "$RELEASE_DIR/site"
mkdir -p "$RELEASE_DIR/site"
rm -f "$RELEASE_DIR/$ASSET_NAME" "$RELEASE_DIR/SHA256SUMS" "$RELEASE_DIR/release-manifest.json" "$RELEASE_DIR/$PLATFORM_MANIFEST"

cargo run -p common_cli -- demo
cargo run -p common_cli -- docs

case "$TARGET_PLATFORM" in
  linux-x86_64)
    cargo build --release -p common_cli
    cp "$ROOT/target/release/common" "$RELEASE_DIR/$ASSET_NAME"
    ;;
  linux-aarch64)
    if [[ "$HOST_PLATFORM" != linux-* ]]; then
      echo "linux-aarch64 builds require a Linux host with cargo-zigbuild" >&2
      exit 1
    fi
    cargo zigbuild --release -p common_cli --target aarch64-unknown-linux-musl
    cp "$ROOT/target/aarch64-unknown-linux-musl/release/common" "$RELEASE_DIR/$ASSET_NAME"
    ;;
  macos-aarch64)
    if [[ "$HOST_PLATFORM" != "macos-aarch64" ]]; then
      echo "macos-aarch64 builds require a macOS arm64 host" >&2
      exit 1
    fi
    cargo build --release -p common_cli --target aarch64-apple-darwin
    cp "$ROOT/target/aarch64-apple-darwin/release/common" "$RELEASE_DIR/$ASSET_NAME"
    ;;
  *)
    echo "unsupported build target: $TARGET_PLATFORM" >&2
    exit 1
    ;;
esac

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
echo "  meta+:  $RELEASE_DIR/$PLATFORM_MANIFEST"
