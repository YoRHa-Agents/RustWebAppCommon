# Demo Storyboard

## 页面地图
- `/`
- `/runtime`
- `/docs-entry`
- `/release-flow`
- `/style-lab`
- `/detail/:topic`

## 页面职责
- `Landing`：总结当前阶段、完整 demo 范围与下一条可执行路径
- `RuntimeMap`：展示 `common_core / common_adapters / app_owned` 边界与 runtime contract
- `DocsEntry`：说明 human / main-agent / subagent 前门入口
- `ReleaseFlow`：解释 Pages、release、install、update 与 GitHub workflow 的共享契约
- `StyleLab`：展示 Nier 黑白灰 token、typography 与 component rules
- `StoryDetail`：承接 runtime、desktop preview、release contract、docs front door 等细节主题

## 当前 detail 主题
- `runtime-decision`
- `desktop-preview`
- `release-contract`
- `docs-front-door`

## 当前约束
- docs 与 demo 必须使用同一套 token 和术语
- demo 不承载业务后端逻辑
- GitHub Pages 路径需要满足 `index.html`、`404.html` 与 nested route fallback contract
- 默认 web runtime 为生成后的静态站点 + adapter HTTP shell
- 默认 desktop preview 为 browser-backed preview，并通过 `/detail/desktop-preview` 验证
