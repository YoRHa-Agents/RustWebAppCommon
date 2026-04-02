# Rust Web Common 证据阈值缺口清单
> 更新时间: 2026-04-01

## 目的
本清单在正式外部调研展开前，明确主线 A/B/C 各自还缺哪些证据，避免后续采样继续停留在“有印象但不够决策”的状态。

## 当前状态（已纳入本地 baseline 与外部官方样本）
| 主线 | 当前覆盖 | 是否达标 | 主要缺口 |
|---|---|---|---|
| A. Common 工程与运行时 | 已有 `Dioxus` 与 `Tauri v2` 两个带 `build/release/CLI` 证据的样本 | 是 | 仍缺 starter repo 级验证 |
| B. Web/demo 承载与分发 | 已有 `Dioxus`、`GitHub Pages`，并由 `Trunk` 补充静态 build 约束 | 是 | 仍缺真实 repo 中的最终 demo 结构验证 |
| C. Agent 文档与风格系统 | 已有 3 个本地 baseline 支撑 docs/index 比较 | 部分达标 | 仍缺一套可直接落地到 markdown/demo 的风格约束框架 |

## 主线 A 剩余缺口
- 需要在真实 starter repo 中验证 `common_core + adapter` 的边界是否真的可执行。
- 需要确认 `Tauri` 是否进入默认 starter，还是保持为可选 desktop adapter。
- 需要把 `Dioxus` 与 `Tauri` 的能力交集收敛成统一 CLI 与 release contract，而不是并列罗列。

## 主线 B 剩余缺口
- 需要把 GitHub Pages、`404.html` fallback、`base_path` 与 `public_url` 进一步收敛成单一 demo host contract。
- 需要将“本地运行入口”和“在线静态展示入口”的映射画成 storyboard / 页面蓝图。
- 需要确认最小可运行 demo 是否也放在 Pages，还是只放静态 storyboard。

## 主线 C 剩余缺口
- 需要把现有 docs/index 模式比较整理成推荐结构，而不是只保留样本。
- 需要一套黑白灰、工业感、可直接映射到 markdown 与 demo 区块的 style token 草案。
- 需要明确哪些 style 规则属于 docs 层，哪些属于 demo 页面层。

## 下一步产出优先级
1. 输出 agent 文档结构建议与 Nier 风格约束草案。
2. 基于现有样本形成 `common_core / common_adapters / app_owned` 的边界方案。
3. 产出 demo storyboard，并把 GitHub Pages 托管假设映射到页面蓝图。
4. 最后打包正式研究报告与下游任务说明。
