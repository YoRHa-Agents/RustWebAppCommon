# Enva Gap Requirements

## Goal
This file records the reusable capabilities that Enva needs from
`RustWebAppCommon` during a common-based reimplementation. It is intentionally
limited to changes that belong in adapters, scripts, workflows, shared test
scaffolds, or shared documentation contracts.

## Already Reusable Today
- Three-layer boundary: `common_core`, `common_adapters`, `app_owned`
- Static `site/` output and GitHub Pages deployment workflow
- Release bundle structure, checksum generation, installer, and update-check
  scripts
- Shared docs/demo front door patterns
- Basic static-site regression tests and route/runtime manifest generation
- Shared release-contract validation for asset naming, manifest schema, and
  release-dir checks
- Common-side release smoke entry: `scripts/validate-release.sh`
- Shared install post-flight seam via `RWC_POST_INSTALL_HOOK`
- Shared CLI subprocess and mock-release-API scaffolding in `tests/`

## Requirements To Extend In Common

### 1. Multi-platform release automation
Common should offer a reusable pattern for multi-platform build and release
contracts instead of only the current host-platform release bundle.

Priority:
- `should_now`

Current common status:
- Asset naming is now centralized in `scripts/release-contract.sh` and can be
  checked for `linux-x86_64`, `linux-aarch64`, `macos-aarch64`, and
  `macos-x86_64`.
- CI can validate platform naming/workflow expectations without pretending to
  produce non-host binaries from `build-release.sh`.

Acceptance:
- Shared scripts or workflow helpers can build or orchestrate Linux x86_64,
  Linux aarch64, and macOS aarch64 artifacts.
- Asset naming stays centralized in one contract file.
- Downstream products can reuse the workflow without re-deriving platform names.

Suggested placement:
- `scripts/`
- `.github/workflows/`
- `doc_auto/`

### 2. Install post-flight smoke hook
Common should provide a standard post-install verification seam so products can
run a product-specific smoke command after installation.

Priority:
- `must_now`

Current common status:
- `scripts/install.sh` now supports `RWC_POST_INSTALL_HOOK`.
- Hook failures return a clear non-zero exit code.
- The default behavior remains generic when no hook is configured.

Acceptance:
- Installer supports a configurable verification command or hook.
- Hook failure produces a clear non-zero exit.
- Default behavior stays generic for starter repos that do not need product
  validation.

Suggested placement:
- `scripts/install.sh`
- `docs/guides/release.md`

### 3. Updater seam beyond update-check
Common currently provides discovery and checksum contracts, but Enva also needs
an explicit extension seam for product updaters that perform binary replacement
or richer verification.

Priority:
- `must_now`

Current common status:
- `scripts/update-check.sh` now reports checksum / manifest presence and a
  migration-readiness summary.
- `release-manifest.json` now carries generic validation capabilities without
  introducing product behavior into `common_core`.
- Binary replacement remains delegated to downstream adapters or product scripts.

Acceptance:
- Common documents a supported adapter/script seam for in-process or delegated
  updater implementations.
- Shared release metadata can carry digest or manifest information without
  hard-coding product behavior into `common_core`.
- Products can plug in binary replacement logic without duplicating the entire
  release contract.

Suggested placement:
- `common/adapters/`
- `scripts/`
- `docs/guides/release.md`

### 4. Dual-surface parity scaffolding
Enva needs embedded UI and static demo parity checks. Common already knows how
to generate `site/`, but it does not yet provide reusable scaffolding for
products that must keep an embedded runtime surface and a Pages demo in sync.

Priority:
- `must_now`

Current common status:
- `scripts/validate-release.sh` checks that repo `site/` and bundled
  `release/site/` share the same route/theme/runtime/docs manifests.
- `tests/test_enva_migration_validation.py` treats this parity check as part of
  the common-side migration smoke baseline.

Acceptance:
- Shared test helpers can assert required DOM hooks or manifest markers across
  more than one runtime surface.
- Common docs explain when a product should use one generated `site/` versus a
  split embedded/static model.
- The shared approach does not force Enva's product-specific route names into
  common contracts.

Suggested placement:
- `tests/`
- `doc_auto/`
- `docs/architecture/`

### 5. CLI integration test scaffolding
Enva relies on subprocess tests, mock release APIs, and end-to-end CLI command
assertions. Common should expose patterns or helpers for products that need more
than static-site contract tests.

Priority:
- `must_now`

Current common status:
- `common/cli/tests/cli_harness.rs` launches the real `common` binary as a
  subprocess.
- `tests/common_side_harness.py` provides temp directory, subprocess, and mock
  release-API helpers for downstream validation patterns.

Acceptance:
- A downstream product can reuse a documented harness pattern for binary launch,
  fixture setup, temp workspace creation, and mock HTTP servers.
- Common keeps the harness generic instead of encoding product command words.
- Docs point downstream repos to the recommended scaffold.

Suggested placement:
- `tests/`
- `docs/`
- `examples/`

### 6. Readonly remote doc/design review seam
Enva needs one reusable seam for reading local `~/.ssh/config` and reviewing
remote `doc/`, `docs/`, `design/`, or `designs/` directories without moving SSH
provider or session semantics into `common_core`.

Priority:
- `must_now`

Current common status:
- Common now owns an adapter-local readonly remote review seam in
  `common/adapters/src/remote_review.rs`: local `~/.ssh/config` parsing, host
  alias normalization, `Include` expansion, and readonly remote directory
  discovery.
- `common review --list-hosts` and `common review --ssh-host ... [--path ...]`
  now provide the shared CLI entry for host listing and readonly review
  summaries.
- Docs/demo and fixture-backed smoke tests now cover the generic flow `local
  SSH config -> remote host -> readonly review summary` without moving SSH
  provider, session, or remote mutation semantics into `common_core`.

Acceptance:
- Common keeps adapter-local parsing for `Host`, `HostName`, `User`, `Port`,
  `IdentityFile`, and common `Include` patterns from local `~/.ssh/config`.
- Common keeps a generic CLI entry that lists normalized SSH hosts and performs
  a readonly review listing for remote doc/design directories.
- Common docs/demo keep explaining the generic flow `local SSH config -> remote
  host -> readonly review summary` without introducing product-owned command
  words.
- Remote write, sync, deploy, conflict handling, session state, and
  product-specific review workflows remain app-owned.

Suggested placement:
- `common/adapters/`
- `common/cli/`
- `docs/`
- `tests/`
- `doc_auto/`

## Must Stay App-Owned In Enva
- Vault crypto, KDF/HMAC policy, vault format, migration, and persistence
- Secret/app domain model and app injection semantics
- Enva CLI words and user-facing error/exit code semantics
- Axum routes, login/session flow, remote SSH write/sync/deploy behavior, and
  product-specific review workflows beyond the readonly common seam
- Conflict resolution rules for deploy/sync and merge preferences

## Explicit Non-Goals For Common
- Do not move Enva vault, SSH provider/session semantics, or remote path state
  into `common_core`.
- Do not copy Enva command names into the common CLI.
- Do not implement remote write, sync, deploy, conflict handling, or product
  review semantics in common adapters or CLI.
- Do not introduce repo-specific GitHub URLs or product constants into
  `common_core`.

## Impact On Enva
- Enva can reuse the common base for structure, release plumbing, site/deploy
  patterns, and adapter boundaries.
- Enva still owns product logic and can adopt common extensions incrementally
  instead of blocking on a full common rewrite.

## Last Updated
- 2026-04-02T09:50:57+00:00
