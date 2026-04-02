mod remote_review;

use common_core::{
    DevLaunchRequest, DocsNode, ReleaseDescriptor, RouteDescriptor, SurfaceKind, ThemeTokenSet,
};
pub use remote_review::{
    default_remote_review_paths, RemoteDirectoryStatus, RemoteDirectoryStatusKind,
    RemoteDocReviewAdapter, RemoteReviewError, RemoteReviewFileKind, RemoteReviewMode,
    RemoteReviewPlanRequest, RemoteReviewReport, RemoteReviewRequest, SshHostCatalog,
    SshHostCatalogIssue, SshHostEntry, StubRemoteDocsReviewAdapter,
};
use serde::Serialize;
use serde_json::to_vec_pretty;
use std::fs;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_WEB_RUNTIME_ID: &str = "static_site_http_shell";
const DEFAULT_WEB_ENTRY_ROUTE: &str = "/runtime";
const DEFAULT_DESKTOP_PREVIEW_MODE: &str = "browser_backed_preview";
const DEFAULT_DESKTOP_PREVIEW_ROUTE: &str = "/detail/desktop-preview";
const DEFAULT_DOCS_INDEX_PATH: &str = "/docs/index.html";
const DEFAULT_PAGES_STRATEGY: &str = "generated_site_tree";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterPlanKind {
    DevLaunch,
    DemoBuild,
    DocsBuild,
    DesktopLaunch,
    ReleaseBuild,
    RemoteReview,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterPlan {
    pub adapter_id: &'static str,
    pub plan_kind: AdapterPlanKind,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiteBuildResult {
    pub site_root: PathBuf,
    pub generated_files: Vec<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct RuntimeContract {
    web_runtime_id: &'static str,
    web_entry_route: &'static str,
    desktop_adapter_id: &'static str,
    desktop_preview_mode: &'static str,
    desktop_preview_route: &'static str,
    docs_index_path: &'static str,
    pages_strategy: &'static str,
}

pub trait WebDemoAdapter {
    fn plan_dev_launch(
        &self,
        request: &DevLaunchRequest,
        routes: &[RouteDescriptor],
    ) -> AdapterPlan;

    fn plan_demo_build(&self, routes: &[RouteDescriptor], theme: &ThemeTokenSet) -> AdapterPlan;

    fn build_demo_site(
        &self,
        site_root: &Path,
        routes: &[RouteDescriptor],
        theme: &ThemeTokenSet,
    ) -> io::Result<SiteBuildResult>;

    fn serve_site(&self, site_root: &Path, request: &DevLaunchRequest) -> io::Result<()>;
}

pub trait DocsSiteAdapter {
    fn plan_docs_build(&self, docs_nodes: &[DocsNode], theme: &ThemeTokenSet) -> AdapterPlan;

    fn build_docs_site(
        &self,
        site_root: &Path,
        docs_nodes: &[DocsNode],
        theme: &ThemeTokenSet,
    ) -> io::Result<SiteBuildResult>;
}

pub trait DesktopTauriAdapter {
    fn plan_desktop_launch(&self, request: &DevLaunchRequest) -> AdapterPlan;
    fn plan_desktop_bundle(&self, release: &ReleaseDescriptor) -> AdapterPlan;

    fn launch_desktop_preview(
        &self,
        site_root: &Path,
        request: &DevLaunchRequest,
    ) -> io::Result<()>;
}

pub trait ReleasePipelineAdapter {
    fn plan_release(&self, release: &ReleaseDescriptor) -> AdapterPlan;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct StubWebDemoAdapter;

impl WebDemoAdapter for StubWebDemoAdapter {
    fn plan_dev_launch(
        &self,
        request: &DevLaunchRequest,
        routes: &[RouteDescriptor],
    ) -> AdapterPlan {
        AdapterPlan {
            adapter_id: "web_demo_adapter",
            plan_kind: AdapterPlanKind::DevLaunch,
            summary: format!(
                "launch web demo on {:?}:{:?} via {:?} with {} routes",
                request.host,
                request.port,
                request.route_entry,
                routes.len()
            ),
        }
    }

    fn plan_demo_build(&self, routes: &[RouteDescriptor], theme: &ThemeTokenSet) -> AdapterPlan {
        AdapterPlan {
            adapter_id: "web_demo_adapter",
            plan_kind: AdapterPlanKind::DemoBuild,
            summary: format!(
                "build static demo with {} routes and {} palette tokens",
                routes.len(),
                theme.palette.len()
            ),
        }
    }

    fn build_demo_site(
        &self,
        site_root: &Path,
        routes: &[RouteDescriptor],
        theme: &ThemeTokenSet,
    ) -> io::Result<SiteBuildResult> {
        fs::create_dir_all(site_root.join("assets"))?;
        fs::create_dir_all(site_root.join("css"))?;
        fs::create_dir_all(site_root.join("js"))?;

        if !site_root.join("index.html").exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "site/index.html is missing",
            ));
        }
        if !site_root.join("demo.html").exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "site/demo.html is missing",
            ));
        }

        let route_manifest = site_root.join("assets").join("route-manifest.json");
        let theme_manifest = site_root.join("assets").join("theme-tokens.json");
        let runtime_contract_manifest = site_root.join("assets").join("runtime-contract.json");
        fs::write(
            &route_manifest,
            to_vec_pretty(routes).map_err(io::Error::other)?,
        )?;
        fs::write(
            &theme_manifest,
            to_vec_pretty(theme).map_err(io::Error::other)?,
        )?;
        fs::write(
            &runtime_contract_manifest,
            to_vec_pretty(&RuntimeContract {
                web_runtime_id: DEFAULT_WEB_RUNTIME_ID,
                web_entry_route: DEFAULT_WEB_ENTRY_ROUTE,
                desktop_adapter_id: "desktop_tauri_adapter",
                desktop_preview_mode: DEFAULT_DESKTOP_PREVIEW_MODE,
                desktop_preview_route: DEFAULT_DESKTOP_PREVIEW_ROUTE,
                docs_index_path: DEFAULT_DOCS_INDEX_PATH,
                pages_strategy: DEFAULT_PAGES_STRATEGY,
            })
            .map_err(io::Error::other)?,
        )?;
        if !site_root.join("404.html").exists() {
            fs::copy(site_root.join("index.html"), site_root.join("404.html"))?;
        }

        Ok(SiteBuildResult {
            site_root: site_root.to_path_buf(),
            generated_files: vec![
                route_manifest,
                theme_manifest,
                runtime_contract_manifest,
                site_root.join("404.html"),
            ],
        })
    }

    fn serve_site(&self, site_root: &Path, request: &DevLaunchRequest) -> io::Result<()> {
        let host = request.host.as_deref().unwrap_or("127.0.0.1");
        let port = request.port.unwrap_or(8080);
        let listener = TcpListener::bind(format!("{host}:{port}"))?;

        println!(
            "Serving web demo at http://{host}:{port}{}",
            request.route_entry.as_deref().unwrap_or("/")
        );

        for stream in listener.incoming() {
            let stream = match stream {
                Ok(stream) => stream,
                Err(error) => {
                    eprintln!("failed to accept incoming connection: {error}");
                    continue;
                }
            };
            if let Err(error) = handle_http_request(stream, site_root) {
                eprintln!("failed to serve request: {error}");
            }
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct StubDocsSiteAdapter;

impl DocsSiteAdapter for StubDocsSiteAdapter {
    fn plan_docs_build(&self, docs_nodes: &[DocsNode], theme: &ThemeTokenSet) -> AdapterPlan {
        AdapterPlan {
            adapter_id: "docs_site_adapter",
            plan_kind: AdapterPlanKind::DocsBuild,
            summary: format!(
                "build docs site with {} docs nodes and {} typography tokens",
                docs_nodes.len(),
                theme.typography.len()
            ),
        }
    }

    fn build_docs_site(
        &self,
        site_root: &Path,
        docs_nodes: &[DocsNode],
        theme: &ThemeTokenSet,
    ) -> io::Result<SiteBuildResult> {
        let docs_dir = site_root.join("docs");
        fs::create_dir_all(&docs_dir)?;
        let docs_manifest = docs_dir.join("docs-index.json");
        let theme_manifest = docs_dir.join("theme-tokens.json");
        fs::write(
            &docs_manifest,
            to_vec_pretty(docs_nodes).map_err(io::Error::other)?,
        )?;
        fs::write(
            &theme_manifest,
            to_vec_pretty(theme).map_err(io::Error::other)?,
        )?;

        Ok(SiteBuildResult {
            site_root: site_root.to_path_buf(),
            generated_files: vec![docs_manifest, theme_manifest],
        })
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct StubDesktopTauriAdapter;

impl DesktopTauriAdapter for StubDesktopTauriAdapter {
    fn plan_desktop_launch(&self, request: &DevLaunchRequest) -> AdapterPlan {
        let surface = match request.surface {
            SurfaceKind::Desktop => "desktop",
            _ => "non-desktop",
        };

        AdapterPlan {
            adapter_id: "desktop_tauri_adapter",
            plan_kind: AdapterPlanKind::DesktopLaunch,
            summary: format!("launch {} shell in {:?} profile", surface, request.profile),
        }
    }

    fn plan_desktop_bundle(&self, release: &ReleaseDescriptor) -> AdapterPlan {
        AdapterPlan {
            adapter_id: "desktop_tauri_adapter",
            plan_kind: AdapterPlanKind::ReleaseBuild,
            summary: format!(
                "prepare desktop bundle namespace={} signed={}",
                release.artifact_namespace, release.requires_signature
            ),
        }
    }

    fn launch_desktop_preview(
        &self,
        site_root: &Path,
        request: &DevLaunchRequest,
    ) -> io::Result<()> {
        let host = request.host.as_deref().unwrap_or("127.0.0.1").to_string();
        let port = request.port.unwrap_or(9080);
        let route_entry = request
            .route_entry
            .clone()
            .unwrap_or_else(|| DEFAULT_DESKTOP_PREVIEW_ROUTE.to_string());
        let url = desktop_preview_url(&host, port, &route_entry);

        if let Err(error) = open_url(&url) {
            eprintln!("failed to open desktop preview at {url}: {error}");
        }
        StubWebDemoAdapter.serve_site(
            site_root,
            &DevLaunchRequest {
                host: Some(host),
                port: Some(port),
                route_entry: Some(route_entry),
                ..request.clone()
            },
        )
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct StubReleasePipelineAdapter;

impl ReleasePipelineAdapter for StubReleasePipelineAdapter {
    fn plan_release(&self, release: &ReleaseDescriptor) -> AdapterPlan {
        AdapterPlan {
            adapter_id: "release_pipeline_adapter",
            plan_kind: AdapterPlanKind::ReleaseBuild,
            summary: format!(
                "release {} targets on {:?} channel",
                release.targets.len(),
                release.channel
            ),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct AdapterRegistry {
    pub web_demo: StubWebDemoAdapter,
    pub docs_site: StubDocsSiteAdapter,
    pub desktop_tauri: StubDesktopTauriAdapter,
    pub release_pipeline: StubReleasePipelineAdapter,
    pub remote_docs_review: StubRemoteDocsReviewAdapter,
}

fn handle_http_request(mut stream: TcpStream, site_root: &Path) -> io::Result<()> {
    let mut buffer = [0_u8; 2048];
    let size = stream.read(&mut buffer)?;
    if size == 0 {
        return Ok(());
    }

    let request = String::from_utf8_lossy(&buffer[..size]);
    let request_line = request.lines().next().unwrap_or_default();
    let path = request_line.split_whitespace().nth(1).unwrap_or("/");

    let (target, content_type) = resolve_asset_path(site_root, path);
    let body = fs::read(&target)?;
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nCache-Control: no-cache\r\n\r\n",
        body.len()
    );

    stream.write_all(response.as_bytes())?;
    stream.write_all(&body)?;
    stream.flush()?;
    Ok(())
}

fn resolve_asset_path(site_root: &Path, request_path: &str) -> (PathBuf, &'static str) {
    let clean_path = request_path
        .split('?')
        .next()
        .unwrap_or("/")
        .trim_start_matches('/');

    let candidate = if clean_path.is_empty() {
        site_root.join("index.html")
    } else {
        site_root.join(clean_path)
    };

    let resolved = if candidate.is_file() {
        candidate
    } else if clean_path.ends_with('/') && site_root.join(clean_path).join("index.html").is_file() {
        site_root.join(clean_path).join("index.html")
    } else if clean_path.contains('.') {
        site_root.join("404.html")
    } else {
        site_root.join("404.html")
    };

    let content_type = match resolved.extension().and_then(|ext| ext.to_str()) {
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("svg") => "image/svg+xml",
        _ => "text/html; charset=utf-8",
    };

    (resolved, content_type)
}

fn desktop_preview_url(host: &str, port: u16, route_entry: &str) -> String {
    format!("http://{host}:{port}{}", normalize_route_entry(route_entry))
}

fn normalize_route_entry(route_entry: &str) -> String {
    if route_entry.starts_with('/') {
        route_entry.to_string()
    } else {
        format!("/{route_entry}")
    }
}

fn open_url(url: &str) -> io::Result<()> {
    #[cfg(target_os = "linux")]
    let program = ("xdg-open", vec![url]);
    #[cfg(target_os = "macos")]
    let program = ("open", vec![url]);
    #[cfg(target_os = "windows")]
    let program = ("cmd", vec!["/C", "start", url]);

    Command::new(program.0).args(program.1).spawn()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use common_core::{
        starter_demo_routes, starter_docs_nodes, starter_release_descriptor, DevLaunchRequest,
        LaunchProfile, SurfaceKind, ThemeTokenSet,
    };
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{
        desktop_preview_url, normalize_route_entry, resolve_asset_path, DesktopTauriAdapter,
        DocsSiteAdapter, ReleasePipelineAdapter, StubDesktopTauriAdapter, StubDocsSiteAdapter,
        StubReleasePipelineAdapter, StubWebDemoAdapter, WebDemoAdapter,
    };

    fn temp_site_root() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("rwc-site-{unique}"));
        fs::create_dir_all(path.join("docs")).expect("site temp dir should be created");
        fs::create_dir_all(path.join("css")).expect("css dir should be created");
        fs::create_dir_all(path.join("js")).expect("js dir should be created");
        fs::write(path.join("index.html"), "<html>index</html>").expect("index should be written");
        fs::write(path.join("demo.html"), "<html>demo</html>").expect("demo should be written");
        path
    }

    #[test]
    fn web_demo_adapter_preserves_network_parameters() {
        let adapter = StubWebDemoAdapter;
        let request = DevLaunchRequest {
            surface: SurfaceKind::Web,
            host: Some("127.0.0.1".to_string()),
            port: Some(8080),
            route_entry: Some("/runtime".to_string()),
            profile: LaunchProfile::Dev,
        };

        let plan = adapter.plan_dev_launch(&request, &starter_demo_routes());
        assert_eq!(plan.adapter_id, "web_demo_adapter");
        assert!(plan.summary.contains("127.0.0.1"));
        assert!(plan.summary.contains("8080"));
    }

    #[test]
    fn web_demo_build_writes_manifests_and_404_copy() {
        let adapter = StubWebDemoAdapter;
        let site_root = temp_site_root();

        let result = adapter
            .build_demo_site(
                &site_root,
                &starter_demo_routes(),
                &ThemeTokenSet::nier_gray(),
            )
            .expect("demo site should build");

        assert_eq!(result.generated_files.len(), 4);
        assert!(site_root.join("assets/route-manifest.json").is_file());
        assert!(site_root.join("assets/theme-tokens.json").is_file());
        assert!(site_root.join("assets/runtime-contract.json").is_file());
        assert!(site_root.join("404.html").is_file());
    }

    #[test]
    fn build_demo_site_preserves_existing_404_shell() {
        let adapter = StubWebDemoAdapter;
        let site_root = temp_site_root();
        fs::write(site_root.join("404.html"), "<html>fallback</html>").expect("404 should exist");

        adapter
            .build_demo_site(
                &site_root,
                &starter_demo_routes(),
                &ThemeTokenSet::nier_gray(),
            )
            .expect("demo site should build");

        let html = fs::read_to_string(site_root.join("404.html")).expect("404 should be readable");
        assert!(html.contains("fallback"));
    }

    #[test]
    fn docs_site_adapter_counts_docs_nodes() {
        let adapter = StubDocsSiteAdapter;
        let plan = adapter.plan_docs_build(&starter_docs_nodes(), &ThemeTokenSet::nier_gray());

        assert_eq!(plan.adapter_id, "docs_site_adapter");
        assert!(plan.summary.contains("docs nodes"));
    }

    #[test]
    fn docs_build_writes_docs_manifest() {
        let adapter = StubDocsSiteAdapter;
        let site_root = temp_site_root();
        let result = adapter
            .build_docs_site(
                &site_root,
                &starter_docs_nodes(),
                &ThemeTokenSet::nier_gray(),
            )
            .expect("docs site should build");

        assert_eq!(result.generated_files.len(), 2);
        assert!(site_root.join("docs/docs-index.json").is_file());
    }

    #[test]
    fn desktop_tauri_adapter_stays_outside_core_details() {
        let adapter = StubDesktopTauriAdapter;
        let request = DevLaunchRequest {
            surface: SurfaceKind::Desktop,
            host: None,
            port: None,
            route_entry: None,
            profile: LaunchProfile::Debug,
        };

        let plan = adapter.plan_desktop_launch(&request);
        assert_eq!(plan.adapter_id, "desktop_tauri_adapter");
        assert!(plan.summary.contains("desktop"));
    }

    #[test]
    fn release_pipeline_adapter_uses_release_descriptor_only() {
        let adapter = StubReleasePipelineAdapter;
        let descriptor = starter_release_descriptor();
        let plan = adapter.plan_release(&descriptor);

        assert_eq!(plan.adapter_id, "release_pipeline_adapter");
        assert!(plan.summary.contains("channel"));
    }

    #[test]
    fn route_paths_fallback_to_404_html() {
        let site_root = temp_site_root();
        fs::write(site_root.join("404.html"), "<html>404</html>").expect("404 should be written");

        let (resolved, content_type) = resolve_asset_path(&site_root, "/runtime");
        assert_eq!(resolved, site_root.join("404.html"));
        assert_eq!(content_type, "text/html; charset=utf-8");
    }

    #[test]
    fn desktop_preview_defaults_to_nested_detail_route() {
        let url = desktop_preview_url("127.0.0.1", 9080, "detail/desktop-preview");
        assert_eq!(url, "http://127.0.0.1:9080/detail/desktop-preview");
    }

    #[test]
    fn normalize_route_entry_adds_leading_slash() {
        assert_eq!(normalize_route_entry("runtime"), "/runtime");
        assert_eq!(normalize_route_entry("/runtime"), "/runtime");
    }
}
