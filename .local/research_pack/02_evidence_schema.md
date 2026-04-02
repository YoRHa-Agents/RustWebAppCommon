# Rust Web Common 证据模板与采样格式
> 更新时间: 2026-04-01

## 目的
本模板用于约束后续所有研究样本的记录格式，确保不同 agent / subagent 能在低上下文下产出可合并、可对比、可复查的证据，而不是只留下无法复用的“结论摘要”。

## 必填字段
| 字段 | 说明 | 填写要求 |
|---|---|---|
| `case_id` | 样本唯一标识 | 使用 `family-name__surface` 形式，例如 `tauri__desktop_web` |
| `source_kind` | 样本来源类型 | `local_repo` / `remote_repo` / `official_docs` / `article` |
| `source_path_or_url` | 样本入口 | 本地路径或官方 URL，必须可复查 |
| `sample_scope` | 样本覆盖面 | 标明观察的是整仓、文档入口、构建流程还是单示例 |
| `maturity_signal` | 成熟度信号 | 记录版本、发布机制、示例规模、文档完整性等可观察线索 |
| `runtime_surface` | 运行形态 | 标明 `native` / `web` / `hybrid` / `demo_only` |
| `build_flow` | 构建方式 | 记录命令入口、工作区组织、产物类型、平台差异 |
| `release_update_flow` | 发布/更新方式 | 记录 release、升级、分发、版本管理模式 |
| `deploy_demo_flow` | 部署/demo 承载方式 | 记录 GitHub Pages、静态托管、服务端托管或桌面分发方式 |
| `cli_surface` | 命令行入口 | 记录是否有统一 CLI、如何传入 `host` / `port` / 页面参数 |
| `routing_surface` | 页面与子页面关联 | 记录是否支持子页面跳转、嵌套路由、入口切换 |
| `docs_index_pattern` | 文档入口模式 | 记录 `README`、`index`、示例索引、分主题入口组织方式 |
| `agent_onboarding_pattern` | Agent 低上下文接入方式 | 记录是否有 quick start、分层入口、模板、索引卡片 |
| `style_system` | 风格系统 | 记录 markdown / docs / demo 是否存在主题、token 或视觉约束 |
| `common_boundary_signal` | 公共层边界信号 | 记录哪些能力适合下沉到 common，哪些应留在应用层 |
| `risks_limitations` | 风险与限制 | 至少 2 条，不允许只写优点 |
| `migration_cost` | 迁移成本 | `low` / `medium` / `high`，并说明触发原因 |
| `evidence_strength` | 证据强度 | `A` / `B` / `C`，其中 `A` 代表有直接代码/文档证据，`C` 代表仅二手描述 |
| `quoted_artifacts` | 直接证据 | 至少列出 2 个文件路径、命令、截图或 URL |
| `open_questions` | 未解问题 | 至少 1 条，为后续研究保留 |

## 证据强度定义
| 等级 | 定义 | 使用场景 |
|---|---|---|
| `A` | 有直接路径、代码、命令或官方文档可佐证 | 可进入对比矩阵并参与决策 |
| `B` | 有较强线索，但仍缺关键实现或官方说明 | 可作为候选样本，不能单独支撑结论 |
| `C` | 只有口碑、文章或间接摘要 | 只能作为待验证线索，不进入最终结论 |

## 单样本记录模板
```md
# Sample: <case_id>

## Snapshot
- source_kind:
- source_path_or_url:
- sample_scope:
- maturity_signal:
- runtime_surface:

## Execution Surfaces
- build_flow:
- release_update_flow:
- deploy_demo_flow:
- cli_surface:
- routing_surface:

## Docs and Agent Onboarding
- docs_index_pattern:
- agent_onboarding_pattern:
- style_system:

## Common Boundary Signals
- suitable_for_common:
- should_stay_in_app:

## Risks and Cost
- risks_limitations:
  - ...
  - ...
- migration_cost:
- evidence_strength:

## Quoted Artifacts
- ...
- ...

## Open Questions
- ...
```

## 低上下文交接卡片
后续 subagent 记录单样本时，优先产出以下最小卡片，而不是长篇叙述：

```yaml
case_id: ""
fit_for_rust_web_common: high|medium|low
runtime_surface: native|web|hybrid|demo_only
docs_index_pattern: ""
common_boundary_signal:
  common: []
  app: []
biggest_risk: ""
migration_cost: low|medium|high
evidence_strength: A|B|C
quoted_artifacts: []
next_check: ""
```

## 样本淘汰规则
出现以下任一情况时，样本只能记录在线索池，不进入正式对比矩阵：
- 只有营销叙述，没有可复查的代码、命令、文档或官方说明。
- 无法观察构建、发布、部署、CLI、文档入口中的至少 2 个维度。
- 样本只是一次性教程或玩具 demo，无法体现公共层或复用边界。
- 与已收录高质量样本完全重复，且没有补充新维度。
- 明显依赖不可迁移的私有基础设施，而未给出可替代路径。

## 收集完成检查
样本记录完成时必须满足：
- 至少 1 个 `common_boundary_signal` 明确指出“适合下沉到 common”的能力。
- 至少 1 个 `common_boundary_signal` 明确指出“应留在业务仓库”的能力。
- `risks_limitations` 至少 2 条。
- `quoted_artifacts` 至少 2 条。
- `evidence_strength` 不得留空。
