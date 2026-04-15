#![allow(missing_docs)]

use std::fs;
use std::path::PathBuf;

use assert_cmd::Command;
use serde_json::Value;
use serial_test::serial;
use tempfile::TempDir;

#[test]
#[serial]
fn shapeup_init_bet_writes_scaffold_under_codexctl_tasks() {
    let fixture = CliFixture::new();

    let output = fixture
        .command()
        .args(["shapeup", "init-bet", "Auth UX", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let payload: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(payload["schema_version"], "shapeup_init_bet/v1");
    assert_eq!(payload["subcommand"], "init-bet");

    let task_path = fixture.repo_dir.join(payload["path"].as_str().unwrap());
    assert!(task_path.exists());

    let scaffold = fs::read_to_string(task_path).unwrap();
    assert!(scaffold.contains("type: codexctl-bet/v1"));
    assert!(scaffold.contains("name: auth-ux"));
}

#[test]
#[serial]
fn shapeup_init_bet_stdout_prints_scaffold_without_writing_file() {
    let fixture = CliFixture::new();

    let output = fixture
        .command()
        .args(["shapeup", "init-bet", "Auth UX", "--stdout"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let scaffold = String::from_utf8(output).unwrap();
    assert!(scaffold.contains("name: auth-ux"));
    assert!(!fixture.repo_dir.join(".codexctl").exists());
}

#[test]
#[serial]
fn shapeup_lint_passes_for_valid_spec() {
    let fixture = CliFixture::new();
    let task_path = fixture.write_task(
        "shapeup-bet-valid.yaml",
        r#"
type: codexctl-bet/v1
name: valid-bet
appetite: 1_week
objective: Improve validation messaging for high-agency operators
bounded_contexts:
  - Validation
success_signal: Operators can understand validation failures without inspecting raw logs
no_gos:
  - Do not add queueing.
acceptance_checks:
  - test -n "ok"
"#,
    );

    let output = fixture
        .command()
        .args([
            "shapeup",
            "lint",
            "--task",
            task_path.to_str().unwrap(),
            "--json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let payload: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(payload["schema_version"], "shapeup_lint/v1");
    assert_eq!(payload["status"], "passed");
    assert_eq!(payload["summary"]["failed"], 0);
}

#[test]
#[serial]
fn shapeup_lint_fails_for_unknown_bounded_context() {
    let fixture = CliFixture::new();
    let task_path = fixture.write_task(
        "shapeup-bet-invalid.yaml",
        r#"
type: codexctl-bet/v1
name: invalid-bet
appetite: 1_week
objective: Improve something meaningful for operators
bounded_contexts:
  - Unknown Context
success_signal: The operator sees explicit improvements in the CLI
no_gos:
  - Do not add queueing.
acceptance_checks:
  - test -n "ok"
"#,
    );

    let output = fixture
        .command()
        .args([
            "shapeup",
            "lint",
            "--task",
            task_path.to_str().unwrap(),
            "--json",
        ])
        .assert()
        .failure()
        .code(14)
        .get_output()
        .stdout
        .clone();

    let payload: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(payload["status"], "failed");
    let reports = payload["reports"].as_array().unwrap();
    assert!(
        reports[0]["issues"]
            .as_array()
            .unwrap()
            .iter()
            .any(|issue| issue["code"] == "unknown_bounded_context")
    );
}

struct CliFixture {
    _temp: TempDir,
    home_dir: PathBuf,
    config_dir: PathBuf,
    repo_dir: PathBuf,
}

impl CliFixture {
    fn new() -> Self {
        let temp = TempDir::new().unwrap();
        let home_dir = temp.path().join("home");
        let config_dir = temp.path().join("profiles");
        let repo_dir = temp.path().join("repo");
        fs::create_dir_all(home_dir.join(".codex")).unwrap();
        fs::create_dir_all(&config_dir).unwrap();
        fs::create_dir_all(&repo_dir).unwrap();

        Self {
            _temp: temp,
            home_dir,
            config_dir,
            repo_dir,
        }
    }

    fn command(&self) -> Command {
        let mut cmd = Command::cargo_bin("codexctl").unwrap();
        cmd.current_dir(&self.repo_dir)
            .env("HOME", &self.home_dir)
            .env("CODEXCTL_DIR", &self.config_dir)
            .env("NO_COLOR", "1");
        cmd
    }

    fn write_task(&self, file_name: &str, content: &str) -> PathBuf {
        let tasks_dir = self.repo_dir.join(".codexctl").join("tasks");
        fs::create_dir_all(&tasks_dir).unwrap();
        let path = tasks_dir.join(file_name);
        fs::write(&path, content).unwrap();
        path
    }
}
