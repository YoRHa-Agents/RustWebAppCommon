# RustWebAppCommon
> follow-on implementation 阶段的公共底座：完整 demo、Enva 对齐、release/GitHub 闭环与防漂移验证。

## 这是什么
`RustWebAppCommon` 是一个面向多仓复用的公共底座。当前仓库提供：
- `common_core` 的最小协议实现
- adapter 的最小真实路径与统一 CLI 语义
- docs/demo 的前门入口与 `site/` 完整静态演示站点
- GitHub Pages、release、install、update 的共享资产契约与 workflow
- 面向后续 starter repo 的验证与交接材料

## 当前状态
- 已有 research pack 与 architecture design
- 已关闭 starter workspace 基线阶段
- 已有 `site/`、本地 web runtime 与 browser-backed desktop preview
- 已有 build / install / update / Pages / release workflows 的共享资产命名契约
- 正在收敛 `Enva` 兼容性裁决与更强 parity 测试
- 原生 desktop shell 仍保留为后续可选路线，不阻塞当前交付

## 快速入口
- 人类入口：`docs/index.md`
- agent 入口：`AGENTS.md`
- 架构设计：`.local/research_pack/16_architecture_design_doc.md`
- 实现前 Gate：`.local/research_pack/17_pre_implementation_validation_checklist.md`

## 快速验证
```bash
cargo test
python -m unittest tests.test_research_pack tests.test_starter_repo
python -m unittest tests.test_enva_migration_validation
cargo run -p common_cli -- dev --surface web --host 127.0.0.1 --port 8080 --route /runtime
cargo run -p common_cli -- dev --surface desktop --route /detail/desktop-preview
cargo run -p common_cli -- demo
cargo run -p common_cli -- docs
cargo run -p common_cli -- release
bash scripts/validate-release.sh
bash scripts/build-release.sh
LOCAL_RELEASE_DIR=release bash scripts/install.sh
LOCAL_RELEASE_DIR=release bash scripts/update-check.sh
```

## 仓库结构
```text
common/
  core/
  adapters/
  cli/
docs/
demo/
site/
examples/
apps/
doc_auto/
.github/workflows/
scripts/
```

## 当前不做的事
- 不在 `common_core` 中绑定 Dioxus、Tauri、Trunk 或 GitHub Pages 的具体实现。
- 不在 `common_core` 中绑定 GitHub Releases API、install 逻辑或 Pages deploy 细节。
- 不把 `Enva` 的产品业务能力直接复制到公共仓库。
