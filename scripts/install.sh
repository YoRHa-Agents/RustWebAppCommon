#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT/scripts/release-contract.sh"

INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
BINARY_NAME="${BINARY_NAME:-common}"
LOCAL_RELEASE_DIR="${LOCAL_RELEASE_DIR:-}"
RWC_POST_INSTALL_HOOK="${RWC_POST_INSTALL_HOOK:-}"
RWC_INSTALL_REQUIRE_MANIFEST="${RWC_INSTALL_REQUIRE_MANIFEST:-0}"

run_post_install_hook() {
  local installed_binary asset_name install_mode manifest_path
  installed_binary="$1"
  asset_name="$2"
  install_mode="$3"
  manifest_path="${4:-}"

  if [[ -z "$RWC_POST_INSTALL_HOOK" ]]; then
    return 0
  fi

  echo "Running post-install hook"
  env \
    RWC_INSTALLED_BINARY="$installed_binary" \
    RWC_INSTALL_ASSET="$asset_name" \
    RWC_INSTALL_MODE="$install_mode" \
    RWC_INSTALL_DIR="$INSTALL_DIR" \
    RWC_INSTALL_MANIFEST_PATH="$manifest_path" \
    RWC_INSTALL_RELEASE_REPO="$(rwc_release_repo)" \
    bash -lc "$RWC_POST_INSTALL_HOOK"
}

main() {
  local asset_name temp_dir source_asset checksum_file manifest_path install_mode platform_manifest
  asset_name="$(rwc_detect_asset_name)"
  platform_manifest="$(rwc_manifest_name_for_target "$(rwc_target_platform)")"
  temp_dir="$(mktemp -d)"
  trap "rm -rf '$temp_dir'" EXIT

  mkdir -p "$INSTALL_DIR"

  if [[ -n "$LOCAL_RELEASE_DIR" && -f "$LOCAL_RELEASE_DIR/$asset_name" ]]; then
    install_mode="local-release-dir"
    source_asset="$LOCAL_RELEASE_DIR/$asset_name"
    checksum_file="$LOCAL_RELEASE_DIR/SHA256SUMS"
    manifest_path="$LOCAL_RELEASE_DIR/$platform_manifest"
    if [[ ! -f "$manifest_path" ]]; then
      manifest_path="$LOCAL_RELEASE_DIR/release-manifest.json"
    fi
    rwc_validate_local_release_dir "$LOCAL_RELEASE_DIR" "$asset_name"
    cp "$source_asset" "$INSTALL_DIR/$BINARY_NAME"
  elif command -v gh >/dev/null 2>&1; then
    install_mode="github-release"
    if [[ -n "${RWC_RELEASE_TAG:-}" ]]; then
      gh release download "$RWC_RELEASE_TAG" \
        --repo "$(rwc_release_repo)" \
        --pattern "$asset_name" \
        --pattern "SHA256SUMS" \
        --pattern "$platform_manifest" \
        --pattern "release-manifest.json" \
        --dir "$temp_dir" \
        --clobber
    else
      gh release download \
        --repo "$(rwc_release_repo)" \
        --pattern "$asset_name" \
        --pattern "SHA256SUMS" \
        --pattern "$platform_manifest" \
        --pattern "release-manifest.json" \
        --dir "$temp_dir" \
        --clobber
    fi
    rwc_verify_checksum "$temp_dir/$asset_name" "$temp_dir/SHA256SUMS"
    manifest_path="$temp_dir/$platform_manifest"
    if [[ ! -f "$manifest_path" ]]; then
      manifest_path="$temp_dir/release-manifest.json"
    fi
    if [[ -f "$manifest_path" ]]; then
      rwc_validate_release_manifest "$manifest_path" "$asset_name" "$(rwc_target_platform)"
    elif [[ "$RWC_INSTALL_REQUIRE_MANIFEST" == "1" ]]; then
      echo "missing release manifest in downloaded release" >&2
      return 1
    fi
    mv "$temp_dir/$asset_name" "$INSTALL_DIR/$BINARY_NAME"
  else
    install_mode="direct-download"
    curl -fsSL "$(rwc_release_download_url "$asset_name")" -o "$temp_dir/$asset_name"
    curl -fsSL "$(rwc_release_download_url "SHA256SUMS")" -o "$temp_dir/SHA256SUMS"
    manifest_path="$temp_dir/$platform_manifest"
    if curl -fsSL "$(rwc_release_download_url "$platform_manifest")" -o "$manifest_path"; then
      rwc_validate_release_manifest "$manifest_path" "$asset_name" "$(rwc_target_platform)"
    elif curl -fsSL "$(rwc_release_download_url "release-manifest.json")" -o "$temp_dir/release-manifest.json"; then
      manifest_path="$temp_dir/release-manifest.json"
      rwc_validate_release_manifest "$manifest_path" "$asset_name" "$(rwc_target_platform)"
    elif [[ "$RWC_INSTALL_REQUIRE_MANIFEST" == "1" ]]; then
      echo "missing release manifest in downloaded release" >&2
      return 1
    else
      manifest_path=""
    fi
    rwc_verify_checksum "$temp_dir/$asset_name" "$temp_dir/SHA256SUMS"
    mv "$temp_dir/$asset_name" "$INSTALL_DIR/$BINARY_NAME"
  fi

  chmod +x "$INSTALL_DIR/$BINARY_NAME"
  if ! run_post_install_hook "$INSTALL_DIR/$BINARY_NAME" "$asset_name" "$install_mode" "$manifest_path"; then
    echo "post-install hook failed for $BINARY_NAME" >&2
    return 1
  fi
  echo "Installed $BINARY_NAME to $INSTALL_DIR"
  echo "Verified asset: $asset_name"
  if [[ -n "${manifest_path:-}" ]]; then
    echo "Verified manifest: $manifest_path"
  fi
  echo "Try: $INSTALL_DIR/$BINARY_NAME demo"
}

main "$@"
