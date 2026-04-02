# Core Contracts

## 已实现最小 contracts
- `WorkspaceIdentity`
- `SurfaceKind`
- `DevLaunchRequest`
- `RouteDescriptor`
- `DocsNode`
- `ThemeTokenSet`
- `ReleaseDescriptor`

## 这些契约解决什么问题
- 给多仓共享一套稳定语言
- 让 docs/demo/CLI/runtime 可以对齐
- 把 host/runtime/tooling 差异挡在 adapter 层外

## 当前 invariant
- core 不包含 GitHub Pages、Dioxus、Tauri、Trunk 的具体实现细节
- `RouteDescriptor` 可表达 docs/demo 页面地图
- `ThemeTokenSet` 对 docs 和 demo 共用
- `ReleaseDescriptor` 描述发布元信息，不绑定具体 CI

## 代码位置
- `common/core/src/lib.rs`
