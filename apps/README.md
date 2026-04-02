# Apps

## 说明
未来接入 `RustWebAppCommon` 的具体应用仓库或子应用应落在这里，或至少遵循这里描述的边界：
- 应用保留页面、业务路由、业务逻辑和 showcase
- 应用复用 `common_core` 术语与 adapter 接口
- 应用不应把业务细节反向写回 common
