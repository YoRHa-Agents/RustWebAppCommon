use common_adapters::{
    AdapterPlan, AdapterRegistry, DesktopTauriAdapter, DocsSiteAdapter, ReleasePipelineAdapter,
    RemoteDocReviewAdapter, RemoteReviewPlanRequest, RemoteReviewRequest, WebDemoAdapter,
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
    Review,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliInvocation {
    pub command: CommandKind,
    pub surface: Option<SurfaceKind>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub route_entry: Option<String>,
    pub ssh_host: Option<String>,
    pub ssh_config_path: Option<PathBuf>,
    pub review_paths: Vec<String>,
    pub list_hosts: bool,
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
        return Err("expected a command: dev/demo/docs/release/review".to_string());
    }

    match args[0].as_ref() {
        "dev" => parse_dev_args(args),
        "demo" => Ok(CliInvocation {
            command: CommandKind::DemoBuild,
            surface: Some(SurfaceKind::Demo),
            host: None,
            port: None,
            route_entry: None,
            ssh_host: None,
            ssh_config_path: None,
            review_paths: Vec::new(),
            list_hosts: false,
        }),
        "docs" => Ok(CliInvocation {
            command: CommandKind::DocsBuild,
            surface: Some(SurfaceKind::Docs),
            host: None,
            port: None,
            route_entry: None,
            ssh_host: None,
            ssh_config_path: None,
            review_paths: Vec::new(),
            list_hosts: false,
        }),
        "release" => Ok(CliInvocation {
            command: CommandKind::ReleaseDesktop,
            surface: Some(SurfaceKind::Desktop),
            host: None,
            port: None,
            route_entry: None,
            ssh_host: None,
            ssh_config_path: None,
            review_paths: Vec::new(),
            list_hosts: false,
        }),
        "review" => parse_review_args(args),
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
        ssh_host: None,
        ssh_config_path: None,
        review_paths: Vec::new(),
        list_hosts: false,
    })
}

fn parse_review_args<S>(args: &[S]) -> Result<CliInvocation, String>
where
    S: AsRef<str>,
{
    let mut ssh_host = None;
    let mut ssh_config_path = None;
    let mut review_paths = Vec::new();
    let mut list_hosts = false;

    let mut index = 1;
    while index < args.len() {
        match args[index].as_ref() {
            "--ssh-host" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    return Err("missing value after --ssh-host".to_string());
                };
                ssh_host = Some(value.as_ref().to_string());
            }
            "--config" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    return Err("missing value after --config".to_string());
                };
                ssh_config_path = Some(PathBuf::from(value.as_ref()));
            }
            "--path" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    return Err("missing value after --path".to_string());
                };
                review_paths.push(value.as_ref().to_string());
            }
            "--list-hosts" => {
                list_hosts = true;
            }
            other => {
                return Err(format!("unsupported review option: {other}"));
            }
        }

        index += 1;
    }

    if list_hosts && ssh_host.is_some() {
        return Err("review --list-hosts cannot be combined with --ssh-host".to_string());
    }

    if list_hosts && !review_paths.is_empty() {
        return Err("review --list-hosts cannot be combined with --path".to_string());
    }

    if !list_hosts && ssh_host.is_none() {
        return Err("review requires --list-hosts or --ssh-host <alias>".to_string());
    }

    Ok(CliInvocation {
        command: CommandKind::Review,
        surface: None,
        host: None,
        port: None,
        route_entry: None,
        ssh_host,
        ssh_config_path,
        review_paths,
        list_hosts,
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

pub fn dispatch(
    invocation: &CliInvocation,
    registry: &AdapterRegistry,
) -> Result<DispatchBundle, String> {
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
        CommandKind::Review => {
            vec![registry
                .remote_docs_review
                .plan_remote_review(&RemoteReviewPlanRequest {
                    config_path: invocation.ssh_config_path.clone(),
                    host_alias: invocation.ssh_host.clone(),
                    explicit_paths: invocation.review_paths.clone(),
                    list_hosts: invocation.list_hosts,
                })]
        }
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
                    messages.push(format!(
                        "launching desktop preview from {}",
                        site_root.display()
                    ));
                    registry
                        .desktop_tauri
                        .launch_desktop_preview(&site_root, &request)
                        .map_err(|error| error.to_string())?;
                }
                SurfaceKind::Docs => {
                    messages.push(format!(
                        "docs artifacts available under {}",
                        site_root.display()
                    ));
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
        CommandKind::Review => {
            if invocation.list_hosts {
                let catalog = registry
                    .remote_docs_review
                    .list_review_hosts(invocation.ssh_config_path.as_deref())
                    .map_err(|error| error.to_string())?;
                messages.extend(format_host_catalog(&catalog));
            } else {
                let ssh_host = invocation
                    .ssh_host
                    .clone()
                    .ok_or_else(|| "review requires --ssh-host <alias>".to_string())?;
                let report = registry
                    .remote_docs_review
                    .review_remote_docs(&RemoteReviewRequest {
                        config_path: invocation.ssh_config_path.clone(),
                        host_alias: ssh_host,
                        explicit_paths: invocation.review_paths.clone(),
                    })
                    .map_err(|error| error.to_string())?;
                messages.extend(format_remote_review_report(&report));
            }
        }
    }

    Ok(ExecutionBundle {
        plans: dispatch_bundle.plans,
        messages,
    })
}

fn format_host_catalog(catalog: &common_adapters::SshHostCatalog) -> Vec<String> {
    let mut messages = vec![format!(
        "ssh config loaded from {}",
        catalog.config_path.display()
    )];

    if catalog.hosts.is_empty() {
        messages.push("no concrete ssh hosts found".to_string());
    } else {
        messages.push(format!("ssh hosts: {}", catalog.hosts.len()));
        for host in &catalog.hosts {
            let user = host.user.as_deref().unwrap_or("-");
            let port = host
                .port
                .map(|value| value.to_string())
                .unwrap_or_else(|| "-".to_string());
            let identity = host
                .identity_file
                .as_ref()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "-".to_string());
            messages.push(format!(
                "ssh host {} -> {} user={} port={} identity={}",
                host.alias, host.hostname, user, port, identity
            ));
        }
    }

    for issue in &catalog.issues {
        messages.push(format!(
            "ssh host {} skipped: {}",
            issue.alias, issue.message
        ));
    }

    messages
}

fn format_remote_review_report(report: &common_adapters::RemoteReviewReport) -> Vec<String> {
    let mut messages = vec![
        format!(
            "remote review host {} -> {}",
            report.host.alias, report.host.hostname
        ),
        format!("ssh config {}", report.config_path.display()),
        format!("discovery mode {}", report.mode.as_str()),
    ];

    for directory in &report.directories {
        messages.push(format!(
            "candidate {}: {} ({} reviewable / {} total)",
            directory.path,
            directory.status.as_str(),
            directory.reviewable_files,
            directory.total_files
        ));
    }

    if report.items.is_empty() {
        messages.push("no reviewable remote doc/design files found".to_string());
    } else {
        messages.push(format!("review items: {}", report.items.len()));
        for item in &report.items {
            messages.push(format!(
                "[{}] {} ({})",
                item.file_kind.as_str(),
                item.remote_path,
                item.directory_path
            ));
        }
    }

    messages
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
    use std::path::PathBuf;

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
        let bundle =
            dispatch(&invocation, &AdapterRegistry::default()).expect("dispatch should work");

        assert_eq!(bundle.plans.len(), 1);
        assert_eq!(bundle.plans[0].adapter_id, "web_demo_adapter");
    }

    #[test]
    fn dispatches_docs_build_to_docs_adapter() {
        let invocation = parse_cli_args(&["docs"]).expect("docs should parse");
        let bundle =
            dispatch(&invocation, &AdapterRegistry::default()).expect("dispatch should work");

        assert_eq!(bundle.plans.len(), 1);
        assert_eq!(bundle.plans[0].adapter_id, "docs_site_adapter");
    }

    #[test]
    fn release_dispatch_emits_desktop_and_release_plans() {
        let invocation = parse_cli_args(&["release"]).expect("release should parse");
        let bundle =
            dispatch(&invocation, &AdapterRegistry::default()).expect("dispatch should work");

        assert_eq!(bundle.plans.len(), 2);
        assert_eq!(bundle.plans[0].adapter_id, "desktop_tauri_adapter");
        assert_eq!(bundle.plans[1].adapter_id, "release_pipeline_adapter");
    }

    #[test]
    fn parses_review_list_hosts_command() {
        let invocation = parse_cli_args(&["review", "--list-hosts", "--config", "/tmp/review-ssh"]);
        let invocation = invocation.expect("review list-hosts should parse");

        assert_eq!(invocation.command, CommandKind::Review);
        assert!(invocation.list_hosts);
        assert_eq!(
            invocation
                .ssh_config_path
                .as_ref()
                .expect("config path should exist"),
            &PathBuf::from("/tmp/review-ssh")
        );
    }

    #[test]
    fn parses_review_command_with_explicit_path() {
        let invocation = parse_cli_args(&[
            "review",
            "--ssh-host",
            "review-host",
            "--config",
            "/tmp/review-ssh",
            "--path",
            "/srv/reviews",
        ])
        .expect("review command should parse");

        assert_eq!(invocation.command, CommandKind::Review);
        assert_eq!(invocation.ssh_host.as_deref(), Some("review-host"));
        assert_eq!(invocation.review_paths, vec!["/srv/reviews"]);
    }

    #[test]
    fn review_requires_list_hosts_or_alias() {
        let error = parse_cli_args(&["review"]).expect_err("review should fail without target");
        assert!(error.contains("review requires --list-hosts or --ssh-host"));
    }

    #[test]
    fn no_command_error_mentions_review() {
        let error = parse_cli_args::<&str>(&[]).expect_err("empty cli should fail");
        assert!(error.contains("review"));
    }

    #[test]
    fn review_rejects_list_hosts_with_paths() {
        let error = parse_cli_args(&["review", "--list-hosts", "--path", "/srv/reviews"])
            .expect_err("list-hosts should reject explicit paths");
        assert!(error.contains("--list-hosts cannot be combined with --path"));
    }

    #[test]
    fn review_rejects_list_hosts_with_alias() {
        let error = parse_cli_args(&["review", "--list-hosts", "--ssh-host", "review-host"])
            .expect_err("list-hosts should reject --ssh-host");
        assert!(error.contains("--list-hosts cannot be combined with --ssh-host"));
    }

    #[test]
    fn review_dispatch_uses_remote_review_adapter() {
        let invocation = parse_cli_args(&["review", "--list-hosts"]).expect("review should parse");
        let bundle =
            dispatch(&invocation, &AdapterRegistry::default()).expect("dispatch should work");

        assert_eq!(bundle.plans.len(), 1);
        assert_eq!(bundle.plans[0].adapter_id, "remote_docs_review_adapter");
    }
}
