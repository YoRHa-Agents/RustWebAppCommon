# Remote Review Guide

## Goal
Define the shared readonly review flow that starts from local `~/.ssh/config`
and ends with a generic remote doc/design summary.

## Command Entry
```bash
cargo run -p common_cli -- review --list-hosts
cargo run -p common_cli -- review --list-hosts --config /tmp/review-ssh
cargo run -p common_cli -- review --ssh-host review-host
cargo run -p common_cli -- review --ssh-host review-host --config /tmp/review-ssh --path /srv/reviews
```

## Shared Behavior
- `common review` reads local `~/.ssh/config` by default and accepts `--config`
  for fixture or override paths.
- `review --list-hosts` prints normalized SSH host aliases plus any skipped
  entries that could not be resolved into a concrete review target.
- `review --ssh-host <alias>` defaults to readonly discovery of `doc/`,
  `docs/`, `design/`, and `designs/`.
- Repeated `--path` switches move the command into explicit-path mode and avoid
  hard-coding a single downstream repo layout.
- Current reviewable file kinds are Markdown, HTML, and image assets.

## Boundary Rules
- SSH config parsing, host alias normalization, remote directory discovery, and
  file classification stay in `common/adapters`.
- CLI output and argument parsing stay in `common/cli`.
- `common_core` does not receive SSH host, credential, provider, session, or
  remote path state semantics.
- Remote write, sync, deploy, conflict handling, and product review workflows
  remain `app_owned`.

## Output Contract
- Host listing prints the resolved SSH config path, host count, normalized host
  entries, and skipped aliases with explicit error messages.
- Remote review prints the selected host alias, discovery mode, directory
  statuses, and per-file summary lines using shared file-kind labels.

## Verification
```bash
cargo test
cargo test -p common_cli --test cli_harness
python -m unittest tests.test_research_pack tests.test_starter_repo
```

- Use `RWC_REMOTE_REVIEW_FIXTURE_ROOT` to run deterministic smoke tests without
  relying on a live SSH target.
- Treat real SSH transport as an opt-in manual verification path, not a default
  CI requirement.
