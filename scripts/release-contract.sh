#!/usr/bin/env bash

rwc_artifact_prefix() {
  echo "${RWC_ARTIFACT_PREFIX:-rustwebappcommon}"
}

rwc_supported_platforms() {
  printf '%s\n' \
    "linux-x86_64" \
    "linux-aarch64" \
    "macos-aarch64" \
    "macos-x86_64"
}

rwc_validate_target_platform() {
  local target
  target="$1"

  case "$target" in
    linux-x86_64 | linux-aarch64 | macos-aarch64 | macos-x86_64)
      ;;
    *)
      echo "unsupported target platform: $target" >&2
      return 1
      ;;
  esac
}

rwc_asset_name_for_target() {
  local target prefix
  target="$1"
  prefix="$(rwc_artifact_prefix)"
  rwc_validate_target_platform "$target"
  echo "${prefix}-${target}"
}

rwc_host_platform() {
  local os arch
  os="$(uname -s)"
  arch="$(uname -m)"

  case "$os" in
    Linux)
      case "$arch" in
        x86_64) echo "linux-x86_64" ;;
        aarch64 | arm64) echo "linux-aarch64" ;;
        *) echo "unsupported Linux architecture: $arch" >&2; return 1 ;;
      esac
      ;;
    Darwin)
      case "$arch" in
        arm64 | aarch64) echo "macos-aarch64" ;;
        x86_64) echo "macos-x86_64" ;;
        *) echo "unsupported macOS architecture: $arch" >&2; return 1 ;;
      esac
      ;;
    *)
      echo "unsupported OS: $os" >&2
      return 1
      ;;
  esac
}

rwc_release_repo() {
  echo "${RWC_RELEASE_REPO:-YoRHa-Agents/RustWebAppCommon}"
}

rwc_release_tag() {
  echo "${RWC_RELEASE_TAG:-}"
}

rwc_target_platform() {
  if [[ -n "${RWC_TARGET_PLATFORM:-}" ]]; then
    rwc_validate_target_platform "$RWC_TARGET_PLATFORM"
    echo "$RWC_TARGET_PLATFORM"
  else
    rwc_host_platform
  fi
}

rwc_detect_asset_name() {
  rwc_asset_name_for_target "$(rwc_target_platform)"
}

rwc_platform_from_asset_name() {
  local asset_name prefix
  asset_name="$1"
  prefix="$(rwc_artifact_prefix)-"

  if [[ "$asset_name" != "$prefix"* ]]; then
    echo "asset name does not match prefix ${prefix}: $asset_name" >&2
    return 1
  fi

  echo "${asset_name#"$prefix"}"
}

rwc_sha_cmd() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1"
  else
    shasum -a 256 "$1"
  fi
}

rwc_sha_value() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}

rwc_extract_expected_sha() {
  local checksum_file target_name
  checksum_file="$1"
  target_name="$2"
  awk -v target="$target_name" '
    {
      split($2, parts, "/")
      if (parts[length(parts)] == target) {
        print $1
        exit
      }
    }
  ' "$checksum_file"
}

rwc_verify_checksum() {
  local target_file checksum_file expected actual
  target_file="$1"
  checksum_file="$2"

  if [[ ! -f "$checksum_file" ]]; then
    echo "missing checksum file: $checksum_file" >&2
    return 1
  fi

  expected="$(rwc_extract_expected_sha "$checksum_file" "$(basename "$target_file")")"
  if [[ -z "$expected" ]]; then
    echo "missing checksum entry for $(basename "$target_file") in $checksum_file" >&2
    return 1
  fi

  actual="$(rwc_sha_value "$target_file")"
  if [[ "$expected" != "$actual" ]]; then
    echo "checksum mismatch for $(basename "$target_file")" >&2
    echo "  expected: $expected" >&2
    echo "  actual:   $actual" >&2
    return 1
  fi
}

rwc_release_api_url() {
  local api_base repo tag
  api_base="${RWC_UPDATE_API_BASE:-https://api.github.com}"
  repo="$(rwc_release_repo)"
  tag="${1:-$(rwc_release_tag)}"

  if [[ -n "$tag" ]]; then
    echo "${api_base}/repos/${repo}/releases/tags/${tag}"
  else
    echo "${api_base}/repos/${repo}/releases/latest"
  fi
}

rwc_release_download_url() {
  local asset_name tag repo
  asset_name="$1"
  tag="${2:-$(rwc_release_tag)}"
  repo="$(rwc_release_repo)"

  if [[ -n "$tag" ]]; then
    echo "https://github.com/${repo}/releases/download/${tag}/${asset_name}"
  else
    echo "https://github.com/${repo}/releases/latest/download/${asset_name}"
  fi
}

rwc_validate_release_manifest() {
  local manifest_path expected_asset expected_platform
  manifest_path="$1"
  expected_asset="${2:-}"
  expected_platform="${3:-}"

  python3 - <<'PY' "$manifest_path" "$expected_asset" "$expected_platform"
import json
import sys
from pathlib import Path

manifest_path = Path(sys.argv[1])
expected_asset = sys.argv[2]
expected_platform = sys.argv[3]

if not manifest_path.is_file():
    raise SystemExit(f"missing release manifest: {manifest_path}")

payload = json.loads(manifest_path.read_text(encoding="utf-8"))
required = [
    "release_repo",
    "binary_name",
    "asset_name",
    "asset_prefix",
    "platform",
    "site_dir",
    "checksum_file",
    "release_tag",
    "validation",
]
missing = [key for key in required if key not in payload]
if missing:
    raise SystemExit(f"release manifest missing keys: {', '.join(missing)}")

validation = payload["validation"]
if not isinstance(validation, dict):
    raise SystemExit("release manifest validation section must be an object")

for key in [
    "supports_install_hook",
    "supports_delegated_updater",
    "supports_release_analysis",
]:
    if not isinstance(validation.get(key), bool):
        raise SystemExit(f"release manifest validation.{key} must be a boolean")

if expected_asset and payload["asset_name"] != expected_asset:
    raise SystemExit(
        f"manifest asset mismatch: expected {expected_asset}, got {payload['asset_name']}"
    )

if expected_platform and payload["platform"] != expected_platform:
    raise SystemExit(
        f"manifest platform mismatch: expected {expected_platform}, got {payload['platform']}"
    )

print("Release manifest validation: ok")
PY
}

rwc_validate_site_contracts() {
  local site_root
  site_root="$1"

  for relative_path in \
    "assets/route-manifest.json" \
    "assets/theme-tokens.json" \
    "assets/runtime-contract.json" \
    "docs/docs-index.json"
  do
    if [[ ! -f "$site_root/$relative_path" ]]; then
      echo "missing site contract file: $site_root/$relative_path" >&2
      return 1
    fi
  done
}

rwc_validate_local_release_dir() {
  local release_dir asset_name checksum_file manifest_path expected_platform
  release_dir="$1"
  asset_name="${2:-$(rwc_detect_asset_name)}"
  checksum_file="$release_dir/SHA256SUMS"
  manifest_path="$release_dir/release-manifest.json"
  expected_platform="${RWC_TARGET_PLATFORM:-$(rwc_platform_from_asset_name "$asset_name")}"

  if [[ ! -f "$release_dir/$asset_name" ]]; then
    echo "missing release asset: $release_dir/$asset_name" >&2
    return 1
  fi
  if [[ ! -d "$release_dir/site" ]]; then
    echo "missing release site directory: $release_dir/site" >&2
    return 1
  fi

  rwc_verify_checksum "$release_dir/$asset_name" "$checksum_file"
  rwc_validate_site_contracts "$release_dir/site"
  rwc_validate_release_manifest "$manifest_path" "$asset_name" "$expected_platform"
}

rwc_write_release_manifest() {
  local release_dir asset_name
  release_dir="$1"
  asset_name="$2"

  python3 - <<'PY' "$release_dir" "$asset_name" "$(rwc_release_repo)" "${BINARY_NAME:-common}" "$(rwc_release_tag)" "$(rwc_platform_from_asset_name "$asset_name")" "$(rwc_artifact_prefix)"
import json
import sys
from pathlib import Path

release_dir = Path(sys.argv[1])
asset_name = sys.argv[2]
repo = sys.argv[3]
binary_name = sys.argv[4]
tag = sys.argv[5]
platform = sys.argv[6]
asset_prefix = sys.argv[7]

manifest = {
    "release_repo": repo,
    "binary_name": binary_name,
    "asset_name": asset_name,
    "asset_prefix": asset_prefix,
    "platform": platform,
    "site_dir": "site",
    "checksum_file": "SHA256SUMS",
    "release_tag": tag or "latest",
    "validation": {
        "supports_install_hook": True,
        "supports_delegated_updater": True,
        "supports_release_analysis": True,
    },
}

(release_dir / "release-manifest.json").write_text(
    json.dumps(manifest, indent=2) + "\n",
    encoding="utf-8",
)
PY
}
