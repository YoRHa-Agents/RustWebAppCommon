use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn common_bin() -> &'static str {
    env!("CARGO_BIN_EXE_common")
}

fn run_common(args: &[&str]) -> std::process::Output {
    Command::new(common_bin())
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("common CLI should launch as a subprocess")
}

fn run_common_with_env(args: &[&str], envs: &[(&str, &str)]) -> std::process::Output {
    let mut command = Command::new(common_bin());
    command.args(args).current_dir(repo_root());
    for (key, value) in envs {
        command.env(key, value);
    }
    command
        .output()
        .expect("common CLI should launch as a subprocess")
}

fn temp_dir(prefix: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time should move forward")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("{prefix}-{unique}"));
    fs::create_dir_all(&path).expect("temp dir should exist");
    path
}

fn write(path: &std::path::Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent dir should exist");
    }
    fs::write(path, content).expect("fixture file should be written");
}

#[test]
fn demo_command_runs_as_a_real_subprocess() {
    let output = run_common(&["demo"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "demo command failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("demo site built at"));
    assert!(stdout.contains("[web_demo_adapter] DemoBuild"));
}

#[test]
fn docs_command_runs_as_a_real_subprocess() {
    let output = run_common(&["docs"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "docs command failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("docs site built at"));
    assert!(stdout.contains("[docs_site_adapter] DocsBuild"));
}

#[test]
fn review_list_hosts_runs_as_a_real_subprocess() {
    let root = temp_dir("rwc-review-hosts");
    let config = root.join("ssh-config");
    write(
        &config,
        "Host review-host\n  HostName review.example.test\n  User revi\n  Port 2222\n",
    );
    let config_string = config.to_string_lossy().into_owned();
    let output = run_common(&["review", "--list-hosts", "--config", &config_string]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "review list-hosts failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("ssh hosts: 1"));
    assert!(stdout.contains("ssh host review-host -> review.example.test"));
    assert!(stdout.contains("[remote_docs_review_adapter] RemoteReview"));
}

#[test]
fn review_list_hosts_reports_skipped_invalid_hosts() {
    let root = temp_dir("rwc-review-hosts-issues");
    let config = root.join("ssh-config");
    write(
        &config,
        "Host review-host\n  HostName review.example.test\nHost broken\n  User revi\n",
    );
    let config_string = config.to_string_lossy().into_owned();
    let output = run_common(&["review", "--list-hosts", "--config", &config_string]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "review list-hosts with issues failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("ssh hosts: 1"));
    assert!(stdout.contains("ssh host review-host -> review.example.test"));
    assert!(stdout.contains("ssh host broken skipped:"));
    assert!(stdout.contains("missing HostName"));
}

#[test]
fn review_command_uses_fixture_transport_for_smoke_path() {
    let root = temp_dir("rwc-review-fixture");
    let config = root.join("ssh-config");
    let fixture_root = root.join("fixtures");
    write(
        &config,
        "Host review-host\n  HostName review.example.test\n  User revi\n",
    );
    write(
        &fixture_root
            .join("review-host")
            .join("docs")
            .join("index.md"),
        "# docs\n",
    );
    write(
        &fixture_root
            .join("review-host")
            .join("designs")
            .join("landing.html"),
        "<html></html>\n",
    );
    write(
        &fixture_root
            .join("review-host")
            .join("docs")
            .join("diagram.png"),
        "png\n",
    );

    let config_string = config.to_string_lossy().into_owned();
    let fixture_string = fixture_root.to_string_lossy().into_owned();
    let output = run_common_with_env(
        &[
            "review",
            "--ssh-host",
            "review-host",
            "--config",
            &config_string,
        ],
        &[("RWC_REMOTE_REVIEW_FIXTURE_ROOT", &fixture_string)],
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "review smoke failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("remote review host review-host -> review.example.test"));
    assert!(stdout.contains("candidate docs: found"));
    assert!(stdout.contains("candidate designs: found"));
    assert!(stdout.contains("[markdown] docs/index.md (docs)"));
    assert!(stdout.contains("[html] designs/landing.html (designs)"));
    assert!(stdout.contains("[image] docs/diagram.png (docs)"));
}

#[test]
fn review_command_supports_explicit_paths_and_dedupes_candidates() {
    let root = temp_dir("rwc-review-explicit");
    let config = root.join("ssh-config");
    let fixture_root = root.join("fixtures");
    write(
        &config,
        "Host review-host\n  HostName review.example.test\n  User revi\n",
    );
    write(
        &fixture_root
            .join("review-host")
            .join("srv")
            .join("reviews")
            .join("landing.md"),
        "# landing\n",
    );
    write(
        &fixture_root
            .join("review-host")
            .join("srv")
            .join("reviews")
            .join("diagram.svg"),
        "<svg></svg>\n",
    );
    write(
        &fixture_root
            .join("review-host")
            .join("srv")
            .join("reviews")
            .join("notes.txt"),
        "plain text\n",
    );

    let config_string = config.to_string_lossy().into_owned();
    let fixture_string = fixture_root.to_string_lossy().into_owned();
    let output = run_common_with_env(
        &[
            "review",
            "--ssh-host",
            "review-host",
            "--config",
            &config_string,
            "--path",
            "/srv/reviews/",
            "--path",
            "/srv/reviews",
        ],
        &[("RWC_REMOTE_REVIEW_FIXTURE_ROOT", &fixture_string)],
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "review explicit-path smoke failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("discovery mode explicit-paths"));
    assert_eq!(stdout.matches("candidate /srv/reviews: found").count(), 1);
    assert!(stdout.contains("candidate /srv/reviews: found (2 reviewable / 3 total)"));
    assert!(stdout.contains("[markdown] /srv/reviews/landing.md (/srv/reviews)"));
    assert!(stdout.contains("[image] /srv/reviews/diagram.svg (/srv/reviews)"));
}
