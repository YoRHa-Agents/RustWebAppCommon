# RustWebAppCommon 实现前验证清单
> 更新时间: 2026-04-02 09:50 UTC

## 目标
本清单用于明确：哪些内容在架构设计阶段已经足够确定，哪些内容在进入 starter repo 与实现阶段前必须再次验证。

## Gate 1: starter repo 验证
| 检查项 | 必须达到的条件 | 证据 |
|---|---|---|
| 分层可落地 | `common_core / common_adapters / app_owned` 能映射到真实目录 | starter repo 目录草图 |
| CLI 可执行 | `dev / demo / docs / release` 至少有 stub 级映射 | 命令接口草案 |
| RouteDescriptor 可用 | 页面地图能表达 demo 与 docs 页面 | 路由草图 |
| docs/demo 共存 | `README + AGENTS + docs/index + demo/storyboard` 能共存且不冲突 | 目录与入口图 |

## Gate 2: `Enva` 兼容性复核
| 检查项 | 必须达到的条件 | 证据 |
|---|---|---|
| 目录兼容 | `common/`、`docs/`、`demo/` 结构不与本地约束冲突 | `Enva` 差异说明 |
| CLI 命名兼容 | 统一命令不会与现有脚本冲突 | 命名映射表 |
| docs 入口兼容 | `README + AGENTS + docs/index` 能与本地文档体系共存 | 文档入口对照 |
| readonly review 边界兼容 | `common review` 保持 generic readonly inspection，不吸收 session/vault/remote write 语义 | 边界裁决说明 |

## Gate 3: `doc_auto` 落点决策
| 检查项 | 必须达到的条件 | 证据 |
|---|---|---|
| 文档同步位置 | 明确 `doc_auto` 是否引入、引入到哪里 | 文档治理决策 |
| 设计文档同步 | architecture/design docs 与实现代码的同步规则明确 | 维护约束 |

## Gate 4: host 与 demo contract 验证
| 检查项 | 必须达到的条件 | 证据 |
|---|---|---|
| GitHub Pages 合约 | `/docs`、`index.html`、`404.html` 规则可满足 | 静态 demo 骨架 |
| path 配置 | `base_path` / `public_url` 能映射到静态资源 | 构建配置草案 |
| 本地/在线一致性 | 本地 `host` / `port` 与在线静态入口的语义不冲突 | 启动说明 |

## Gate 5: release path 验证
| 检查项 | 必须达到的条件 | 证据 |
|---|---|---|
| desktop adapter 可选 | Tauri 路径可选而非强制 | adapter 实现策略 |
| release metadata 可映射 | `ReleaseDescriptor` 能映射到实际流水线 | release contract |
| 签名/更新不污染 core | updater/signing 仍保留在 adapter 层 | 边界复核 |

## 当前状态
- **已完成**:
  - 架构分层设计
  - core contracts 设计
  - adapter 边界设计
  - docs/demo IA 设计
  - starter repo 基线实现
  - `doc_auto` 作为实现同步记录区引入
  - readonly remote review seam 已在 adapter / CLI / docs / tests 中落地，并保持只读边界
- **待验证**:
  - `Enva`
  - release path 落地
  - browser-backed desktop preview 之后是否需要切换到更原生壳层

## 放行规则
- 只有当 Gate 1 和 Gate 2 通过后，才允许进入 starter repo 与实现。
- Gate 3 可在实现启动前完成，但不能拖到实现中后期才决定。
- Gate 4 和 Gate 5 至少需要 stub 级验证，才能宣布“设计已可实现”。

## 实现前必须回答的问题
1. `common_core` 的目录结构是否真的能放进真实仓库？
   - 已回答：可以，当前 `common/` 目录已映射三层模型。
2. 哪个 web runtime 会成为 starter repo 的默认候选？
   - 已回答：生成后的静态站点 + adapter HTTP shell。
3. Tauri 是默认桌面 adapter 还是可选 adapter？
   - 已回答：`desktop_tauri_adapter` 仍是 seam，但当前默认预览实现为 browser-backed preview。
4. `doc_auto` 是设计阶段引入，还是实现阶段引入？
   - 已回答：实现阶段引入，作为 truth-source 同步记录区。
5. `Enva` 是否要求调整 CLI 或 docs 入口命名？
   - 当前裁决：保持 `common dev/demo/docs/release` 与 `README + AGENTS + docs/index` 入口，不复制产品命名。
6. readonly remote review 是否进入 `common_core`？
   - 已回答：不会；它只停留在 adapter / CLI / docs / tests，并通过 `common review` 暴露 generic inspection flow。
