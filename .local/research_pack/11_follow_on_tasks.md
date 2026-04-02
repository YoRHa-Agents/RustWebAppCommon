# RustWebAppCommon 后续任务包
> 更新时间: 2026-04-01

## 目标
本文件把本轮调研产物拆分为下游可直接执行的任务包，确保后续设计、实现和本地复核不需要重新阅读原始 brief。

## 任务 A: 统一架构设计稿
- **输入**:
  - `08_common_architecture_proposal.md`
  - `10_final_research_report.md`
  - `07_docs_index_and_style_recommendation.md`
- **输出**:
  - architecture 设计文档
  - CLI 协议定义
  - docs/index 结构图
  - adapter 边界说明
- **依赖**:
  - 无，可立即开始
- **验证方式**:
  - 文档能明确回答 `common_core / common_adapters / app_owned` 的职责边界
  - CLI、docs、demo、release 四类能力都有归属
- **建议 agent**:
  - `generalPurpose`

## 任务 B: demo 页面与信息架构设计
- **输入**:
  - `09_demo_storyboard.md`
  - `07_docs_index_and_style_recommendation.md`
  - `10_final_research_report.md`
- **输出**:
  - demo 页面结构图
  - 页面导航与内容区块定义
  - markdown 与页面共享的 token 映射
- **依赖**:
  - 任务 A 至少完成 `common` 边界锁定
- **验证方式**:
  - 页面地图与 `common` 边界一致
  - GitHub Pages 静态托管假设清晰
- **建议 agent**:
  - `generalPurpose`，必要时可扩展到 canvas/UI 设计能力

## 任务 C: `Enva` 约束复核
- **输入**:
  - `10_final_research_report.md`
  - `08_common_architecture_proposal.md`
- **输出**:
  - `Enva` 兼容性评审
  - 是否需要调整目录结构、CLI 命名、docs 入口的差异说明
- **依赖**:
  - 任务 A 完成初版设计稿
- **验证方式**:
  - 明确列出冲突点与可接受差异
  - 明确指出是否允许进入实现阶段
- **建议 agent**:
  - `explore` / `generalPurpose`

## 任务 D: starter repo 与最小实现
- **输入**:
  - 任务 A、B、C 的输出
  - `08_common_architecture_proposal.md`
- **输出**:
  - starter repo 骨架
  - 最小 CLI
  - 最小 docs/index
  - 最小静态 demo
- **依赖**:
  - 任务 C 明确放行
- **验证方式**:
  - 能本地运行 web 入口
  - 能传入 `host` / `port`
  - 能构建静态 docs/demo 产物
  - 至少一条 release 路径可验证
- **建议 agent**:
  - 主 agent `direct` + `shell`

## Blocker 判断
| 项目 | 是否 blocker | 说明 |
|---|---|---|
| starter repo 验证 | 是 | 没有真实 repo，架构无法进入实现落地 |
| `Enva` 复核 | 是 | 任何实现前都需要确认本地约束不冲突 |
| `doc_auto` 引入 | 否（研究阶段） / 是（实现阶段） | 当前不阻塞调研，但一旦开始写代码，需要明确 docs 同步落点 |
| GitHub Pages host contract | 否 | 已有足够证据，可直接用于设计与 demo 规划 |

## 推荐执行顺序
1. 任务 A
2. 任务 B
3. 任务 C
4. 任务 D
