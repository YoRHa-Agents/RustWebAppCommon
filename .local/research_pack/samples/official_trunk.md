# Sample: trunk__static_web_build_contract

## Snapshot
- source_kind: `official_docs`
- source_path_or_url: `https://trunk-rs.github.io/trunk/`
- sample_scope: Trunk 官方站点、docs.rs 配置页与官方仓库说明
- maturity_signal: 官方网站、docs.rs 文档、GitHub 仓库、面向 Rust WASM 的成熟 bundler 定位
- runtime_surface: `web`

## Execution Surfaces
- build_flow: Trunk 官方主页明确其定位是 “Build, bundle & ship your Rust WASM application to the web”，并说明 `trunk build` 会把产物输出到 `dist/`，用于 Web 静态托管。
- release_update_flow: 它更偏前端静态产物构建器，不提供 updater 或桌面级 release 流程；适合作为 demo/web build adapter，而不是全局 release solution。
- deploy_demo_flow: `public_url` 是关键配置字段，用来控制静态资源的公开路径；对 GitHub Pages 或子目录部署尤其重要。
- cli_surface: 官方资料显示 Trunk 有 serve/build 的 CLI 面；搜索到的官方文档摘要指出 `serve` 默认端口为 `8080`，并支持通过 `--port` 与 `--serve-base` 调整本地服务入口。
- routing_surface: Trunk 负责静态资源路径与 dev server，不直接定义业务路由；真正的 client-side routing 仍需 UI/router 框架配合。

## Docs and Agent Onboarding
- docs_index_pattern: 官方站点信息密度较低，更像一个“构建器入口页”，适合被放入 `common_web_adapter` 的工具说明，而不是主 README 的核心叙事。
- agent_onboarding_pattern: 对 agent 而言，Trunk 的价值在于约束静态构建输出、`public_url`、serve 参数和 dist 目录语义。
- style_system: 无视觉主题能力。

## Common Boundary Signals
- suitable_for_common:
  - 适合作为静态 web/demo 适配层的构建约束：`dist/` 输出目录、`public_url`、serve 参数。
  - 适合为后续 `common dev/build demo` 命令提供底层实现参考。
- should_stay_in_app:
  - 路由定义、页面结构、SPA fallback 细节与业务组件仍应保留在应用或具体 web adapter 中。
  - 是否采用 Trunk 不应影响 common core 的配置模型。

## Risks and Cost
- risks_limitations:
  - 现有官方可抓取内容更偏构建器定位和配置摘要，serve 细节抓取不如 Dioxus/Tauri 完整。
  - Trunk 不解决 GitHub Pages 的 `404.html` fallback 和 SPA 路由问题，需要与 host 约束共同设计。
- migration_cost: `medium`
- evidence_strength: `B`

## Quoted Artifacts
- `https://trunk-rs.github.io/trunk/`
- `https://docs.rs/crate/trunk/latest/source/site/content/configuration.md`
- `https://github.com/trunk-rs/trunk`

## Open Questions
- 在 `RustWebAppCommon` 中，Trunk 应该作为默认 web build adapter，还是只作为可替换的静态构建选项？
