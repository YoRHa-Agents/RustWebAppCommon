# Sample: dioxus__crossplatform_publish_and_routes

## Snapshot
- source_kind: `official_docs`
- source_path_or_url: `https://dioxuslabs.com/learn/0.7/guides/deploy/`
- sample_scope: Dioxus CLI 配置、发布指南与路由定义
- maturity_signal: 官方 versioned docs、CLI、web/desktop/fullstack 路径、专门的 publishing 与 routing 指南
- runtime_surface: `hybrid`

## Execution Surfaces
- build_flow: `Dioxus.toml` 可定义 `default_platform`、`out_dir`、`asset_dir`、`base_path` 与 `sub_package`；`dx bundle --release` 产出打包结果，`dx bundle --out-dir docs` 可直接为 GitHub Pages 生成静态产物。
- release_update_flow: Dioxus 官方 publishing 指南覆盖 Web 发布与 Desktop bundling，桌面端通过 `cargo install dioxus-cli` 后运行 `dx bundle --release`，产物位于 `dist/bundle/`。
- deploy_demo_flow: 官方明确支持 GitHub Pages：设置 `base_path = "your_repo"`，运行 `dx bundle --out-dir docs`，再把 `docs/public/*` 移到 `docs/`，并复制 `docs/index.html` 为 `docs/404.html` 以支持 client-side routing。
- cli_surface: `dx` 提供统一 CLI 表面，既覆盖 web 构建/发布，也覆盖 desktop bundling。
- routing_surface: `Routable` 支持静态段、动态段、查询段、hash 和 nested routes；`#[nest(\"/path\")]` 能组织子页面层级。

## Docs and Agent Onboarding
- docs_index_pattern: versioned docs 将 tutorial、guides、essentials/router、deploy 分层组织，适合作为统一框架的“从入门到部署”型入口。
- agent_onboarding_pattern: 对 agent 最有价值的是 `Dioxus.toml` 的显式配置面与 publishing guide 的顺序化步骤，可被转化为后续 common adapter 的 contract。
- style_system: 官方文档并不强调视觉主题，但提供了把 `style` / `script` 资源放入 `web.resource` 的机制，说明文档层与资源层可以分离。

## Common Boundary Signals
- suitable_for_common:
  - 适合作为 `common_web_adapter` 或 `common_ui_adapter` 的参考：统一 `base_path`、`out_dir`、资产目录与 GitHub Pages 发布约束。
  - 适合作为 route descriptor 与页面层级结构的参考，特别是 nested routes 和 query/hash segment 的描述方式。
- should_stay_in_app:
  - 实际页面组件、业务 route variant、页面内容与交互逻辑仍应保留在应用仓库。
  - 是否选择 Dioxus 作为统一 UI runtime，不应在没有 starter repo 验证前直接锁死。

## Risks and Cost
- risks_limitations:
  - Dioxus 同时覆盖 web/desktop，但其 bundling 与桌面发布并不等于通用 release/update 体系，仍需和独立 release pipeline 配合。
  - GitHub Pages 方案依赖 `base_path` 与 `404.html` 复制等特定静态托管约束，迁移到其他 host 时需要重新适配。
- migration_cost: `medium`
- evidence_strength: `A`

## Quoted Artifacts
- `https://dioxuslabs.com/learn/0.4/CLI/configure/`
- `https://dioxuslabs.com/learn/0.7/guides/deploy/`
- `https://dioxuslabs.com/learn/0.7/essentials/router/routes/`
- `https://dioxuslabs.com/learn/0.6/router/reference/routes/nested`

## Open Questions
- 如果最终不采用 Dioxus 作为统一 UI 层，哪些 `base_path`、route descriptor 和 static export 约束仍值得保留在 `RustWebAppCommon` 的 adapter contract 中？
