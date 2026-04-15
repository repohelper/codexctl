use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};
use serde::Serialize;

use crate::utils::task::BetSpec;

pub const SHAPEUP_TASK_PREFIX: &str = "shapeup-bet-";
pub const SHAPEUP_LINT_FAILED_EXIT_CODE: u8 = 14;
pub const BET_SCHEMA_TYPE: &str = "codexctl-bet/v1";
const PLACEHOLDER_TOKENS: &[&str] = &[
    "todo",
    "tbd",
    "replace me",
    "replace_this",
    "replace-with",
    "placeholder",
    "<fill",
    "your objective here",
    "your success signal here",
];

pub const ALLOWED_BOUNDED_CONTEXTS: &[&str] = &[
    "Profile Catalog",
    "Live Auth Projection",
    "Usage Intelligence",
    "Validation",
    "Task Definition",
    "Run Orchestration",
    "Run Ledger",
    "Release Engineering",
];

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LintSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LintIssue {
    pub severity: LintSeverity,
    pub code: String,
    pub field: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LintStatus {
    Passed,
    Warnings,
    Failed,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LintReport {
    pub path: String,
    pub task_name: Option<String>,
    pub status: LintStatus,
    pub issues: Vec<LintIssue>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LintSummary {
    pub total_files: usize,
    pub passed: usize,
    pub warnings: usize,
    pub failed: usize,
    pub issue_count: usize,
}

pub fn slugify_bet_name(name: &str) -> String {
    let mut slug = String::with_capacity(name.len());
    let mut previous_was_dash = false;

    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            previous_was_dash = false;
        } else if !previous_was_dash {
            slug.push('-');
            previous_was_dash = true;
        }
    }

    let slug = slug.trim_matches('-').to_string();
    if slug.is_empty() {
        "new-bet".to_string()
    } else {
        slug
    }
}

pub fn default_task_dir(repo_root: &Path) -> PathBuf {
    repo_root.join(".codexctl").join("tasks")
}

pub fn default_task_path(repo_root: &Path, name: &str) -> PathBuf {
    default_task_dir(repo_root).join(format!(
        "{}{}.yaml",
        SHAPEUP_TASK_PREFIX,
        slugify_bet_name(name)
    ))
}

pub fn default_bounded_contexts() -> Vec<String> {
    vec!["Task Definition".to_string()]
}

pub fn render_bet_scaffold(raw_name: &str, appetite: &str, bounded_contexts: &[String]) -> String {
    let bet_name = slugify_bet_name(raw_name);
    let context_lines = bounded_contexts
        .iter()
        .map(|context| format!("  - {context}"))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "type: {BET_SCHEMA_TYPE}\n\
name: {bet_name}\n\
appetite: {appetite}\n\
objective: Replace this with the shaped outcome for {bet_name}\n\
bounded_contexts:\n\
{context_lines}\n\
success_signal: Replace this with an operator-visible success signal\n\
no_gos:\n\
  - Do not broaden scope beyond the stated appetite.\n\
context_files: []\n\
constraints: []\n\
acceptance_checks:\n\
  - echo 'replace with deterministic acceptance checks' >&2 && exit 1\n\
review_checks: []\n\
notes: |\n\
  Edit the objective, success signal, bounded contexts, and checks before running this bet.\n"
    )
}

pub fn discover_task_specs(repo_root: &Path) -> Result<Vec<PathBuf>> {
    let task_dir = default_task_dir(repo_root);
    if !task_dir.exists() {
        return Ok(Vec::new());
    }

    let mut paths = Vec::new();
    for entry in std::fs::read_dir(&task_dir)
        .with_context(|| format!("Failed to read task directory: {}", task_dir.display()))?
    {
        let entry = entry.with_context(|| {
            format!(
                "Failed to read an entry from task directory: {}",
                task_dir.display()
            )
        })?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
            continue;
        };
        if matches!(extension, "yaml" | "yml") {
            paths.push(path);
        }
    }

    paths.sort();
    Ok(paths)
}

pub async fn lint_task_file(repo_root: &Path, task_path: &Path) -> LintReport {
    let display_path = relative_display(repo_root, task_path);
    let mut issues = Vec::new();
    let file_name = task_path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default();

    if !(file_name.starts_with(SHAPEUP_TASK_PREFIX)
        && matches!(
            task_path.extension().and_then(|value| value.to_str()),
            Some("yaml" | "yml")
        ))
    {
        issues.push(LintIssue {
            severity: LintSeverity::Warning,
            code: "file_name_convention".to_string(),
            field: None,
            message: format!(
                "Task files should use '{}<name>.yaml' under .codexctl/tasks",
                SHAPEUP_TASK_PREFIX
            ),
        });
    }

    let spec = match BetSpec::load_from_path(task_path).await {
        Ok(spec) => spec,
        Err(error) => {
            issues.push(LintIssue {
                severity: LintSeverity::Error,
                code: "spec_invalid".to_string(),
                field: None,
                message: error.to_string(),
            });
            return LintReport {
                path: display_path,
                task_name: None,
                status: LintStatus::Failed,
                issues,
            };
        }
    };

    let task_name = Some(spec.name.clone());

    for context in &spec.bounded_contexts {
        if !ALLOWED_BOUNDED_CONTEXTS.contains(&context.as_str()) {
            issues.push(LintIssue {
                severity: LintSeverity::Error,
                code: "unknown_bounded_context".to_string(),
                field: Some("bounded_contexts".to_string()),
                message: format!(
                    "Unknown bounded context '{}'. Allowed values: {}",
                    context,
                    ALLOWED_BOUNDED_CONTEXTS.join(", ")
                ),
            });
        }
    }

    if contains_placeholder(&spec.objective) {
        issues.push(LintIssue {
            severity: LintSeverity::Error,
            code: "objective_placeholder".to_string(),
            field: Some("objective".to_string()),
            message: "Objective still contains placeholder language".to_string(),
        });
    } else if spec.objective.len() < 24 {
        issues.push(LintIssue {
            severity: LintSeverity::Warning,
            code: "objective_too_short".to_string(),
            field: Some("objective".to_string()),
            message: "Objective is unusually short for a shaped bet".to_string(),
        });
    }

    if contains_placeholder(&spec.success_signal) {
        issues.push(LintIssue {
            severity: LintSeverity::Error,
            code: "success_signal_placeholder".to_string(),
            field: Some("success_signal".to_string()),
            message: "Success signal still contains placeholder language".to_string(),
        });
    } else if spec.success_signal.len() < 24 {
        issues.push(LintIssue {
            severity: LintSeverity::Warning,
            code: "success_signal_too_short".to_string(),
            field: Some("success_signal".to_string()),
            message: "Success signal is unusually short for operator-facing validation".to_string(),
        });
    }

    for no_go in &spec.no_gos {
        if contains_placeholder(no_go) {
            issues.push(LintIssue {
                severity: LintSeverity::Error,
                code: "no_go_placeholder".to_string(),
                field: Some("no_gos".to_string()),
                message: "A no-go entry still contains placeholder language".to_string(),
            });
        } else if !no_go.starts_with("Do not ") {
            issues.push(LintIssue {
                severity: LintSeverity::Warning,
                code: "no_go_style".to_string(),
                field: Some("no_gos".to_string()),
                message: format!("No-go should usually start with 'Do not': {no_go}"),
            });
        }
    }

    for check in &spec.acceptance_checks {
        if contains_placeholder(check) {
            issues.push(LintIssue {
                severity: LintSeverity::Error,
                code: "acceptance_check_placeholder".to_string(),
                field: Some("acceptance_checks".to_string()),
                message: "Acceptance checks still contain placeholder language".to_string(),
            });
        } else if matches!(check.trim(), "true" | ":" | "exit 0") {
            issues.push(LintIssue {
                severity: LintSeverity::Warning,
                code: "acceptance_check_weak".to_string(),
                field: Some("acceptance_checks".to_string()),
                message: format!("Acceptance check is too weak to prove success: {check}"),
            });
        }
    }

    for check in &spec.review_checks {
        if contains_placeholder(check) {
            issues.push(LintIssue {
                severity: LintSeverity::Error,
                code: "review_check_placeholder".to_string(),
                field: Some("review_checks".to_string()),
                message: "Review checks still contain placeholder language".to_string(),
            });
        }
    }

    let status = lint_status(&issues);
    LintReport {
        path: display_path,
        task_name,
        status,
        issues,
    }
}

pub fn summarize_reports(reports: &[LintReport]) -> LintSummary {
    let mut summary = LintSummary {
        total_files: reports.len(),
        passed: 0,
        warnings: 0,
        failed: 0,
        issue_count: 0,
    };

    for report in reports {
        summary.issue_count += report.issues.len();
        match report.status {
            LintStatus::Passed => summary.passed += 1,
            LintStatus::Warnings => summary.warnings += 1,
            LintStatus::Failed => summary.failed += 1,
        }
    }

    summary
}

fn lint_status(issues: &[LintIssue]) -> LintStatus {
    if issues
        .iter()
        .any(|issue| issue.severity == LintSeverity::Error)
    {
        LintStatus::Failed
    } else if issues.is_empty() {
        LintStatus::Passed
    } else {
        LintStatus::Warnings
    }
}

fn contains_placeholder(value: &str) -> bool {
    let normalized = value.trim().to_ascii_lowercase();
    PLACEHOLDER_TOKENS
        .iter()
        .any(|token| normalized.contains(token))
}

fn relative_display(repo_root: &Path, task_path: &Path) -> String {
    task_path
        .strip_prefix(repo_root)
        .unwrap_or(task_path)
        .display()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_handles_spaces_and_symbols() {
        assert_eq!(slugify_bet_name("My New Bet!"), "my-new-bet");
        assert_eq!(slugify_bet_name("___"), "new-bet");
    }

    #[test]
    fn scaffold_contains_shapeup_prefix_fields() {
        let rendered = render_bet_scaffold("Auth UX", "1_week", &default_bounded_contexts());
        assert!(rendered.contains("type: codexctl-bet/v1"));
        assert!(rendered.contains("name: auth-ux"));
        assert!(rendered.contains("bounded_contexts:"));
    }
}
