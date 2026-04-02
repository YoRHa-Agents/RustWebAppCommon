# RustWebAppCommon common_core 核心契约
> 更新时间: 2026-04-01

## 目标
`common_core` 的职责不是直接实现所有运行面，而是定义所有仓库都可以共享的协议、术语和结构契约。它必须足够稳定，能同时支撑 docs、demo、release、desktop 和多应用接入，但又不能把具体 runtime/host 约束泄漏进 core。

## common_core 设计原则
- **协议优先**: core 只定义结构、命名、输入输出和 invariant。
- **实现可替换**: 任何 runtime/host/tooling 都只能挂在 adapter 层。
- **多仓共享**: 进入 core 的内容必须对多个应用都成立。
- **可文档化**: core 契约必须能被 README、docs/index、demo 和实现代码共同引用。

## 状态边界模型
| 状态类别 | 是否持久化 | 典型内容 | 归属 |
|---|---|---|---|
| `RuntimeSessionState` | 否 | 当前进程的 `host`、`port`、临时 dev flags、调试状态 | adapter / runtime |
| `DurableWorkspaceConfig` | 是 | app id、repo 名、surface 配置、base path、adapter 开关 | `common_core` |
| `DocsAndDemoSchema` | 是 | docs/index 目录、页面地图、route descriptor、theme token | `common_core` |
| `ReleaseDescriptor` | 是 | channel、artifact 目标、版本策略、发布元信息 | `common_core` |
| `IntegrationMetadata` | 视情况 | adapter 级私有配置、部署目标、签名路径、工具差异 | adapter |

## 核心契约清单
### 1. `WorkspaceIdentity`
用于定义一个接入 `RustWebAppCommon` 的应用/仓库身份。

建议字段：
```rust
struct WorkspaceIdentity {
    workspace_id: String,
    repo_name: String,
    app_name: String,
    default_surface: SurfaceKind,
}
```

### 2. `SurfaceKind`
统一运行面的枚举，而不是工具名枚举。

建议值：
- `web`
- `desktop`
- `docs`
- `demo`

### 3. `DevLaunchRequest`
描述一次本地启动请求。

建议字段：
```rust
struct DevLaunchRequest {
    surface: SurfaceKind,
    host: Option<String>,
    port: Option<u16>,
    route_entry: Option<String>,
    profile: LaunchProfile,
}
```

### 4. `RouteDescriptor`
用于描述主页面、子页面、详情页和静态入口，而不是绑定具体前端框架。

建议字段：
```rust
struct RouteDescriptor {
    route_id: String,
    path: String,
    route_kind: RouteKind,
    parent_route_id: Option<String>,
    static_fallback: bool,
}
```

### 5. `DocsNode`
用于统一 docs/index 的信息架构。

建议字段：
```rust
struct DocsNode {
    node_id: String,
    title: String,
    kind: DocsNodeKind,
    path: String,
    audience: AudienceKind,
    children: Vec<String>,
}
```

### 6. `ThemeTokenSet`
用于统一 docs 与 demo 的视觉语法，而不是直接绑定 CSS 文件。

建议字段：
```rust
struct ThemeTokenSet {
    palette: BTreeMap<String, String>,
    typography: BTreeMap<String, String>,
    component_rules: BTreeMap<String, String>,
}
```

### 7. `ReleaseDescriptor`
用于描述 release 目标与元信息。

建议字段：
```rust
struct ReleaseDescriptor {
    channel: ReleaseChannel,
    targets: Vec<ReleaseTarget>,
    artifact_namespace: String,
    requires_signature: bool,
}
```

## CLI 协议
`common_core` 统一命令语义，而不是统一底层命令实现。

| 命令族 | 语义 | core 保证什么 |
|---|---|---|
| `dev` | 本地运行某个 surface | 参数名、surface 语义、route entry 协议 |
| `demo` | 生成 demo/storyboard 产物 | 产物类型、输出语义、host contract |
| `docs` | 构建 docs/index 产物 | docs schema 与入口组织 |
| `release` | 生成 release 相关产物 | artifact metadata、channel、目标面定义 |

## Invariants
- `common_core` 不能依赖具体 UI runtime 包名。
- `common_core` 不能写入 GitHub Pages 的具体路径复制逻辑，只能描述 host contract。
- `RouteDescriptor` 必须能同时表达静态页面和运行时页面。
- `DocsNode` 必须支持 human、main-agent、subagent 三类入口。
- `ThemeTokenSet` 必须对 docs 与 demo 共用，而不是分裂为两套。

## Non-goals
- 不在 core 中定义 Dioxus/Tauri/Trunk 的命令细节。
- 不在 core 中定义桌面 updater、签名、Actions workflow。
- 不在 core 中持有业务页面、业务 API 或 showcase story。
- 不在 core 中直接实现 markdown 渲染器或 CSS pipeline。

## 未来迁移点
- `SurfaceKind` 未来可扩展为 `mobile` 或 `service`，但不影响现有核心协议。
- `ReleaseDescriptor` 可在后续加入多 channel、多 flavor 或回滚策略。
- `RouteDescriptor` 可在 starter repo 验证后增强 query/hash 或 nested route 元数据，但当前不绑定具体框架语法。

## 设计结论
`common_core` 的设计重点不是“给所有人一个默认实现”，而是“给所有应用一个共同的语言和边界”。一旦 core 契约稳定，adapter 与应用仓库就能围绕相同语义协作，而不需要共享全部实现。
