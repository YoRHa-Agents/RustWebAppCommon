# Starter Repo Sync

## 当前实现映射
- 已创建 Rust workspace：`Cargo.toml`
- 已创建 core crate：`common/core`
- 已创建 adapters crate：`common/adapters`
- 已创建 cli crate：`common/cli`
- 已创建 docs/demo/examples/apps 骨架
- 已创建根级 `README.md` 与 `AGENTS.md`
- 已创建根级 `.gitignore`，忽略 `target/`、`release/` 与本地 IDE/Python cache，同时保留 `site/` 与 `Cargo.lock`
- 已创建 `site/` 静态演示站点、`site/docs/index.html` 与 `site/assets/runtime-contract.json`
- 已创建 `.github/workflows` 中的 Pages 与 release artifact workflow
- 已创建 `scripts/build-release.sh`、`scripts/release-contract.sh`、`scripts/install.sh`、`scripts/update-check.sh`
- 已创建 `scripts/validate-release.sh` 与 Enva 迁移验证脚手架测试

## 与设计稿的关系
- `16_architecture_design_doc.md` 中的三层模型已映射到 starter 目录
- `13_common_core_contracts.md` 中的最小 contracts 已落到 `common/core/src/lib.rs`
- `14_adapter_boundary_matrix.md` 中的 adapter 集已落到 `common/adapters/src/lib.rs`
- `15_docs_demo_information_architecture.md` 中的 docs/demo 入口已映射到 `docs/`、`demo/`、`examples/`
- 默认 web runtime、desktop preview 与 release asset contract 已回写到 docs、site manifests 与 `doc_auto`

## 仍待验证
- `doc_auto` 的长期自动同步方式
- 原生 desktop shell 是否替换当前 browser-backed preview
- 更深度的多平台构建 / signing / self-update 实现路线

## 当前验证结果
- `cargo test`：通过
- `cargo run -p common_cli -- dev --surface web --host 127.0.0.1 --port 8080 --route /runtime`：通过完整本地 runtime smoke
- `cargo run -p common_cli -- dev --surface desktop --route /detail/desktop-preview`：通过 browser-backed desktop preview smoke
- `python -m unittest tests.test_research_pack tests.test_starter_repo`：通过
- `cargo run -p common_cli -- demo`：通过并生成 `site/assets/*.json`
- `cargo run -p common_cli -- docs`：通过并生成 `site/docs/docs-index.json`
- `cargo run -p common_cli -- release`：通过并生成 `release/` 产物
- `scripts/install.sh`：可从本地 `release/` 安装并校验 `SHA256SUMS`
- `scripts/update-check.sh`：可校验本地 `release/` 资产并检查远端 release asset
- `scripts/validate-release.sh`：可串联 release dir validation、release-site parity、install hook 与 update readiness smoke

## Last Updated
- 2026-04-02T06:28:00+00:00
