# Sample: tauri_v2__desktop_release_and_updater

## Snapshot
- source_kind: `official_docs`
- source_path_or_url: `https://v2.tauri.app/reference/cli/`
- sample_scope: Tauri v2 CLI、updater 插件与 GitHub 发布流水线
- maturity_signal: 官方 v2 文档、CLI reference、updater plugin、GitHub Actions pipeline、跨平台 bundle artifacts
- runtime_surface: `native`

## Execution Surfaces
- build_flow: Tauri CLI 的 `build` 命令会以 release 模式构建应用并生成 bundles/installers；`bundle` 命令可以对已构建应用单独产出安装包。`init` 还显式支持 `frontend-dist`、`dev-url`、`before-dev-command` 和 `before-build-command` 等前后端桥接配置。
- release_update_flow: updater 插件要求签名不可禁用；`signer generate` 生成密钥，`tauri build` 在配置 `createUpdaterArtifacts: true` 后会产出各平台更新 bundle 与 `.sig` 文件。GitHub pipeline 指南明确展示了通过 `tauri-action` 在 GitHub Actions 中构建多平台产物并创建 GitHub release。
- deploy_demo_flow: Tauri 本身不解决 GitHub Pages 静态 demo 托管；它更适合作为 desktop shell / updater / release channel 方案，而不是在线 demo host。
- cli_surface: `tauri build`、`tauri bundle`、`tauri signer generate` 形成了清晰的 release CLI 表面；`init` 对 `dev-url` 的支持说明其天然适合与一个 web demo 或 frontend dev server 拼接。
- routing_surface: 页面路由主要留给前端应用和 `dev-url` 指向的 Web UI，Tauri 本身不提供网页子路由语义。

## Docs and Agent Onboarding
- docs_index_pattern: 官方文档把 CLI、plugin、distribution pipeline 分开讲解，适合作为“桌面分发能力不与 web/demo 混写”的结构参考。
- agent_onboarding_pattern: 对 agent 最重要的是它把构建、签名、更新、GitHub release 这些职责切分得很明确，便于转译为 `common_release_adapter`。
- style_system: 无视觉主题能力，重点在分发、更新与运行时桥接。

## Common Boundary Signals
- suitable_for_common:
  - 适合把“桌面壳层 + 更新 + 签名 + GitHub 发布流水线”建模为可选的 `common_desktop_adapter`。
  - 适合沉淀统一 release metadata、artifact naming 和 updater contract。
- should_stay_in_app:
  - 具体前端页面、`dev-url` 指向的 UI、业务导航和业务配置应留在应用仓库。
  - 是否启用 updater、具体 endpoint、安装模式等环境级配置不应硬编码进 common core。

## Risks and Cost
- risks_limitations:
  - Tauri 对 release/updater 非常强，但对纯网页 demo 托管没有帮助，因此不能单独作为统一范式答案。
  - updater 依赖签名与 endpoint 配置，安全与运维门槛高于静态网页方案。
- migration_cost: `high`
- evidence_strength: `A`

## Quoted Artifacts
- `https://v2.tauri.app/reference/cli/`
- `https://v2.tauri.app/plugin/updater`
- `https://v2.tauri.app/distribute/pipelines/github/`

## Open Questions
- 对 `RustWebAppCommon` 而言，Tauri 应该只作为“可选桌面 adapter”，还是需要进入默认 starter 结构？
