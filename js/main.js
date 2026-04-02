function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

function normalizeBasePath(pathname) {
  const exactSuffixes = ["/index.html", "/demo.html", "/404.html", "/docs/index.html"];
  for (const suffix of exactSuffixes) {
    if (pathname.endsWith(suffix)) {
      return pathname.slice(0, -suffix.length);
    }
  }

  const topLevelRoutes = ["/runtime", "/docs-entry", "/release-flow", "/style-lab"];
  for (const route of topLevelRoutes) {
    if (pathname.endsWith(route)) {
      return pathname.slice(0, -route.length);
    }
  }

  const detailIndex = pathname.indexOf("/detail/");
  if (detailIndex !== -1) {
    return pathname.slice(0, detailIndex);
  }

  if (pathname === "/") {
    return "";
  }

  return pathname.endsWith("/") ? pathname.slice(0, -1) : pathname;
}

function withBase(basePath, path) {
  const normalized = path.startsWith("/") ? path : `/${path}`;
  return `${basePath}${normalized}` || normalized;
}

async function loadJson(basePath, path) {
  const response = await fetch(withBase(basePath, path));
  if (!response.ok) {
    throw new Error(`failed to load ${path}`);
  }
  return response.json();
}

function resolveCurrentPath(pathname, basePath, defaultRoute) {
  let currentPath = pathname;
  if (basePath && pathname.startsWith(basePath)) {
    currentPath = pathname.slice(basePath.length) || "/";
  }

  if (currentPath === "" || currentPath === "/" || currentPath === "/index.html") {
    return defaultRoute === "/" ? "/" : defaultRoute;
  }
  if (currentPath === "/demo.html" || currentPath === "/404.html") {
    return defaultRoute;
  }
  return currentPath;
}

function audienceLabel(audience) {
  return {
    Human: "Human",
    MainAgent: "Main Agent",
    Subagent: "Subagent",
  }[audience] || audience;
}

function routeLabel(routeId) {
  return routeId
    .split("-")
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(" ");
}

function groupDocsByAudience(docs) {
  const groups = {
    Human: [],
    MainAgent: [],
    Subagent: [],
  };

  for (const doc of docs) {
    if (!groups[doc.audience]) {
      groups[doc.audience] = [];
    }
    groups[doc.audience].push(doc);
  }

  return groups;
}

function makeCommand(title, body, command) {
  return { title, body, command };
}

function makePathway(heading, body, href, hrefLabel) {
  return { heading, body, href, hrefLabel };
}

function detailTopicContent(topic, manifests) {
  const docsByAudience = groupDocsByAudience(manifests.docs);
  const detailMap = {
    "runtime-decision": {
      title: "Runtime Decision",
      summary:
        "默认 web runtime 选择生成后的静态站点 + adapter 内部 HTTP shell；这样 Pages 与本地 `common dev --surface web` 可以共享同一套页面、manifest 与 smoke path。",
      statuses: [
        { label: "Default web runtime", value: manifests.runtime.web_runtime_id },
        { label: "Web entry route", value: manifests.runtime.web_entry_route },
        { label: "Pages strategy", value: manifests.runtime.pages_strategy },
      ],
      summaryItems: [
        "route/theme/docs manifest 由同一条 build path 生成，减少 demo 与 docs 的漂移点。",
        "静态产物直接适配 `index.html + 404.html` Pages contract，而不需要把 provider 逻辑写入 `common_core`。",
        "本地调试依旧使用 `host / port / route` 协议，不绑定具体框架命令语法。",
      ],
      commands: [
        makeCommand(
          "Web dev smoke",
          "验证默认 web runtime 与 route fallback。",
          "cargo run -p common_cli -- dev --surface web --host 127.0.0.1 --port 8080 --route /runtime",
        ),
      ],
      details: [
        {
          heading: "Why not lock a framework here?",
          body: "默认 runtime 是一个可验证的 adapter implementation，而不是把 Dioxus、Trunk 或 GitHub Pages provider 细节升级为 core contract。",
        },
        {
          heading: "Nested route support",
          body: "静态前端通过 base-path 推断来加载 `assets/`，保证 `/detail/*` 和 project Pages 子路径都能回到正确的 manifest。",
        },
      ],
      pathways: [
        makePathway(
          "Go to runtime map",
          "返回完整的分层图与 route 覆盖面。",
          "/runtime",
          "Open Runtime Map",
        ),
      ],
    },
    "desktop-preview": {
      title: "Desktop Preview",
      summary:
        "当前 desktop 默认路线明确为 browser-backed preview：保留 `desktop_tauri_adapter` 作为长期 seam，但当前可验证路径是打开本地 URL 并服务 `/detail/desktop-preview` 页面。",
      statuses: [
        { label: "Desktop adapter", value: manifests.runtime.desktop_adapter_id },
        { label: "Preview mode", value: manifests.runtime.desktop_preview_mode },
        { label: "Preview route", value: manifests.runtime.desktop_preview_route },
      ],
      summaryItems: [
        "优先保证可运行、可 smoke、可交付，而不是在没有 parity 基线前强推原生壳层。",
        "若本机无法自动打开浏览器，adapter 仍会启动本地服务，并把错误显式打印出来。",
        "后续若切到原生壳层，应复用同一条 route/theme/release contract。",
      ],
      commands: [
        makeCommand(
          "Desktop preview smoke",
          "默认打开 desktop preview 的 detail route。",
          "cargo run -p common_cli -- dev --surface desktop --route /detail/desktop-preview",
        ),
      ],
      details: [
        {
          heading: "Current value",
          body: "这条路径已经不再是纯文本 plan，而是实际启动本地服务并渲染一个桌面演示页面。",
        },
        {
          heading: "Long-term seam",
          body: "仍保留 `desktop_tauri_adapter` 命名与 release contract，方便未来替换为更原生的壳层实现。",
        },
      ],
      pathways: [
        makePathway(
          "Release contract",
          "desktop preview 与 release path 使用同一个 adapter seam。",
          "/detail/release-contract",
          "Open Release Contract",
        ),
      ],
    },
    "release-contract": {
      title: "Release Contract",
      summary:
        "release / install / update / GitHub workflow 共用一套资产命名与校验契约：本地 `release/` 目录、GitHub Release asset 与 `SHA256SUMS` 必须能被同一组脚本消费。",
      statuses: [
        { label: "Artifact prefix", value: "rustwebappcommon-<platform>" },
        { label: "Install binary", value: "common" },
        { label: "Checksums", value: "SHA256SUMS" },
      ],
      summaryItems: [
        "本地 `build-release.sh` 输出 host 平台二进制、静态 `site/`、`SHA256SUMS` 与 release manifest。",
        "`install.sh` 支持本地 `release/` 安装或 GitHub Release 安装，并校验 `SHA256SUMS`。",
        "`update-check.sh` 同时支持本地 release 校验与 GitHub Release 资产发现。",
      ],
      commands: [
        makeCommand(
          "Build release bundle",
          "生成 release/ 目录、checksums 与 manifest。",
          "bash scripts/build-release.sh",
        ),
        makeCommand(
          "Install from local release",
          "使用本地产物走完整安装路径。",
          "LOCAL_RELEASE_DIR=release bash scripts/install.sh",
        ),
        makeCommand(
          "Verify update path against local release",
          "用本地产物验证 update-check 资产选择与 checksum。",
          "LOCAL_RELEASE_DIR=release bash scripts/update-check.sh",
        ),
      ],
      details: [
        {
          heading: "GitHub boundary",
          body: "release provider 逻辑保留在脚本和 workflow；`common_core` 只保留 release metadata，不知道 GitHub API 细节。",
        },
        {
          heading: "Pages split",
          body: "Pages 继续只负责 `site/` 静态面，release workflow 负责二进制与 checksums，不把两条发布链混成一个步骤。",
        },
      ],
      pathways: [
        makePathway(
          "Open release flow",
          "返回完整的 Pages / release / update 分层说明。",
          "/release-flow",
          "Open Release Flow",
        ),
      ],
    },
    "docs-front-door": {
      title: "Docs Front Door",
      summary:
        "docs 入口保持一个短前门，先区分 Human / Main Agent / Subagent，再让每条路径进入相同词汇体系下的 README、AGENTS、architecture、guides 与 demo。",
      statuses: [
        { label: "Human docs", value: String(docsByAudience.Human.length) },
        { label: "Main agent docs", value: String(docsByAudience.MainAgent.length) },
        { label: "Subagent docs", value: String(docsByAudience.Subagent.length) },
      ],
      summaryItems: [
        "采用类似 docs sample 的 front-door pattern：短首页、路径选择、再进入具体 guide/reference。",
        "不把 docs/demo 词汇拆成两套命名，避免 README、site、storyboard 各说各话。",
        "静态 docs page 从 `docs-index.json` 读取同一份入口清单。",
      ],
      commands: [
        makeCommand(
          "Rebuild docs entry",
          "刷新 `site/docs/index.html` 使用的 `docs-index.json`。",
          "cargo run -p common_cli -- docs",
        ),
      ],
      details: Object.entries(docsByAudience).map(([key, nodes]) => ({
        heading: audienceLabel(key),
        body: nodes.map((node) => `${node.title} -> ${node.path}`).join(" | "),
      })),
      pathways: [
        makePathway(
          "Open docs entry",
          "查看静态 docs front door。",
          "/docs-entry",
          "Open Docs Entry",
        ),
      ],
    },
  };

  return (
    detailMap[topic] || {
      title: "Story Detail",
      summary: `当前详情主题：${topic}`,
      statuses: [
        { label: "Topic", value: topic },
        { label: "Docs index", value: manifests.runtime.docs_index_path },
        { label: "Route count", value: String(manifests.routes.length) },
      ],
      summaryItems: [
        "这个 detail route 用来承接更细粒度的 handoff、runtime 决策与 release 说明。",
        "继续阅读 docs、release flow 或 runtime map 来找到更上层的上下文。",
      ],
      commands: [],
      details: [
        {
          heading: "Linked docs",
          body: "继续阅读 docs/index、architecture、theme 与 examples 入口。",
        },
        {
          heading: "Next step",
          body: "沿着 doc_auto 中的 compatibility、delta、handoff 与 sync 文档继续推进实现。",
        },
      ],
      pathways: [
        makePathway("Back to landing", "返回总览页面。", "/", "Open Landing"),
      ],
    }
  );
}

function routeContent(routePath, manifests) {
  const docsByAudience = groupDocsByAudience(manifests.docs);
  const themePaletteEntries = Object.entries(manifests.theme.palette);
  const themeTypographyEntries = Object.entries(manifests.theme.typography);
  const themeComponentEntries = Object.entries(manifests.theme.component_rules);

  if (routePath.startsWith("/detail/")) {
    return detailTopicContent(routePath.replace("/detail/", ""), manifests);
  }

  const map = {
    "/": {
      title: "Landing",
      summary:
        "RustWebAppCommon 已经从 starter baseline 进入 follow-on implementation：当前重点是完整 demo、Enva 对齐、release / GitHub 闭环与防漂移验证。",
      statuses: [
        { label: "Phase", value: "follow-on implementation" },
        { label: "Web runtime", value: manifests.runtime.web_runtime_id },
        { label: "Desktop preview", value: manifests.runtime.desktop_preview_mode },
        { label: "Pages strategy", value: manifests.runtime.pages_strategy },
      ],
      summaryItems: [
        "同一套 route/theme/docs manifest 同时驱动静态 `site/`、本地 `common dev --surface web` 和 desktop preview。",
        "当前 release / install / update / workflow 继续在 adapter、scripts 与 workflow 层收敛，不进入 `common_core`。",
        "starter repo、基础 docs/demo 入口和最小测试已经完成，不再重复首轮骨架工作。",
      ],
      commands: [
        makeCommand(
          "Run web demo",
          "从默认 runtime route 启动完整本地 demo。",
          "cargo run -p common_cli -- dev --surface web --host 127.0.0.1 --port 8080 --route /runtime",
        ),
        makeCommand(
          "Build static demo",
          "刷新 `site/`、route/theme/runtime manifests。",
          "cargo run -p common_cli -- demo",
        ),
        makeCommand(
          "Build docs entry",
          "刷新 docs front door 与 `docs-index.json`。",
          "cargo run -p common_cli -- docs",
        ),
      ],
      details: [
        {
          heading: "Common Core",
          body: "统一路由、docs schema、theme token 与 release metadata，但不接纳 GitHub/provider/runtime 细节。",
        },
        {
          heading: "Adapters",
          body: "web demo、docs site、desktop preview 与 release pipeline 继续作为可替换 seams 演进。",
        },
        {
          heading: "Follow-on scope",
          body: "本轮只补齐完整演示面、兼容性裁决、发布契约和回归验证，不扩展业务产品能力。",
        },
      ],
      pathways: [
        makePathway(
          "Runtime decision",
          "查看默认 web runtime 为什么选为静态 site + local HTTP shell。",
          "/detail/runtime-decision",
          "Open Runtime Decision",
        ),
        makePathway(
          "Docs front door",
          "查看 docs/index 和静态 docs page 的入口组织方式。",
          "/detail/docs-front-door",
          "Open Docs Front Door",
        ),
        makePathway(
          "Release contract",
          "查看 build/install/update/workflow 如何共享同一契约。",
          "/detail/release-contract",
          "Open Release Contract",
        ),
      ],
    },
    "/runtime": {
      title: "Runtime Map",
      summary:
        "这条路径展示 `common_core / common_adapters / app_owned` 的职责边界，以及 route/theme/docs manifest 怎样落成静态站点与本地运行面。",
      statuses: [
        { label: "Route count", value: String(manifests.routes.length) },
        { label: "Default route", value: manifests.runtime.web_entry_route },
        { label: "Docs path", value: manifests.runtime.docs_index_path },
        { label: "Desktop path", value: manifests.runtime.desktop_preview_route },
      ],
      summaryItems: [
        "`common_core` 只描述稳定契约：路由、docs 节点、theme token、release metadata。",
        "`common_adapters` 负责生成 manifests、启动本地 server、打开 desktop preview 与承接 release 流水线。",
        "`app_owned` 仍然保留未来业务页面与产品语义，不把产品能力提前挤进公共层。",
      ],
      commands: [
        makeCommand(
          "Web runtime smoke",
          "验证本地 HTTP shell 和 runtime route。",
          "cargo run -p common_cli -- dev --surface web --host 127.0.0.1 --port 8080 --route /runtime",
        ),
        makeCommand(
          "Desktop preview smoke",
          "验证 browser-backed desktop preview route。",
          "cargo run -p common_cli -- dev --surface desktop --route /detail/desktop-preview",
        ),
      ],
      details: [
        {
          heading: "Routes",
          body: manifests.routes.map((route) => `${route.route_id} -> ${route.path}`).join(" | "),
        },
        {
          heading: "Shared manifests",
          body: "当前页面依赖 `route-manifest.json`、`theme-tokens.json`、`docs-index.json` 和 `runtime-contract.json`。",
        },
        {
          heading: "Pages fallback",
          body: "未知路径回到 `404.html` shell，再由 pathname 继续渲染实际 route 内容。",
        },
      ],
      pathways: [
        makePathway(
          "Desktop preview detail",
          "查看默认 desktop route 的说明。",
          "/detail/desktop-preview",
          "Open Desktop Preview Detail",
        ),
      ],
    },
    "/docs-entry": {
      title: "Docs Entry",
      summary:
        "Human、Main Agent 与 Subagent 共享同一套术语，但通过不同入口进入：README、AGENTS、docs/index 与更细的 architecture / guide / theme 页面。",
      statuses: [
        { label: "Human docs", value: String(docsByAudience.Human.length) },
        { label: "Main agent docs", value: String(docsByAudience.MainAgent.length) },
        { label: "Subagent docs", value: String(docsByAudience.Subagent.length) },
        { label: "Static docs path", value: manifests.runtime.docs_index_path },
      ],
      summaryItems: [
        "入口页面尽量保持短小，先回答“从哪开始读”，再让用户进入更深的 architecture / guide / example 页面。",
        "`docs-index.json` 是静态 docs entry 的事实来源，避免站点和 markdown 导航分叉。",
        "该组织方式借鉴 docs sample 的 front-door pattern，但继续保留本仓库的 `common_core / common_adapters / app_owned` 词汇。",
      ],
      commands: [
        makeCommand(
          "Open generated docs entry",
          "查看静态 docs 页面使用的入口数据。",
          "cargo run -p common_cli -- docs",
        ),
      ],
      details: [
        {
          heading: "Human",
          body: docsByAudience.Human.map((node) => `${node.title} -> ${node.path}`).join(" | "),
        },
        {
          heading: "Main Agent",
          body: docsByAudience.MainAgent.map((node) => `${node.title} -> ${node.path}`).join(" | "),
        },
        {
          heading: "Subagent",
          body: docsByAudience.Subagent.map((node) => `${node.title} -> ${node.path}`).join(" | "),
        },
      ],
      pathways: [
        makePathway(
          "Docs front door detail",
          "查看入口组织背后的选择。",
          "/detail/docs-front-door",
          "Open Docs Front Door",
        ),
      ],
    },
    "/release-flow": {
      title: "Release Flow",
      summary:
        "Pages、desktop preview、release build、install 与 update-check 采用同一资产命名与 checksum 契约，但 provider 逻辑继续留在 scripts/workflows/adapters。",
      statuses: [
        { label: "Artifact pattern", value: "rustwebappcommon-<platform>" },
        { label: "Checksums", value: "SHA256SUMS" },
        { label: "Install binary", value: "common" },
        { label: "Desktop route", value: manifests.runtime.desktop_preview_route },
      ],
      summaryItems: [
        "`build-release.sh` 生成 host 平台二进制、静态 `site/`、`SHA256SUMS` 与 release manifest。",
        "`install.sh` 可以消费本地 `release/` 或 GitHub Release，并在安装前校验 checksum。",
        "`update-check.sh` 可以检查本地 release 目录或 GitHub Release 是否包含当前平台的资产。",
      ],
      commands: [
        makeCommand(
          "Build release bundle",
          "生成 release 目录、manifest 与 checksums。",
          "bash scripts/build-release.sh",
        ),
        makeCommand(
          "Install locally",
          "使用本地 release 目录走完整安装路径。",
          "LOCAL_RELEASE_DIR=release bash scripts/install.sh",
        ),
        makeCommand(
          "Check update",
          "检查本地或远端 release 是否包含当前平台资产。",
          "LOCAL_RELEASE_DIR=release bash scripts/update-check.sh",
        ),
      ],
      details: [
        {
          heading: "Pages deploy",
          body: "Pages workflow 重新生成 `site/` 后再上传 artifact，从而把 adapter/CLI 变化纳入部署链路。",
        },
        {
          heading: "Release publish",
          body: "tag push 会构建 release bundle、保留 workflow artifact，并向 GitHub Release 附加 host 资产与 `SHA256SUMS`。",
        },
        {
          heading: "Core boundary",
          body: "release metadata 在 core；GitHub API、asset URL 与 workflow provider 仍只存在于脚本与 workflow。",
        },
      ],
      pathways: [
        makePathway(
          "Release contract detail",
          "查看 install/update/workflow 的共享契约。",
          "/detail/release-contract",
          "Open Release Contract Detail",
        ),
      ],
    },
    "/style-lab": {
      title: "Style Lab",
      summary:
        "Nier 黑白灰 token 同时服务 docs 与 demo：palette 负责静态站点表面，typography 与 component rules 负责统一语气与布局节奏。",
      statuses: [
        { label: "Palette tokens", value: String(themePaletteEntries.length) },
        { label: "Typography tokens", value: String(themeTypographyEntries.length) },
        { label: "Component rules", value: String(themeComponentEntries.length) },
        { label: "Audience reuse", value: "docs + demo" },
      ],
      summaryItems: [
        "tokens 仍保留在 `common_core`，具体 CSS 和组件布局由 `site/` 与 docs/demo 展示面消费。",
        "同一套 token 命名是防漂移的关键，避免 docs 与 demo 各自造新的视觉词汇。",
      ],
      commands: [
        makeCommand(
          "Rebuild demo tokens",
          "刷新 palette / typography / rules manifests。",
          "cargo run -p common_cli -- demo",
        ),
      ],
      details: [
        ...themePaletteEntries.map(([token, value]) => ({
          heading: token,
          body: `<div class="token-swatch" style="background:${escapeHtml(value)}"></div><code>${escapeHtml(value)}</code>`,
          rawHtml: true,
        })),
        ...themeTypographyEntries.map(([token, value]) => ({
          heading: token,
          body: value,
        })),
        ...themeComponentEntries.map(([token, value]) => ({
          heading: token,
          body: value,
        })),
      ],
      pathways: [
        makePathway(
          "Back to landing",
          "回到完整 demo 总览。",
          "/",
          "Open Landing",
        ),
      ],
    },
  };

  return (
    map[routePath] || {
      title: "Fallback Route",
      summary: `当前路径 ${routePath} 未在显式 route map 中命中，静态 shell 已回退到 client-side 渲染。`,
      statuses: [
        { label: "Requested path", value: routePath },
        { label: "Fallback shell", value: "404.html" },
        { label: "Pages strategy", value: manifests.runtime.pages_strategy },
      ],
      summaryItems: [
        "如果这是一个 detail route，请改用 `/detail/<topic>`。",
        "如果这是一个新页面，请同步更新 `starter_demo_routes()`、site 内容和 parity tests。",
      ],
      commands: [],
      details: [
        {
          heading: "Suggested routes",
          body: manifests.routes.map((route) => route.path).join(" | "),
        },
      ],
      pathways: [
        makePathway("Back to landing", "返回总览页面。", "/", "Open Landing"),
      ],
    }
  );
}

function renderNav(currentPath, routes, basePath) {
  const topnav = document.getElementById("topnav");
  topnav.innerHTML = routes
    .filter((route) => route.path !== "/detail/:topic")
    .map((route) => {
      const active = currentPath === route.path ? "active" : "";
      return `<a class="${active}" href="${escapeHtml(withBase(basePath, route.path))}">${escapeHtml(routeLabel(route.route_id))}</a>`;
    })
    .join("");
}

function renderStatus(statuses) {
  document.getElementById("status-strip").innerHTML = statuses
    .map(
      (status) => `
        <article class="status-card">
          <span class="status-label">${escapeHtml(status.label)}</span>
          <span class="status-value">${escapeHtml(status.value)}</span>
        </article>
      `,
    )
    .join("");
}

function renderHero(content) {
  document.getElementById("hero-card").innerHTML = `
    <h2>${escapeHtml(content.title)}</h2>
    <p>${escapeHtml(content.summary)}</p>
    <ul class="hero-list">
      ${content.summaryItems.map((item) => `<li>${escapeHtml(item)}</li>`).join("")}
    </ul>
  `;
}

function renderSummaryPanel(currentPath) {
  document.getElementById("summary-panel").innerHTML = `
    <h2>Current Route</h2>
    <p>当前页面与 <code>common_core::starter_demo_routes()</code>、<code>ThemeTokenSet::nier_gray()</code> 以及 adapter 生成的 runtime contract 保持一致。</p>
    <div class="callout">
      <strong>Rendered path</strong>
      <code>${escapeHtml(currentPath)}</code>
    </div>
  `;
}

function renderCommandPanel(commands) {
  const container = document.getElementById("command-panel");
  if (commands.length === 0) {
    container.innerHTML = `
      <h2>Commands</h2>
      <p>这个页面没有额外命令，继续通过导航或 detail routes 浏览即可。</p>
    `;
    return;
  }

  container.innerHTML = `
    <h2>Executable Paths</h2>
    <div class="command-list">
      ${commands
        .map(
          (command) => `
            <article class="command-card">
              <h3>${escapeHtml(command.title)}</h3>
              <p>${escapeHtml(command.body)}</p>
              <code>${escapeHtml(command.command)}</code>
            </article>
          `,
        )
        .join("")}
    </div>
  `;
}

function renderDetailGrid(details) {
  document.getElementById("detail-grid").innerHTML = details
    .map((detail) => {
      const body = detail.rawHtml ? detail.body : `<p>${escapeHtml(detail.body)}</p>`;
      return `
        <article class="detail-card">
          <h3>${escapeHtml(detail.heading)}</h3>
          ${body}
        </article>
      `;
    })
    .join("");
}

function renderPathways(content, basePath) {
  const container = document.getElementById("pathways-panel");
  container.innerHTML = `
    <h2>Choose Next Path</h2>
    <p>从当前页面继续进入更细的 detail route、runtime map、docs front door 或 release contract。</p>
    <div class="pathway-grid">
      ${content.pathways
        .map(
          (pathway) => `
            <article class="pathway-card">
              <h3>${escapeHtml(pathway.heading)}</h3>
              <p>${escapeHtml(pathway.body)}</p>
              <div class="button-row">
                <a class="button-link" href="${escapeHtml(withBase(basePath, pathway.href))}">${escapeHtml(pathway.hrefLabel)}</a>
              </div>
            </article>
          `,
        )
        .join("")}
    </div>
  `;
}

function renderPage(currentPath, content, basePath) {
  document.title = `RustWebAppCommon | ${content.title}`;
  renderStatus(content.statuses);
  renderHero(content);
  renderSummaryPanel(currentPath);
  renderCommandPanel(content.commands);
  renderDetailGrid(content.details);
  renderPathways(content, basePath);
}

async function main() {
  const defaultRoute = document.body.dataset.defaultRoute || "/";
  const basePath = normalizeBasePath(window.location.pathname);
  const [routes, docs, theme, runtime] = await Promise.all([
    loadJson(basePath, "/assets/route-manifest.json"),
    loadJson(basePath, "/docs/docs-index.json"),
    loadJson(basePath, "/assets/theme-tokens.json"),
    loadJson(basePath, "/assets/runtime-contract.json"),
  ]);

  const currentPath = resolveCurrentPath(window.location.pathname, basePath, defaultRoute);
  const manifests = { routes, docs, theme, runtime };
  renderNav(currentPath, routes, basePath);
  renderPage(currentPath, routeContent(currentPath, manifests), basePath);
}

main().catch((error) => {
  document.getElementById("status-strip").innerHTML = "";
  document.getElementById("hero-card").innerHTML = `<h2>Demo build missing</h2><p>${escapeHtml(error.message)}</p>`;
  document.getElementById("summary-panel").innerHTML = `
    <h2>Next Step</h2>
    <p>运行 <code>cargo run -p common_cli -- demo</code> 和 <code>cargo run -p common_cli -- docs</code> 重新生成 manifests。</p>
  `;
  document.getElementById("command-panel").innerHTML = "";
  document.getElementById("detail-grid").innerHTML = "";
  document.getElementById("pathways-panel").innerHTML = "";
});
