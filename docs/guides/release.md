# Release Guide

## 当前策略
- release metadata 在 `common_core`
- 具体 CI/provider 在 `release_pipeline_adapter`
- desktop bundling/update 在 `desktop_tauri_adapter`
- GitHub Pages 静态托管 contract 只影响 docs/demo，不进入 core
- `rustwebappcommon-<platform>` 与 `SHA256SUMS` 是 build / install / update / workflow 的共享资产契约
- 多平台目标命名、manifest 校验与 release 目录校验统一留在 `scripts/release-contract.sh`

## 当前命令语义
```bash
cargo run -p common_cli -- release
bash scripts/build-release.sh
bash scripts/validate-release.sh
LOCAL_RELEASE_DIR=release bash scripts/install.sh
LOCAL_RELEASE_DIR=release bash scripts/update-check.sh
```

## 当前已实现
- `scripts/release-contract.sh` 会集中维护平台命名、asset contract、`release-manifest.json` schema 与 release 目录校验逻辑
- `scripts/build-release.sh` 会生成 `release/` 目录、host 平台二进制、静态 `site/`、`SHA256SUMS` 与 `release-manifest.json`，并在收尾阶段自校验 release contract
- `scripts/install.sh` 支持从本地 `release/` 或 GitHub Release 安装，并校验 `SHA256SUMS`；如存在 manifest，也会做 schema 校验
- `scripts/install.sh` 支持可选的 post-install smoke hook，下游产品可以在安装后执行自己的验证命令
- `scripts/update-check.sh` 支持本地 release 校验与 GitHub Release 当前平台资产发现，并能报告 `release-manifest.json` / `SHA256SUMS` 是否齐备
- `scripts/validate-release.sh` 会串联 build、release 目录校验、release-site parity、install hook 与 update-check，是推荐的 common 侧迁移验证入口
- `.github/workflows/deploy-pages.yml` 会在 CI 中运行 Rust tests、Python trio（含 `tests.test_enva_migration_validation`）、重新生成 `site/` 后发布到 Pages
- `.github/workflows/release-artifacts.yml` 现在会对 `linux-x86_64`、`linux-aarch64` 与 `macos-aarch64` 做真实矩阵构建，并在 tag push 时把多平台 binary、`SHA256SUMS`、聚合 `release-manifest.json` 与平台级 `release-manifest-<platform>.json` 一起发布到 GitHub Release

## 推荐验证入口
```bash
bash scripts/validate-release.sh
python -m unittest tests.test_research_pack tests.test_starter_repo tests.test_enva_migration_validation
```

## 迁移验证接缝
### Shared contract
- `RWC_TARGET_PLATFORM`：在 contract 校验或 matrix job 中指定目标平台，不把目标平台命名散落到 workflow
- `release-manifest-<platform>.json`：平台级 manifest，供 install / update 入口优先校验当前目标平台的 asset
- `release-manifest.json`：聚合 manifest，记录 release 中包含的多平台 asset 与对应平台 manifest 名称

### Install seam
- `RWC_POST_INSTALL_HOOK`：安装完成后执行自定义 smoke command
- `RWC_INSTALL_REQUIRE_MANIFEST=1`：要求安装来源必须带 `release-manifest.json`
- post-install hook 失败会明确返回非零退出码，供下游仓库或 CI 中止流程

### Update seam
- `RWC_UPDATE_REQUIRE_MANIFEST=1`：要求远端 release 必须包含 `release-manifest.json`
- `RWC_UPDATE_REQUIRE_CHECKSUMS=1`：要求远端 release 必须包含 `SHA256SUMS`
- `scripts/update-check.sh` 会优先识别 `release-manifest-<platform>.json`，找不到时再回退到聚合 `release-manifest.json`

### Downstream scaffold
- Rust CLI subprocess smoke：`common/cli/tests/cli_harness.rs`
- Python migration harness：`tests/common_side_harness.py`
- end-to-end release validation：`tests/test_enva_migration_validation.py`
- 下游产品可复用同样模式：临时安装目录、fixture release dir、mock release API、post-install hook

## 当前限制
- 还没有真实签名与 in-process binary self-update
- `desktop_tauri_adapter` 当前仍以 browser-backed preview 为默认实现，而不是原生桌面壳层
- Linux aarch64 与 macOS aarch64 当前先通过 contract-safe matrix 验证命名与 workflow 编排，真实原生产物构建仍应继续留在 scripts/workflows 层扩展
- `common_core` 不承载 GitHub API、产品 updater、产品命令词汇或 Enva 业务状态
