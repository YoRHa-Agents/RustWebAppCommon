use crate::{AdapterPlan, AdapterPlanKind};
use glob::{glob, Pattern};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_SSH_CONFIG_RELATIVE_PATH: &str = ".ssh/config";
const DEFAULT_REMOTE_REVIEW_PATHS: [&str; 4] = ["doc", "docs", "design", "designs"];
const REMOTE_FIND_MAX_DEPTH: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteReviewMode {
    AutoDiscover,
    ExplicitPaths,
}

impl RemoteReviewMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AutoDiscover => "auto-discover",
            Self::ExplicitPaths => "explicit-paths",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteDirectoryStatusKind {
    Found,
    Empty,
    Missing,
}

impl RemoteDirectoryStatusKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Found => "found",
            Self::Empty => "empty",
            Self::Missing => "missing",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteReviewFileKind {
    Markdown,
    Html,
    Image,
}

impl RemoteReviewFileKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Markdown => "markdown",
            Self::Html => "html",
            Self::Image => "image",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SshHostEntry {
    pub alias: String,
    pub hostname: String,
    pub user: Option<String>,
    pub port: Option<u16>,
    pub identity_file: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SshHostCatalogIssue {
    pub alias: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SshHostCatalog {
    pub config_path: PathBuf,
    pub hosts: Vec<SshHostEntry>,
    pub issues: Vec<SshHostCatalogIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteReviewPlanRequest {
    pub config_path: Option<PathBuf>,
    pub host_alias: Option<String>,
    pub explicit_paths: Vec<String>,
    pub list_hosts: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteReviewRequest {
    pub config_path: Option<PathBuf>,
    pub host_alias: String,
    pub explicit_paths: Vec<String>,
}

impl RemoteReviewRequest {
    pub fn mode(&self) -> RemoteReviewMode {
        if self.explicit_paths.is_empty() {
            RemoteReviewMode::AutoDiscover
        } else {
            RemoteReviewMode::ExplicitPaths
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteDirectoryStatus {
    pub path: String,
    pub status: RemoteDirectoryStatusKind,
    pub total_files: usize,
    pub reviewable_files: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteReviewItem {
    pub host_alias: String,
    pub directory_path: String,
    pub remote_path: String,
    pub file_kind: RemoteReviewFileKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteReviewReport {
    pub config_path: PathBuf,
    pub host: SshHostEntry,
    pub mode: RemoteReviewMode,
    pub directories: Vec<RemoteDirectoryStatus>,
    pub items: Vec<RemoteReviewItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteReviewError {
    message: String,
}

impl RemoteReviewError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for RemoteReviewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for RemoteReviewError {}

pub trait RemoteDocReviewAdapter {
    fn plan_remote_review(&self, request: &RemoteReviewPlanRequest) -> AdapterPlan;
    fn list_review_hosts(
        &self,
        config_path: Option<&Path>,
    ) -> Result<SshHostCatalog, RemoteReviewError>;
    fn review_remote_docs(
        &self,
        request: &RemoteReviewRequest,
    ) -> Result<RemoteReviewReport, RemoteReviewError>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct StubRemoteDocsReviewAdapter;

impl RemoteDocReviewAdapter for StubRemoteDocsReviewAdapter {
    fn plan_remote_review(&self, request: &RemoteReviewPlanRequest) -> AdapterPlan {
        let mode = if request.list_hosts {
            "list-hosts".to_string()
        } else if request.explicit_paths.is_empty() {
            format!("auto-discover {}", DEFAULT_REMOTE_REVIEW_PATHS.len())
        } else {
            format!("explicit-paths {}", request.explicit_paths.len())
        };
        let config_path = request
            .config_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "~/.ssh/config".to_string());

        AdapterPlan {
            adapter_id: "remote_docs_review_adapter",
            plan_kind: AdapterPlanKind::RemoteReview,
            summary: format!(
                "review ssh config {config_path} mode={mode} alias={:?}",
                request.host_alias
            ),
        }
    }

    fn list_review_hosts(
        &self,
        config_path: Option<&Path>,
    ) -> Result<SshHostCatalog, RemoteReviewError> {
        load_ssh_host_catalog(config_path)
    }

    fn review_remote_docs(
        &self,
        request: &RemoteReviewRequest,
    ) -> Result<RemoteReviewReport, RemoteReviewError> {
        if let Ok(root) = env::var("RWC_REMOTE_REVIEW_FIXTURE_ROOT") {
            let transport = FixtureRemoteReviewTransport {
                root: PathBuf::from(root),
            };
            return review_remote_docs_with_transport(request, &transport);
        }

        review_remote_docs_with_transport(request, &SshCommandRemoteReviewTransport)
    }
}

pub fn default_remote_review_paths() -> &'static [&'static str] {
    &DEFAULT_REMOTE_REVIEW_PATHS
}

fn load_ssh_host_catalog(config_path: Option<&Path>) -> Result<SshHostCatalog, RemoteReviewError> {
    let parsed = parse_ssh_config(config_path)?;
    let mut hosts = Vec::new();
    let mut issues = Vec::new();

    for alias in &parsed.aliases {
        match resolve_host_entry(&parsed, alias) {
            Ok(host) => hosts.push(host),
            Err(error) => issues.push(SshHostCatalogIssue {
                alias: alias.clone(),
                message: error.to_string(),
            }),
        }
    }

    Ok(SshHostCatalog {
        config_path: parsed.config_path,
        hosts,
        issues,
    })
}

fn review_remote_docs_with_transport<T: RemoteReviewTransport>(
    request: &RemoteReviewRequest,
    transport: &T,
) -> Result<RemoteReviewReport, RemoteReviewError> {
    if request.host_alias.trim().is_empty() {
        return Err(RemoteReviewError::new(
            "remote review requires a non-empty ssh host alias",
        ));
    }

    let parsed = parse_ssh_config(request.config_path.as_deref())?;
    let host = resolve_host_entry(&parsed, &request.host_alias)?;
    let requested_paths = requested_review_paths(request)?;
    let inspections = transport.inspect_paths(&parsed.config_path, &host, &requested_paths)?;

    let mut items = Vec::new();
    let mut directories = Vec::new();

    for inspection in inspections {
        let mut reviewable = inspection
            .files
            .iter()
            .filter_map(|remote_path| {
                classify_remote_file(remote_path).map(|file_kind| RemoteReviewItem {
                    host_alias: host.alias.clone(),
                    directory_path: inspection.path.clone(),
                    remote_path: remote_path.clone(),
                    file_kind,
                })
            })
            .collect::<Vec<_>>();
        reviewable.sort_by(|left, right| left.remote_path.cmp(&right.remote_path));

        let status = if !inspection.exists {
            RemoteDirectoryStatusKind::Missing
        } else if reviewable.is_empty() {
            RemoteDirectoryStatusKind::Empty
        } else {
            RemoteDirectoryStatusKind::Found
        };

        directories.push(RemoteDirectoryStatus {
            path: inspection.path.clone(),
            status,
            total_files: inspection.files.len(),
            reviewable_files: reviewable.len(),
        });
        items.append(&mut reviewable);
    }

    items.sort_by(|left, right| left.remote_path.cmp(&right.remote_path));

    Ok(RemoteReviewReport {
        config_path: parsed.config_path,
        host,
        mode: request.mode(),
        directories,
        items,
    })
}

fn requested_review_paths(request: &RemoteReviewRequest) -> Result<Vec<String>, RemoteReviewError> {
    if request.explicit_paths.is_empty() {
        return Ok(default_remote_review_paths()
            .iter()
            .map(|path| (*path).to_string())
            .collect());
    }

    let mut normalized_paths = Vec::new();
    let mut seen = BTreeSet::new();
    for path in &request.explicit_paths {
        let normalized = normalize_remote_path(path)?;
        if seen.insert(normalized.clone()) {
            normalized_paths.push(normalized);
        }
    }
    Ok(normalized_paths)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedSshConfig {
    config_path: PathBuf,
    sections: Vec<SshHostSection>,
    aliases: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SshHostSection {
    patterns: Vec<String>,
    host_name: Option<String>,
    user: Option<String>,
    port: Option<u16>,
    identity_file: Option<PathBuf>,
}

impl SshHostSection {
    fn new(patterns: Vec<String>) -> Self {
        Self {
            patterns,
            host_name: None,
            user: None,
            port: None,
            identity_file: None,
        }
    }
}

fn parse_ssh_config(config_path: Option<&Path>) -> Result<ParsedSshConfig, RemoteReviewError> {
    let config_path = resolved_ssh_config_path(config_path)?;
    let mut sections = Vec::new();
    let mut aliases = BTreeSet::new();
    let mut visited = BTreeSet::new();
    load_config_recursive(&config_path, &mut sections, &mut aliases, &mut visited)?;

    Ok(ParsedSshConfig {
        config_path,
        sections,
        aliases: aliases.into_iter().collect(),
    })
}

fn resolved_ssh_config_path(config_path: Option<&Path>) -> Result<PathBuf, RemoteReviewError> {
    let path = match config_path {
        Some(path) => expand_local_path(path, None),
        None => {
            let home = home_dir()?;
            home.join(DEFAULT_SSH_CONFIG_RELATIVE_PATH)
        }
    };

    if !path.is_file() {
        return Err(RemoteReviewError::new(format!(
            "ssh config not found at {}",
            path.display()
        )));
    }

    Ok(path)
}

fn load_config_recursive(
    path: &Path,
    sections: &mut Vec<SshHostSection>,
    aliases: &mut BTreeSet<String>,
    visited: &mut BTreeSet<PathBuf>,
) -> Result<(), RemoteReviewError> {
    let canonical = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    if !visited.insert(canonical) {
        return Ok(());
    }

    let content = fs::read_to_string(path).map_err(|error| {
        RemoteReviewError::new(format!(
            "failed to read ssh config {}: {error}",
            path.display()
        ))
    })?;

    let mut current_section: Option<SshHostSection> = None;
    for raw_line in content.lines() {
        let stripped_line = strip_comments(raw_line);
        let line = stripped_line.trim();
        if line.is_empty() {
            continue;
        }

        let (keyword, value) = split_keyword_value(line)?;
        match keyword.to_ascii_lowercase().as_str() {
            "include" => {
                for include_path in expand_include_patterns(value, path.parent())? {
                    load_config_recursive(&include_path, sections, aliases, visited)?;
                }
            }
            "host" => {
                if let Some(section) = current_section.take() {
                    sections.push(section);
                }
                let patterns = split_ssh_words(value);
                if patterns.is_empty() {
                    return Err(RemoteReviewError::new(format!(
                        "Host entry in {} is missing an alias or pattern",
                        path.display()
                    )));
                }
                for pattern in &patterns {
                    if is_concrete_alias(pattern) {
                        aliases.insert(pattern.clone());
                    }
                }
                current_section = Some(SshHostSection::new(patterns));
            }
            "hostname" => {
                if let Some(section) = current_section.as_mut() {
                    section.host_name = Some(unquote_value(value));
                }
            }
            "user" => {
                if let Some(section) = current_section.as_mut() {
                    section.user = Some(unquote_value(value));
                }
            }
            "port" => {
                if let Some(section) = current_section.as_mut() {
                    let raw_port = unquote_value(value);
                    let parsed_port = raw_port.parse::<u16>().map_err(|_| {
                        RemoteReviewError::new(format!(
                            "invalid Port `{raw_port}` in {}",
                            path.display()
                        ))
                    })?;
                    section.port = Some(parsed_port);
                }
            }
            "identityfile" => {
                if let Some(section) = current_section.as_mut() {
                    let identity = unquote_value(value);
                    section.identity_file =
                        Some(expand_local_path(Path::new(&identity), path.parent()));
                }
            }
            _ => {}
        }
    }

    if let Some(section) = current_section.take() {
        sections.push(section);
    }

    Ok(())
}

fn expand_include_patterns(
    value: &str,
    base_dir: Option<&Path>,
) -> Result<Vec<PathBuf>, RemoteReviewError> {
    let mut expanded = Vec::new();
    for pattern in split_ssh_words(value) {
        let candidate = expand_local_path(Path::new(&pattern), base_dir);
        let pattern_text = candidate.to_string_lossy().into_owned();
        if contains_glob_syntax(&pattern_text) {
            let matches = glob(&pattern_text).map_err(|error| {
                RemoteReviewError::new(format!("invalid Include pattern `{pattern_text}`: {error}"))
            })?;
            for entry in matches {
                let path = entry.map_err(|error| {
                    RemoteReviewError::new(format!(
                        "failed to expand Include pattern `{pattern_text}`: {error}"
                    ))
                })?;
                if path.is_file() {
                    expanded.push(path);
                }
            }
        } else if candidate.is_file() {
            expanded.push(candidate);
        }
    }

    Ok(expanded)
}

fn resolve_host_entry(
    parsed: &ParsedSshConfig,
    alias: &str,
) -> Result<SshHostEntry, RemoteReviewError> {
    let mut matched = false;
    let mut host_name = None;
    let mut user = None;
    let mut port = None;
    let mut identity_file = None;

    for section in &parsed.sections {
        if section_matches_alias(section, alias) {
            matched = true;
            if host_name.is_none() {
                host_name = section.host_name.clone();
            }
            if user.is_none() {
                user = section.user.clone();
            }
            if port.is_none() {
                port = section.port;
            }
            if identity_file.is_none() {
                identity_file = section.identity_file.clone();
            }
        }
    }

    if !matched {
        return Err(RemoteReviewError::new(format!(
            "ssh alias `{alias}` not found in {}",
            parsed.config_path.display()
        )));
    }

    let hostname = host_name.ok_or_else(|| {
        RemoteReviewError::new(format!(
            "ssh alias `{alias}` is missing HostName in {}",
            parsed.config_path.display()
        ))
    })?;

    Ok(SshHostEntry {
        alias: alias.to_string(),
        hostname,
        user,
        port,
        identity_file,
    })
}

fn section_matches_alias(section: &SshHostSection, alias: &str) -> bool {
    let mut has_positive_match = false;
    for pattern in &section.patterns {
        if let Some(negated) = pattern.strip_prefix('!') {
            if pattern_matches(negated, alias) {
                return false;
            }
        } else if pattern_matches(pattern, alias) {
            has_positive_match = true;
        }
    }

    has_positive_match
}

fn pattern_matches(pattern: &str, alias: &str) -> bool {
    Pattern::new(pattern)
        .map(|compiled| compiled.matches(alias))
        .unwrap_or_else(|_| pattern == alias)
}

fn is_concrete_alias(pattern: &str) -> bool {
    !pattern.starts_with('!') && !contains_glob_syntax(pattern)
}

fn contains_glob_syntax(pattern: &str) -> bool {
    pattern.contains('*') || pattern.contains('?') || pattern.contains('[')
}

fn strip_comments(line: &str) -> String {
    let mut result = String::new();
    let mut quote = None;
    for character in line.chars() {
        if character == '"' || character == '\'' {
            if quote == Some(character) {
                quote = None;
            } else if quote.is_none() {
                quote = Some(character);
            }
        }

        if character == '#' && quote.is_none() {
            break;
        }
        result.push(character);
    }
    result
}

fn split_keyword_value(line: &str) -> Result<(&str, &str), RemoteReviewError> {
    let Some(first_whitespace) = line.find(char::is_whitespace) else {
        return Err(RemoteReviewError::new(format!(
            "invalid ssh config line `{line}`"
        )));
    };

    let keyword = &line[..first_whitespace];
    let value = line[first_whitespace..].trim();
    if value.is_empty() {
        return Err(RemoteReviewError::new(format!(
            "ssh config field `{keyword}` is missing a value"
        )));
    }

    Ok((keyword, value))
}

fn split_ssh_words(value: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut quote = None;
    let mut escaped = false;

    for character in value.chars() {
        if escaped {
            current.push(character);
            escaped = false;
            continue;
        }

        if character == '\\' {
            escaped = true;
            continue;
        }

        match quote {
            Some(active_quote) if character == active_quote => {
                quote = None;
            }
            Some(_) => current.push(character),
            None if character == '"' || character == '\'' => {
                quote = Some(character);
            }
            None if character.is_whitespace() => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            None => current.push(character),
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}

fn unquote_value(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.len() >= 2 {
        let first = trimmed.chars().next().unwrap_or_default();
        let last = trimmed.chars().last().unwrap_or_default();
        if (first == '"' && last == '"') || (first == '\'' && last == '\'') {
            return trimmed[1..trimmed.len() - 1].to_string();
        }
    }
    trimmed.to_string()
}

fn expand_local_path(path: &Path, base_dir: Option<&Path>) -> PathBuf {
    let as_text = path.to_string_lossy();
    if let Some(stripped) = as_text.strip_prefix("~/") {
        return home_dir()
            .map(|home| home.join(stripped))
            .unwrap_or_else(|_| PathBuf::from(as_text.as_ref()));
    }

    if path.is_absolute() {
        path.to_path_buf()
    } else if let Some(base_dir) = base_dir {
        base_dir.join(path)
    } else {
        path.to_path_buf()
    }
}

fn home_dir() -> Result<PathBuf, RemoteReviewError> {
    env::var("HOME")
        .map(PathBuf::from)
        .map_err(|_| RemoteReviewError::new("HOME is not set, cannot locate ~/.ssh/config"))
}

fn normalize_remote_path(path: &str) -> Result<String, RemoteReviewError> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(RemoteReviewError::new(
            "review path arguments must not be empty",
        ));
    }

    if trimmed == "/" {
        return Ok(trimmed.to_string());
    }

    Ok(trimmed.trim_end_matches('/').to_string())
}

fn classify_remote_file(remote_path: &str) -> Option<RemoteReviewFileKind> {
    let extension = Path::new(remote_path)
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_lowercase())?;

    match extension.as_str() {
        "md" | "markdown" => Some(RemoteReviewFileKind::Markdown),
        "html" | "htm" => Some(RemoteReviewFileKind::Html),
        "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" => Some(RemoteReviewFileKind::Image),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RemotePathInspection {
    path: String,
    exists: bool,
    files: Vec<String>,
}

trait RemoteReviewTransport {
    fn inspect_paths(
        &self,
        config_path: &Path,
        host: &SshHostEntry,
        paths: &[String],
    ) -> Result<Vec<RemotePathInspection>, RemoteReviewError>;
}

#[derive(Debug, Clone, Copy)]
struct SshCommandRemoteReviewTransport;

impl RemoteReviewTransport for SshCommandRemoteReviewTransport {
    fn inspect_paths(
        &self,
        config_path: &Path,
        host: &SshHostEntry,
        paths: &[String],
    ) -> Result<Vec<RemotePathInspection>, RemoteReviewError> {
        let script = build_remote_inspection_script(paths);
        let output = Command::new("ssh")
            .arg("-o")
            .arg("BatchMode=yes")
            .arg("-o")
            .arg("ConnectTimeout=5")
            .arg("-F")
            .arg(config_path)
            .arg(&host.alias)
            .arg("sh")
            .arg("-lc")
            .arg(script)
            .output()
            .map_err(|error| {
                RemoteReviewError::new(format!(
                    "failed to run ssh for alias `{}`: {error}",
                    host.alias
                ))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(RemoteReviewError::new(format!(
                "ssh review command for `{}` failed: {}",
                host.alias,
                stderr.trim()
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut inspections = BTreeMap::new();
        for line in stdout.lines() {
            if let Some(rest) = line.strip_prefix("__RWC_STATUS__\t") {
                let mut parts = rest.splitn(2, '\t');
                let path = parts.next().unwrap_or_default().to_string();
                let status = parts.next().unwrap_or_default();
                inspections.insert(
                    path.clone(),
                    RemotePathInspection {
                        path,
                        exists: status == "found",
                        files: Vec::new(),
                    },
                );
                continue;
            }

            if let Some(rest) = line.strip_prefix("__RWC_FILE__\t") {
                let mut parts = rest.splitn(2, '\t');
                let directory = parts.next().unwrap_or_default().to_string();
                let file = parts.next().unwrap_or_default().to_string();
                inspections
                    .entry(directory.clone())
                    .or_insert_with(|| RemotePathInspection {
                        path: directory.clone(),
                        exists: true,
                        files: Vec::new(),
                    })
                    .files
                    .push(file);
            }
        }

        Ok(paths
            .iter()
            .map(|path| {
                inspections
                    .remove(path)
                    .unwrap_or_else(|| RemotePathInspection {
                        path: path.clone(),
                        exists: false,
                        files: Vec::new(),
                    })
            })
            .collect())
    }
}

#[derive(Debug, Clone)]
struct FixtureRemoteReviewTransport {
    root: PathBuf,
}

impl RemoteReviewTransport for FixtureRemoteReviewTransport {
    fn inspect_paths(
        &self,
        _config_path: &Path,
        host: &SshHostEntry,
        paths: &[String],
    ) -> Result<Vec<RemotePathInspection>, RemoteReviewError> {
        let host_root = self.root.join(&host.alias);
        let mut inspections = Vec::new();

        for path in paths {
            let fixture_path = host_root.join(path.trim_start_matches('/'));
            if !fixture_path.is_dir() {
                inspections.push(RemotePathInspection {
                    path: path.clone(),
                    exists: false,
                    files: Vec::new(),
                });
                continue;
            }

            let mut files = Vec::new();
            collect_fixture_files(&fixture_path, &fixture_path, path, &mut files)?;
            files.sort();
            inspections.push(RemotePathInspection {
                path: path.clone(),
                exists: true,
                files,
            });
        }

        Ok(inspections)
    }
}

fn collect_fixture_files(
    root: &Path,
    current: &Path,
    remote_base: &str,
    files: &mut Vec<String>,
) -> Result<(), RemoteReviewError> {
    let entries = fs::read_dir(current).map_err(|error| {
        RemoteReviewError::new(format!(
            "failed to read fixture directory {}: {error}",
            current.display()
        ))
    })?;

    for entry in entries {
        let entry = entry.map_err(|error| {
            RemoteReviewError::new(format!(
                "failed to inspect fixture entry in {}: {error}",
                current.display()
            ))
        })?;
        let path = entry.path();
        if path.is_dir() {
            collect_fixture_files(root, &path, remote_base, files)?;
        } else if path.is_file() {
            let relative = path.strip_prefix(root).map_err(|error| {
                RemoteReviewError::new(format!(
                    "failed to build remote path for fixture {}: {error}",
                    path.display()
                ))
            })?;
            let relative_text = relative.to_string_lossy().replace('\\', "/");
            let normalized_base = normalize_remote_path(remote_base)?;
            let remote_path = if relative_text.is_empty() {
                normalized_base
            } else if normalized_base == "/" {
                format!("/{relative_text}")
            } else {
                format!("{normalized_base}/{relative_text}")
            };
            files.push(remote_path);
        }
    }

    Ok(())
}

fn build_remote_inspection_script(paths: &[String]) -> String {
    let mut script = String::from("set -eu\n");
    for path in paths {
        let quoted = shell_single_quote(path);
        script.push_str(&format!("target={quoted}\n"));
        script.push_str("if [ -d \"$target\" ]; then\n");
        script.push_str("  printf '__RWC_STATUS__\\t%s\\tfound\\n' \"$target\"\n");
        script.push_str(&format!(
            "  find \"$target\" -maxdepth {REMOTE_FIND_MAX_DEPTH} -type f -print | while IFS= read -r file; do printf '__RWC_FILE__\\t%s\\t%s\\n' \"$target\" \"$file\"; done\n"
        ));
        script.push_str("else\n");
        script.push_str("  printf '__RWC_STATUS__\\t%s\\tmissing\\n' \"$target\"\n");
        script.push_str("fi\n");
    }
    script
}

fn shell_single_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}

#[cfg(test)]
mod tests {
    use super::{
        load_ssh_host_catalog, parse_ssh_config, requested_review_paths, resolve_host_entry,
        review_remote_docs_with_transport, FixtureRemoteReviewTransport, RemoteDirectoryStatusKind,
        RemoteDocReviewAdapter, RemoteReviewFileKind, RemoteReviewPlanRequest, RemoteReviewRequest,
        StubRemoteDocsReviewAdapter,
    };
    use crate::AdapterPlanKind;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(prefix: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("{prefix}-{unique}"));
        fs::create_dir_all(&path).expect("temp dir should be created");
        path
    }

    fn write(path: &Path, content: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("parent dir should exist");
        }
        fs::write(path, content).expect("file should be written");
    }

    #[test]
    fn reads_single_host_from_ssh_config() {
        let root = temp_dir("rwc-ssh-config");
        let config = root.join("ssh-config");
        write(
            &config,
            "Host review-host\n  HostName review.example.test\n  User revi\n  Port 2222\n  IdentityFile ~/.ssh/revi_key\n",
        );

        let catalog = load_ssh_host_catalog(Some(config.as_path())).expect("catalog should load");
        assert_eq!(catalog.hosts.len(), 1);
        let host = &catalog.hosts[0];
        assert_eq!(host.alias, "review-host");
        assert_eq!(host.hostname, "review.example.test");
        assert_eq!(host.user.as_deref(), Some("revi"));
        assert_eq!(host.port, Some(2222));
        assert!(host
            .identity_file
            .as_ref()
            .expect("identity file should exist")
            .ends_with(".ssh/revi_key"));
    }

    #[test]
    fn catalog_collects_multiple_hosts_and_reports_invalid_entries() {
        let root = temp_dir("rwc-ssh-catalog");
        let config = root.join("ssh-config");
        write(
            &config,
            "Host *\n  User revi\n  IdentityFile ~/.ssh/revi_key\nHost alpha\n  HostName alpha.example.test\nHost broken\n  User nope\nHost zebra\n  HostName zebra.example.test\n  Port 2202\n",
        );

        let catalog = load_ssh_host_catalog(Some(config.as_path())).expect("catalog should load");
        let aliases = catalog
            .hosts
            .iter()
            .map(|host| host.alias.as_str())
            .collect::<Vec<_>>();
        assert_eq!(aliases, vec!["alpha", "zebra"]);
        assert_eq!(catalog.hosts[0].user.as_deref(), Some("revi"));
        assert!(catalog.hosts[0]
            .identity_file
            .as_ref()
            .expect("identity file should exist")
            .ends_with(".ssh/revi_key"));
        assert_eq!(catalog.hosts[1].port, Some(2202));
        assert_eq!(catalog.issues.len(), 1);
        assert_eq!(catalog.issues[0].alias, "broken");
        assert!(catalog.issues[0].message.contains("missing HostName"));
    }

    #[test]
    fn supports_include_patterns_and_global_defaults() {
        let root = temp_dir("rwc-ssh-include");
        let config = root.join("ssh-config");
        let include = root.join("conf.d").join("review.conf");
        write(&config, "Host *\n  User revi\nInclude conf.d/*.conf\n");
        write(
            &include,
            "Host review-host\n  HostName review.example.test\n  Port 2201\n  ForwardAgent yes\n",
        );

        let parsed = parse_ssh_config(Some(config.as_path())).expect("config should parse");
        let host = resolve_host_entry(&parsed, "review-host").expect("host should resolve");
        assert_eq!(host.hostname, "review.example.test");
        assert_eq!(host.user.as_deref(), Some("revi"));
        assert_eq!(host.port, Some(2201));
    }

    #[test]
    fn reports_missing_hostname_as_a_clear_error() {
        let root = temp_dir("rwc-ssh-error");
        let config = root.join("ssh-config");
        write(&config, "Host broken\n  User revi\n");

        let parsed = parse_ssh_config(Some(config.as_path())).expect("config should parse");
        let error = resolve_host_entry(&parsed, "broken").expect_err("host should fail");
        assert!(error
            .to_string()
            .contains("ssh alias `broken` is missing HostName"));
    }

    #[test]
    fn review_report_discovers_candidate_directories_with_fixture_transport() {
        let root = temp_dir("rwc-remote-review");
        let config = root.join("ssh-config");
        write(
            &config,
            "Host review-host\n  HostName review.example.test\n  User revi\n",
        );
        write(
            &root
                .join("fixtures")
                .join("review-host")
                .join("docs")
                .join("index.md"),
            "# docs\n",
        );
        write(
            &root
                .join("fixtures")
                .join("review-host")
                .join("docs")
                .join("diagram.png"),
            "png\n",
        );
        write(
            &root
                .join("fixtures")
                .join("review-host")
                .join("designs")
                .join("mockup.html"),
            "<html></html>\n",
        );

        let report = review_remote_docs_with_transport(
            &RemoteReviewRequest {
                config_path: Some(config.clone()),
                host_alias: "review-host".to_string(),
                explicit_paths: Vec::new(),
            },
            &FixtureRemoteReviewTransport {
                root: root.join("fixtures"),
            },
        )
        .expect("report should load");

        assert_eq!(report.items.len(), 3);
        assert_eq!(report.directories.len(), 4);
        assert_eq!(report.directories[0].path, "doc");
        assert_eq!(
            report.directories[0].status,
            RemoteDirectoryStatusKind::Missing
        );
        assert_eq!(report.directories[1].path, "docs");
        assert_eq!(
            report.directories[1].status,
            RemoteDirectoryStatusKind::Found
        );
        assert_eq!(report.directories[3].path, "designs");
        assert_eq!(
            report.directories[3].status,
            RemoteDirectoryStatusKind::Found
        );
        assert!(report
            .items
            .iter()
            .any(|item| item.file_kind == RemoteReviewFileKind::Markdown));
        assert!(report
            .items
            .iter()
            .any(|item| item.file_kind == RemoteReviewFileKind::Image));
        assert!(report
            .items
            .iter()
            .any(|item| item.file_kind == RemoteReviewFileKind::Html));
    }

    #[test]
    fn review_report_supports_explicit_paths() {
        let root = temp_dir("rwc-explicit-review");
        let config = root.join("ssh-config");
        write(
            &config,
            "Host review-host\n  HostName review.example.test\n",
        );
        write(
            &root
                .join("fixtures")
                .join("review-host")
                .join("srv")
                .join("reviews")
                .join("landing.md"),
            "# landing\n",
        );

        let report = review_remote_docs_with_transport(
            &RemoteReviewRequest {
                config_path: Some(config.clone()),
                host_alias: "review-host".to_string(),
                explicit_paths: vec!["/srv/reviews".to_string()],
            },
            &FixtureRemoteReviewTransport {
                root: root.join("fixtures"),
            },
        )
        .expect("report should load");

        assert_eq!(report.directories.len(), 1);
        assert_eq!(report.directories[0].path, "/srv/reviews");
        assert_eq!(
            report.directories[0].status,
            RemoteDirectoryStatusKind::Found
        );
        assert_eq!(report.items[0].remote_path, "/srv/reviews/landing.md");
    }

    #[test]
    fn explicit_paths_are_normalized_and_deduped() {
        let paths = requested_review_paths(&RemoteReviewRequest {
            config_path: None,
            host_alias: "review-host".to_string(),
            explicit_paths: vec![
                " /srv/reviews/ ".to_string(),
                "/srv/reviews".to_string(),
                "/designs//".to_string(),
            ],
        })
        .expect("paths should normalize");

        assert_eq!(paths, vec!["/srv/reviews", "/designs"]);
    }

    #[test]
    fn explicit_paths_reject_blank_values() {
        let error = requested_review_paths(&RemoteReviewRequest {
            config_path: None,
            host_alias: "review-host".to_string(),
            explicit_paths: vec!["   ".to_string()],
        })
        .expect_err("blank path should fail");

        assert!(error.to_string().contains("must not be empty"));
    }

    #[test]
    fn review_report_marks_existing_non_reviewable_directory_as_empty() {
        let root = temp_dir("rwc-empty-review");
        let config = root.join("ssh-config");
        write(
            &config,
            "Host review-host\n  HostName review.example.test\n  User revi\n",
        );
        write(
            &root
                .join("fixtures")
                .join("review-host")
                .join("docs")
                .join("notes.txt"),
            "plain text\n",
        );

        let report = review_remote_docs_with_transport(
            &RemoteReviewRequest {
                config_path: Some(config.clone()),
                host_alias: "review-host".to_string(),
                explicit_paths: Vec::new(),
            },
            &FixtureRemoteReviewTransport {
                root: root.join("fixtures"),
            },
        )
        .expect("report should load");

        let docs_status = report
            .directories
            .iter()
            .find(|directory| directory.path == "docs")
            .expect("docs directory should be present");
        assert_eq!(docs_status.status, RemoteDirectoryStatusKind::Empty);
        assert_eq!(docs_status.total_files, 1);
        assert_eq!(docs_status.reviewable_files, 0);
        assert!(report.items.is_empty());
    }

    #[test]
    fn review_adapter_plan_uses_remote_review_plan_kind() {
        let plan = StubRemoteDocsReviewAdapter.plan_remote_review(&RemoteReviewPlanRequest {
            config_path: None,
            host_alias: Some("review-host".to_string()),
            explicit_paths: Vec::new(),
            list_hosts: false,
        });

        assert_eq!(plan.adapter_id, "remote_docs_review_adapter");
        assert_eq!(plan.plan_kind, AdapterPlanKind::RemoteReview);
        assert!(plan.summary.contains("review-host"));
    }
}
