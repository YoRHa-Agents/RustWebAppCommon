# Development Guide

## 目标
统一本地开发、静态 demo 构建与 desktop preview 验证入口。

## 当前命令语义
```bash
cargo run -p common_cli -- dev --surface web --host 127.0.0.1 --port 8080 --route /runtime
cargo run -p common_cli -- dev --surface desktop --route /detail/desktop-preview
cargo run -p common_cli -- docs
cargo run -p common_cli -- demo
cargo run -p common_cli -- release
```

## 当前验证
```bash
cargo test
python -m unittest tests.test_research_pack tests.test_starter_repo tests.test_enva_migration_validation
bash scripts/validate-release.sh
```

## 说明
- 当前 `dev --surface web` 会重建 `site/`，并以本地 HTTP shell 启动默认 web runtime。
- 当前 `dev --surface desktop` 会尝试打开 `/detail/desktop-preview`；若本机无法自动打开浏览器，仍会启动本地服务并打印错误。
- `host` / `port` / `route` 语义已经固定，可供后续 adapter 实现复用。
- `demo` 与 `docs` 会生成/刷新 `site/assets/route-manifest.json`、`site/assets/theme-tokens.json`、`site/assets/runtime-contract.json` 与 `site/docs/docs-index.json`。
- 静态前端会根据当前 pathname 推断 base path，确保 project Pages 子路径和 `/detail/*` 这类 nested route 都能正确读取 manifests。

## Enva 迁移验证脚手架
- `common/cli/tests/cli_harness.rs` 使用真实 `common` 二进制做 subprocess smoke，覆盖 `demo` 与 `docs` 的最小 CLI 入口。
- `tests/common_side_harness.py` 提供临时目录、mock release API 与 subprocess 执行辅助，用于下游仓库复用 common 侧验证模式。
- `tests/test_enva_migration_validation.py` 负责 release/install/update smoke、release-site parity 与远端 release readiness 报告。
- 如需给下游产品定制安装后验证，优先通过 `RWC_POST_INSTALL_HOOK` 注入命令，而不是把产品逻辑写回 common 脚本。
