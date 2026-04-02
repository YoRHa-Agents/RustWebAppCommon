# RustWebAppCommon adapter 边界与运行面矩阵
> 更新时间: 2026-04-01

## 目标
本文件定义 `common_adapters` 的最小集合、各 adapter 的输入/输出、依赖关系和替换条件。它的核心任务是把 runtime/host/tooling 的差异留在 adapter 层，不让这些差异泄漏到 `common_core`。

## adapter 总览
| adapter | 主职责 | 依赖的 core 契约 | 不负责的内容 | 可替换性 |
|---|---|---|---|---|
| `web_demo_adapter` | 本地 web/demo 启动、静态 demo 产物构建、`base_path` / `public_url` 适配 | `DevLaunchRequest`、`RouteDescriptor`、`ThemeTokenSet` | 业务页面内容、GitHub Pages 发布动作、desktop release | 高 |
| `docs_site_adapter` | docs/index 站点组织、文档入口映射、静态 docs 产物 | `DocsNode`、`ThemeTokenSet`、`WorkspaceIdentity` | 业务 API、release、desktop 行为 | 高 |
| `desktop_tauri_adapter` | 桌面壳层、窗口启动、桌面 build/bundle/updater 接口 | `WorkspaceIdentity`、`DevLaunchRequest`、`ReleaseDescriptor` | 文档站组织、GitHub Pages、业务页面逻辑 | 中 |
| `release_pipeline_adapter` | release metadata、artifact 目标、签名与发布流水线映射 | `ReleaseDescriptor`、`WorkspaceIdentity` | 页面渲染、docs/index、业务运行时 | 中 |

## 运行面矩阵
| 场景 | 主 adapter | 次 adapter | 输出 |
|---|---|---|---|
| 本地 web 运行 | `web_demo_adapter` | `docs_site_adapter`（可选） | 本地可访问页面、demo 路由 |
| 本地 docs 预览 | `docs_site_adapter` | `web_demo_adapter`（可选） | docs/index 页面 |
| 静态 demo 构建 | `web_demo_adapter` | `docs_site_adapter` | 静态 demo/storyboard 产物 |
| GitHub Pages 发布 | `docs_site_adapter` | `release_pipeline_adapter`（仅部署动作） | `/docs` 或 artifact |
| 本地 desktop 运行 | `desktop_tauri_adapter` | `web_demo_adapter`（若需要前端 dev server） | 桌面壳层 |
| desktop release | `desktop_tauri_adapter` | `release_pipeline_adapter` | 安装包、签名产物、release metadata |

## adapter 详细定义
### 1. `web_demo_adapter`
**存在原因**
- 负责把 `common_core` 的 route/schema/theme 契约映射到 web/demo 运行面。

**输入**
- `DevLaunchRequest`
- `RouteDescriptor`
- `ThemeTokenSet`

**输出**
- 本地 web 运行入口
- 静态 demo 产物
- 对 `base_path` / `public_url` / fallback 的具体适配

**不负责**
- 业务页面内容
- GitHub Pages 部署动作
- desktop 壳层与签名

**候选实现**
- Dioxus
- Trunk + 其他 Rust Web UI 运行时

**Failure Modes**
- 把具体 router 语法提升进 `RouteDescriptor`
- 把 host 约束写进 core
- 把静态 demo 规则和业务页面实现绑在一起

### 2. `docs_site_adapter`
**存在原因**
- 负责把 `DocsNode` 和 theme token 转成可浏览 docs/index 站点。

**输入**
- `DocsNode`
- `ThemeTokenSet`
- `WorkspaceIdentity`

**输出**
- README/index/docs 站点
- agent 入口页
- examples 与 storyboard 的导航入口

**不负责**
- release pipeline
- business runtime
- desktop shell

**候选实现**
- 纯 markdown/static docs
- 静态站点生成器

**Failure Modes**
- docs/index 与 demo/storyboard 各写一套术语
- README、AGENTS、docs/index 没有固定分工

### 3. `desktop_tauri_adapter`
**存在原因**
- 负责桌面壳层、窗口生命周期、bundle/install/update 语义。

**输入**
- `WorkspaceIdentity`
- `DevLaunchRequest`
- `ReleaseDescriptor`

**输出**
- 本地 desktop 运行入口
- desktop build/bundle 对接面
- updater / signing 的接入点

**不负责**
- docs/index 组织
- GitHub Pages host
- 业务页面内容

**候选实现**
- Tauri v2

**Failure Modes**
- 把桌面约束变成所有应用默认要求
- 把 updater 配置沉淀进 core

### 4. `release_pipeline_adapter`
**存在原因**
- 负责把 `ReleaseDescriptor` 映射成构建/签名/发布流水线。

**输入**
- `ReleaseDescriptor`
- `WorkspaceIdentity`

**输出**
- artifact naming
- signing/release channel 映射
- GitHub release 或其他 pipeline 接口

**不负责**
- 页面渲染
- docs/index
- 业务 runtime

**候选实现**
- GitHub Actions + Tauri release
- 其他 CI/release provider

**Failure Modes**
- 把 CI 平台语义直接写进 core
- 把静态 docs 发布与 desktop release 混成一条不可替换路径

## adapter 选择规则
- 如果能力只是“所有应用都要用到的名字和结构”，它属于 `common_core`。
- 如果能力需要依赖具体 host/runtime/tooling，它属于 adapter。
- 如果能力和页面内容、业务逻辑、展示故事直接耦合，它属于 `app_owned`。

## 可替换性规则
- `web_demo_adapter` 可替换为任意满足 `RouteDescriptor + ThemeTokenSet + DevLaunchRequest` 的实现。
- `docs_site_adapter` 可替换，只要保持 `DocsNode` 导航结构不变。
- `desktop_tauri_adapter` 目前只有强候选，没有被锁定为唯一实现。
- `release_pipeline_adapter` 必须允许未来从 GitHub Actions 迁移。

## 迁移成本判断
| adapter | 当前迁移成本 | 原因 |
|---|---|---|
| `web_demo_adapter` | `medium` | Web runtime 可替换，但静态 host 约束需要重新对齐 |
| `docs_site_adapter` | `low-medium` | 信息架构稳定后，实现可替换成本较低 |
| `desktop_tauri_adapter` | `high` | updater、bundle、签名路径较重 |
| `release_pipeline_adapter` | `medium-high` | 与产物命名、签名和 CI 权限高度相关 |

## 设计结论
adapter 层的目标不是隐藏差异，而是隔离差异。只有当每个 adapter 的边界和失败模式都清楚时，`RustWebAppCommon` 才能既统一多个仓库，又不把未来演进空间锁死。
