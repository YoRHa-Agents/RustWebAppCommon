# Sample: contextos__layered_platform_docs

## Snapshot
- source_kind: `local_repo`
- source_path_or_url: `/home/agent/reference/ContextOS`
- sample_scope: 顶层 README、wiki home 与 architecture 分层说明
- maturity_signal: README、wiki 入口、分层架构文档、能力矩阵、CLI/MCP 统一叙事
- runtime_surface: `hybrid`

## Execution Surfaces
- build_flow: 顶层文档强调“单次安装、统一能力平台”的产品叙事，但不提供 Rust/Web 工程构建链。
- release_update_flow: README 与 wiki 体现版本化叙事和平台能力演进，但没有给出适合本项目直接复用的 release/update 实现细节。
- deploy_demo_flow: 没有 GitHub Pages 或网页 demo 的直接托管方案。
- cli_surface: README 明确把 CLI 与 MCP 作为统一入口讲述，这对 `RustWebAppCommon` 的“统一命令入口 + 统一能力层”非常有借鉴价值。
- routing_surface: 没有网页路由或页面结构的直接证据。

## Docs and Agent Onboarding
- docs_index_pattern: `README + wiki/Home + Architecture` 的双入口模式，README 负责愿景与能力矩阵，wiki 负责导航与细分主题。
- agent_onboarding_pattern: wiki Home 通过导航表和 quick links 把复杂体系压缩成低上下文入口，适合作为 common 文档入口的结构参考。
- style_system: 偏平台叙事和表格密集型说明，不是视觉主题样本。

## Common Boundary Signals
- suitable_for_common:
  - “层级化能力地图”非常适合沉淀到 common 的顶层 README/index。
  - 统一 CLI/MCP 入口叙事适合迁移为 `RustWebAppCommon` 的命令入口与文档入口 framing。
- should_stay_in_app:
  - 具体页面、demo 内容和业务路由不应被这种平台文档框架吸收进 common。
  - 如果 wiki 导航引用的页面不完整，应用层不能依赖这种未验证索引。

## Risks and Cost
- risks_limitations:
  - 文档导航中的某些页面链接未必都实际存在，说明“目录看起来完整”不等于真正可维护。
  - 它更强于平台叙事，不强于具体 Web/demo 实践，因此不能单独支撑本项目的 runtime 决策。
- migration_cost: `medium`，因为可迁移的是“平台分层表达法”，而不是实现本身。
- evidence_strength: `A`

## Quoted Artifacts
- `/home/agent/reference/ContextOS/README.md`
- `/home/agent/reference/ContextOS/docs/wiki/Home.md`
- `/home/agent/reference/ContextOS/docs/wiki/Architecture.md`

## Open Questions
- `RustWebAppCommon` 的顶层文档是否也应该拆成“愿景 README”与“结构化 wiki/index”两层，而不是只维护一个 README？
