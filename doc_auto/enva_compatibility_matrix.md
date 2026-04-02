# Enva Compatibility Matrix

## 目标
本矩阵用于回答：`RustWebAppCommon` 与 `Enva` 在当前 follow-on 实现阶段需要对齐到什么程度，以及这些能力应该落在 `common_core`、adapter、脚本还是 GitHub workflow 中。

## 对齐级别定义
- `same_capability`：达到同能力面，不要求同一实现。
- `same_entry`：入口、命名或路径尽量可映射，但内部实现允许不同。
- `not_in_core`：能力重要，但只能落在 adapter、脚本或 workflow。
- `out_of_scope_now`：不是本轮关闭项，先记录为后续能力。

## 能力对齐矩阵
| Enva 能力 | Enva 位置 | RWC 当前状态 | 目标级别 | 决议与落点 |
|---|---|---|---|---|
| 完整静态 demo 站点 | `site/demo.html` | `site/index.html`、`site/demo.html`、`site/404.html`、`site/docs/index.html` 与 `site/js/main.js` 已形成完整页面地图与静态展示面 | `same_capability` | 保留 `site/` 作为 Pages 面，内容由 `web_demo_adapter` 生成的 manifests 驱动 |
| GitHub Pages 部署 | `.github/workflows/deploy-pages.yml` | Pages workflow 已存在，并改为在 CI 中重新生成 `site/` 后再上传 artifact | `same_capability` | `.github/workflows/deploy-pages.yml` |
| 本地 Web demo | `site/demo.html` + 嵌入式 UI | `common dev --surface web --host --port --route` 会重建 site 并启动本地 HTTP shell | `same_capability` | `web_demo_adapter` + `common_cli` |
| 本地 desktop preview | 本地产品壳层/预览面 | `common dev --surface desktop` 默认打开 `/detail/desktop-preview` 并启动 browser-backed preview | `same_capability` | `desktop_tauri_adapter` 保留为 seam，当前默认实现仍在 adapter 层 |
| 静态页面与本地运行面对齐测试 | `crates/enva/tests/static_pages.rs` | 已有 `common/cli/tests/static_site.rs`；本轮继续加固 shell、manifest、runtime contract 与 release contract 断言 | `same_capability` | Rust integration tests + Python repo tests |
| 构建脚本 | `build.sh` | `scripts/build-release.sh` 已生成 host 平台二进制、静态 `site/`、`SHA256SUMS` 与 `release-manifest.json` | `same_capability` | `scripts/build-release.sh` |
| 安装脚本 | `scripts/install.sh` | `scripts/install.sh` 支持本地 `release/` 或 GitHub Release 下载，并校验 `SHA256SUMS` | `same_capability` | `scripts/install.sh` |
| 安装后 smoke hook | 安装后产品自检命令 | `scripts/install.sh` 已支持 `RWC_POST_INSTALL_HOOK`，并在 hook 失败时返回非零退出码 | `same_capability` | `scripts/install.sh` + `docs/guides/release.md` |
| 更新检查 | `crates/enva/src/update.rs` | `scripts/update-check.sh` 支持本地 release 校验与 GitHub Release 资产发现；当前不做二进制自替换 | `same_capability` | `scripts/update-check.sh` + `release_pipeline_adapter` |
| release readiness / manifest 分析 | release metadata + product updater 前置检查 | `scripts/release-contract.sh` 与 `scripts/update-check.sh` 已支持 manifest schema、checksum、asset presence 与 migration readiness 报告 | `same_capability` | `scripts/release-contract.sh` + `scripts/update-check.sh` |
| 二进制自替换 updater | `crates/enva/src/update.rs` | 当前没有 in-process self-replace；只完成 release asset discovery 与 checksum contract | `out_of_scope_now` | 如后续需要，应继续落在 adapter / script 层，不进入 `common_core` |
| GitHub Releases 资产发布 | `update.rs` + release assets | tag push 时 release workflow 构建 bundle、上传 workflow artifact，并向 GitHub Release 附加二进制、`SHA256SUMS` 与 manifest | `same_capability` | `.github/workflows/release-artifacts.yml` |
| artifact naming | `enva-linux-x86_64` 等 | `rustwebappcommon-<platform>` 已由 shared shell contract 统一到 build/install/update | `same_entry` | `scripts/release-contract.sh` + scripts/workflows |
| CLI 命令风格 | `enva update`、`enva serve` 等 | 保留 `common dev/demo/docs/release` 词汇，但在 docs 中记录与 `Enva` 的能力映射 | `same_entry` | `common_cli` + docs，不复制产品命令名 |
| CLI subprocess / mock release API 测试脚手架 | `subprocess` + mock API + fixture workspace | 已有 `common/cli/tests/cli_harness.rs` 与 `tests/common_side_harness.py` / `tests/test_enva_migration_validation.py` 形成可复用 common 侧脚手架 | `same_capability` | Rust integration tests + Python harness |
| 目录布局 | `crates/` + `site/` | 保留 `common/` + `docs/` + `demo/` + `site/` + `doc_auto/` | `same_entry` | 借鉴能力分层，不复制目录 |
| docs 前门组织 | `site/` + README 组合入口 | `README.md`、`AGENTS.md`、`docs/index.md` 与 `site/docs/index.html` 组成 front door | `same_capability` | docs/demo/site 同步维护 |
| Vault / Secrets / 业务语义 | `enva-core` + web API | 当前公共仓库不承载业务能力 | `out_of_scope_now` | 后续 `app_owned` 或具体产品仓库 |

## 冲突点与裁决
### 目录布局
- 决议：保留 `common/`、`docs/`、`demo/`、`site/` 组织，不复制 `Enva` 的 `crates/` 目录结构。
- 理由：当前仓库的主目标是公共能力底座，目录要服务三层模型而不是复制某个产品仓库。

### CLI 命名
- 决议：保留 `common dev/demo/docs/release` 作为公共命令词汇。
- 理由：需要与 `common_core` 的 vocabulary 保持一致；只做 capability mapping，不复制 `enva` 产品命令名。

### 文档入口
- 决议：继续使用 `README + AGENTS + docs/index + demo/storyboard + site/docs/index.html` 的 front-door 组合。
- 理由：这条路径同时满足 human、main-agent、subagent 和静态 Pages 浏览。

### release / update / GitHub 能力
- 决议：对齐到“同一资产命名 + checksum + workflow 发布契约”，而不把 GitHub API 或 provider 细节写进 core。
- 理由：这是最接近 `Enva` 能力面的闭环，同时保持 `common_core` 干净。

### runtime / desktop 路线
- 决议：默认 web runtime 锁定为生成后的静态站点 + adapter HTTP shell；desktop 默认路线锁定为 browser-backed preview，并保留 `desktop_tauri_adapter` 作为长期 seam。
- 理由：当前这条路径已经可运行、可 smoke、可验证；原生壳层仍可后续演进，但不应阻塞本轮闭环。

## 与架构边界的关系
### 允许进入 `common_core`
- release metadata
- CLI vocabulary
- route schema
- docs/demo 共享 token 与术语

### 必须留在 adapter / scripts / workflow
- `site/` 产物与 runtime contract
- GitHub Pages workflow
- build / install / update 脚本
- GitHub Releases API 与 asset URL
- artifact naming 的 provider 落地逻辑

### 不能直接写进 `common_core`
- `YoRHa-Agents/EnvA` 或 `YoRHa-Agents/RustWebAppCommon` 这样的 repo 常量
- GitHub API URL
- Pages deploy 细节
- updater / provider 实现细节

## 当前实施顺序
1. 先刷新 truth sources 与 compatibility matrix。
2. 并行完成完整 demo 内容面与 `Enva` 对齐裁决。
3. 再统一 release / install / update / GitHub workflow 契约。
4. 最后补齐 parity 测试、migration validation 与 handoff 文档。

## Last Updated
- 2026-04-02T04:33:25+00:00
