#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT/scripts/release-contract.sh"

LOCAL_RELEASE_DIR="${LOCAL_RELEASE_DIR:-}"
RWC_CURRENT_TAG="${RWC_CURRENT_TAG:-}"
RWC_UPDATE_REQUIRE_MANIFEST="${RWC_UPDATE_REQUIRE_MANIFEST:-0}"
RWC_UPDATE_REQUIRE_CHECKSUMS="${RWC_UPDATE_REQUIRE_CHECKSUMS:-0}"

main() {
  local asset_name response manifest_name
  asset_name="$(rwc_detect_asset_name)"
  manifest_name="$(rwc_manifest_name_for_target "$(rwc_target_platform)")"

  if [[ -n "$LOCAL_RELEASE_DIR" && -f "$LOCAL_RELEASE_DIR/$asset_name" ]]; then
    rwc_validate_local_release_dir "$LOCAL_RELEASE_DIR" "$asset_name"
    if [[ ! -f "$LOCAL_RELEASE_DIR/$manifest_name" ]]; then
      manifest_name="release-manifest.json"
    fi
    echo "Update source: local release directory"
    echo "Release dir: $LOCAL_RELEASE_DIR"
    echo "Asset name: $asset_name"
    echo "Platform: $(rwc_target_platform)"
    echo "Checksum: verified"
    echo "Manifest: $LOCAL_RELEASE_DIR/$manifest_name"
    echo "Migration ready: yes"
    return 0
  fi

  response="$(curl -fsSL "$(rwc_release_api_url)")"
  python3 - <<'PY' "$response" "$asset_name" "$manifest_name" "$RWC_CURRENT_TAG" "$RWC_UPDATE_REQUIRE_MANIFEST" "$RWC_UPDATE_REQUIRE_CHECKSUMS"
import json
import sys

payload = json.loads(sys.argv[1])
asset_name = sys.argv[2]
manifest_name = sys.argv[3]
current_tag = sys.argv[4]
require_manifest = sys.argv[5] == "1"
require_checksums = sys.argv[6] == "1"

tag = payload.get("tag_name", "unknown")
assets = payload.get("assets", [])
selected = next((asset for asset in assets if asset.get("name") == asset_name), None)
checksums = next((asset for asset in assets if asset.get("name") == "SHA256SUMS"), None)
manifest = next((asset for asset in assets if asset.get("name") == manifest_name), None)
if manifest is None:
    manifest = next((asset for asset in assets if asset.get("name") == "release-manifest.json"), None)

if selected is None:
    print(f"Latest release tag: {tag}")
    print(f"Expected asset: {asset_name}")
    print("Asset present: no")
    raise SystemExit(1)

print(f"Latest release tag: {tag}")
print(f"Expected asset: {asset_name}")
print("Asset present: yes")
print(f"Asset URL: {selected.get('browser_download_url', 'unknown')}")
print(f"Checksums present: {'yes' if checksums else 'no'}")
print(f"Manifest present: {'yes' if manifest else 'no'}")
migration_ready = checksums is not None and manifest is not None
print(f"Migration ready: {'yes' if migration_ready else 'no'}")
if current_tag:
    print(f"Current tag: {current_tag}")
    print(f"Update available: {'no' if current_tag == tag else 'yes'}")

if require_checksums and checksums is None:
    raise SystemExit("release is missing SHA256SUMS")

if require_manifest and manifest is None:
    raise SystemExit("release is missing release manifest")
PY
}

main "$@"
