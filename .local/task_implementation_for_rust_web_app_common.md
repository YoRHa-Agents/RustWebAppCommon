# 任务: RustWebAppCommon 基于设计的最小实现任务
> 基于现有 research pack 与 architecture design，将 `RustWebAppCommon` 从“研究/设计资产”推进到“starter repo 与最小实现骨架”。本任务聚焦实现阶段的最小可行交付：目录骨架、`common_core` 契约、adapter stub、docs/demo 入口与基础验证；不在本轮解决完整产品化、完整 release 服务或 `Enva` 深度兼容实现。

## Targets
1. **实施前放行条件**: 明确 starter repo、`Enva`、`doc_auto` 三类前置条件的状态与处理方式，验证方式为输出可执行的实现前决议表。
2. **Starter 骨架**: 落地 `common/`、`docs/`、`demo/`、`examples/`、`apps/` 的最小目录骨架，验证方式为目录结构能映射既定架构分层。
3. **Core Contracts 最小实现**: 落地 `WorkspaceIdentity`、`SurfaceKind`、`DevLaunchRequest`、`RouteDescriptor`、`DocsNode`、`ThemeTokenSet`、`ReleaseDescriptor` 的最小实现或 schema，验证方式为其具备基础测试与清晰边界。
4. **Adapter 与 CLI 接线**: 建立 `web_demo_adapter`、`docs_site_adapter`、`desktop_tauri_adapter`、`release_pipeline_adapter` 的 stub/interface，以及统一 `dev/demo/docs/release` 命令语义，验证方式为命令与 adapter 的输入输出关系可被测试或 stub 验证。
5. **Docs / Demo 最小入口**: 落地 `README + AGENTS + docs/index + examples/README + demo/storyboard` 的最小可用入口，验证方式为 docs/demo 与 core contracts 使用同一套术语和 token。
6. **实现验证与交接**: 补齐基础测试、docs 同步决策与 blocker 状态说明，验证方式为后续实现 agent 无需重读全部 research 即可接棒。

## Acceptance Criteria
### Step 1: 实现前放行条件固化（无依赖，起始步骤）
- [ ] 输出 starter repo、`Enva`、`doc_auto` 的状态决议表，明确哪些是本轮记录项，哪些是实现 blocker。
- [ ] 明确本轮实现范围仅覆盖 starter 骨架、核心契约、adapter stub、docs/demo 最小入口与基础验证。
- [ ] 明确 `common_core / common_adapters / app_owned` 的目录落点和职责边界。
- [ ] 明确在 `Enva` 复核完成前，不关闭本地兼容性相关决策。

### Step 2: Starter 骨架与 common_core 契约落地（依赖 Step 1）
- [ ] 创建 `common/core/`、`common/adapters/`、`common/cli/`、`docs/`、`demo/`、`examples/`、`apps/` 的最小目录骨架。
- [ ] 落地 `WorkspaceIdentity`、`SurfaceKind`、`DevLaunchRequest`、`RouteDescriptor`、`DocsNode`、`ThemeTokenSet`、`ReleaseDescriptor` 的最小实现或 schema。
- [ ] 核心契约具备测试，且测试能证明输入/输出边界成立。
- [ ] 实现中不得将 GitHub Pages、Dioxus、Tauri、Trunk 等具体 host/runtime 细节写入 `common_core`。

### Step 3: Docs / Demo 入口与主题契约落地（依赖 Step 2）
- [ ] 落地 `README.md`、`AGENTS.md`、`docs/index.md`、`examples/README.md`、`demo/storyboard.md` 的最小骨架。
- [ ] docs 与 demo 共用同一套 token 命名、章节术语和分层词汇。
- [ ] 页面地图与 `RouteDescriptor` 保持一致，至少覆盖 landing、runtime、docs-entry、release-flow、style-lab、detail 路由。
- [ ] docs 入口能够区分 human、main-agent、subagent 三类阅读路径。

### Step 4: Adapter stub 与统一 CLI 接线（依赖 Step 2，与 Step 3 并行）
- [ ] 建立 `web_demo_adapter`、`docs_site_adapter`、`desktop_tauri_adapter`、`release_pipeline_adapter` 的最小 stub/interface。
- [ ] 统一 `common dev`、`common demo`、`common docs`、`common release` 的命令语义与参数协议。
- [ ] `host` / `port` 参数协议能够贯通 web/demo 启动路径。
- [ ] adapter 不得重新定义 `common_core` 术语，也不得把 host/tooling 细节反向写回 core。

### Step 5: 最小运行验证、测试与交接封口（依赖 Step 3 和 Step 4）
- [ ] 至少一条本地 web/demo 路径可运行，或具备可测试的 stub 级运行验证。
- [ ] 至少一条 docs/demo 静态构建路径可验证，且与 GitHub Pages host contract 不冲突。
- [ ] 新增/更新测试覆盖 core contracts、关键 wiring 与实现前放行条件。
- [ ] 文档同步策略、blocker 状态和后续 `Enva` 复核入口被写入交接说明。

## Context
### Repos:
- `/home/agent/workspace/RustWebAppCommon` — 当前实现任务工作区，包含 research 与 architecture 设计资产，也是 starter repo 的目标落点。
  - `.local/task_implementation_for_rust_web_app_common.md` — 本次生成的实现任务文档。
  - `.local/research_pack/16_architecture_design_doc.md` — 正式架构设计稿，是本实现任务的主设计输入。
  - `.local/research_pack/17_pre_implementation_validation_checklist.md` — 实现前验证 gate 与 blocker 清单。
  - `.local/research_pack/13_common_core_contracts.md` — `common_core` 协议面定义。
  - `.local/research_pack/14_adapter_boundary_matrix.md` — adapter 职责边界与运行面矩阵。
  - `tests/test_research_pack.py` — 当前文档资产的验证测试入口，可在文档入口变更时继续使用。
- `/home/agent/reference/openai-agents-python` — 本地参考之一，用于借鉴 docs/index、多层导航与 examples 入口设计。
  - `README.md` — 顶层项目与文档入口样本。
  - `mkdocs.yml` — docs tree 与 reference 导航组织样本。
- `/home/agent/reference/agentic-context-engine` — 本地参考之一，用于借鉴 AGENTS 契约与 examples hub 结构。
  - `AGENTS.md` — agent 入口规则与设计文档锚点样本。
  - `examples/README.md` — 渐进式 examples 入口样本。
- `/home/agent/reference/ContextOS` — 本地参考之一，用于借鉴平台级分层叙事与 README/wiki 分层结构。
  - `README.md` — 平台能力与统一入口叙事样本。
  - `docs/wiki/Architecture.md` — 分层架构表达样本。

### Docs:
**实现输入:**
- `/home/agent/workspace/RustWebAppCommon/.local/research_pack/16_architecture_design_doc.md`: 当前正式 architecture design 输入。
- `/home/agent/workspace/RustWebAppCommon/.local/research_pack/17_pre_implementation_validation_checklist.md`: 实现前验证 gate 与 blocker。
- `/home/agent/workspace/RustWebAppCommon/.local/research_pack/11_follow_on_tasks.md`: 下游任务拆分，其中“任务 D”定义了 starter repo 与最小实现目标。

**设计基础:**
- `/home/agent/workspace/RustWebAppCommon/.local/research_pack/13_common_core_contracts.md`: core 协议模型。
- `/home/agent/workspace/RustWebAppCommon/.local/research_pack/14_adapter_boundary_matrix.md`: adapter 责任矩阵。
- `/home/agent/workspace/RustWebAppCommon/.local/research_pack/15_docs_demo_information_architecture.md`: docs/demo 信息架构设计。

**本地参考:**
- `/home/agent/reference/openai-agents-python/README.md`: 文档层级与 examples 入口样本。
- `/home/agent/reference/agentic-context-engine/AGENTS.md`: agent 契约与先读路径样本。
- `/home/agent/reference/ContextOS/docs/wiki/Architecture.md`: 分层架构叙事样本。

### Developer insights:
- **实现目标转移**: 当前目标已经从 research/design 转为 starter repo 与最小实现，不再是继续补充调研。
- **三层模型已锁定**: `common_core / common_adapters / app_owned` 是本轮实现必须遵守的主边界。
- **契约先于实现**: 需要先落 core contracts，再落 adapter stub，避免工具细节污染 core。
- **docs/demo 同步**: 文档入口和 demo 页面不是附属资产，而是必须与实现同步落地的设计面。
- **统一命令语义**: `dev / demo / docs / release` 是要优先固定的命令协议，而不是某个底层工具命令名。
- **Host contract 已明确**: GitHub Pages 的 `/docs`、`index.html`、`404.html` 规则可以直接进入实现约束。
- **桌面路径可选**: `desktop_tauri_adapter` 是强候选，但当前仍应保持为 adapter，而不是 core 默认实现。
- **前置 blocker 仍存在**: starter repo 验证、`Enva` 复核和 `doc_auto` 落点决策仍是实现阶段必须显式处理的前置条件。

### Editable Paths
- `/home/agent/workspace/RustWebAppCommon/.local/task_implementation_for_rust_web_app_common.md` — 本次实现任务文档。
- `/home/agent/workspace/RustWebAppCommon/common/` — 计划中的 `common_core`、adapter、CLI 实现根目录。
- `/home/agent/workspace/RustWebAppCommon/docs/` — 计划中的 docs/index 与 architecture/guides 目录。
- `/home/agent/workspace/RustWebAppCommon/demo/` — 计划中的 storyboard 与 demo 页面目录。
- `/home/agent/workspace/RustWebAppCommon/examples/` — 计划中的样例入口目录。
- `/home/agent/workspace/RustWebAppCommon/README.md` — 计划中的顶层人类入口。
- `/home/agent/workspace/RustWebAppCommon/AGENTS.md` — 计划中的主 agent / subagent 入口契约。
- `/home/agent/workspace/RustWebAppCommon/tests/` — 计划中的实现测试与现有文档测试目录。

### Agent Rules
- 在实现前优先使用 plan mode 明确方案。
- 存在歧义时先向用户确认。
- 进入实现阶段前先补齐对应测试。
- 使用 `.cursor/` 相关流程支持 develop-test-debug 循环。
- 对可拆分工作优先使用 subagent 并行下钻。

---

- **先过 Gate 再编码**: 先处理 `17_pre_implementation_validation_checklist.md` 中的 Gate 1-3，再推进实际实现。
- **契约先于适配**: 必须先落 `common_core` 协议和测试，再写 adapter stub 与 wiring。
- **adapter 不反向污染 core**: Dioxus、Tauri、Trunk、GitHub Pages 的具体实现不得写进 core。
- **docs/demo 同步建模**: `README`、`AGENTS`、`docs/index`、`demo/storyboard` 需要与 starter 骨架同步落地。
- 每个 Step 完成后运行相应验证：Rust 逻辑运行 `cargo test`，文档入口调整时运行 `python -m unittest tests.test_research_pack`。

## Skills
### Open URL
使用浏览器打开 URL 并阅读内容。
如果 URL 属于 *feishu.cn*，先让用户在浏览器中完成登录。

### Code Exploration
使用代码搜索与文件阅读确认 starter repo 结构、核心契约落点、测试模式和文档入口的一致性。

### Parallel Subagent
使用并行 subagent 分拆 `Enva` 复核、adapter 细化、docs/demo 落地与测试策略设计等子任务。

## TODOs
### Phase 1: 实现前放行与目录落点（Step 1，无依赖，起始阶段）
- [ ] 1.1 固化 starter repo、`Enva`、`doc_auto` 的状态决议表。
- [ ] 1.2 明确本轮最小实现范围、非目标和 blocker。
- [ ] 1.3 确认 `common/`、`docs/`、`demo/`、`examples/`、`apps/` 的目标目录结构。

### Phase 2: common_core 骨架与契约实现（Step 2，依赖 Phase 1）
- [ ] 2.1 落地 `common/core/`、`common/cli/` 的最小目录和模块骨架。
- [ ] 2.2 实现 `WorkspaceIdentity`、`SurfaceKind`、`DevLaunchRequest`、`RouteDescriptor`、`DocsNode`、`ThemeTokenSet`、`ReleaseDescriptor`。
- [ ] 2.3 为 core contracts 与 CLI vocabulary 补齐基础测试。

### Phase 3: docs/demo 入口与主题落地（Step 3，依赖 Phase 2）
- [ ] 3.1 落地 `README.md`、`AGENTS.md`、`docs/index.md`、`examples/README.md`、`demo/storyboard.md` 最小骨架。
- [ ] 3.2 将页面地图与 `RouteDescriptor` 对齐。
- [ ] 3.3 将 Nier 黑白灰 token 映射到 docs/demo 的最小主题结构。

### Phase 4: adapter stub 与统一命令接线（Step 4，依赖 Phase 2，与 Phase 3 并行）
- [ ] 4.1 落地 `web_demo_adapter`、`docs_site_adapter`、`desktop_tauri_adapter`、`release_pipeline_adapter` 的 stub/interface。
- [ ] 4.2 统一 `common dev`、`common demo`、`common docs`、`common release` 的命令协议。
- [ ] 4.3 打通 `host` / `port` 与 route entry 的最小 wiring。

### Phase 5: 最小运行验证与交接封口（Step 5，依赖 Phase 3 和 Phase 4）
- [ ] 5.1 验证至少一条本地 web/demo 路径可运行或可通过 stub 验证。
- [ ] 5.2 验证至少一条 docs/demo 静态构建路径与 GitHub Pages contract 一致。
- [ ] 5.3 更新测试、文档同步决策和后续 `Enva` 复核交接说明。
