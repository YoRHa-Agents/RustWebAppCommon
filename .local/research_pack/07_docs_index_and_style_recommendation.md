# Rust Web Common 文档入口与风格系统建议
> 更新时间: 2026-04-01

## 目标
本文件将主线 C 的样本比较收敛为可直接落地的建议，回答两个问题：
- `RustWebAppCommon` 的 `README + index + 子主题入口` 应如何组织，才能让主 agent 与 subagent 低上下文接入。
- Nier: Automata 黑白灰方向的视觉约束，应该如何转译成 markdown 和 demo 可复用的风格 token。

## 对照模式
| 样本 | 强项 | 弱项 | 可迁移结论 |
|---|---|---|---|
| `openai-agents-python` | `README + docs/index + API/reference + pattern README + examples runner` 分层非常清晰，且支持多语言 docs tree | examples 发现偏分散，需要目录 README 和运行器共同配合 | 适合借鉴“顶层概览 + 深层 reference + pattern catalog”的结构 |
| `agentic-context-engine` | `examples/README + quick start + AGENTS 规则 + design docs` 非常利于 agent 低上下文接入 | 规则密度高，若照搬会使 common 层治理过重 | 适合借鉴“入口规则 + 先读文档 + examples hub” |
| `ContextOS` | `README + wiki/Home + Architecture` 的平台叙事强，适合先讲层次再讲细节 | wiki 导航可能比落地页面更完整，存在索引漂移风险 | 适合借鉴“愿景 README 与结构 index 分层” |

## 推荐入口结构
### 顶层入口
- `README.md`
  - 面向人类与首次进入的 agent。
  - 只回答“这是什么、能解决什么、如何最快跑起来、去哪看更细内容”。
- `AGENTS.md`
  - 面向主 agent / subagent 的操作契约。
  - 说明先读路径、受保护目录、产物同步约束、哪些内容不要猜。
- `docs/index.md`
  - 面向系统化阅读。
  - 作为统一目录，把 architecture、guides、demo、theme、examples 组织起来。

### 次级入口
- `docs/architecture/overview.md`
  - 解释 `common_core / common_adapters / app_owned` 分层。
- `docs/guides/dev.md`
  - 统一 CLI、本地启动、`host` / `port`、常见工作流。
- `docs/guides/release.md`
  - release、deploy、demo、GitHub Pages、desktop adapter 相关路径。
- `examples/README.md`
  - 样例、starter、storyboard 入口，不与架构说明混写。
- `demo/storyboard.md`
  - 页面蓝图、用户流和展示目标。
- `theme/nier_gray_tokens.md`
  - 风格 token、markdown 组件规范和视觉语法。

## 推荐信息层次
1. **What**: `README.md`
2. **Where**: `docs/index.md`
3. **How**: `docs/guides/*.md`
4. **Why this way**: `docs/architecture/*.md`
5. **Show me**: `examples/README.md` + `demo/storyboard.md`
6. **How it should look**: `theme/nier_gray_tokens.md`

## 主 agent 最小入口包
主 agent 在低上下文场景下，优先读取：
1. `README.md`
2. `AGENTS.md`
3. `docs/index.md`
4. `docs/architecture/overview.md`

## subagent 最小入口包
不同子任务的 subagent 只需要少量入口：
- 研究类 subagent:
  - `docs/index.md`
  - `examples/README.md`
  - `theme/nier_gray_tokens.md`
- 设计类 subagent:
  - `docs/architecture/overview.md`
  - `demo/storyboard.md`
  - `theme/nier_gray_tokens.md`
- 实现类 subagent:
  - `docs/guides/dev.md`
  - `docs/guides/release.md`
  - `AGENTS.md`

## Nier 风格方向
### 视觉原则
- **单色优先**: 黑、灰、暖白为主，不依赖高饱和强调色。
- **工业排版**: 使用强分隔线、窄栏位标签、盒状信息块与编号式结构。
- **冷静克制**: 信息密度高，但层次必须清晰，避免花哨动画。
- **UI 噪点可控**: 允许扫描线、网格底纹、机械标签，但必须服务于信息导航。

### 色板 Token
| Token | 值 | 用途 |
|---|---|---|
| `bg.canvas` | `#111111` | 页面主背景 |
| `bg.panel` | `#1B1A17` | 卡片/信息面板 |
| `bg.raised` | `#262522` | 悬浮区块、引用块 |
| `fg.primary` | `#ECE6D9` | 主文本 |
| `fg.secondary` | `#B7B0A3` | 次级说明文字 |
| `fg.muted` | `#8E887E` | 弱提示、元信息 |
| `line.strong` | `#D7D0C4` | 主边框、强调分隔 |
| `line.soft` | `#5F5A52` | 次级边框、表格线 |
| `accent.signal` | `#CFC7B8` | 标签、状态块轻强调 |

### 字体与层级建议
- 标题层: 高对比 serif 或 quasi-serif，字重中高，字距略放大。
- 正文层: 清晰的 sans 或 humanist sans，强调可读性。
- 元信息层: monospace 或窄体，用于路径、命令、版本、标签。
- 小标题统一采用“编号 + 标签”形式，例如 `03 / Route Surface`。

## Markdown 组件映射
| Markdown 组件 | 视觉语义 | 用法 |
|---|---|---|
| `##` 标题 | 章节牌 | 前置短编号或短标签 |
| blockquote | 任务提示 / 世界观摘要 | 只用于 1-3 句高价值摘要 |
| table | 能力矩阵 / token 表 | 以强边框和浅灰标题行呈现 |
| code block | 命令、路径、配置契约 | 保持高对比底板与轻边框 |
| list | 执行步骤 / 边界清单 | 使用短句，不堆层级 |
| horizontal rule | 章节切换器 | 用作“机械隔断”而不是装饰 |

## Demo 与文档的统一约束
- docs 与 demo 共用同一套 token 命名，而不是各写一份颜色值。
- docs 页面优先强调“索引、对比、结论”；demo 页面优先强调“体验、层级、流程”。
- 如果 demo 需要额外动效，必须保持黑白灰主调，不引入抢眼高饱和色。
- 所有页面都应保留“当前层级 / 当前模块 / 下一步入口”的清晰提示。

## 推荐结论
- 文档入口上，优先采用 `README + AGENTS + docs/index + examples/README` 的四层结构。
- 风格系统上，优先沉淀 token、标题语法、组件映射和信息块规则，而不是先沉淀具体 CSS 细节。
- demo 层必须和 docs 层共用术语、标签和 token，避免“文档像一个项目，demo 像另一个项目”。
