# AGENTS
> 面向主 agent / subagent 的最小入口契约。

## 先读顺序
1. `README.md`
2. `docs/index.md`
3. `.local/research_pack/16_architecture_design_doc.md`
4. `.local/research_pack/17_pre_implementation_validation_checklist.md`

## 当前仓库边界
- `common/core/`：协议与共享契约，禁止写入 host/runtime/provider 细节。
- `common/adapters/`：adapter stub 和 provider 接缝。
- `common/cli/`：统一命令协议与最小 dispatch。
- `docs/`、`demo/`：信息架构与展示层，不承载业务逻辑。
- `doc_auto/`：实现与文档同步记录区。

## 不要猜的内容
- 不要假定最终默认 web runtime 已经锁定。
- 不要假定 `desktop_tauri_adapter` 是唯一桌面路径。
- 不要假定 `Enva` 已经兼容当前目录与命令命名。
- 不要假定 `doc_auto` 已有自动同步流程。

## 当前工作规则
- 先维护 `common_core` 契约，再维护 adapter。
- docs/demo 与实现词汇必须保持一致。
- 任何新增代码改动都要同步更新 `doc_auto/` 中对应说明并追加修改时间。
- 若修改 docs/demo 入口或 research pack 索引，运行：
  - `python -m unittest tests.test_research_pack tests.test_starter_repo`
- 若修改 Rust 代码，运行：
  - `cargo test`
