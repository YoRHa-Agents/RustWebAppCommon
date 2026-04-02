# Sample: github_pages__static_docs_and_demo_host

## Snapshot
- source_kind: `official_docs`
- source_path_or_url: `https://docs.github.com/en/pages/getting-started-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site`
- sample_scope: GitHub Pages 发布源、`index.html` 顶层入口与自定义 `404.html`
- maturity_signal: 官方 GitHub Docs、branch/docs 发布模式、GitHub Actions 部署模式、404 troubleshooting
- runtime_surface: `demo_only`

## Execution Surfaces
- build_flow: GitHub Pages 自身不构建 Rust/Web 应用；它只负责发布已经生成好的静态文件。官方建议如果需要自定义构建过程，应使用 GitHub Actions workflow 上传静态产物并部署。
- release_update_flow: 适合 docs/demo 的静态分发，但不适合作为桌面安装包或 updater 渠道。
- deploy_demo_flow: 官方支持从任意 branch 的根目录或 `/docs` 目录发布，也支持通过 GitHub Actions 发布。对 `RustWebAppCommon`，这意味着静态 demo 和文档非常适合统一落在 `/docs` 或 Actions artifact 中。
- cli_surface: 无 CLI；需要外部构建器把最终静态文件送到 Pages 可部署的目录。
- routing_surface: Pages 是静态文件 host，要求发布源顶层存在 `index.html`。对于不存在的路径，默认返回 404；官方支持 `404.html` 作为自定义错误页，这与 Dioxus 的“复制 `index.html` 为 `404.html`”形成了可操作的 SPA fallback 组合。

## Docs and Agent Onboarding
- docs_index_pattern: 官方把发布源、Actions 工作流、404 troubleshooting 拆成独立文档页面，说明“host 规则”应该独立于业务页面设计记录。
- agent_onboarding_pattern: 对 agent 来说，最关键的是它提供了明确的 host contract：`/docs`、`index.html` 顶层入口、Actions artifact 与 `404.html` fallback。
- style_system: 无主题系统，完全依赖上游静态产物。

## Common Boundary Signals
- suitable_for_common:
  - 适合作为 `common_demo_host_contract`：规范 `/docs` 目录、Actions 发布、`index.html` 顶层入口与 `404.html` fallback。
  - 适合承载统一文档站、静态 demo storyboard、可浏览的样式说明。
- should_stay_in_app:
  - 具体页面内容、SPA router 策略、路径结构和静态资源组织仍应由 web/demo 适配层或应用仓库决定。
  - GitHub Pages 不能决定 runtime 构建或 updater 流程。

## Risks and Cost
- risks_limitations:
  - GitHub Pages 是静态 host，不提供服务端逻辑、动态 API 或桌面分发能力。
  - 若没有正确处理 `base_path`、`index.html` 顶层入口和 `404.html`，client-side routing 很容易失效。
- migration_cost: `low`
- evidence_strength: `A`

## Quoted Artifacts
- `https://docs.github.com/en/pages/getting-started-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site`
- `https://docs.github.com/en/pages/getting-started-with-github-pages/troubleshooting-404-errors-for-github-pages-sites`
- `https://docs.github.com/en/pages/getting-started-with-github-pages/creating-a-custom-404-page-for-your-github-pages-site`

## Open Questions
- 对未来的 `RustWebAppCommon` demo 而言，GitHub Pages 应只承载静态 storyboard 和 docs，还是也承载最小可运行 web demo？
