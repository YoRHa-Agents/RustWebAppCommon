use common_adapters::{
    AdapterPlan, AdapterRegistry, DesktopTauriAdapter, DocsSiteAdapter, ReleasePipelineAdapter,
    WebDemoAdapter,
};
use common_core::{
    starter_demo_routes, starter_docs_nodes, starter_release_descriptor, DevLaunchRequest,
    LaunchProfile, SurfaceKind, ThemeTokenSet,
};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandKind {
    Dev,
    DemoBuild,
    DocsBuild,
    ReleaseDesktop,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliInvocation {
    pub command: CommandKind,
    pub surface: Option<SurfaceKind>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub route_entry: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchBundle {
    pub plans: Vec<AdapterPlan>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionBundle {
    pub plans: Vec<AdapterPlan>,
    pub messages: Vec<String>,
}

pub fn parse_cli_args<S>(args: &[S]) -> Result<CliInvocation, String>
where
    S: AsRef<str>,
{
    if args.is_empty() {
        return Err("expected a command: dev/demo/docs/release".to_string());
    }

    match args[0].as_ref() {
        "dev" => parse_dev_args(args),
        "demo" => Ok(CliInvocation {
            command: CommandKind::DemoBuild,
            surface: Some(SurfaceKind::Demo),
            host: None,
            port: None,
            route_entry: None,
        }),
        "docs" => Ok(CliInvocation {
            command: CommandKind::DocsBuild,
            surface: Some(SurfaceKind::Docs),
            host: None,
            port: None,
            route_entry: None,
        }),
        "release" => Ok(CliInvocation {
            command: CommandKind::ReleaseDesktop,
            surface: Some(SurfaceKind::Desktop),
            host: None,
            port: None,
            route_entry: None,
        }),
        other => Err(format!("unsupported command: {other}")),
    }
}

fn parse_dev_args<S>(args: &[S]) -> Result<CliInvocation, String>
where
    S: AsRef<str>,
{
    let mut surface = None;
    let mut host = None;
    let mut port = None;
    let mut route_entry = None;

    let mut index = 1;
    while index < args.len() {
        match args[index].as_ref() {
            "--surface" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    return Err("missing value after --surface".to_string());
                };
                surface = Some(parse_surface(value.as_ref())?);
            }
            "--host" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    return Err("missing value after --host".to_string());
                };
                host = Some(value.as_ref().to_string());
            }
            "--port" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    return Err("missing value after --port".to_string());
                };
                let parsed = value
                    .as_ref()
                    .parse::<u16>()
                    .map_err(|_| "port must be a valid u16".to_string())?;
                port = Some(parsed);
            }
            "--route" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    return Err("missing value after --route".to_string());
                };
                route_entry = Some(value.as_ref().to_string());
            }
            other => {
                return Err(format!("unsupported dev option: {other}"));
            }
        }

        index += 1;
    }

    let surface = surface.ok_or_else(|| "dev requires --surface".to_string())?;

    Ok(CliInvocation {
        command: CommandKind::Dev,
        surface: Some(surface),
        host,
        port,
        route_entry,
    })
}

fn parse_surface(value: &str) -> Result<SurfaceKind, String> {
    match value {
        "web" => Ok(SurfaceKind::Web),
        "desktop" => Ok(SurfaceKind::Desktop),
        "docs" => Ok(SurfaceKind::Docs),
        "demo" => Ok(SurfaceKind::Demo),
        other => Err(format!("unsupported surface: {other}")),
    }
}

pub fn dispatch(invocation: &CliInvocation, registry: &AdapterRegistry) -> Result<DispatchBundle, String> {
    let theme = ThemeTokenSet::nier_gray();
    let routes = starter_demo_routes();
    let docs = starter_docs_nodes();
    let release = starter_release_descriptor();

    let plans = match invocation.command {
        CommandKind::Dev => {
            let surface = invocation
                .surface
                .ok_or_else(|| "dev requires a surface".to_string())?;
            let request = DevLaunchRequest {
                surface,
                host: invocation.host.clone(),
                port: invocation.port,
                route_entry: invocation.route_entry.clone(),
                profile: LaunchProfile::Dev,
            };

            match surface {
                SurfaceKind::Web | SurfaceKind::Demo => {
                    vec![registry.web_demo.plan_dev_launch(&request, &routes)]
                }
                SurfaceKind::Desktop => {
                    vec![registry.desktop_tauri.plan_desktop_launch(&request)]
                }
                SurfaceKind::Docs => vec![registry.docs_site.plan_docs_build(&docs, &theme)],
            }
        }
        CommandKind::DemoBuild => vec![registry.web_demo.plan_demo_build(&routes, &theme)],
        CommandKind::DocsBuild => vec![registry.docs_site.plan_docs_build(&docs, &theme)],
        CommandKind::ReleaseDesktop => vec![
            registry.desktop_tauri.plan_desktop_bundle(&release),
            registry.release_pipeline.plan_release(&release),
        ],
    };

    Ok(DispatchBundle { plans })
}

pub fn execute(
    invocation: &CliInvocation,
    registry: &AdapterRegistry,
) -> Result<ExecutionBundle, String> {
    let dispatch_bundle = dispatch(invocation, registry)?;
    let theme = ThemeTokenSet::nier_gray();
    let routes = starter_demo_routes();
    let docs = starter_docs_nodes();
    let site_root = repo_root().join("site");

    let mut messages = Vec::new();

    match invocation.command {
        CommandKind::Dev => {
            let surface = invocation
                .surface
                .ok_or_else(|| "dev requires a surface".to_string())?;
            let request = DevLaunchRequest {
                surface,
                host: invocation.host.clone(),
                port: invocation.port,
                route_entry: invocation.route_entry.clone(),
                profile: LaunchProfile::Dev,
            };

            registry
                .web_demo
                .build_demo_site(&site_root, &routes, &theme)
                .map_err(|error| error.to_string())?;
            registry
                .docs_site
                .build_docs_site(&site_root, &docs, &theme)
                .map_err(|error| error.to_string())?;

            match surface {
                SurfaceKind::Web | SurfaceKind::Demo => {
                    messages.push(format!("serving site from {}", site_root.display()));
                    registry
                        .web_demo
                        .serve_site(&site_root, &request)
                        .map_err(|error| error.to_string())?;
                }
                SurfaceKind::Desktop => {
                    messages.push(format!("launching desktop preview from {}", site_root.display()));
                    registry
                        .desktop_tauri
                        .launch_desktop_preview(&site_root, &request)
                        .map_err(|error| error.to_string())?;
                }
                SurfaceKind::Docs => {
                    messages.push(format!("docs artifacts available under {}", site_root.display()));
                }
            }
        }
        CommandKind::DemoBuild => {
            let result = registry
                .web_demo
                .build_demo_site(&site_root, &routes, &theme)
                .map_err(|error| error.to_string())?;
            messages.push(format!("demo site built at {}", result.site_root.display()));
        }
        CommandKind::DocsBuild => {
            let result = registry
                .docs_site
                .build_docs_site(&site_root, &docs, &theme)
                .map_err(|error| error.to_string())?;
            messages.push(format!("docs site built at {}", result.site_root.display()));
        }
        CommandKind::ReleaseDesktop => {
            let script = repo_root().join("scripts").join("build-release.sh");
            if script.is_file() {
                let status = Command::new("bash")
                    .arg(script)
                    .status()
                    .map_err(|error| error.to_string())?;
                if !status.success() {
                    return Err(format!("release script exited with {status}"));
                }
                messages.push("release bundle generated via scripts/build-release.sh".to_string());
            } else {
                return Err("missing scripts/build-release.sh".to_string());
            }
        }
    }

    Ok(ExecutionBundle {
        plans: dispatch_bundle.plans,
        messages,
    })
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("cli crate should have parent")
        .parent()
        .expect("common dir should have parent")
        .to_path_buf()
}

#[cfg(test)]
mod tests {
    use common_adapters::AdapterRegistry;

    use super::{dispatch, parse_cli_args, CommandKind, SurfaceKind};

    #[test]
    fn parses_web_dev_command_with_host_and_port() {
        let invocation = parse_cli_args(&[
            "dev",
            "--surface",
            "web",
            "--host",
            "127.0.0.1",
            "--port",
            "8080",
            "--route",
            "/runtime",
        ])
        .expect("cli invocation should parse");

        assert_eq!(invocation.command, CommandKind::Dev);
        assert_eq!(invocation.surface, Some(SurfaceKind::Web));
        assert_eq!(invocation.host.as_deref(), Some("127.0.0.1"));
        assert_eq!(invocation.port, Some(8080));
        assert_eq!(invocation.route_entry.as_deref(), Some("/runtime"));
    }

    #[test]
    fn dispatches_web_dev_to_web_demo_adapter() {
        let invocation = parse_cli_args(&[
            "dev",
            "--surface",
            "web",
            "--host",
            "0.0.0.0",
            "--port",
            "3000",
        ])
        .expect("web dev should parse");
        let bundle = dispatch(&invocation, &AdapterRegistry::default()).expect("dispatch should work");

        assert_eq!(bundle.plans.len(), 1);
        assert_eq!(bundle.plans[0].adapter_id, "web_demo_adapter");
    }

    #[test]
    fn dispatches_docs_build_to_docs_adapter() {
        let invocation = parse_cli_args(&["docs"]).expect("docs should parse");
        let bundle = dispatch(&invocation, &AdapterRegistry::default()).expect("dispatch should work");

        assert_eq!(bundle.plans.len(), 1);
        assert_eq!(bundle.plans[0].adapter_id, "docs_site_adapter");
    }

    #[test]
    fn release_dispatch_emits_desktop_and_release_plans() {
        let invocation = parse_cli_args(&["release"]).expect("release should parse");
        let bundle = dispatch(&invocation, &AdapterRegistry::default()).expect("dispatch should work");

        assert_eq!(bundle.plans.len(), 2);
        assert_eq!(bundle.plans[0].adapter_id, "desktop_tauri_adapter");
        assert_eq!(bundle.plans[1].adapter_id, "release_pipeline_adapter");
    }
}

