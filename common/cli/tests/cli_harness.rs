use std::path::PathBuf;
use std::process::Command;

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
