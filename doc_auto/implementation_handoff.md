# Implementation Handoff

## 当前已确认的基线
- starter repo 阶段已经结束；当前仓库已经具备 workspace、core contracts、adapter 最小真实路径、统一 CLI、`site/`、scripts、workflow 与基础测试。
- 本轮实现已经完成完整 demo、`Enva` 对齐、release/update/GitHub 闭环和运行时路线收敛。
- 当前仓库已新增 readonly remote review seam：adapter-local `~/.ssh/config` 解析、host alias 归一化、只读远程目录发现与 `common review` CLI 入口。
- research pack / truth-source docs 现已补齐 `remote_docs_review_adapter`、`common review` 与 readonly remote review 的边界说明，不再只有实现代码与 handoff 记录提及该能力。
- 当前推进应以仓库现状和 `doc_auto/` 为准，不再以旧 task 中未勾选的 starter 复选框为准。

## 当前待关闭项
- `doc_auto` 长期自动同步机制仍未决定。
- browser-backed desktop preview 之后是否继续演进为更原生的桌面壳层仍待后续定案。
- in-process binary self-update、签名与更深的多平台构建能力仍未实现。
- Enva 下游仍需把 product smoke command、product updater 与产品级 CI 校验接到 common 侧验证接缝上。
- live SSH transport policy、session/auth 管理、remote write/sync/deploy 与产品级 review workflow 仍保持 app-owned，不属于当前 remote review seam。

## 当前已验证结果
1. `cargo test`：通过。
2. `python -m unittest tests.test_research_pack tests.test_starter_repo tests.test_enva_migration_validation`：通过。
3. `cargo run -p common_cli -- demo` 与 `cargo run -p common_cli -- docs`：通过，并刷新 route/theme/runtime/docs manifests。
4. review parser / fixture smoke：通过，已覆盖 `review --list-hosts`、SSH host alias 解析、multi-host catalog、invalid host issue、explicit `--path` 去重与 readonly remote review summary。
5. `bash scripts/build-release.sh`、`LOCAL_RELEASE_DIR=release bash scripts/install.sh`、`LOCAL_RELEASE_DIR=release bash scripts/update-check.sh`：通过。
6. served web/runtime 与 served desktop-preview smoke checks：通过。
7. `bash scripts/validate-release.sh`：通过，已覆盖 release dir validation、release-site parity、install hook 与 local update readiness。
8. `tests.test_enva_migration_validation`：通过，已覆盖 target override、release smoke、mock release API 与 migration readiness 报告。
9. Pages workflow 与 release workflow 现在都包含 migration validation 基线，不再只有 release 路径跑完整 Python trio。

## 建议命令
```bash
cargo test
python -m unittest tests.test_research_pack tests.test_starter_repo
python -m unittest tests.test_enva_migration_validation
cargo run -p common_cli -- dev --surface web --host 127.0.0.1 --port 8080 --route /runtime
cargo run -p common_cli -- dev --surface desktop --route /detail/desktop-preview
cargo run -p common_cli -- demo
cargo run -p common_cli -- docs
cargo run -p common_cli -- review --list-hosts
cargo run -p common_cli -- review --ssh-host review-host --config /tmp/review-ssh --path /srv/reviews
cargo run -p common_cli -- release
bash scripts/validate-release.sh
LOCAL_RELEASE_DIR=release bash scripts/install.sh
LOCAL_RELEASE_DIR=release bash scripts/update-check.sh
```

## 交付给 Enva 的建议文档
- `doc_auto/enva_migration_validation_recommendations.md`：总结 common 已完成的迁移验证、Enva 仍需跟进的产品级动作，以及建议接入顺序。

## Remote Review Handoff
- common owns the readonly remote review seam in adapters / CLI / docs / tests / `doc_auto`.
- Enva keeps live SSH policy、session、vault、remote write/sync/deploy 与产品级 review 语义。
- downstream repos should treat `common review` as a generic inspection surface, then layer product-specific workflows on top.

## Last Updated
- 2026-04-02T09:50:57+00:00
