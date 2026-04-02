# 任务: Rust + Web Common 统一范式研究任务
> 面向未来多个 `rust + web page` 应用仓库，规划一项研究型任务，用于系统梳理可复用的 `common` 范式、成熟案例与最佳实践。本轮仅产出任务文档，不执行 clone、外部资料检索、代码实现或网页 demo 开发；后续允许在 `/home/agent/reference` 落地参考样本，并保留 `/home/agent/workspace/Enva` 作为单独 sub-agent 处理的延后输入。

## Targets
1. **研究范围基线**: 明确 `RustWebAppCommon` 需覆盖的能力边界、非目标与评价维度，验证方式为形成一份可直接用于后续调研的比较维度清单。
2. **案例采样方案**: 规划至少 3 类候选案例来源与采样方法，验证方式为输出可执行的样本收集策略与统一证据模板。
3. **最佳实践对比矩阵**: 定义覆盖构建、发布、部署、在线 demo、CLI 启动、agent 文档、风格化 markdown 的比较矩阵，验证方式为矩阵字段足以支撑后续方案取舍。
4. **统一范式建议**: 产出一份面向 `RustWebAppCommon` 的建议性研究报告结构，验证方式为其能够直接支撑后续设计文档编写与任务拆分。
5. **后续任务切分**: 将研究结论拆为设计、实现、demo 三类后续任务，验证方式为每个任务都具备明确输入、输出、依赖与验证方式。

## Acceptance Criteria
### Step 1: 研究目标与评价框架固化（无依赖，起始步骤）
- [ ] 明确说明本轮仅生成研究任务，不执行 clone、代码实现、网页制作或外部资料处理。
- [ ] `common` 能力边界至少覆盖跨平台构建、更新、部署/release、GitHub Pages/在线 demo、统一 CLI 启动、host/port 配置、子页面跳转关联。
- [ ] 评价维度至少覆盖技术栈契合度、跨仓复用性、运维复杂度、agent 接入成本、风格一致性与演示成本。
- [ ] 输出中明确哪些内容属于后续单独任务，而不是当前研究任务本体。

### Step 2: 候选案例范围与证据采集方式定义（依赖 Step 1）
- [ ] 规划至少 3 类候选案例来源，例如 Rust 工程能力、Web 承载与 demo 形态、agent 友好文档体系。
- [ ] 为每类案例定义统一的采集证据格式，使后续不同 agent 能以低上下文方式记录发现。
- [ ] 明确 `/home/agent/reference` 是允许的 clone 落地点，但本任务文档本身不要求立刻 clone 仓库。
- [ ] 明确 `/home/agent/workspace/Enva` 仅作为后续单独 sub-agent 的参考入口，不纳入当前轮阅读范围。

### Step 3: 横向比较与模式归纳标准成形（依赖 Step 2）
- [ ] 比较矩阵字段足以回答“`common` 应抽象到哪一层、哪些能力保留在各业务仓库中”的决策问题。
- [ ] 比较矩阵能区分构建/发布流程、网页入口组织、在线 demo 托管方式、文档索引策略、CLI 启动体验等关键差异。
- [ ] 归纳模板要求同时记录成熟做法、适用前提、潜在缺点和迁移成本，避免只记录正向亮点。
- [ ] 研究产出能够直接服务后续设计文档，而不需要再次从零整理材料。

### Step 4: 方案建议与演示蓝图输出要求明确（依赖 Step 3）
- [ ] 研究报告需给出 `RustWebAppCommon` 推荐形态，包括 common 层、网页入口、文档入口、命令入口的建议划分。
- [ ] 对 agent 友好文档至少提出 `README + index + 子主题入口` 的信息架构建议，并说明如何降低上下文占用。
- [ ] 对风格化 markdown 至少给出一套可研究的视觉与结构约束，覆盖 Nier: Automata 黑白灰主题方向。
- [ ] 对 web demo 只要求输出展示蓝图或 storyboard，不要求当前轮实现可运行页面。

### Step 5: 后续执行任务可直接接棒（依赖 Step 4）
- [ ] 后续设计任务、实现任务、demo 任务之间的依赖关系在文档中明确且可追踪。
- [ ] 每个后续任务至少包含输入材料、预期交付物、验证方式与建议使用的 agent/skill。
- [ ] 交付文本可被未来 sub-agent 直接消费，而无需重新解释背景目标。

## Context
### Repos:
- `/home/agent/workspace/RustWebAppCommon` — 当前任务文档工作区，现阶段仅包含本次研究任务说明。
  - `.local/task_design_for_rust_web_app_common.md` — 当前结构化 task 文档，内容整理自用户提供的原始 brief。
- `/home/agent/reference/openai-agents-python` — 本地参考语料之一，可作为 agent-friendly 文档组织与 examples 导航的候选观察对象。
  - `README.md` — 顶层项目入口，可作为研究阶段的文档导航样本。
  - `examples/agent_patterns/README.md` — 可作为 agent pattern 呈现方式的候选对照样本。
- `/home/agent/reference/agentic-context-engine` — 本地参考语料之一，可作为低上下文接入与 examples 索引设计的候选观察对象。
  - `README.md` — 顶层研究入口，可作为项目总览样本。
  - `examples/README.md` — 示例索引入口，可作为研究阶段的导航结构样本。
- `/home/agent/reference/ContextOS` — 本地参考语料之一，可作为 context/index 组织方式的候选观察对象。
  - `README.md` — 顶层入口，可作为后续研究阶段的上下文设计样本。

### Docs:
**现有需求输入:**
- `/home/agent/workspace/RustWebAppCommon/.local/task_design_for_rust_web_app_common.md`: 当前结构化任务文档，其内容整理自用户提供的原始需求 brief。

**本地参考入口（后续研究阶段使用）:**
- `/home/agent/reference/ContextOS/README.md`: 可候选用于观察 context/index 入口组织方式。
- `/home/agent/reference/openai-agents-python/README.md`: 可候选用于观察 agent-friendly README 与 examples 导航方式。
- `/home/agent/reference/agentic-context-engine/examples/README.md`: 可候选用于观察示例总索引的编排方式。

**延后处理入口:**
- `/home/agent/workspace/Enva`: 用户指定后续用单独 sub-agent 处理，本轮不阅读。

### Developer insights:
- **统一依赖目标**: 用户希望多个 Rust + Web 仓库依赖同一个 `common`，形成统一底座。
- **基础能力范围**: 目标底座需覆盖跨平台构建、更新、部署/release、GitHub Pages、在线 demo 与统一命令行启动。
- **页面入口要求**: 启动方式需要支持给定 `host` 与 `port`，并能拉起页面或关联子页面跳转。
- **Agent 接入要求**: 需要结构化 `README` 与 `index`，使 agent 在低上下文下结合 subagent 快速接入。
- **风格系统要求**: 需要面向 agent 预留风格化 markdown 规范，方向参考 Nier: Automata 黑白灰主题。
- **研究导向优先**: 用户当前优先需要成熟案例与 best practice 的研究任务，不要求立即执行研究过程。
- **参考目录约束**: 若后续需要 clone 仓库，允许使用 `/home/agent/reference` 作为落地点。
- **本地项目约束**: `/home/agent/workspace/Enva` 仅能在后续独立 sub-agent 流程中处理，本轮不得展开阅读。

### Editable Paths
- `/home/agent/workspace/RustWebAppCommon/.local/task_design_for_rust_web_app_common.md` — 当前研究任务文档的编辑目标。
- `/home/agent/reference/` — 后续研究阶段允许 clone 参考仓库的落地区域，本轮不改动。
- `/home/agent/workspace/Enva` — 后续单独 sub-agent 的参考入口，本轮不读取、不编辑。

### Agent Rules
- 在实现前优先使用 plan mode 明确方案。
- 存在歧义时先向用户确认。
- 进入实现阶段前先补齐对应测试。
- 使用 `.cursor/` 相关流程支持 develop-test-debug 循环。
- 对可拆分工作优先使用 subagent 并行下钻。

---

- **本任务仅出研究计划**: 当前只生成研究型 task 文档，不执行 clone、代码实现、页面开发或外部资料检索。
- **延后读取 `Enva`**: 除非用户后续明确授权并单独发起，否则不读取 `/home/agent/workspace/Enva`。
- **证据模板优先**: 后续研究必须先有统一证据模板，再开始采样或对比，避免结论不可复用。
- **行为化验收**: 验收项必须描述可观察结果，例如“完成对比矩阵并可用于决策”，而不是“新建某个文件”。
- **阶段时间上限**: 后续每个 Phase 需控制在单个 agent 约 30 分钟内，优先保证可交接性与可中断性。

## Skills
### Open URL
使用浏览器打开 URL 并阅读内容。
如果 URL 属于 *feishu.cn*，先让用户在浏览器中完成登录。

### Web Research
使用 Web 搜索或本地参考仓库对比成熟案例，收集证据并填写预定义比较矩阵。

### Parallel Subagent
使用并行 subagent 将研究拆分为 build/release、web demo 托管、agent 文档与风格系统等方向，再将结果合并为单份报告。

## TODOs
### Phase 1: 明确研究问题与评价基线（Step 1，无依赖，起始阶段）
- [ ] 1.1 将现有 brief 中的目标能力、约束和非目标整理为研究问题清单。
- [ ] 1.2 定义统一比较维度、评分方式与证据记录模板。
- [ ] 1.3 标记当前轮明确不做的内容：clone、实现、网页开发、`Enva` 阅读。

### Phase 2: 规划案例来源与采样批次（Step 2，依赖 Phase 1）
- [ ] 2.1 拆分 3 条研究主线：Rust/common 工程能力、web/demo 承载、agent 文档与风格系统。
- [ ] 2.2 为每条主线列出优先样本来源、本地参考入口和后续允许 clone 的目标目录。
- [ ] 2.3 定义单个样本的记录字段、链接规范与风险备注字段。

### Phase 3: 设计对比矩阵与归纳模板（Step 3，依赖 Phase 2）
- [ ] 3.1 设计对比矩阵列项，覆盖 build、release、deploy、demo、CLI、docs、style。
- [ ] 3.2 设计模式归纳模板，强制记录适用前提、局限与迁移成本。
- [ ] 3.3 设计适合后续 subagent 的最小上下文输入格式。

### Phase 4: 汇总结论并拆分后续任务（Step 4-5，依赖 Phase 3）
- [ ] 4.1 将研究结论整理为 `RustWebAppCommon` 的候选统一范式摘要。
- [ ] 4.2 拆出后续设计任务、实现任务、demo 任务和各自依赖关系。
- [ ] 4.3 定义最终研究报告、设计附录和 demo storyboard 的交付清单。
