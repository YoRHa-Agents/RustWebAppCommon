# Docs Index

## Start Here
- `README.md`：人类入口与仓库现状
- `AGENTS.md`：agent 先读顺序与仓库边界
- `demo/storyboard.md`：静态 demo 与本地运行面的页面地图

## Choose Your Path
### Human
- `README.md`
- `docs/architecture/overview.md`
- `docs/guides/review.md`
- `docs/guides/release.md`

### Main Agent
- `AGENTS.md`
- `docs/architecture/core-contracts.md`
- `docs/guides/dev.md`
- `docs/guides/review.md`

### Subagent
- `docs/architecture/adapters.md`
- `docs/theme/nier_gray_tokens.md`
- `examples/README.md`

## Core Navigation
- `docs/architecture/overview.md`
- `docs/architecture/core-contracts.md`
- `docs/architecture/adapters.md`
- `docs/guides/dev.md`
- `docs/guides/review.md`
- `docs/guides/release.md`
- `docs/theme/nier_gray_tokens.md`
- `demo/storyboard.md`
- `examples/README.md`

## Current Decisions
- docs 与 demo 共用同一套术语：`common_core`、`common_adapters`、`app_owned`
- 页面地图以 `RouteDescriptor` 与 `site/assets/runtime-contract.json` 为准
- 默认 web runtime 为生成后的静态站点 + adapter HTTP shell
- 默认 desktop preview 路线为 browser-backed preview，并保留 `desktop_tauri_adapter` 作为长期 seam
- readonly remote review 继续停留在 adapter / CLI，不把 SSH provider、session 或远程路径状态写回 `common_core`
- `common review` 是远程审阅的统一 CLI 入口：支持 `--list-hosts`、默认目录自动发现与显式 `--path`
