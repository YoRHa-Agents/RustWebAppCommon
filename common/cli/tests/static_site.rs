use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use common_adapters::{DocsSiteAdapter, StubDocsSiteAdapter, StubWebDemoAdapter, WebDemoAdapter};
use common_core::{starter_demo_routes, starter_docs_nodes, ThemeTokenSet};
use serde_json::Value;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn prepared_site_root() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time should move forward")
        .as_nanos();
    let site_root = std::env::temp_dir().join(format!("rwc-static-site-{unique}"));
    let source_root = repo_root().join("site");

    fs::create_dir_all(&site_root).unwrap();
    fs::copy(source_root.join("index.html"), site_root.join("index.html")).unwrap();
    fs::copy(source_root.join("demo.html"), site_root.join("demo.html")).unwrap();

    site_root
}

#[test]
fn demo_pages_contain_shared_shell_sections() {
    let site_root = prepared_site_root();

    StubWebDemoAdapter
        .build_demo_site(&site_root, &starter_demo_routes(), &ThemeTokenSet::nier_gray())
        .unwrap();
    StubDocsSiteAdapter
        .build_docs_site(&site_root, &starter_docs_nodes(), &ThemeTokenSet::nier_gray())
        .unwrap();

    let index_html = fs::read_to_string(site_root.join("index.html")).unwrap();
    let demo_html = fs::read_to_string(site_root.join("demo.html")).unwrap();

    for html in [index_html, demo_html] {
        assert!(html.contains("id=\"topnav\""));
        assert!(html.contains("id=\"status-strip\""));
        assert!(html.contains("id=\"hero-card\""));
        assert!(html.contains("id=\"summary-panel\""));
        assert!(html.contains("id=\"command-panel\""));
        assert!(html.contains("id=\"detail-grid\""));
        assert!(html.contains("id=\"pathways-panel\""));
    }
}

#[test]
fn generated_site_manifests_exist_after_build() {
    let site_root = prepared_site_root();

    StubWebDemoAdapter
        .build_demo_site(&site_root, &starter_demo_routes(), &ThemeTokenSet::nier_gray())
        .unwrap();
    StubDocsSiteAdapter
        .build_docs_site(&site_root, &starter_docs_nodes(), &ThemeTokenSet::nier_gray())
        .unwrap();

    assert!(site_root.join("404.html").is_file());
    assert!(site_root.join("assets/route-manifest.json").is_file());
    assert!(site_root.join("assets/theme-tokens.json").is_file());
    assert!(site_root.join("assets/runtime-contract.json").is_file());
    assert!(site_root.join("docs/docs-index.json").is_file());
}

#[test]
fn runtime_contract_records_web_and_desktop_defaults() {
    let site_root = prepared_site_root();

    StubWebDemoAdapter
        .build_demo_site(&site_root, &starter_demo_routes(), &ThemeTokenSet::nier_gray())
        .unwrap();

    let runtime_contract = fs::read_to_string(site_root.join("assets/runtime-contract.json")).unwrap();
    let parsed: Value = serde_json::from_str(&runtime_contract).unwrap();

    assert_eq!(parsed["web_runtime_id"], "static_site_http_shell");
    assert_eq!(parsed["web_entry_route"], "/runtime");
    assert_eq!(parsed["desktop_preview_mode"], "browser_backed_preview");
    assert_eq!(parsed["desktop_preview_route"], "/detail/desktop-preview");
}

#[test]
fn site_frontend_knows_nested_routes_and_detail_topics() {
    let root = repo_root();
    let script = fs::read_to_string(root.join("site/js/main.js")).unwrap();
    let docs_index = fs::read_to_string(root.join("site/docs/index.html")).unwrap();

    assert!(script.contains("normalizeBasePath"));
    assert!(script.contains("/detail/desktop-preview"));
    assert!(script.contains("/detail/release-contract"));
    assert!(script.contains("runtime-contract.json"));
    assert!(docs_index.contains("Choose Your Path"));
}
