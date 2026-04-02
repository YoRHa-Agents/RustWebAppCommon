#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT/scripts/release-contract.sh"

RWC_VALIDATE_BUILD="${RWC_VALIDATE_BUILD:-1}"
RWC_VALIDATE_COMPARE_SITE="${RWC_VALIDATE_COMPARE_SITE:-1}"
RWC_VALIDATE_INSTALL_DIR="${RWC_VALIDATE_INSTALL_DIR:-}"
RWC_VALIDATE_INSTALL_HOOK="${RWC_VALIDATE_INSTALL_HOOK:-}"

compare_site_contracts() {
  python3 - <<'PY' "$ROOT/site" "$ROOT/release/site"
import json
import sys
from pathlib import Path

repo_site = Path(sys.argv[1])
release_site = Path(sys.argv[2])
targets = [
    "assets/route-manifest.json",
    "assets/theme-tokens.json",
    "assets/runtime-contract.json",
    "docs/docs-index.json",
]

for relative_path in targets:
    left = repo_site / relative_path
    right = release_site / relative_path
    if not left.is_file():
        raise SystemExit(f"missing repo site contract: {left}")
    if not right.is_file():
        raise SystemExit(f"missing release site contract: {right}")
    if json.loads(left.read_text(encoding="utf-8")) != json.loads(
        right.read_text(encoding="utf-8")
    ):
        raise SystemExit(f"site contract drift detected for {relative_path}")

print("Site contract parity: ok")
PY
}

main() {
  local asset_name install_dir cleanup_install_dir post_install_hook
  asset_name="$(rwc_detect_asset_name)"

  if [[ "$RWC_VALIDATE_BUILD" == "1" ]]; then
    bash "$ROOT/scripts/build-release.sh"
  fi

  rwc_validate_local_release_dir "$ROOT/release" "$asset_name"
  echo "Validated release directory for $asset_name"

  if [[ "$RWC_VALIDATE_COMPARE_SITE" == "1" ]]; then
    compare_site_contracts
  fi

  cleanup_install_dir="0"
  if [[ -n "$RWC_VALIDATE_INSTALL_DIR" ]]; then
    install_dir="$RWC_VALIDATE_INSTALL_DIR"
    mkdir -p "$install_dir"
  else
    install_dir="$(mktemp -d)"
    cleanup_install_dir="1"
  fi

  post_install_hook="${RWC_VALIDATE_INSTALL_HOOK:-\"\$RWC_INSTALLED_BINARY\" demo >/dev/null}"

  INSTALL_DIR="$install_dir" \
    LOCAL_RELEASE_DIR="$ROOT/release" \
    RWC_INSTALL_REQUIRE_MANIFEST=1 \
    RWC_POST_INSTALL_HOOK="$post_install_hook" \
    bash "$ROOT/scripts/install.sh"

  LOCAL_RELEASE_DIR="$ROOT/release" \
    RWC_UPDATE_REQUIRE_MANIFEST=1 \
    RWC_UPDATE_REQUIRE_CHECKSUMS=1 \
    bash "$ROOT/scripts/update-check.sh"

  if [[ "$cleanup_install_dir" == "1" ]]; then
    rm -rf "$install_dir"
  fi

  echo "Release validation: ok"
}

main "$@"
