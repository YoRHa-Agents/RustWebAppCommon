# Rust Web Common 对比矩阵与归纳模板
> 更新时间: 2026-04-01

## 目标
本矩阵用于回答两个核心问题：
- 哪些能力应该沉淀在 `RustWebAppCommon`。
- 哪些能力必须留在各应用仓库，或者仅以扩展点形式暴露。

所有候选样本都必须先按 `02_evidence_schema.md` 记录后，才允许进入本矩阵。

## 矩阵列定义
| 列名 | 记录内容 | 决策用途 |
|---|---|---|
| `candidate` | 候选样本名称 | 用于追踪来源 |
| `family` | 所属类别 | 区分工程层、demo 层、文档层 |
| `runtime_surface` | `native` / `web` / `hybrid` / `docs` | 判断是否适合统一底座 |
| `build_release` | 构建与发布核心模式 | 判断 common 是否应统一构建编排 |
| `deploy_demo` | demo / 部署承载模式 | 判断是否应抽象统一演示能力 |
| `cli_routing` | CLI、`host` / `port`、页面路由线索 | 判断命令入口与页面入口该放在哪层 |
| `docs_index` | README / index / quick start / examples 组织法 | 判断 agent 接入结构如何复用 |
| `style_system` | 主题与视觉约束形态 | 判断风格应沉淀在 docs 层还是应用层 |
| `common_candidate` | 适合下沉到 common 的能力 | 做正向归纳 |
| `app_candidate` | 应留在应用仓库的能力 | 防止过度抽象 |
| `migration_cost` | 迁移与接入成本 | 排序落地优先级 |
| `evidence_strength` | `A` / `B` / `C` | 控制结论可信度 |
| `verdict` | `adopt` / `adapt` / `reject` / `baseline_only` | 形成最终建议 |

## 基线矩阵
下表是当前本地可验证样本的“基线行”。它们主要用于提供文档入口、分层叙事和 CLI/平台统一表达的线索，不代表已经找到 Rust Web 直接可用答案。

| candidate | family | runtime_surface | build_release | deploy_demo | cli_routing | docs_index | style_system | common_candidate | app_candidate | migration_cost | evidence_strength | verdict |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `openai-agents-python` | agent framework / docs | `docs` | 示例运行器与分主题 examples 结构清晰，但不是 Rust build/release 样本 | 无直接 demo 托管结论 | 有示例运行入口，但没有针对 `host` / `port` 的统一网页路由约束 | `README + 分主题 README + run_examples.py` 三段式入口 | 弱，未形成独立主题系统 | 文档入口分层、样本导航、运行器式索引 | 具体 Rust/Web 运行时与页面路由 | `medium` | `A` | `baseline_only` |
| `agentic-context-engine` | learning framework / docs | `docs` | quick start + integrations 叙事强，工程分层有借鉴意义 | 有 browser-use 示例，但不是 Rust Web demo 托管样本 | 无统一网页启动入口结论 | `examples/README + quick start + docs` 渐进式入口 | 弱，主题不是重点 | 渐进式 onboarding、分层 examples、低上下文导航 | Rust 构建/发布与页面入口 | `medium` | `A` | `baseline_only` |
| `ContextOS` | context platform / architecture | `hybrid` | 有平台级分层和 CLI/MCP 一体化叙事，但缺 Rust Web 工程细节 | 无直接 GitHub Pages/demo 结论 | 统一 CLI/MCP 入口叙事强，可作为 common 表达样本 | `README + wiki/Architecture` 分层说明清晰 | 弱，风格不是核心 | 平台级分层、统一入口叙事、能力分层表达 | 具体页面路由、静态 demo 和前端主题实现 | `medium` | `A` | `baseline_only` |
| `Dioxus` | rust ui / crossplatform | `hybrid` | `Dioxus.toml` 显式定义 `default_platform`、`out_dir`、`base_path`，`dx bundle --release` 产出 desktop/web 构建结果 | 官方 publishing 明确 GitHub Pages 路径、`docs` 输出和 `404.html` fallback | `dx` 统一 CLI 覆盖 bundling；router 支持 nested/dynamic/query/hash routes | versioned docs + guides + router + deploy 分层 | 可加载样式资源，但主题系统需自建 | `base_path` 合约、route descriptor 语义、web adapter 约束 | 具体页面组件、业务路由内容、最终 UI runtime 选择 | `medium` | `A` | `adapt` |
| `Tauri v2` | desktop shell / release | `native` | `tauri build` 生成 bundles/installers，`tauri bundle` 负责安装包产物，CLI 同时提供 `dev-url` 等桥接参数 | updater、签名、GitHub release pipeline 成熟，但不负责静态网页 demo 托管 | `tauri build`、`tauri bundle`、`tauri signer generate` 构成稳定 release CLI 面 | CLI/plugin/distribute 文档分层清晰 | 无 | release metadata、签名/updater 合约、desktop adapter | 前端页面、业务路由、demo 页面内容 | `high` | `A` | `adapt` |
| `Trunk` | rust wasm bundler | `web` | `trunk build` 输出 `dist/`，`public_url` 控制静态资源路径 | 适合静态 web/demo 构建，不覆盖 updater 或桌面分发 | serve/build CLI 明确，但 host/port 细节证据弱于 Dioxus/Tauri | 站点入口偏工具型说明 | 无 | 静态 web build adapter、`dist/` 与 `public_url` 约束 | SPA fallback、业务路由与 host 策略 | `medium` | `B` | `adapt` |
| `GitHub Pages` | static host | `demo_only` | 不构建应用，只发布静态文件；支持 branch 根目录、`/docs` 或 GitHub Actions artifact | 静态 docs/demo 托管强，但要求顶层 `index.html`；不存在的路径默认 404，可用 `404.html` 做 fallback | 无 | host contract 型文档，而非应用文档结构 | 无 | `/docs` 发布约束、Actions deploy、`index.html`/`404.html` 契约 | 服务端逻辑、runtime 构建、桌面分发与动态更新 | `low` | `A` | `adapt` |

## 待填充矩阵模板
后续新样本进入矩阵时，直接复制以下空白行并补全：

| candidate | family | runtime_surface | build_release | deploy_demo | cli_routing | docs_index | style_system | common_candidate | app_candidate | migration_cost | evidence_strength | verdict |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `<name>` | `<family>` | `<surface>` | `<summary>` | `<summary>` | `<summary>` | `<summary>` | `<summary>` | `<what fits common>` | `<what stays app>` | `<low/medium/high>` | `<A/B/C>` | `<adopt/adapt/reject/baseline_only>` |

## 当前阈值状态
| 主线 | 当前状态 | 判断 |
|---|---|---|
| A. Common 工程与运行时 | `Dioxus` 与 `Tauri v2` 都提供了 `build/release/CLI` 直接证据 | 已满足最低样本数量，但仍需 starter repo 验证 |
| B. Web/demo 承载与分发 | `Dioxus` 与 `GitHub Pages` 组合回答了 GitHub Pages、`404.html` fallback、静态托管与路由边界；`Trunk` 补充了 `public_url` 和静态 build 约束 | 已满足最低问题覆盖 |
| C. Agent 文档与风格系统 | 3 个本地 baseline 已满足 docs/index 模式比较，但“可落地 style framework”仍待单独产出 | 尚未完全满足，需在 WP-3 补齐 |

## 模式归纳模板
每完成一类候选样本对比后，必须额外产出一张归纳卡，而不是只保留矩阵行：

```md
# Pattern: <pattern_name>

## Solves
- 这个模式解决什么问题

## Preconditions
- 采用它之前必须满足的前提

## What Belongs in Common
- ...

## What Must Stay in App
- ...

## Failure Modes
- ...
- ...

## Migration Cost
- low|medium|high
- 成本原因

## Recommendation
- adopt | adapt | reject
- 适用范围
```

## 最低证据门槛
在输出最终建议前，至少满足以下条件：
- 主线 A 至少有 2 个 `A` 级样本，且都包含 `build_flow + release_update_flow + cli_surface` 的直接证据。
- 主线 B 至少有 2 个样本覆盖 `deploy_demo_flow + routing_surface`，其中至少 1 个能正面回答 GitHub Pages 的适用边界。
- 主线 C 至少有 3 种不同的 `docs_index_pattern`，且至少 1 套可落地到 markdown 的风格约束框架。
- 至少有 1 个样本被明确标记为 `reject` 或 `adapt`，防止研究只收集正面样本。

## 决策规则
- 如果一项能力对所有应用都稳定存在，且不依赖具体页面内容，则优先考虑下沉到 `common`。
- 如果一项能力强依赖页面结构、业务路由、业务数据源或展示文案，则默认留在应用仓库。
- 如果一项能力需要统一接口但实现会随托管方式变化，则优先设计成 `common` 中的适配层，而不是单一实现。
- 如果一项能力只为 demo 服务，不直接影响公共运行时，则优先放在 demo/storyboard 层，而不是 common 核心层。
