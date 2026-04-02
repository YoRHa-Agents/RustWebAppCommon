# Rust Web Research Pack
> 更新时间: 2026-04-01

## 说明
本目录实现了 `Rust Web Research Plan` 以及后续架构设计计划的核心产物，目标是把原始需求整理成一组可复用的 research 与 architecture 资产，供后续主 agent 或 subagent 直接消费。当前目录只包含研究、设计与交接文档，不包含 Rust/Web 实现代码，也不包含对 `/home/agent/workspace/Enva` 的读取结果。

## 文件清单
1. `01_research_charter.md`
   - 固化研究目标、关键问题、评价维度与非目标。
2. `02_evidence_schema.md`
   - 统一样本记录模板、低上下文交接卡片与淘汰规则。
3. `03_research_streams.md`
   - 划分三条研究主线、定义批次、起始输入和停止条件。
4. `04_comparison_matrix.md`
   - 提供决策矩阵、基线样本、模式归纳模板和最低证据门槛。
5. `05_execution_handoffs.md`
   - 定义正式研究报告大纲，以及后续设计 / 实现 / demo 的交接结构。
6. `06_threshold_gap_register.md`
   - 记录各主线的证据阈值状态、剩余缺口与下一步重点。
7. `07_docs_index_and_style_recommendation.md`
   - 收敛文档入口建议与 Nier 黑白灰风格 token。
8. `08_common_architecture_proposal.md`
   - 给出 `common_core / common_adapters / app_owned` 的边界方案。
9. `09_demo_storyboard.md`
   - 定义 demo 页面地图、导航流、本地/在线展示映射。
10. `10_final_research_report.md`
   - 正式研究报告，汇总推荐结论、风险和后续路线。
11. `11_follow_on_tasks.md`
   - 后续设计、实现、`Enva` 复核与 blocker 判断。
12. `samples/`
   - 按统一 schema 记录的本地与外部正式样本 dossier。
13. `12_architecture_scope_and_decisions.md`
   - 锁定本轮架构设计的范围、决策状态与 blocker 边界。
14. `13_common_core_contracts.md`
   - 定义 `common_core` 的核心协议、状态边界与 invariants。
15. `14_adapter_boundary_matrix.md`
   - 定义各 adapter 的职责、依赖、failure modes 与替换规则。
16. `15_docs_demo_information_architecture.md`
   - 收敛 docs/demo 的统一信息架构与页面职责。
17. `16_architecture_design_doc.md`
   - 正式 architecture design doc，整合 layering、contracts、adapters 与 IA。
18. `17_pre_implementation_validation_checklist.md`
   - 实现前验证清单，覆盖 starter repo、`Enva`、`doc_auto` 与 release path。

## 建议阅读顺序
1. 先读 `01_research_charter.md`，理解边界与研究问题。
2. 再读 `02_evidence_schema.md`，保证所有样本按统一格式记录。
3. 之后读 `03_research_streams.md`，选择当前要推进的主线与批次。
4. 再读 `samples/` 与 `04_comparison_matrix.md`，把样本与决策视图对应起来。
5. 继续读 `07_docs_index_and_style_recommendation.md`、`08_common_architecture_proposal.md`、`09_demo_storyboard.md`。
6. 再读 `10_final_research_report.md` 与 `11_follow_on_tasks.md`，理解研究收敛结果。
7. 最后读 `12_architecture_scope_and_decisions.md` 到 `17_pre_implementation_validation_checklist.md`，进入正式架构设计包。

## 当前本地基线
- `openai-agents-python`: 适合作为 `README + 分主题 README + 运行器式索引` 的导航样本。
- `agentic-context-engine`: 适合作为 `examples/README + quick start + integrations` 的渐进式入口样本。
- `ContextOS`: 适合作为 `README + 架构 wiki + 统一入口叙事` 的分层表达样本。
- `Dioxus`: 适合作为 `base_path + GitHub Pages + nested routes` 的混合 runtime 样本。
- `Tauri v2`: 适合作为 `desktop release + updater + GitHub pipeline` 的桌面 adapter 样本。
- `GitHub Pages`: 适合作为静态 docs/demo host contract 样本。

## 本目录刻意不做的事
- 不执行远程 clone。
- 不读取 `/home/agent/workspace/Enva`。
- 不生成最终技术选型结论。
- 不实现 Rust 工程、CLI 或网页 demo。

## 适合下一位 agent 的最小入口
如果后续 agent 只允许读取少量上下文，优先读取：
1. `01_research_charter.md`
2. `02_evidence_schema.md`
3. `04_comparison_matrix.md`
4. `10_final_research_report.md`
5. `16_architecture_design_doc.md`
