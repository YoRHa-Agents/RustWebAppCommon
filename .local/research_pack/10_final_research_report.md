# RustWebAppCommon 正式研究报告
> 更新时间: 2026-04-01

## 1. 执行摘要
本次调研的结论不是“选择某一个框架统治一切”，而是建议把 `RustWebAppCommon` 建成一个**协议层 + 适配层 + 应用层**分明的公共底座：
- 协议层负责统一 CLI、配置、路由描述、文档入口和主题 token。
- 适配层负责 Web demo、GitHub Pages、桌面壳层、release pipeline 等不同承载面。
- 应用层保留页面内容、业务路由、业务逻辑和 demo story。

当前证据已经足以支持**调研收尾、方案设计和 demo 蓝图设计**；但在真正进入实现前，仍需要 starter repo 验证与 `Enva` 约束复核。

## 2. 研究边界与证据门槛
### 本轮纳入范围
- Rust/common 工程能力
- Web/demo 承载与 GitHub Pages 约束
- agent 友好文档入口
- Nier 黑白灰风格系统
- common/app 边界与 demo blueprint

### 本轮排除范围
- 真实工程实现
- `Enva` 读取与本地兼容性验证
- 完整 starter repo 建设
- 真实 release pipeline 与 updater 服务部署

### 证据门槛状态
| 主线 | 状态 | 说明 |
|---|---|---|
| A. Common 工程与运行时 | 达标 | `Dioxus` 与 `Tauri v2` 提供了 `build/release/CLI` 级直接证据 |
| B. Web/demo 承载与分发 | 达标 | `Dioxus` 与 `GitHub Pages` 已覆盖 GH Pages、`404.html` fallback、静态发布边界，`Trunk` 补充静态 build 约束 |
| C. Agent 文档与风格系统 | 达标 | 3 个本地 baseline 完成 docs/index 模式比较，且已产出风格 token 草案 |

## 3. 样本总览
| 样本 | 类型 | 结论标签 | 主要贡献 |
|---|---|---|---|
| `openai-agents-python` | 本地 baseline | `baseline_only` | docs/index、pattern catalog、examples runner |
| `agentic-context-engine` | 本地 baseline | `baseline_only` | examples hub、AGENTS 契约、设计文档锚点 |
| `ContextOS` | 本地 baseline | `baseline_only` | README/wiki 分层叙事、平台能力地图 |
| `Dioxus` | 官方外部 | `adapt` | `base_path`、GitHub Pages、nested routes、crossplatform bundling |
| `Tauri v2` | 官方外部 | `adapt` | desktop release、signing、updater、GitHub release pipeline |
| `Trunk` | 官方外部 | `adapt` | `dist/`、`public_url`、静态 build contract |
| `GitHub Pages` | 官方外部 | `adapt` | `/docs` 发布、`index.html` 顶层入口、`404.html` fallback |

## 4. 对比矩阵结论
### 结论 1
`common` 不应直接拥有页面内容与业务路由，但必须拥有**描述它们的公共协议**。

### 结论 2
静态 demo 与桌面 release 是两种不同承载面：
- GitHub Pages 适合静态 docs/storyboard/最小 Web demo
- Tauri 适合桌面壳层、安装包、updater 和 release

### 结论 3
`README + AGENTS + docs/index + examples/README` 是最值得迁移到 common 的文档入口骨架。

### 结论 4
风格系统应该先收敛为 token 与 markdown 语法，而不是先绑定某个 CSS 实现。

## 5. Recommendation Summary
- adopt_now:
  - `common_core / common_adapters / app_owned` 三层模型
  - 统一 CLI 语义：`dev / demo / docs / release`
  - 统一文档入口结构：`README + AGENTS + docs/index + examples/README`
  - 统一 GitHub Pages 静态 host contract
- adapt_before_adopt:
  - Dioxus 的 `base_path`、nested routes、static export 语义
  - Tauri 的签名、updater、GitHub release pipeline
  - Trunk 的 `public_url` 与静态 build 约束
- keep_in_app:
  - 页面组件、业务路由、业务数据与 story 内容
  - demo 的领域文案与专用展示逻辑
- postpone:
  - 最终 starter repo 技术栈
  - `Enva` 兼容性收敛前的实现细节
  - `doc_auto` 的真实同步方案

## 6. Common Boundary
### common_core
- 配置模型
- CLI 协议
- 路由描述模型
- docs/index schema
- 主题 token
- release metadata

### common_adapters
- `web_demo_adapter`
- `docs_site_adapter`
- `desktop_tauri_adapter`
- `release_pipeline_adapter`

### app_owned
- 页面与组件
- 业务路由
- 业务数据源
- demo story / showcase 内容

## 7. Agent 文档与索引建议
- 顶层 README 只保留“是什么、做什么、从哪开始”。
- `AGENTS.md` 专门服务 agent，说明先读路径、约束和边界。
- `docs/index.md` 负责系统性导航。
- `examples/README.md` 和 `demo/storyboard.md` 服务可视化与样例入口。

## 8. 风格系统建议
推荐采用黑、灰、暖白为主的单色调体系，并将其沉淀为：
- color token
- typography token
- markdown component mapping
- 页面区块语法

具体 token 与组件映射见 `07_docs_index_and_style_recommendation.md`。

## 9. Web/demo 蓝图
推荐 demo 至少包含以下页面：
- Landing
- RuntimeMap
- DocsEntry
- ReleaseFlow
- StyleLab
- StoryDetail

推荐在线托管策略：
- GitHub Pages 承载静态文档、storyboard 与最小 web demo
- 对 client-side routing 使用 `404.html` fallback
- 所有静态资源路径必须遵守 `base_path` / `public_url`

## 10. 风险、限制与待验证项
- 当前没有 starter repo，所有架构结论仍缺一次真实 repo 验证。
- `Tauri` 与 `Dioxus` 都强，但不应在未验证前直接成为唯一实现。
- GitHub Pages 是静态 host，不应被误当作完整 demo backend。
- `Enva` 尚未纳入本轮，因此实现前必须进行一次约束复核。
- `doc_auto` 尚不存在，因此它不是研究 blocker，但可能是实现阶段的文档治理 blocker。

## 11. Evidence Notes
- strongest_support:
  - Dioxus 对 `base_path`、GitHub Pages 与 nested routes 的直接说明
  - Tauri 对 build/signing/updater/release 的完整路径
  - GitHub Pages 对 `/docs`、`index.html` 与 `404.html` 的 host contract
- weakest_area:
  - 真实 starter repo 下的综合验证
  - Trunk serve 参数的官方细页证据完整度
- missing_samples:
  - 与 `Enva` 兼容的本地项目样本
  - 一个真实的 Rust Web starter repo 对照

## 12. 后续执行路线
1. 输出设计稿，锁定 `common_core / common_adapters / app_owned`。
2. 输出 demo storyboard 与最小静态展示结构。
3. 使用单独流程复核 `Enva` 约束。
4. 再进入实现和 starter repo 验证。
