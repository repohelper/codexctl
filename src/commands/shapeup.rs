use std::path::PathBuf;

use anyhow::{Context as _, Result};
use colored::Colorize as _;
use serde::Serialize;

use crate::utils::command_exit::fail;
use crate::utils::config::Config;
use crate::utils::shapeup::{
    LintReport, LintSeverity, LintStatus, SHAPEUP_LINT_FAILED_EXIT_CODE, default_bounded_contexts,
    default_task_dir, default_task_path, discover_task_specs, lint_task_file, render_bet_scaffold,
    summarize_reports,
};

#[allow(clippy::too_many_arguments)]
pub async fn init_bet(
    _config: Config,
    name: String,
    appetite: String,
    bounded_contexts: Vec<String>,
    force: bool,
    stdout: bool,
    json: bool,
    quiet: bool,
) -> Result<()> {
    if stdout && json {
        return fail(2, "shapeup init-bet accepts only one of --stdout or --json");
    }

    let repo_root =
        std::env::current_dir().context("Failed to resolve current working directory")?;
    let bounded_contexts = if bounded_contexts.is_empty() {
        default_bounded_contexts()
    } else {
        bounded_contexts
    };
    let task_path = default_task_path(&repo_root, &name);
    let scaffold = render_bet_scaffold(&name, &appetite, &bounded_contexts);
    let existed = task_path.exists();

    if stdout {
        print!("{scaffold}");
        return Ok(());
    }

    let parent = task_path
        .parent()
        .map(PathBuf::from)
        .unwrap_or_else(|| default_task_dir(&repo_root));
    tokio::fs::create_dir_all(&parent)
        .await
        .with_context(|| format!("Failed to create {}", parent.display()))?;

    if existed && !force {
        return fail(
            2,
            format!(
                "Bet scaffold already exists: {} (use --force to overwrite)",
                task_path.display()
            ),
        );
    }

    tokio::fs::write(&task_path, scaffold)
        .await
        .with_context(|| format!("Failed to write {}", task_path.display()))?;

    let relative_path = task_path
        .strip_prefix(&repo_root)
        .unwrap_or(&task_path)
        .display()
        .to_string();

    if json {
        let payload = InitBetJson {
            schema_version: "shapeup_init_bet/v1".to_string(),
            command: "shapeup".to_string(),
            subcommand: "init-bet".to_string(),
            path: relative_path,
            appetite,
            bounded_contexts,
            overwritten: existed && force,
        };
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else if !quiet {
        println!("{}", "Shapeup Init".bold().cyan());
        println!("  Path: {}", relative_path.green());
        println!("  Appetite: {}", appetite.cyan());
        println!("  Contexts: {}", bounded_contexts.join(", "));
        println!("  Next: edit the scaffold, then run `codexctl shapeup lint`");
    }

    Ok(())
}

pub async fn lint(_config: Config, task: Option<PathBuf>, json: bool, quiet: bool) -> Result<()> {
    let repo_root =
        std::env::current_dir().context("Failed to resolve current working directory")?;
    let task_paths = if let Some(task_path) = task {
        vec![task_path]
    } else {
        discover_task_specs(&repo_root)?
    };

    if task_paths.is_empty() {
        return fail(23, "No bet specs found under .codexctl/tasks");
    }

    let mut reports = Vec::with_capacity(task_paths.len());
    for task_path in &task_paths {
        reports.push(lint_task_file(&repo_root, task_path).await);
    }

    let summary = summarize_reports(&reports);
    if json {
        let payload = LintJson {
            schema_version: "shapeup_lint/v1".to_string(),
            command: "shapeup".to_string(),
            subcommand: "lint".to_string(),
            status: if summary.failed > 0 {
                LintStatus::Failed
            } else if summary.warnings > 0 {
                LintStatus::Warnings
            } else {
                LintStatus::Passed
            },
            summary: summary.clone(),
            reports,
        };
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else if !quiet {
        print_lint_summary(&reports, &summary);
    }

    if summary.failed > 0 {
        return fail(
            SHAPEUP_LINT_FAILED_EXIT_CODE,
            "One or more bet specs failed Shape Up lint checks",
        );
    }

    Ok(())
}

#[derive(Debug, Serialize)]
struct InitBetJson {
    schema_version: String,
    command: String,
    subcommand: String,
    path: String,
    appetite: String,
    bounded_contexts: Vec<String>,
    overwritten: bool,
}

#[derive(Debug, Serialize)]
struct LintJson {
    schema_version: String,
    command: String,
    subcommand: String,
    status: LintStatus,
    summary: crate::utils::shapeup::LintSummary,
    reports: Vec<LintReport>,
}

fn print_lint_summary(reports: &[LintReport], summary: &crate::utils::shapeup::LintSummary) {
    println!("{}", "Shapeup Lint".bold().cyan());
    println!(
        "  Files: {} total, {} passed, {} warnings, {} failed",
        summary.total_files,
        summary.passed.to_string().green(),
        summary.warnings.to_string().yellow(),
        summary.failed.to_string().red(),
    );

    for report in reports {
        let status = match report.status {
            LintStatus::Passed => "passed".green(),
            LintStatus::Warnings => "warnings".yellow(),
            LintStatus::Failed => "failed".red(),
        };
        println!("  {} {}", status, report.path);
        for issue in &report.issues {
            let severity = match issue.severity {
                LintSeverity::Warning => "warning".yellow(),
                LintSeverity::Error => "error".red(),
            };
            if let Some(field) = &issue.field {
                println!("    - {} [{}] {}", severity, field, issue.message);
            } else {
                println!("    - {} {}", severity, issue.message);
            }
        }
    }
}
