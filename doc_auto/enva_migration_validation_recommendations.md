# Task: Enva 迁移验证建议与下游接线任务
> 基于 `RustWebAppCommon` 当前仓库能力、`Enva` 现有兼容基线，以及两边已经形成的对齐矩阵与 handoff 文档，生成一份面向 common 侧的迁移验证建议任务文档。范围聚焦 release/install/update-check、静态页面 parity、CLI subprocess 脚手架、以及 Enva 下游如何接入这些接缝；不把 vault、SSH、产品 API 或 updater 业务逻辑直接下放到 `common_core`。

## Targets
1. **验证边界固化**: 明确 `RustWebAppCommon` 当前已经承担的迁移验证面、Enva 仍需自持的 product 验证面，以及两者之间的 handoff 边界；验证方式为输出一份可执行的边界清单与非目标列表。
2. **发布链路验证建议**: 输出围绕 release contract、install hook、update-check、manifest/readiness 与多平台资产命名的验证建议；验证方式为建议项能直接映射到现有脚本、workflow 或 test harness。
3. **运行面与 parity 验证建议**: 输出静态 `site/`、Pages、runtime contract、CLI subprocess 与 mock release API 的验证建议；验证方式为建议项可关联到当前测试或新增 common 侧验证接缝。
4. **Enva 下游接线顺序**: 给出 Enva 团队采用 common 验证接缝的推荐顺序、前置条件和残留风险；验证方式为形成按依赖排序的 adoption checklist。
5. **文档化交接封口**: 生成一份可直接放入 `doc_auto/` 的任务文档，供后续 agent 或开发者不重读全部历史就能接手；验证方式为文档包含完整 Context、Acceptance Criteria、TODOs 与 follow-up 指引。

## Acceptance Criteria
### Step 1: 固化验证目标、真相源与边界（无依赖，起始步骤）
- [ ] 明确本任务以 `RustWebAppCommon` 为主仓库、`Enva` 为只读参考输入，不把 Enva 产品实现细节直接迁入 `common_core`。
- [ ] 输出一份 truth sources 清单，至少覆盖 `README.md`、`doc_auto/enva_compatibility_matrix.md`、`doc_auto/remaining_implementation_delta.md`、`docs/guides/release.md` 与 Enva 的 `docs/design/en/common_alignment.md`。
- [ ] 明确列出本任务的非目标：不修改 Enva 产品代码、不重命名 Enva CLI、不把 vault/SSH/session/API 逻辑迁入 common。
- [ ] 将 common 已有验证接缝与 Enva 仍待接线项拆分成两个独立列表，避免建议文档混淆“common 已实现”和“Enva 尚未采用”。
- [ ] 将 `Enva/.local/reimpl_for_enva.md` 标记为本地辅助参考，而不是可移植的唯一真相源。

### Step 2: 设计 release / install / update-check 验证建议（depends on Step 1）
- [ ] 输出 release contract 验证建议，覆盖资产命名、checksum、manifest、release-dir readiness 与本地/远端更新检查。
- [ ] 输出 install 验证建议，覆盖 `RWC_POST_INSTALL_HOOK`、安装后 smoke、自定义失败退出码和 manifest enforcement 的使用方式。
- [ ] 输出 update-check 验证建议，覆盖 release metadata、asset presence、migration readiness 报告和 delegated updater seam。
- [ ] 每条建议都必须映射到现有 common 侧入口，如 `scripts/release-contract.sh`、`scripts/install.sh`、`scripts/update-check.sh`、`scripts/validate-release.sh` 或相关 workflow。
- [ ] 明确写出 `scripts/build-release.sh` 与 `scripts/validate-release.sh` 的前后关系，以及 `RWC_INSTALL_REQUIRE_MANIFEST`、`RWC_UPDATE_REQUIRE_MANIFEST`、`RWC_UPDATE_REQUIRE_CHECKSUMS` 的推荐使用场景。

### Step 3: 设计静态页面、runtime 与 CLI harness 验证建议（parallel with Step 2）
- [ ] 输出 `site/`、Pages artifact、runtime contract 与 release-site parity 的验证建议，说明哪些检查属于 common、哪些仍需下游产品补齐。
- [ ] 输出 CLI subprocess 与 mock release API 测试建议，覆盖 temp workspace、真实二进制执行、fixture 管理和 fake release source。
- [ ] 建议项必须可映射到现有 common 侧脚手架，如 `common/cli/tests/cli_harness.rs`、`tests/common_side_harness.py`、`tests/test_enva_migration_validation.py`。
- [ ] 明确指出 Enva 的 embedded/static 双运行面 parity 仍是 product-owned 语义，common 只负责提供可复用的验证接缝与模式。
- [ ] 将 `common/cli/tests/static_site.rs` 与 `doc_auto/implementation_handoff.md` 纳入建议参考，避免只从 Enva 的 `static_pages.rs` 反推 common 侧实践。
- [ ] 对 `.github/workflows/deploy-pages.yml` 与 `.github/workflows/release-artifacts.yml` 的验证基线是否一致给出明确裁决，并记录 Pages 是否需要跑完整 migration validation。

### Step 4: 设计 Enva 下游 adoption 顺序与 follow-up（depends on Step 2 and Step 3）
- [ ] 形成一条明确的 adoption 顺序，至少覆盖 install hook、release validation、mock release API harness、product CI 接线与后续 matrix 扩展。
- [ ] 明确列出 Enva 仍需自持的验证面，如 product smoke command、binary self-replace updater、Axum route/product state 验证、SSH sync/conflict 逻辑。
- [ ] 给出“何时应该继续扩 common，何时应该停留在 Enva 下游”的裁决标准。
- [ ] 每个 follow-up 项都要说明它属于 `must_now`、`should_now` 还是 `later`。
- [ ] adoption 顺序必须显式覆盖：`validate-release.sh` -> `RWC_POST_INSTALL_HOOK` -> `tests/common_side_harness.py` / `cli_harness.rs` 复用 -> product CI 接线。

### Step 5: 文档化交接与质量自检（depends on Step 4）
- [ ] 生成的文档包含完整的 `Targets`、`Acceptance Criteria`、`Context`、`Skills` 与 `TODOs`，可直接作为 common 侧任务文档使用。
- [ ] `Context` 必须包含 repo 级文件说明、开发者洞察、可编辑路径和明确的 agent 规则。
- [ ] `TODOs` 必须形成按依赖排序的 phase 列表，且每个 phase 的粒度控制在单个 agent 约 30 分钟内可推进。
- [ ] 文档末尾追加更新时间，并在产出后完成一次质量自检评分，确认总分达到可交付阈值。
- [ ] 修正 `doc_auto` 中与本任务直接相关的更新时间与入口描述，避免 handoff 文档之间出现明显漂移。

## Context
### Repos:
- `/home/agent/workspace/RustWebAppCommon` — 当前迁移验证建议的主仓库，也是本任务的目标输出位置。
  - `README.md` — 公共底座定位、快速验证命令和当前边界说明。
  - `doc_auto/enva_compatibility_matrix.md` — common 与 Enva 的能力对齐矩阵，是验证建议的首要真相源。
  - `doc_auto/remaining_implementation_delta.md` — common 当前已完成项、未关闭缺口与后续范围说明。
  - `doc_auto/enva_gap_requirements.md` — common 侧尚需扩展的 Enva 相关能力沉淀。
  - `doc_auto/implementation_handoff.md` — 当前推荐验证命令与 handoff 汇总。
  - `scripts/release-contract.sh` — 资产命名、manifest/checksum 与 release readiness 相关 contract。
  - `scripts/build-release.sh` — release 目录生成与 preflight contract 自校验入口。
  - `scripts/install.sh` — install flow 与 `RWC_POST_INSTALL_HOOK` 的承载入口。
  - `scripts/update-check.sh` — release discovery、checksum/manifest 检查与 readiness 报告入口。
  - `scripts/validate-release.sh` — common 侧 release / install / update / parity 一体化验证入口。
  - `common/cli/tests/cli_harness.rs` — common CLI subprocess 脚手架。
  - `common/cli/tests/static_site.rs` — common 侧静态页面 / manifest / runtime contract 防漂移测试。
  - `tests/common_side_harness.py` — Python 侧 temp-dir / subprocess / mock release API 公共脚手架。
  - `tests/test_enva_migration_validation.py` — Enva 迁移验证相关的 common 侧测试聚合点。
  - `.github/workflows/release-artifacts.yml` — release bundle 与 GitHub Release assets 的 CI 落点。
  - `.github/workflows/deploy-pages.yml` — `site/` 再生成与 Pages artifact 发布的 CI 落点。
- `/home/agent/workspace/Enva` — 只读参考仓库，提供当前 product compatibility oracle。
  - `README.md` — Enva 对外承诺的 install、demo、update、web UI 与 CLI 行为基线。
  - `docs/design/en/common_alignment.md` — Enva 与 common 的 capability map 与边界说明。
  - `build.sh` — Enva release asset naming 和多平台构建契约参考。
  - `scripts/install.sh` — Enva 现有 installer 与 product smoke 期望参考。
  - `crates/enva/src/update.rs` — Enva 自更新语义、asset selection 和 verification 路径参考。
  - `crates/enva/tests/static_pages.rs` — Enva static demo / embedded UI parity 护栏参考。
  - `crates/enva/tests/cli_integration.rs` — Enva CLI / updater / exit-code 行为基线参考。
  - `crates/enva/tests/reimplementation_oracle.rs` — Enva 文档与 release 契约的补充 compatibility oracle。
  - `.local/reimpl_for_enva.md` — 本地任务稿，仅作 adoption 顺序辅助参考，不应成为唯一可移植真相源。

### Docs:
**Common 侧真相源：**
- `/home/agent/workspace/RustWebAppCommon/README.md`: 公共底座范围、验证命令与仓库结构入口。
- `/home/agent/workspace/RustWebAppCommon/doc_auto/enva_compatibility_matrix.md`: common 与 Enva 能力对齐矩阵。
- `/home/agent/workspace/RustWebAppCommon/doc_auto/remaining_implementation_delta.md`: 当前完成项、未关闭缺口与范围声明。
- `/home/agent/workspace/RustWebAppCommon/doc_auto/enva_gap_requirements.md`: common 需补能力的需求沉淀。
- `/home/agent/workspace/RustWebAppCommon/doc_auto/implementation_handoff.md`: 当前推荐命令与 handoff 汇总。
- `/home/agent/workspace/RustWebAppCommon/docs/guides/release.md`: build / install / update-check / release workflow 说明。

**Enva 侧参考基线：**
- `/home/agent/workspace/Enva/README.md`: 产品公开行为与 release/install/update/demo 描述。
- `/home/agent/workspace/Enva/docs/design/en/common_alignment.md`: Enva 与 common 的 reuse / extend / app-owned 边界。
- `/home/agent/workspace/Enva/docs/design/en/migration_adoption.md`: Enva tracked downstream adoption / oracle portability / roadmap 文档。
- `/home/agent/workspace/Enva/.local/reimpl_for_enva.md`: Enva 本地任务稿，仅作 adoption 顺序辅助参考。

### Developer insights:
- **Common 已不是骨架阶段**: `RustWebAppCommon` 当前已经有真实 `site/`、Pages workflow、release/install/update-check 和基础 parity 验证，不应把建议文档写成“从零开始搭建”。
- **验证建议必须区分责任归属**: common 侧只负责 contract、adapter seam、脚手架和 workflow，不负责 Enva 的 vault、SSH、session、route 业务语义。
- **install hook 是下游接线关键**: `RWC_POST_INSTALL_HOOK` 这类通用接缝只有被 Enva 接入 product smoke command 后，才能真正形成迁移闭环。
- **update-check 不等于 updater**: common 侧可以提供 readiness 和 delegated seam，但 Enva 的 in-process binary self-replace 仍属于 product-owned。
- **release 验证要围绕共享契约写**: 建议文档应围绕 manifest、checksum、asset naming、release-dir validation 等共享 contract，而不是写死任何产品 repo 常量。
- **CLI harness 是可复用资产**: common 侧 subprocess 与 mock release API 脚手架是重要复用点，建议应强调“复用模式”而不是只列出测试文件名。
- **迁移验证文档本身要可 handoff**: 这份文档放在 `doc_auto/`，后续 agent 需要能够直接用它决定 common 侧验证工作和 Enva 下游接线顺序。
- **`scripts/build-release.sh` 是验证前置**: `validate-release.sh` 依赖 release bundle 生成与 release-dir contract 自检，不应被描述成独立魔法入口。
- **README 验证入口并不完整**: common 根 `README.md` 的快速验证命令未完整覆盖 `tests.test_enva_migration_validation`，应以 `docs/guides/release.md` 和 `implementation_handoff.md` 为更权威入口。
- **`.local` 参考不是跨环境契约**: Enva 的 `.local/reimpl_for_enva.md` 在当前树存在，但它不适合作为所有开发者或 CI 环境都可依赖的唯一参考。
- **common 侧 `static_site.rs` 是 parity 对照面**: 设计建议时应并列参考 common 的 `static_site.rs` 与 Enva 的 `static_pages.rs`，不要只从 Enva 方向反推。
- **Pages gate 也是迁移 gate 的一部分**: 若 `site/**` 变更会影响 release/install/update/readiness 叙事，Pages workflow 也应运行 migration validation，而不只做 HTML 校验。
- **Enva 现在已有 tracked adoption 文档**: `docs/design/en/migration_adoption.md` 可替代 `.local` 成为 clean clone / CI 可见的 adoption handoff 入口。
- **Enva installer 已具备 hook 兼容层**: Enva 侧已可接受 `RWC_POST_INSTALL_HOOK` 作为兼容别名，但 full downstream adoption 仍需把产品 CI 与 release validation 继续接上。

### Editable Paths
- `/home/agent/workspace/RustWebAppCommon/doc_auto/enva_migration_validation_recommendations.md` — 本次产出的目标任务文档。
- `/home/agent/workspace/RustWebAppCommon/doc_auto/README.md` — 如需同步 `doc_auto` 导航时更新。
- `/home/agent/workspace/RustWebAppCommon/doc_auto/remaining_implementation_delta.md` — 如需同步“已完成/未完成”边界时更新。
- `/home/agent/workspace/RustWebAppCommon/README.md` — 如需修正 quick validation 入口时更新。
- `/home/agent/workspace/RustWebAppCommon/doc_auto/enva_compatibility_matrix.md` — 若建议文档要求修正 capability matrix 时参考。
- `/home/agent/workspace/RustWebAppCommon/doc_auto/enva_gap_requirements.md` — 如需与 gap 需求文档建立引用关系时参考。

### Agent Rules
- 在实现前优先使用 plan mode 明确方案。
- 存在歧义时先向用户确认。
- 进入实现阶段前先补齐对应测试。
- 使用 `.cursor/` 相关流程支持 develop-test-debug 循环。
- 对可拆分工作优先使用 subagent 并行下钻。
- ---
- **只写任务文档与相关索引**: 本任务主要生成和收口结构化 handoff 文档，以及必要的文档入口修正；不修改 common 或 Enva 的实现代码、测试逻辑和 workflow 行为。
- **行为验收优先**: Acceptance Criteria 必须描述“能验证什么结果”，不要写成“改了哪个文件/函数”。
- **common / Enva 责任拆分明确**: 每个建议项都要标注它属于 common 已实现能力、common 可扩展能力，还是 Enva 仍需自持的 product 验证面。
- **Phase 粒度受控**: 每个 TODO phase 应控制在单个 agent 约 30 分钟可推进的粒度，避免写成大而空的迁移工程。
- **不污染 common_core**: 文档里必须反复强调不把 updater/provider/repo 常量、vault、SSH、route 语义直接写进 `common_core`。
- **优先使用 tracked truth sources**: `.local`、个人环境变量和本地目录结构只能作为辅助说明，tracked docs/tests 才是主真相源。

## Skills
### Open URL
使用浏览器打开 URL 并阅读内容。
如果 URL 属于 *feishu.cn*，先让用户在浏览器中完成登录。

### Code Exploration
使用代码搜索与文件阅读确认 common 侧 release/install/update-check、CLI harness、静态页面与 Enva 兼容基线之间的关系。

### Parallel Subagent
使用并行 subagent 分拆 common contract、测试脚手架、release/install/update-check 与 Enva 下游 adoption 等子问题。

## TODOs
### Phase 1: 固化真相源与责任边界（Step 1，无依赖，起始阶段）
- [ ] 1.1 列出 common 侧 truth sources 与 Enva 只读参考输入。
- [ ] 1.2 明确 common 已负责的验证面、Enva 仍自持的 product 验证面与非目标列表。
- [ ] 1.3 固化建议文档中的责任边界术语：`common-side validation`、`downstream adoption`、`product-owned validation`。
- [ ] 1.4 将 `.local/reimpl_for_enva.md` 降级为本地辅助参考，并把 tracked docs/tests 设为主真相源。

### Phase 2: 设计 release / install / update-check 建议（Step 2，depends on Phase 1）
- [ ] 2.1 梳理 release contract、manifest、checksum、asset naming 与 readiness 验证建议。
- [ ] 2.2 梳理 install hook、post-install smoke 与 manifest enforcement 建议。
- [ ] 2.3 梳理 update-check / delegated updater seam 的建议与限制。
- [ ] 2.4 将 `scripts/build-release.sh`、`scripts/validate-release.sh`、`RWC_INSTALL_REQUIRE_MANIFEST` 与 `RWC_UPDATE_REQUIRE_*` 串成一条验证链路叙事。

### Phase 3: 设计 runtime / parity / harness 建议（Step 3，parallel with Phase 2）
- [ ] 3.1 梳理 `site/`、Pages、runtime contract 与 release-site parity 建议。
- [ ] 3.2 梳理 CLI subprocess、temp workspace 与 mock release API 脚手架建议。
- [ ] 3.3 明确 common 提供的可复用验证模式与 Enva 仍需自持的 embedded/static product parity。
- [ ] 3.4 并列纳入 `common/cli/tests/static_site.rs`、`tests/test_enva_migration_validation.py` 与 `doc_auto/implementation_handoff.md` 作为推荐依据。

### Phase 4: 设计 Enva 下游 adoption 顺序（Step 4，depends on Phase 2 and Phase 3）
- [ ] 4.1 给出 Enva 接入 common 验证接缝的推荐顺序。
- [ ] 4.2 标出 `must_now`、`should_now`、`later` 三个优先级层级。
- [ ] 4.3 输出不应继续下放到 common 的 product-owned follow-up 列表。
- [ ] 4.4 明确 `validate-release.sh`、`RWC_POST_INSTALL_HOOK`、harness 复用与 product CI 接线的前后依赖关系。

### Phase 5: 生成 handoff 文档并完成自检（Step 5，depends on Phase 4）
- [ ] 5.1 写入完整任务文档结构：Targets、Acceptance Criteria、Context、Skills、TODOs。
- [ ] 5.2 在文末追加更新时间，并确保 `doc_auto/` 场景下可以直接 handoff。
- [ ] 5.3 按质量 rubric 做一次自检，若总分低于可交付阈值则先修正文档再交付。
- [ ] 5.4 同步 `doc_auto/README.md` 与直接相关 handoff 文档的时间戳，避免读者误判新旧顺序。

## Priority
### must_now
- release contract、manifest、checksum、release-dir readiness 建议
- `RWC_POST_INSTALL_HOOK` 与 install 后 product smoke 接线建议
- runtime/site parity、CLI subprocess 与 mock release API harness 建议

### should_now
- Enva 下游 adoption 顺序、product CI 接线顺序与 matrix 扩展建议
- 将 README-only 的快速验证入口与更权威的 release/handoff 指引做边界说明
- 将 Pages workflow 与 release workflow 的验证基线差距继续收敛到统一口径

### later
- in-process binary self-replace
- signing / provenance
- 更深的多平台原生产物构建与原生 desktop shell 深化

## Quality Self-Check
- D1 Scout: 5/5
- D2 Decomposition: 5/5
- D3 Executability: 5/5
- Weighted total: 5.0

## Last Updated
- 2026-04-02T07:11:49+00:00

