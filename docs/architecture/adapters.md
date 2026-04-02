# Adapters

## 当前 adapter stub
- `web_demo_adapter`
- `docs_site_adapter`
- `desktop_tauri_adapter`
- `release_pipeline_adapter`
- `remote_docs_review_adapter`

## 边界规则
- adapter 可以依赖 core 契约
- adapter 不得重新定义 core 术语
- adapter 不得把 provider/host 细节写回 `common_core`
- adapter 可以被替换，core 语义必须保持稳定

## 当前角色
- `web_demo_adapter`：本地 web/demo 启动与静态 demo 产物规划
- `docs_site_adapter`：docs/index 站点规划
- `desktop_tauri_adapter`：桌面壳层与 bundle/update 接缝
- `release_pipeline_adapter`：release/signing/CI 接缝
- `remote_docs_review_adapter`：读取本机 `~/.ssh/config`、归一化 SSH host alias，并以只读方式发现远程 doc/design 目录

## 当前 remote review seam
- `remote_docs_review_adapter` 只负责 adapter-local SSH 配置读取、只读目录发现与文件类型归一化，不把 SSH provider、session、vault 或 remote path state 写回 `common_core`
- `common_cli review` 是该 adapter 的公共入口：`--list-hosts` 用于列 host，`--ssh-host` 用于走只读 remote review
- remote write、sync、deploy、conflict resolution 与产品级 review workflow 继续留给 `app_owned`

## 当前 release / updater seam
- `release_pipeline_adapter` 继续只承载 release / CI / signing / updater 的 adapter 责任边界，不把 provider 细节写回 `common_core`
- `scripts/release-contract.sh` 承担共享 asset naming、manifest schema 与 release 目录校验
- `scripts/install.sh` 通过 `RWC_POST_INSTALL_HOOK` 提供安装后 smoke seam
- `scripts/update-check.sh` 只负责发现、校验与 readiness 报告；产品级 binary replacement 仍留给下游 adapter 或产品脚本

## 代码位置
- `common/adapters/src/lib.rs`
