# 任务: RustWebAppCommon 完整 Demo 与 Enva 对齐后续实现任务
> 基于当前已经完成的 starter workspace、core contracts、adapter 最小真实路径、`site/` 静态演示站点与 release/install/update 基础脚本，继续推进 `RustWebAppCommon` 的下一阶段实现。本任务聚焦两件事：先把 Web + Desktop demo 做成真正可交付的完整演示面，再在不污染 `common_core` 的前提下，将框架能力推进到尽量与 `Enva` 做 1:1 对齐。

## Targets
1. **完整 Demo 交付**: 将当前 `site/`、本地 `common dev --surface web` 与 desktop preview 从“最小可运行”推进到“完整可演示”，验证方式为关键页面、静态输出与本地运行路径都具备一致内容和可验证结果。
2. **Enva 对齐闭环**: 基于 `doc_auto/enva_compatibility_matrix.md` 关闭目录、CLI、docs 入口、build/install/update/GitHub 能力的关键差距，验证方式为输出明确的兼容性决议与已实现能力映射。
3. **Release / GitHub 能力完善**: 强化 `site/`、Pages workflow、release artifact、install/update 路径，验证方式为脚本、workflow 和产物命名形成可执行闭环。
4. **运行时路线收敛**: 锁定默认 web runtime 与 desktop adapter 路线，验证方式为相关决策在 adapter 层显式落地，并写入 `doc_auto` 与设计/交接文档。
5. **防漂移验证**: 增加强约束测试，防止静态 demo、本地运行面、release/update 和 `doc_auto` 说明再次分叉，验证方式为 Rust/Python 回归测试和 smoke checks 全部通过。

## Acceptance Criteria
### Step 1: 固化当前实现与剩余差距（无依赖，起始步骤）
- [ ] 输出一份新的“已完成 / 未完成 / 下一阶段范围”决议，明确 starter 骨架阶段已结束。
- [ ] 明确本轮不再重复 `common_core` 骨架、基础 docs/demo 入口与基础测试搭建。
- [ ] 明确本轮优先事项是完整 demo、Enva 对齐、release/GitHub 闭环与运行时路线收敛。
- [ ] 明确 `Enva` 兼容性未关闭前，目录、CLI 命名和 docs 入口相关决策仍需保留可调整空间。

### Step 2: 完整 Web + Desktop demo 提升（依赖 Step 1）
- [ ] `site/` 至少覆盖 landing、runtime、docs-entry、release-flow、style-lab、detail 的完整内容展示，而不是只保留结构框架。
- [ ] `common dev --surface web --host H --port P --route /runtime` 能启动真实本地 demo 路径，并能访问关键页面。
- [ ] `common demo` 生成的静态树满足 GitHub Pages contract：`index.html`、`404.html`、资源路径、manifest 与 docs 索引都一致。
- [ ] desktop preview 至少具备一条真实可运行或清晰可验证的演示路径，而不是纯文本 plan 输出。

### Step 3: Enva 能力对齐与兼容性裁决（依赖 Step 1，与 Step 2 并行）
- [ ] 对 `Enva` 的 `site/`、`build.sh`、`install.sh`、`update.rs`、Pages workflow、`static_pages.rs` 建立 1:1 对齐矩阵。
- [ ] 每项能力都被标记为 `same_capability`、`same_entry`、`not_in_core` 或 `out_of_scope_now`，并写出理由。
- [ ] 明确目录布局、CLI 命名、docs 入口与 GitHub 能力的冲突点与处理决议。
- [ ] 对齐过程中不得把 GitHub/provider/runtime 细节写入 `common_core`。

### Step 4: Release / Update / GitHub 能力增强（依赖 Step 2 和 Step 3）
- [ ] `build-release.sh`、`install.sh`、`update-check.sh` 与 `.github/workflows/` 形成更接近 `Enva` 的最小闭环。
- [ ] artifact naming、`release/` 输出、`SHA256SUMS`、Pages deploy、install/update 体验具备可执行路径。
- [ ] release/update/GitHub 逻辑保留在 adapter、scripts、workflow 层，而不是进入 `common_core`。
- [ ] 至少一条 desktop/release/update/GitHub 路径不再只是 stub，而具备实际运行价值。

### Step 5: 运行时路线收敛、测试加固与交接封口（依赖 Step 4）
- [ ] 默认 web runtime 与 desktop adapter 的长期路线被明确记录，并解释为什么这样选。
- [ ] 增加类似 `Enva static_pages.rs` 的防漂移测试，锁住静态 demo 与本地运行面的一致性。
- [ ] 运行 `cargo test`、`python -m unittest tests.test_research_pack tests.test_starter_repo` 与关键 smoke checks 全部通过。
- [ ] `doc_auto/` 中的 delta、compatibility、gate、sync、handoff 文档全部更新到最新状态，足以让下一位 agent 无需回读全部历史计划即可接手。

## Context
### Repos:
- `/home/agent/workspace/RustWebAppCommon` — 当前主工作区，已具备 starter workspace、`site/`、scripts、workflow、docs/demo 入口和测试，是本轮后续实现的核心仓库。
  - `.local/task_follow_on_for_rust_web_app_common.md` — 本次生成的后续实现任务文档。
  - `.local/task_implementation_for_rust_web_app_common.md` — 上一阶段的最小实现任务文档，现主要作为背景输入。
  - `.local/research_pack/16_architecture_design_doc.md` — 当前架构设计主输入，固定了三层边界和运行面策略。
  - `.local/research_pack/17_pre_implementation_validation_checklist.md` — 实现前 Gate 定义，现已部分转化为真实仓库能力。
  - `common/core/src/lib.rs` — `common_core` 协议实现与 route/theme/release helpers。
  - `common/adapters/src/lib.rs` — adapter traits、静态站点生成、本地 web/desktop preview 与 release/update 相关接缝。
  - `common/cli/src/lib.rs` — 统一 CLI 协议与执行路径。
  - `site/index.html` — 静态站点主入口。
  - `site/demo.html` — 当前完整 demo 的静态演示入口。
  - `.github/workflows/deploy-pages.yml` — GitHub Pages 发布 workflow。
  - `scripts/build-release.sh` — 当前 release 产物生成脚本。
  - `scripts/install.sh` — 当前 install 脚本。
  - `scripts/update-check.sh` — 当前 GitHub Releases 检查脚本。
  - `doc_auto/enva_compatibility_matrix.md` — Enva 对齐矩阵。
  - `doc_auto/remaining_implementation_delta.md` — 当前剩余实现差距的真相源。
  - `tests/test_starter_repo.py` — starter / site / scripts / workflows / doc_auto 验证测试。
- `/home/agent/workspace/Enva` — 对齐目标仓库，用于参考完整 demo、build/install/update、GitHub 与产品级目录结构。
  - `site/demo.html` — 当前完整静态 demo 参考样本。
  - `.github/workflows/deploy-pages.yml` — Pages workflow 参考。
  - `build.sh` — 多目标构建脚本参考。
  - `scripts/install.sh` — 安装脚本参考。
  - `crates/enva/src/update.rs` — 更新能力参考。
  - `crates/enva/tests/static_pages.rs` — 静态页面与嵌入式 UI 对齐测试参考。
- `/home/agent/reference/openai-agents-python` — docs/index 与 examples 结构样本，供 docs/demo 导航继续参考。
  - `README.md` — 顶层项目入口样本。
  - `mkdocs.yml` — 多层 docs tree 组织样本。

### Docs:
**当前设计与实现基准:**
- `/home/agent/workspace/RustWebAppCommon/.local/research_pack/16_architecture_design_doc.md`: 当前架构设计总稿。
- `/home/agent/workspace/RustWebAppCommon/.local/research_pack/17_pre_implementation_validation_checklist.md`: Gate 与验证基线。
- `/home/agent/workspace/RustWebAppCommon/doc_auto/remaining_implementation_delta.md`: 当前已完成 / 未完成 / 下一阶段范围。
- `/home/agent/workspace/RustWebAppCommon/doc_auto/enva_compatibility_matrix.md`: Enva 1:1 对齐矩阵。
- `/home/agent/workspace/RustWebAppCommon/doc_auto/implementation_handoff.md`: 交接说明与 blocker。

**对齐参考:**
- `/home/agent/workspace/Enva/.github/workflows/deploy-pages.yml`: Pages 部署参考。
- `/home/agent/workspace/Enva/build.sh`: 构建脚本参考。
- `/home/agent/workspace/Enva/scripts/install.sh`: 安装脚本参考。
- `/home/agent/workspace/Enva/crates/enva/src/update.rs`: 更新路径参考。
- `/home/agent/workspace/Enva/crates/enva/tests/static_pages.rs`: 防漂移测试参考。

### Developer insights:
- **当前阶段已变化**: 仓库已经不再是“最小 starter 未完成”，而是进入“剩余实现与产品能力补齐”阶段。
- **真实站点已存在**: `site/`、Pages workflow、release/install/update 基础脚本已经落地，本轮重点是增强而不是首次创建。
- **Enva 对齐要守边界**: 1:1 对齐重点在 demo、build/install/update、GitHub 和 tests，不在 `common_core` 复制产品业务。
- **进度真相源已迁移**: 今后以仓库、`doc_auto/` 和测试为准，不以旧 task 复选框为准。
- **Web 已部分闭环**: 本地 `common dev --surface web`、`common demo`、`common docs`、`common release` 已可执行。
- **Desktop 仍未锁定**: desktop preview 目前仍偏演示用途，是否演进到真正原生壳层仍需本轮定案。
- **doc_auto 仍需长期策略**: 当前同步记录已足够支撑交接，但自动同步机制仍未决定。
- **GitHub 能力仍可增强**: 目前已具备 Pages 与 release artifact workflow，但距离 `Enva` 的产品级 GitHub / release 体验仍有差距。

### Editable Paths
- `/home/agent/workspace/RustWebAppCommon/.local/task_follow_on_for_rust_web_app_common.md` — 本次后续任务文档。
- `/home/agent/workspace/RustWebAppCommon/common/` — `common_core`、adapter 与 CLI 的后续实现根目录。
- `/home/agent/workspace/RustWebAppCommon/site/` — 静态站点与 demo 页面。
- `/home/agent/workspace/RustWebAppCommon/docs/` — docs/index、architecture、guides 与 theme 文档。
- `/home/agent/workspace/RustWebAppCommon/demo/` — storyboard 与页面职责说明。
- `/home/agent/workspace/RustWebAppCommon/scripts/` — build/install/update 等脚本。
- `/home/agent/workspace/RustWebAppCommon/.github/` — Pages / release workflows。
- `/home/agent/workspace/RustWebAppCommon/doc_auto/` — delta、compatibility、gate、sync、handoff 真相源。
- `/home/agent/workspace/RustWebAppCommon/tests/` — Python 结构回归测试。
- `/home/agent/workspace/RustWebAppCommon/common/cli/tests/` — Rust 集成测试。

### Agent Rules
- 在实现前优先使用 plan mode 明确方案。
- 存在歧义时先向用户确认。
- 进入实现阶段前先补齐对应测试。
- 使用 `.cursor/` 相关流程支持 develop-test-debug 循环。
- 对可拆分工作优先使用 subagent 并行下钻。

---

- **以现状为准**: 不再按空仓库或旧 task 复选框假设推进实现。
- **完整 demo 优先**: 先把 Web + Desktop demo 做成完整展示面，再扩展其它能力。
- **Enva 对齐不污染 core**: GitHub、release、install、update、provider 细节只能落在 adapter、scripts、workflow 层。
- **先裁决再锁定**: 在 `Enva` 兼容性复核和 runtime 选型未完成前，不关闭目录、CLI 命名和 docs 入口决策。
- 每个 Step 完成后运行相应验证：`cargo test`、`python -m unittest tests.test_research_pack tests.test_starter_repo`，以及必要的 demo/release smoke checks。

## Skills
### Open URL
使用浏览器打开 URL 并阅读内容。
如果 URL 属于 *feishu.cn*，先让用户在浏览器中完成登录。

### Code Exploration
使用代码搜索与文件阅读确认 `site/`、adapter、CLI、scripts、workflow 与 `doc_auto` 的一致性。

### Parallel Subagent
使用并行 subagent 分拆 `Enva` 对齐分析、desktop 路线决策、GitHub/release 能力细化与测试策略设计。

## TODOs
### Phase 1: 固化当前实现与剩余差距（Step 1，无依赖，起始阶段）
- [ ] 1.1 以 `doc_auto/remaining_implementation_delta.md`、`gate_decision_table.md`、`implementation_handoff.md` 为真相源确认当前状态。
- [ ] 1.2 明确下一阶段不再重复 starter 骨架与 stub 任务。
- [ ] 1.3 锁定完整 demo、Enva 对齐、release/GitHub 能力与运行时路线为本轮范围。

### Phase 2: Enva 对齐矩阵与冲突裁决（Step 2，依赖 Phase 1）
- [ ] 2.1 完善 `doc_auto/enva_compatibility_matrix.md`，补齐 same_capability / same_entry / not_in_core / out_of_scope_now 分类。
- [ ] 2.2 输出目录、CLI 命名、docs 入口、GitHub 能力的冲突决议。
- [ ] 2.3 明确哪些能力需要 1:1，对哪些能力保留差异。

### Phase 3: 完整 Web + Desktop demo 提升（Step 3，依赖 Phase 1，与 Phase 2 并行）
- [ ] 3.1 将 `site/` 从结构性 shell 提升为完整 demo 内容面。
- [ ] 3.2 强化本地 `common dev --surface web` 与 desktop preview 路径。
- [ ] 3.3 确保 `site/`、`404.html`、manifest、docs 索引与 Pages contract 持续一致。

### Phase 4: Release / Update / GitHub 能力增强（Step 4，依赖 Phase 2 和 Phase 3）
- [ ] 4.1 增强 build/install/update/release/GitHub 路径，向 `Enva` 能力面对齐。
- [ ] 4.2 决定 desktop adapter 的长期路线与默认 web runtime。
- [ ] 4.3 保持 provider 细节留在 adapter、scripts、workflow，而不进入 core。

### Phase 5: parity 测试、回归验证与交接封口（Step 5，依赖 Phase 4）
- [ ] 5.1 增加更强的静态 demo / 本地运行面防漂移测试。
- [ ] 5.2 运行并扩展 Rust/Python 测试与关键 smoke checks。
- [ ] 5.3 更新 `doc_auto/` 中的 delta、compatibility、gate、sync、handoff 文档到最终状态。
