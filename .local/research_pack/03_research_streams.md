# Rust Web Common 研究主线与采样批次
> 更新时间: 2026-04-01

## 总览
后续研究分为三条主线，分别覆盖公共工程能力、Web/demo 承载能力，以及 agent 文档与风格系统。三条主线都共享 `02_evidence_schema.md` 的采样模板，但起始输入、优先样本和后续研究边界不同。

| 主线 | 目标 | 当前可直接开始的输入 | 是否需要后续 Web 研究 | 主要产出 |
|---|---|---|---|---|
| A. Common 工程与运行时 | 定义 `common` 层的能力边界与命令入口 | `ContextOS` 架构文档、本地 task brief | 是 | common/app 职责边界与工程模式候选 |
| B. Web/demo 承载与分发 | 定义在线 demo、静态托管与页面路由承载模式 | 当前本地输入较弱，仅有 brief | 是 | demo 承载模式、GitHub Pages 策略与路由约束 |
| C. Agent 文档与风格系统 | 定义 README/index/低上下文入口与主题约束 | `openai-agents-python`、`agentic-context-engine`、`ContextOS` | 是，但本地主线可先做 | agent 友好文档模式与风格系统约束 |

## 主线 A: Common 工程与运行时
### 核心问题
- `common` 应提供哪些稳定能力：构建、更新、release、启动入口、路由桥接、还是仅定义接口。
- 多仓依赖 `common` 时，如何避免把应用特有的页面结构、部署细节或 demo 内容强绑进公共层。
- 统一 CLI 是否只负责启动与路由分派，还是还要承载构建/发布编排。

### 当前起始输入
- `/home/agent/workspace/RustWebAppCommon/.local/research_pack/01_research_charter.md`
- `/home/agent/reference/ContextOS/README.md`
- `/home/agent/reference/ContextOS/docs/wiki/Architecture.md`

### 后续优先搜索方向
- Rust workspace 与多包组织模式。
- 跨平台打包与分发工具链。
- 统一 CLI 与多入口应用启动模式。
- 兼顾 native 与 web 的工程结构样本。

### 推荐 subagent
- `explore`: 收集候选仓库与官方文档入口。
- `generalPurpose`: 归纳公共层边界、迁移成本与能力分层。

### 停止条件
- 至少获得 2 个具备 `build_flow + release_update_flow + cli_surface` 直接证据的候选样本。
- 至少有 1 个样本明确体现 `common` 与应用层边界，而不是单体工程。

## 主线 B: Web/demo 承载与分发
### 核心问题
- 在线 demo 应优先采用静态托管、服务端托管还是双轨模式。
- GitHub Pages 是否足以承担文档与 demo 展示，以及其对路由、资源产物与回退策略的限制是什么。
- `host` / `port` 参数在本地运行、演示环境与子页面跳转中如何保持一致接口。

### 当前起始输入
- `/home/agent/workspace/RustWebAppCommon/.local/research_pack/01_research_charter.md`
- 当前本地参考仓库更多提供文档与 agent 入口模式，对 Web/demo 承载帮助有限。

### 后续优先搜索方向
- Rust Web 应用与静态导出样本。
- GitHub Pages、静态站点与 demo 托管模式。
- 支持子页面、嵌套路由或多入口页面的路由方案。
- 本地运行与在线演示之间可复用的命令入口设计。

### 推荐 subagent
- `explore`: 扫描候选仓库、官方文档和托管说明。
- `generalPurpose`: 把 demo 承载与路由能力映射到 common / app 分层。

### 停止条件
- 至少获得 2 个具备 `deploy_demo_flow + routing_surface` 直接证据的候选样本。
- 至少获得 1 个能明确回答 GitHub Pages 适用边界的样本。

## 主线 C: Agent 文档与风格系统
### 核心问题
- 如何通过 `README + index + 子主题入口` 让主 agent 与 subagent 用最少上下文理解体系结构。
- 示例目录、quick start、integration 文档之间应如何分层，才能兼顾初学者入口与深层扩展。
- Nier: Automata 黑白灰风格应落在 markdown 约束、组件样式、还是 docs 主题 token 层。

### 当前起始输入
- `/home/agent/reference/openai-agents-python/README.md`
- `/home/agent/reference/openai-agents-python/examples/agent_patterns/README.md`
- `/home/agent/reference/openai-agents-python/examples/run_examples.py`
- `/home/agent/reference/agentic-context-engine/examples/README.md`
- `/home/agent/reference/agentic-context-engine/docs/getting-started/quick-start.md`
- `/home/agent/reference/ContextOS/README.md`

### 后续优先搜索方向
- 文档入口与示例索引设计模式。
- Agent 低上下文接入模板、SKILL/README/index 组织法。
- Markdown / docs 主题系统、样式 token、灰阶视觉系统约束。

### 推荐 subagent
- `explore`: 抽取 README、index、examples、quick start 的组织模式。
- `generalPurpose`: 归纳面向 agent 的信息架构与主题约束。

### 停止条件
- 至少整理出 3 种不同的文档入口模式。
- 至少获得 1 套可落地到 markdown 的风格约束框架。

## 批次建议
### Batch 1: 本地基线样本
- 优先完成主线 C 的本地对照，因为本地证据最完整。
- 同步从主线 A 提取“分层叙事”和“CLI / MCP 统一入口”线索。
- 主线 B 只建立问题清单和搜索边界，不急于下结论。

### Batch 2: 外部成熟样本
- 为主线 A 与 B 扩充官方文档与成熟仓库样本。
- 使用 `02_evidence_schema.md` 统一记录，不允许自由格式记笔记。

### Batch 3: 归纳与决策
- 将三条主线合并进对比矩阵。
- 强制回答 `common` 与应用层分界问题。
- 生成最终研究报告大纲和后续任务交接清单。

## 并行与依赖
- 主线 A 与主线 C 可以在统一模板完成后并行推进。
- 主线 B 依赖外部样本更多，但其问题清单可以与 A/C 同步建立。
- 所有主线的正式结论都必须汇总到同一对比矩阵中，避免各写各的。
