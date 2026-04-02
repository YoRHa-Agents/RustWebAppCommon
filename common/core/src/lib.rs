use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceIdentity {
    pub workspace_id: String,
    pub repo_name: String,
    pub app_name: String,
    pub default_surface: SurfaceKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SurfaceKind {
    Web,
    Desktop,
    Docs,
    Demo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum LaunchProfile {
    Dev,
    Debug,
    Release,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RouteKind {
    Landing,
    RuntimeMap,
    DocsEntry,
    ReleaseFlow,
    StyleLab,
    StoryDetail,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DevLaunchRequest {
    pub surface: SurfaceKind,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub route_entry: Option<String>,
    pub profile: LaunchProfile,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RouteDescriptor {
    pub route_id: String,
    pub path: String,
    pub route_kind: RouteKind,
    pub parent_route_id: Option<String>,
    pub static_fallback: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DocsNodeKind {
    Index,
    Architecture,
    Guide,
    Example,
    Demo,
    Theme,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AudienceKind {
    Human,
    MainAgent,
    Subagent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DocsNode {
    pub node_id: String,
    pub title: String,
    pub kind: DocsNodeKind,
    pub path: String,
    pub audience: AudienceKind,
    pub children: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ThemeTokenSet {
    pub palette: BTreeMap<String, String>,
    pub typography: BTreeMap<String, String>,
    pub component_rules: BTreeMap<String, String>,
}

impl ThemeTokenSet {
    pub fn nier_gray() -> Self {
        let palette = BTreeMap::from([
            ("bg.canvas".to_string(), "#111111".to_string()),
            ("bg.panel".to_string(), "#1B1A17".to_string()),
            ("fg.primary".to_string(), "#ECE6D9".to_string()),
            ("fg.secondary".to_string(), "#B7B0A3".to_string()),
            ("line.strong".to_string(), "#D7D0C4".to_string()),
            ("line.soft".to_string(), "#5F5A52".to_string()),
            ("accent.signal".to_string(), "#CFC7B8".to_string()),
        ]);
        let typography = BTreeMap::from([
            ("title.family".to_string(), "serif-display".to_string()),
            ("body.family".to_string(), "humanist-sans".to_string()),
            ("meta.family".to_string(), "monospace".to_string()),
        ]);
        let component_rules = BTreeMap::from([
            ("heading.syntax".to_string(), "NN / Section".to_string()),
            ("table.style".to_string(), "strong-border".to_string()),
            ("card.style".to_string(), "thin-border-panel".to_string()),
        ]);

        Self {
            palette,
            typography,
            component_rules,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ReleaseChannel {
    Dev,
    Preview,
    Stable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ReleaseTarget {
    WebDemo,
    DocsSite,
    DesktopBundle,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReleaseDescriptor {
    pub channel: ReleaseChannel,
    pub targets: Vec<ReleaseTarget>,
    pub artifact_namespace: String,
    pub requires_signature: bool,
}

pub fn starter_demo_routes() -> Vec<RouteDescriptor> {
    vec![
        RouteDescriptor {
            route_id: "landing".to_string(),
            path: "/".to_string(),
            route_kind: RouteKind::Landing,
            parent_route_id: None,
            static_fallback: true,
        },
        RouteDescriptor {
            route_id: "runtime-map".to_string(),
            path: "/runtime".to_string(),
            route_kind: RouteKind::RuntimeMap,
            parent_route_id: Some("landing".to_string()),
            static_fallback: true,
        },
        RouteDescriptor {
            route_id: "docs-entry".to_string(),
            path: "/docs-entry".to_string(),
            route_kind: RouteKind::DocsEntry,
            parent_route_id: Some("landing".to_string()),
            static_fallback: true,
        },
        RouteDescriptor {
            route_id: "release-flow".to_string(),
            path: "/release-flow".to_string(),
            route_kind: RouteKind::ReleaseFlow,
            parent_route_id: Some("landing".to_string()),
            static_fallback: true,
        },
        RouteDescriptor {
            route_id: "style-lab".to_string(),
            path: "/style-lab".to_string(),
            route_kind: RouteKind::StyleLab,
            parent_route_id: Some("landing".to_string()),
            static_fallback: true,
        },
        RouteDescriptor {
            route_id: "detail".to_string(),
            path: "/detail/:topic".to_string(),
            route_kind: RouteKind::StoryDetail,
            parent_route_id: None,
            static_fallback: true,
        },
    ]
}

pub fn starter_docs_nodes() -> Vec<DocsNode> {
    vec![
        DocsNode {
            node_id: "readme".to_string(),
            title: "README".to_string(),
            kind: DocsNodeKind::Index,
            path: "README.md".to_string(),
            audience: AudienceKind::Human,
            children: vec!["docs-index".to_string()],
        },
        DocsNode {
            node_id: "agents".to_string(),
            title: "AGENTS".to_string(),
            kind: DocsNodeKind::Guide,
            path: "AGENTS.md".to_string(),
            audience: AudienceKind::MainAgent,
            children: vec!["docs-index".to_string()],
        },
        DocsNode {
            node_id: "docs-index".to_string(),
            title: "Docs Index".to_string(),
            kind: DocsNodeKind::Index,
            path: "docs/index.md".to_string(),
            audience: AudienceKind::Subagent,
            children: vec![
                "architecture-overview".to_string(),
                "dev-guide".to_string(),
                "release-guide".to_string(),
                "theme".to_string(),
            ],
        },
        DocsNode {
            node_id: "architecture-overview".to_string(),
            title: "Architecture Overview".to_string(),
            kind: DocsNodeKind::Architecture,
            path: "docs/architecture/overview.md".to_string(),
            audience: AudienceKind::Human,
            children: vec![],
        },
        DocsNode {
            node_id: "dev-guide".to_string(),
            title: "Development Guide".to_string(),
            kind: DocsNodeKind::Guide,
            path: "docs/guides/dev.md".to_string(),
            audience: AudienceKind::MainAgent,
            children: vec![],
        },
        DocsNode {
            node_id: "release-guide".to_string(),
            title: "Release Guide".to_string(),
            kind: DocsNodeKind::Guide,
            path: "docs/guides/release.md".to_string(),
            audience: AudienceKind::Subagent,
            children: vec![],
        },
        DocsNode {
            node_id: "theme".to_string(),
            title: "Theme Tokens".to_string(),
            kind: DocsNodeKind::Theme,
            path: "docs/theme/nier_gray_tokens.md".to_string(),
            audience: AudienceKind::Subagent,
            children: vec![],
        },
    ]
}

pub fn starter_release_descriptor() -> ReleaseDescriptor {
    ReleaseDescriptor {
        channel: ReleaseChannel::Preview,
        targets: vec![
            ReleaseTarget::WebDemo,
            ReleaseTarget::DocsSite,
            ReleaseTarget::DesktopBundle,
        ],
        artifact_namespace: "rustwebappcommon".to_string(),
        requires_signature: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nier_gray_tokens_expose_expected_keys() {
        let tokens = ThemeTokenSet::nier_gray();

        assert!(tokens.palette.contains_key("bg.canvas"));
        assert!(tokens.palette.contains_key("fg.primary"));
        assert!(tokens.component_rules.contains_key("heading.syntax"));
    }

    #[test]
    fn starter_routes_cover_demo_page_map() {
        let routes = starter_demo_routes();
        let paths: Vec<&str> = routes.iter().map(|route| route.path.as_str()).collect();

        assert_eq!(
            paths,
            vec![
                "/",
                "/runtime",
                "/docs-entry",
                "/release-flow",
                "/style-lab",
                "/detail/:topic",
            ]
        );
        assert!(routes.iter().all(|route| route.static_fallback));
    }

    #[test]
    fn docs_nodes_cover_human_and_agent_audiences() {
        let docs_nodes = starter_docs_nodes();

        assert!(docs_nodes
            .iter()
            .any(|node| node.audience == AudienceKind::Human));
        assert!(docs_nodes
            .iter()
            .any(|node| node.audience == AudienceKind::MainAgent));
        assert!(docs_nodes
            .iter()
            .any(|node| node.audience == AudienceKind::Subagent));
    }

    #[test]
    fn dev_launch_request_preserves_network_fields() {
        let request = DevLaunchRequest {
            surface: SurfaceKind::Web,
            host: Some("127.0.0.1".to_string()),
            port: Some(8080),
            route_entry: Some("/runtime".to_string()),
            profile: LaunchProfile::Dev,
        };

        assert_eq!(request.host.as_deref(), Some("127.0.0.1"));
        assert_eq!(request.port, Some(8080));
        assert_eq!(request.route_entry.as_deref(), Some("/runtime"));
    }

    #[test]
    fn starter_release_descriptor_keeps_signing_optional() {
        let descriptor = starter_release_descriptor();

        assert_eq!(descriptor.channel, ReleaseChannel::Preview);
        assert_eq!(descriptor.targets.len(), 3);
        assert!(!descriptor.requires_signature);
    }
}

