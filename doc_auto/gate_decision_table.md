# Gate Decision Table

## 当前决议
| Gate | 当前状态 | 本轮处理方式 | 是否阻止当前后续实现 |
|---|---|---|---|
| Gate 1: starter repo 验证 | 已关闭 | starter workspace、contracts、docs/demo、scripts/workflows 已形成真实基线，不再重复首轮骨架任务 | 否 |
| Gate 2: `Enva` 兼容性复核 | 已关闭 | 已更新 `doc_auto/enva_compatibility_matrix.md`，并对目录、CLI、docs、release/update/GitHub 能力给出裁决 | 否 |
| Gate 3: `doc_auto` 落点决策 | 已处理 | 继续以 `doc_auto/` 作为同步记录区，并在本轮强化 truth-source 作用 | 否 |
| Gate 4: host 与 demo contract 验证 | 已通过并加固 | `site/`、`index.html`、`404.html`、本地 web/runtime 与 docs/front-door contract 已通过测试与 smoke checks 锁定 | 否 |
| Gate 5: release path 验证 | 已通过并闭环 | `build-release.sh`、`install.sh`、`update-check.sh`、Pages workflow 与 release workflow 已统一到共享资产契约 | 否 |
| Gate 6: Pages 与 migration validation 对齐 | 已通过并加固 | `deploy-pages.yml` 现已运行 `tests.test_enva_migration_validation`，避免只有 release workflow 才覆盖完整 migration validation | 否 |
| Gate 7: Enva tracked adoption / oracle portability | 已通过并转入下游 adoption | Enva 已新增 tracked adoption 文档与 installer hook 兼容层；后续重点转向 full downstream adoption 与终局 CI 关闭条件 | 否 |

## 本轮已完成范围
- 刷新 `doc_auto` 真相源并明确 starter baseline 已结束。
- 把 `site/` 与本地 web / desktop preview 提升到“可完整演示”的内容面。
- 完成 `Enva` 对齐矩阵与冲突裁决。
- 收敛 release / install / update / GitHub workflow 契约并强化测试。
- 把 Pages workflow 的验证基线提升到与 migration validation handoff 一致的层级。
- 把 Enva 的 tracked adoption/oracle 可移植性与 installer hook 兼容入口补到位。

## 当前仍为非目标
- 不把 GitHub/provider/runtime 细节写入 `common_core`。
- 不把 `desktop_tauri_adapter` 立即强行升级为唯一原生桌面路线。
- 不重新开启 starter workspace 的骨架搭建工作。

## 当前执行顺序
1. 先刷新 Phase 1 truth sources。
2. 再并行推进 `Enva` 对齐与 demo surface 提升。
3. 最后统一 release/runtime 决议并补齐 parity 测试与 handoff。

## Last Updated
- 2026-04-02T07:11:49+00:00
