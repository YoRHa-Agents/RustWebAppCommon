# Architecture Overview

## 三层模型
- `common_core`：共享协议、配置、路由、docs schema、theme token、release metadata
- `common_adapters`：运行面与 provider 适配层
- `app_owned`：页面内容、业务逻辑、业务路由、showcase

## 设计原则
- core 只定义契约，不绑定实现
- adapter 隔离差异，不反向污染 core
- app 保留内容和业务特性
- docs/demo 与核心术语保持一致

## 运行面
- web/demo：`web_demo_adapter`，默认 runtime 为生成后的静态站点 + 本地 HTTP shell
- docs：`docs_site_adapter`
- desktop：`desktop_tauri_adapter`，当前默认实现为 browser-backed preview
- release：`release_pipeline_adapter`，release metadata 在 core，GitHub/provider 细节留在 scripts/workflows

## 当前决议
- Pages 与本地 demo 共享 `site/`、route manifests、theme tokens 与 runtime contract
- desktop preview 默认进入 `/detail/desktop-preview`，作为当前可验证的桌面演示路径
- `build-release.sh`、`install.sh`、`update-check.sh` 与 release workflow 共享 `rustwebappcommon-<platform>` 资产命名与 `SHA256SUMS`

## 深入阅读
- `docs/architecture/core-contracts.md`
- `docs/architecture/adapters.md`
- `.local/research_pack/16_architecture_design_doc.md`
