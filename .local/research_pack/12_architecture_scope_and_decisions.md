# RustWebAppCommon 架构范围与决策边界
> 更新时间: 2026-04-01

## 目标
本文件用于锁定本轮架构设计必须回答的问题，并明确哪些内容属于后续 starter repo 验证、`Enva` 复核或实现阶段，而不是在架构设计稿中假装已经解决。

## 本轮必须锁定的内容
- `common_core / common_adapters / app_owned` 的分层边界。
- `common_core` 应提供的协议面：配置模型、CLI 协议、路由描述、docs/index schema、theme token、release metadata。
- adapter 层的最小集合与职责边界。
- 文档入口与 demo 入口的统一信息架构。
- GitHub Pages、静态 demo 与 desktop release 的分层关系。
- 设计稿中的 failure modes、迁移成本与验证清单。

## 本轮明确不锁定的内容
- 最终 starter repo 技术栈。
- 最终 Web UI runtime 是否选 Dioxus、Trunk 或其他方案。
- 最终 desktop adapter 是否默认采用 Tauri。
- `Enva` 的兼容性结论。
- `doc_auto` 的自动同步实现。
- 任何运行代码、打包脚本、release pipeline 或 demo 页面实现。

## 决策状态表
| 主题 | 状态 | 当前结论 | 后续验证 |
|---|---|---|---|
| 分层模型 | `locked_now` | 采用 `common_core / common_adapters / app_owned` | starter repo 验证 |
| CLI 语义 | `locked_now` | 统一 `dev / demo / docs / release` | `Enva` 命名兼容 |
| docs/index 结构 | `locked_now` | 采用 `README + AGENTS + docs/index + examples/README + demo/storyboard` | 真实仓库落位验证 |
| GitHub Pages contract | `locked_now` | `/docs`、顶层 `index.html`、必要时 `404.html` fallback | 静态 demo 验证 |
| theme token 方向 | `locked_now` | 采用 Nier 黑白灰 token + markdown component mapping | UI/文档实现验证 |
| Dioxus | `adapt_before_adopt` | 可作为 web/demo adapter 候选 | starter repo 验证 |
| Tauri | `adapt_before_adopt` | 可作为 desktop/release adapter 候选 | starter repo 与 release 验证 |
| Trunk | `adapt_before_adopt` | 可作为静态 web build adapter 候选 | starter repo 验证 |
| `Enva` 兼容性 | `blocked_later` | 当前不读取、不裁决 | 单独复核 |
| `doc_auto` | `blocked_later` | 当前只定义责任，不定义自动化实现 | 实现阶段决策 |

## 设计成功标准
- 设计稿能明确说明哪些能力属于协议层，哪些属于适配层，哪些必须留在应用层。
- 后续实现 agent 不需要重新阅读全部 research，依赖设计稿即可理解边界。
- 文档入口、demo 页面地图与 core contracts 使用同一套术语。
- 每一个高风险决定都带有“为什么现在锁定 / 为什么现在不锁定”的说明。

## 设计阶段 blocker
### 不是 blocker，但必须被记录
- 当前没有 `doc_auto/`。
- 当前没有 starter repo。
- 当前没有 `Enva` 兼容性结论。

### 是实现阶段 blocker
- starter repo 验证。
- `Enva` 兼容性复核。
- 文档同步落点确认。

## 设计稿应输出的产物
1. 正式 architecture design doc。
2. `common_core` 合同文档。
3. adapter 边界与运行面矩阵。
4. docs/demo 信息架构文档。
5. 实现前验证清单。

## 设计稿写作原则
- 用“协议”和“契约”描述 core，不用“默认实现”描述 core。
- 用“适配点”和“依赖面”描述 adapter，不用“唯一工具”描述 adapter。
- 任何落不到当前证据上的决定，都必须被标记为待验证项。
- 任何会影响多仓接入体验的术语，都优先在本轮统一命名。
