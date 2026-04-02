# 任务: 面向 Enva 的公共能力增强改进任务
> 基于当前已经完成的完整 demo、`Enva` 对齐矩阵、release/install/update 基础闭环与回归验证，本任务不再重复已关闭的 starter 或首轮兼容性收敛工作，而是聚焦 `doc_auto/enva_gap_requirements.md` 中仍值得在公共层继续增强的能力。任务类型为 hybrid：先收敛剩余 gap、优先级与落点，再实现可复用的 adapter / scripts / workflow / tests / docs 增强，同时保持 `common_core` 不承载 `Enva` 产品语义。

## Targets
1. **剩余 Enva 改进范围收敛**: 将 `doc_auto/enva_gap_requirements.md` 中的多平台 release、install post-flight hook、updater seam、dual-surface parity scaffolding、CLI integration harness 收敛为明确的本轮范围与非目标，验证方式为输出可执行的优先级与落点决议。
2. **多平台 release 与安装扩展**: 让共享 build/install/release 路径支持更接近 `Enva` 的复用需求，验证方式为统一脚本或 workflow helper 能覆盖至少 Linux x86_64、Linux aarch64、macOS aarch64 三类目标，并保留共享资产命名契约。
3. **updater 与测试接缝增强**: 为二进制替换验证、双运行面对齐检查与 CLI 集成测试提供共享接缝，验证方式为下游仓库可以复用文档化的 seam / harness，而无需复制整套实现。
4. **`doc_auto` 真相源强化**: 将新增增强点同步写入 `doc_auto`，明确哪些能力已进入 common、哪些仍属 `Enva app-owned`，验证方式为对应同步文档具备最新时间戳与边界说明。
5. **回归与交接闭环**: 让 Rust、Python 与关键脚本验证覆盖新增能力，验证方式为 `cargo test`、`python -m unittest tests.test_research_pack tests.test_starter_repo` 与目标 smoke checks 通过，并生成可直接接手的交接说明。

## Acceptance Criteria
### Step 1: 固化当前增强范围与真相源（无依赖，起始步骤）
- [ ] 明确本轮起点是“已完成首轮 Enva 对齐闭环后的增强阶段”，而不是重新开启 starter repo 或首轮兼容性裁决。
- [ ] 输出一份新的范围说明，明确本轮只处理公共可复用能力增强，不处理 `Enva` 的 vault、session、SSH、产品 CLI 词汇与业务路由。
- [ ] 明确 `doc_auto/enva_gap_requirements.md`、`doc_auto/remaining_implementation_delta.md`、`doc_auto/implementation_handoff.md` 是本轮真相源。

### Step 2: 复核 Enva 剩余 gap 并确定优先级（依赖 Step 1）
- [ ] 对多平台 release、install hook、updater seam、dual-surface parity、CLI integration harness 五类 gap 给出 `must_now`、`should_now`、`later` 的优先级标记。
- [ ] 每项 gap 都有明确落点：`common/adapters/`、`scripts/`、`.github/workflows/`、`tests/`、`docs/` 或 `doc_auto/`。
- [ ] 每项 gap 都说明为何能进入 common，或为何仍必须保持为 app-owned / out-of-scope。

### Step 3: 设计共享扩展契约与验证策略（依赖 Step 2）
- [ ] 为多平台 release、install hook、updater seam、parity scaffolding、CLI harness 分别定义最小共享接口、脚本约定或文档契约。
- [ ] 设计结果明确哪些信息可以进入 `ReleaseDescriptor` 或 adapter trait，哪些 provider / GitHub / binary replacement 细节必须停留在 adapter、script 或 workflow 层。
- [ ] 为每个增强点定义至少一条可执行验证路径，避免只留下结构性占位或“后续补测”的空白。

### Step 4: 多平台 release 与安装 hook 增强（依赖 Step 3，与 Step 5 并行）
- [ ] 共享 release 路径可以通过统一命令、脚本或 workflow helper 编排至少 Linux x86_64、Linux aarch64、macOS aarch64 三类目标，并保持 `rustwebappcommon-<platform>` 与 `SHA256SUMS` 契约不变。
- [ ] 安装流程允许配置可选的 post-flight smoke hook；未配置时保持通用默认行为，配置失败时返回明确的非零退出。
- [ ] 多平台产物元信息与安装验证路径能够被下游仓库复用，而不要求复制 `Enva` 的产品命名或 provider 常量。

### Step 5: updater、双运行面对齐与 CLI harness 增强（依赖 Step 3，与 Step 4 并行）
- [ ] 公共层提供清晰的 updater extension seam，可承载二进制替换或更深验证流程，但不直接实现 `Enva` 的产品级更新逻辑。
- [ ] 共享测试脚手架能够在至少两个运行面上校验 manifest、DOM hook 或 runtime marker 的一致性，而不是只验证单一静态页面存在。
- [ ] CLI integration harness 至少覆盖子进程启动、临时工作区、fixture 准备与 mock HTTP 服务的最小模式，并能被文档或示例直接引用。

### Step 6: `doc_auto`、docs 与边界说明同步（依赖 Step 4 和 Step 5）
- [ ] `doc_auto` 中的 gap、matrix、handoff 或新增同步文档明确记录新增公共能力、未关闭项与边界裁决，并在文末追加最新修改时间。
- [ ] `docs/guides/release.md`、相关 architecture/docs 入口说明何时使用共享扩展，何时保留为下游产品自有实现。
- [ ] 文档同步后，`README` / `AGENTS` / `docs/index` / `doc_auto` 的术语继续与 `common_core`、adapter 和 scripts 保持一致。

### Step 7: 回归验证与交接封口（依赖 Step 6）
- [ ] `cargo test` 与 `python -m unittest tests.test_research_pack tests.test_starter_repo` 全部通过，且新增测试不依赖产品私有命名。
- [ ] 与本轮增强相关的关键 smoke checks 至少覆盖 release/build、install hook、update-check 或 parity 断言中的必要子集。
- [ ] 交接说明能够让下一位 agent 直接知道：哪些 Enva 导向能力已进入 common、哪些仍待后续路线决策、下一步应优先推进什么。

## Context
### Repos:
- `/home/agent/workspace/RustWebAppCommon` — 当前主工作区，已经完成首轮 follow-on implementation 闭环，本任务是在现有代码与 `doc_auto` 真相源基础上继续做 Enva 导向的公共能力增强。
  - `.local/task_improve_for_enva.md` — 本次生成的 Enva 增强任务文档。
  - `doc_auto/enva_gap_requirements.md` — 本轮最核心的剩余 gap 真相源。
  - `doc_auto/enva_compatibility_matrix.md` — 已关闭的首轮 Enva 对齐矩阵与边界裁决。
  - `doc_auto/remaining_implementation_delta.md` — 当前已完成范围、仍未关闭项与下一阶段建议。
  - `doc_auto/implementation_handoff.md` — 当前实现基线、验证结果与待关闭项交接。
  - `common/adapters/src/lib.rs` — adapter traits、runtime contract 与 release / desktop / site 接缝。
  - `common/cli/tests/static_site.rs` — 现有静态站点与 runtime contract 回归测试入口。
  - `scripts/build-release.sh` — 现有 release 产物构建入口。
  - `scripts/release-contract.sh` — 共享资产命名与 checksum 相关契约脚本。
  - `scripts/install.sh` — 现有安装流程入口，是 post-flight hook 的候选扩展点。
  - `scripts/update-check.sh` — 现有更新检查入口，是 updater seam 的候选扩展点。
  - `.github/workflows/release-artifacts.yml` — 现有 release workflow，后续多平台能力的主要扩展点。
  - `.github/workflows/deploy-pages.yml` — Pages workflow，双运行面对齐与静态输出一致性的间接约束。
  - `docs/guides/release.md` — 当前 release / install / update 边界与限制说明。
  - `tests/test_starter_repo.py` — 当前结构、workflow、`doc_auto` 与 release contract 的 Python 回归入口。

### Docs:
**当前真相源:**
- `/home/agent/workspace/RustWebAppCommon/doc_auto/enva_gap_requirements.md`: Enva 仍希望 common 继续增强的能力列表与验收方向。
- `/home/agent/workspace/RustWebAppCommon/doc_auto/enva_compatibility_matrix.md`: 首轮 Enva 对齐已关闭项、冲突点与边界裁决。
- `/home/agent/workspace/RustWebAppCommon/doc_auto/remaining_implementation_delta.md`: 已完成事项、未关闭路线与下一阶段范围说明。
- `/home/agent/workspace/RustWebAppCommon/doc_auto/implementation_handoff.md`: 当前验证结果与 handoff 基线。

**边界与验证基线:**
- `/home/agent/workspace/RustWebAppCommon/.local/research_pack/16_architecture_design_doc.md`: 三层边界、运行面策略与 adapter 责任总稿。
- `/home/agent/workspace/RustWebAppCommon/.local/research_pack/17_pre_implementation_validation_checklist.md`: Enva、`doc_auto`、release path 等 Gate 定义。
- `/home/agent/workspace/RustWebAppCommon/docs/guides/release.md`: 当前 release 策略、已实现能力与限制。
- `/home/agent/workspace/RustWebAppCommon/tests/test_starter_repo.py`: `doc_auto`、workflow、release contract 的仓库级验证基线。

### Developer insights:
- **首轮 Enva 对齐已结束**: 目录、CLI 命名、docs 入口、Pages/release/update 基础闭环已经有明确裁决，不应重新作为本轮主目标。
- **剩余空间主要在增强而非复制**: `doc_auto/enva_gap_requirements.md` 关注的是“让下游更容易复用 common”，而不是把 `Enva` 产品实现搬进公共仓库。
- **release 仍是最强的 Enva 驱动点**: 多平台构建、安装后 smoke、updater seam 与 asset contract 都围绕 scripts/workflows/adapters 展开。
- **测试支撑仍偏静态站点**: 现有 `common/cli/tests/static_site.rs` 和 `tests/test_starter_repo.py` 已覆盖结构与 manifest，但 CLI 子进程和双运行面对齐脚手架仍有扩展空间。
- **`doc_auto` 是阶段真相源**: 本轮任何结构性增强都必须及时同步到 `doc_auto`，否则后续 agent 会把已完成与未完成状态重新混淆。
- **`common_core` 边界仍需严格遵守**: provider、GitHub API、产品命名、vault / session / SSH 等能力不能因为 Enva 驱动而回写进 core。
- **browser-backed desktop preview 不是当前 blocker**: 它仍可作为默认路线存在，本任务更偏向在其外围增强 release、updater 与测试接缝。

### Editable Paths
- `/home/agent/workspace/RustWebAppCommon/.local/task_improve_for_enva.md` — 本次 Enva 增强任务文档。
- `/home/agent/workspace/RustWebAppCommon/common/adapters/` — adapter trait 与 runtime / release 接缝增强入口。
- `/home/agent/workspace/RustWebAppCommon/common/cli/` — 统一 CLI 协议与集成测试接线入口。
- `/home/agent/workspace/RustWebAppCommon/common/cli/tests/` — Rust 侧 parity 与 CLI 回归测试入口。
- `/home/agent/workspace/RustWebAppCommon/scripts/` — build / install / update / release contract 增强入口。
- `/home/agent/workspace/RustWebAppCommon/.github/workflows/` — 多平台 release 与 CI 编排增强入口。
- `/home/agent/workspace/RustWebAppCommon/docs/` — release guide、architecture 与使用方式说明更新入口。
- `/home/agent/workspace/RustWebAppCommon/doc_auto/` — Enva gap、matrix、handoff 与同步记录真相源。
- `/home/agent/workspace/RustWebAppCommon/tests/` — Python 仓库级回归测试入口。

### Agent Rules
- 在进入实现前优先使用 plan mode 明确方案。
- 存在歧义时先向用户确认。
- 新增逻辑前先补齐或更新对应测试。
- 使用 `.cursor/` 相关流程支撑 develop-test-debug 循环。
- 对可并行拆分的工作优先使用 subagent 下钻。

---

- **以 `doc_auto` 为准**: 本轮以 `doc_auto/enva_gap_requirements.md` 与 `remaining_implementation_delta.md` 为唯一优先级真相源，不重复已关闭的 follow-on 任务。
- **Enva 导向但不复制产品**: 只增强共享 release、install、update、tests、docs、workflow 能力，不引入 `Enva` 的 vault、session、SSH、业务路由或产品 CLI。
- **守住 core 边界**: GitHub/provider/平台细节只能留在 adapter、scripts、workflow、tests 与 docs，不进入 `common_core`。
- **共享命名不能回退**: 保持 `common dev/demo/docs/release` 词汇与 `rustwebappcommon-<platform>` 资产命名，不回退到产品专有命名。
- **结构性改动必更 `doc_auto`**: 每次结构性增强都同步更新对应 `doc_auto` 文档并追加最新时间戳。
- **每个 Step 都要验证**: 至少运行 `cargo test`、`python -m unittest tests.test_research_pack tests.test_starter_repo`，并按需要运行 `bash scripts/build-release.sh`、`LOCAL_RELEASE_DIR=release bash scripts/install.sh`、`LOCAL_RELEASE_DIR=release bash scripts/update-check.sh`。

## Skills
### Open URL
使用浏览器打开 URL 并阅读内容。
如果 URL 属于 *feishu.cn*，先让用户在浏览器中完成登录。

### Code Exploration
使用代码搜索与文件阅读确认 adapter、CLI、scripts、workflows、tests 与 `doc_auto` 的一致性，并定位 Enva 导向增强的真实落点。

### Parallel Subagent
使用并行 subagent 分拆多平台 release 设计、install/update 接缝细化、parity 测试设计与 `doc_auto` 同步策略整理。

## TODOs
### Phase 1: 固化增强范围与真相源（Step 1，无依赖，起始阶段）
- [ ] 1.1 以 `doc_auto/enva_gap_requirements.md`、`remaining_implementation_delta.md`、`implementation_handoff.md` 确认本轮起点与非目标。
- [ ] 1.2 明确本轮不再重复 starter repo、首轮 Enva 对齐矩阵与基础 release/update 闭环任务。
- [ ] 1.3 产出一份新的 Enva 导向增强范围说明，限定在 common 可复用能力内。

### Phase 2: 复核 Enva 剩余 gap 与优先级（Step 2，依赖 Phase 1）
- [ ] 2.1 对五类 gap 标记 `must_now`、`should_now`、`later`。
- [ ] 2.2 为每类 gap 指定落点目录与验证方式。
- [ ] 2.3 明确哪些能力进入 common，哪些继续保留为 app-owned 或 later。

### Phase 3: 设计共享扩展契约与测试策略（Step 3，依赖 Phase 2）
- [ ] 3.1 为多平台 release、install hook、updater seam、parity scaffolding、CLI harness 定义最小共享契约。
- [ ] 3.2 为每类增强补齐最小测试与 smoke 验证方案。
- [ ] 3.3 明确哪些字段可进入 adapter trait 或 release metadata，哪些必须留在 scripts/workflows/docs。

### Phase 4: 多平台 release 与安装 hook 增强（Step 4，依赖 Phase 3，与 Phase 5 并行）
- [ ] 4.1 扩展 build/release/workflow helper 以覆盖至少 Linux x86_64、Linux aarch64、macOS aarch64。
- [ ] 4.2 在安装流程中加入可选 post-flight smoke hook。
- [ ] 4.3 保持 `rustwebappcommon-<platform>`、`SHA256SUMS` 与共享 checksum 校验契约稳定。

### Phase 5: updater、parity 与 CLI harness 增强（Step 5，依赖 Phase 3，与 Phase 4 并行）
- [ ] 5.1 增加可复用的 updater extension seam，而不是直接复制产品级 binary self-replace。
- [ ] 5.2 增加双运行面对齐脚手架，覆盖 manifest、DOM hook 或 runtime marker 断言。
- [ ] 5.3 补齐 CLI integration harness 的最小示例、文档或测试模式。

### Phase 6: `doc_auto` 与 docs 同步封口（Step 6，依赖 Phase 4 和 Phase 5）
- [ ] 6.1 更新 `doc_auto` 中与 Enva gap、matrix、handoff 相关的真相源文档。
- [ ] 6.2 更新 release guide 与相关 architecture/docs 说明共享增强的使用边界。
- [ ] 6.3 在所有同步文档末尾追加最新修改时间。

### Phase 7: 回归验证与交接（Step 7，依赖 Phase 6）
- [ ] 7.1 运行 Rust 与 Python 回归测试，确认新增能力不破坏现有入口与 contract。
- [ ] 7.2 运行必要的 build/install/update/parity smoke checks。
- [ ] 7.3 输出下一步建议，明确尚未关闭的 desktop 路线、签名与更深 updater 议题。
