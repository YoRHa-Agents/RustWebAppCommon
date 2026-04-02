# Remaining Implementation Delta

## 当前已经完成
- starter workspace、`common_core` 最小契约、adapter 最小真实路径与统一 CLI 语义已经完成，本仓库不再处于“等待骨架落地”的阶段。
- `common dev --surface web`、`common demo`、`common docs`、`common release` 都已经有真实执行路径，不再只是 plan 输出。
- `site/` 已补足 landing、runtime、docs-entry、release-flow、style-lab 与 detail 内容面，并通过 `runtime-contract.json` 固化默认 web/runtime 与 desktop preview 决议。
- Pages workflow 会在 CI 中重新生成 `site/`，release/install/update 路径已经通过 shared asset contract 收敛到 `rustwebappcommon-<platform>` + `SHA256SUMS`。
- `README.md`、`docs/index.md`、`demo/storyboard.md`、architecture/guides 与 `doc_auto/` 已同步到 follow-on 实现阶段。
- Rust/Python 回归、release/install/update 本地 smoke、served web/runtime smoke、served desktop-preview smoke 与 `site/js/main.js` 语法检查均已通过。
- 已新增 common 侧 Enva 迁移验证接缝：`RWC_POST_INSTALL_HOOK`、manifest/readiness 报告、`scripts/validate-release.sh`、CLI subprocess harness、mock release API harness。
- 已新增并刷新面向 Enva 团队的交付文档：`doc_auto/enva_migration_validation_recommendations.md`，其中已明确 truth sources、adoption 顺序与 quality self-check。
- Pages workflow 现已补齐 `tests.test_enva_migration_validation`，不再只有 release workflow 覆盖完整 migration validation 基线。

## 当前仍未关闭
- `doc_auto` 的长期自动同步机制仍未决定。
- browser-backed desktop preview 之后是否继续演进为更原生的桌面壳层仍是后续路线问题。
- in-process binary self-update、签名与更深的多平台原生构建细节仍未实现。
- Enva 自身仍需要把 product smoke command、product updater 与产品级 CI 校验接到新的 common-side seams 上。

## 本轮明确范围
1. follow-on implementation 的完整 demo、`Enva` 对齐矩阵、release/runtime 决议与 parity 验证已经闭环。
2. 后续范围应转向原生桌面壳层深化、真正的 updater/signing、多平台原生构建扩展、Enva 下游接线与 `doc_auto` 自动同步。

## 本轮明确非目标
- 不把 `Enva` 的 repo 常量、GitHub API 细节或 provider 逻辑直接写进 `common_core`。
- 不为了追求“原生桌面壳层”而打破当前已经可验证的 browser-backed desktop preview。
- 不重复 starter workspace、最小 contracts、基础 docs/demo 入口或基础测试的首轮搭建工作。

## 执行顺序说明
- 以 `.local/task_follow_on_for_rust_web_app_common.md` 的 `## TODOs` Phase 依赖关系为准推进，而不是按 Acceptance Step 编号字面顺序推进。
- 当前建议顺序为：Phase 1 -> (Phase 2 并行 Phase 3) -> Phase 4 -> Phase 5。

## 进度真相源
- 仓库中的实际代码、脚本、workflow 与 `site/` 输出。
- `doc_auto/` 中与实现同步的记录。
- `cargo test`、`python -m unittest tests.test_research_pack tests.test_starter_repo` 与必要 smoke checks。

不要再以 `.local/task_implementation_for_rust_web_app_common.md` 中未勾选的复选框作为唯一进度来源。

## Last Updated
- 2026-04-02T06:32:04+00:00
