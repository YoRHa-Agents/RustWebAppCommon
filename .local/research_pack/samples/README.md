# Rust Web Common 样本目录
> 更新时间: 2026-04-01

## 说明
本目录存放按 `02_evidence_schema.md` 记录的正式样本 dossier。每个样本都必须包含：
- 可复查的来源路径或 URL
- `common` / `app` 边界信号
- 风险、迁移成本与未解问题
- 至少 2 条直接证据

## 当前样本
### 本地 baseline
1. `local_openai_agents_python.md`
   - 文档入口、分主题导航、运行器式索引样本。
2. `local_agentic_context_engine.md`
   - 渐进式 examples hub、AGENTS 规则与设计文档锚点样本。
3. `local_contextos.md`
   - 平台级 README/wiki 分层叙事与统一入口表达样本。

### 官方外部样本
4. `official_dioxus.md`
   - Rust 跨平台 UI、Dioxus CLI、GitHub Pages 发布与路由样本。
5. `official_tauri_v2.md`
   - 桌面壳层、release/updater、构建与分发流水线样本。
6. `official_trunk.md`
   - Rust WASM 静态构建、`public_url`、本地 serve 入口样本。
7. `official_github_pages.md`
   - 静态托管、`/docs` 发布、`index.html`/`404.html` 边界样本。

## 使用顺序
1. 先阅读 3 个本地 baseline，把 docs/index/style 的模式装进对比矩阵。
2. 再阅读外部样本，补足 `build/release/CLI` 与 `deploy/demo/routing` 证据。
3. 最后把结论汇总回 `04_comparison_matrix.md` 与最终研究报告。
