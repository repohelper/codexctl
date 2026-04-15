#![allow(missing_docs)]

use std::fs;
use std::path::PathBuf;
use std::process::Command as StdCommand;

use assert_cmd::Command;
use serde_json::{Value, json};
use serial_test::serial;
use tempfile::TempDir;

#[test]
#[serial]
fn run_loop_succeeds_and_persists_run_state() {
    let fixture = CliFixture::new();
    let task_path = fixture.write_task(
        "success.yaml",
        r#"
type: codexctl-bet/v1
name: success-bet
appetite: 1_week
objective: Produce a file and validate it
bounded_contexts:
  - Run Orchestration
success_signal: The loop creates loop.txt and validation passes
no_gos:
  - Do not add queueing.
acceptance_checks:
  - test -f loop.txt
agent:
  command:
    - bash
    - -lc
    - printf 'done\n' > loop.txt
"#,
    );

    let output = fixture
        .command()
        .args(["run-loop", "--task", task_path.to_str().unwrap(), "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let payload: Value = serde_json::from_slice(&output).unwrap();
    let run_id = payload["run_id"].as_str().unwrap();
    assert_eq!(payload["schema_version"], "run_loop/v1");
    assert_eq!(payload["status"], "succeeded");
    assert_eq!(payload["iteration_count"], 1);
    assert_eq!(payload["review_status"], "skipped");

    let run_dir = fixture.config_dir.join("runs").join(run_id);
    assert!(run_dir.join("run.json").exists());
    assert!(run_dir.join("final-report.md").exists());
    assert!(run_dir.join("task.snapshot.yaml").exists());
    assert!(run_dir.join("iterations").join("001.prompt.md").exists());
    assert!(run_dir.join("iterations").join("001.summary.md").exists());
    assert!(
        run_dir
            .join("iterations")
            .join("001.validation.json")
            .exists()
    );

    let latest_output = fixture
        .command()
        .args(["runs", "--latest", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let latest_payload: Value = serde_json::from_slice(&latest_output).unwrap();
    assert_eq!(latest_payload["run"]["run_id"], run_id);
    assert_eq!(latest_payload["run"]["status"], "succeeded");
    assert_eq!(latest_payload["run"]["repo_state"]["is_git_repo"], false);

    let list_output = fixture
        .command()
        .args(["runs", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let list_payload: Value = serde_json::from_slice(&list_output).unwrap();
    assert_eq!(list_payload["items"].as_array().unwrap().len(), 1);
}

#[test]
#[serial]
fn run_loop_returns_exit_21_when_budget_is_exhausted() {
    let fixture = CliFixture::new();
    let task_path = fixture.write_task(
        "budget.yaml",
        r#"
type: codexctl-bet/v1
name: failing-bet
appetite: 1_week
objective: Demonstrate a failing run
bounded_contexts:
  - Validation
success_signal: Acceptance checks stay red
no_gos:
  - Do not mutate the budget logic.
acceptance_checks:
  - false
agent:
  command:
    - bash
    - -lc
    - printf 'ran\n' > ran.txt
budgets:
  max_iterations: 1
  max_consecutive_failures: 1
"#,
    );

    let output = fixture
        .command()
        .args(["run-loop", "--task", task_path.to_str().unwrap(), "--json"])
        .assert()
        .failure()
        .code(21)
        .get_output()
        .stdout
        .clone();

    let payload: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(payload["status"], "budget_exhausted");
    assert_eq!(payload["stop_reason"], "max_consecutive_failures_reached");
    assert_eq!(payload["latest_validation"]["failed"], 1);
}

#[test]
#[serial]
fn run_loop_resume_executes_from_persisted_repo_root() {
    let fixture = CliFixture::new();
    let run_id = "20260414T000000Z-deadbeef";
    let run_dir = fixture.config_dir.join("runs").join(run_id);
    fs::create_dir_all(run_dir.join("iterations")).unwrap();
    fs::create_dir_all(run_dir.join("logs")).unwrap();

    fs::write(
        run_dir.join("task.snapshot.yaml"),
        r#"
type: codexctl-bet/v1
name: resumed-bet
appetite: 1_week
objective: Resume from a persisted run record
bounded_contexts:
  - Run Ledger
success_signal: The resumed run creates resumed.txt
no_gos:
  - Do not reinitialize the run.
acceptance_checks:
  - test -f resumed.txt
agent:
  command:
    - bash
    - -lc
    - printf 'resumed\n' > resumed.txt
"#,
    )
    .unwrap();

    fs::write(
        run_dir.join("run.json"),
        serde_json::to_vec_pretty(&json!({
            "schema_version": "runs/v1",
            "run_id": run_id,
            "status": "queued",
            "stop_reason": null,
            "task_name": "resumed-bet",
            "task_path": fixture.repo_dir.join(".codexctl/tasks/resume.yaml").display().to_string(),
            "repo_root": fixture.repo_dir.display().to_string(),
            "profile": null,
            "auth_mode": null,
            "iteration_count": 0,
            "started_at": "2026-04-14T00:00:00Z",
            "updated_at": "2026-04-14T00:00:00Z",
            "finished_at": null,
            "latest_validation": {
                "status": null,
                "passed": 0,
                "failed": 0,
                "timed_out": 0,
                "errors": 0
            }
        }))
        .unwrap(),
    )
    .unwrap();

    let alternate_dir = fixture.temp.path().join("elsewhere");
    fs::create_dir_all(&alternate_dir).unwrap();

    let output = fixture
        .command_in(&alternate_dir)
        .args(["run-loop", "--resume", run_id, "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let payload: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(payload["status"], "succeeded");
    assert!(fixture.repo_dir.join("resumed.txt").exists());
}

#[test]
#[serial]
fn run_loop_uses_profile_auth_and_restores_original_auth() {
    let fixture = CliFixture::new();
    fixture.write_profile("builder", br#"{"api_key":"profile-key"}"#);
    fixture.write_live_auth(br#"{"api_key":"original-key"}"#);
    let task_path = fixture.write_task(
        "profile.yaml",
        r#"
type: codexctl-bet/v1
name: profile-bet
appetite: 1_week
objective: Confirm profile auth is active during the agent step
bounded_contexts:
  - Auth Switching
success_signal: The agent observes profile auth and local auth is restored after the run
no_gos:
  - Do not mutate non-auth Codex state.
acceptance_checks:
  - grep -q 'profile-key' profile_seen.json
agent:
  command:
    - bash
    - -lc
    - cat "$HOME/.codex/auth.json" > profile_seen.json
"#,
    );

    fixture
        .command()
        .args([
            "run-loop",
            "--task",
            task_path.to_str().unwrap(),
            "--profile",
            "builder",
            "--json",
        ])
        .assert()
        .success();

    assert!(fixture.repo_dir.join("profile_seen.json").exists());
    let seen_auth = fs::read_to_string(fixture.repo_dir.join("profile_seen.json")).unwrap();
    assert!(seen_auth.contains("profile-key"));

    let live_auth = fs::read_to_string(fixture.home_dir.join(".codex").join("auth.json")).unwrap();
    assert!(live_auth.contains("original-key"));
}

#[test]
#[serial]
fn run_loop_restores_profile_auth_after_agent_failure() {
    let fixture = CliFixture::new();
    fixture.write_profile("builder", br#"{"api_key":"profile-key"}"#);
    fixture.write_live_auth(br#"{"api_key":"original-key"}"#);
    let task_path = fixture.write_task(
        "profile-fail.yaml",
        r#"
type: codexctl-bet/v1
name: profile-fail-bet
appetite: 1_week
objective: Restore auth even when the agent command fails
bounded_contexts:
  - Auth Switching
success_signal: Original auth is restored after a failing agent command
no_gos:
  - Do not mutate non-auth Codex state.
acceptance_checks:
  - true
agent:
  command:
    - bash
    - -lc
    - exit 7
"#,
    );

    fixture
        .command()
        .args([
            "run-loop",
            "--task",
            task_path.to_str().unwrap(),
            "--profile",
            "builder",
            "--json",
        ])
        .assert()
        .failure()
        .code(25);

    let live_auth = fs::read_to_string(fixture.home_dir.join(".codex").join("auth.json")).unwrap();
    assert!(live_auth.contains("original-key"));
}

#[test]
#[serial]
fn run_loop_blocks_dirty_git_repositories_on_initial_start() {
    let fixture = CliFixture::new();
    fixture.init_git_repo();
    fs::write(fixture.repo_dir.join("dirty.txt"), "dirty").unwrap();

    let task_path = fixture.write_task(
        "dirty.yaml",
        r#"
type: codexctl-bet/v1
name: dirty-bet
appetite: 1_week
objective: Refuse unattended execution from a dirty repo
bounded_contexts:
  - Run Orchestration
success_signal: The run is blocked before the first iteration
no_gos:
  - Do not auto-clean the working tree.
acceptance_checks:
  - true
agent:
  command:
    - bash
    - -lc
    - true
"#,
    );

    let output = fixture
        .command()
        .args(["run-loop", "--task", task_path.to_str().unwrap(), "--json"])
        .assert()
        .failure()
        .code(20)
        .get_output()
        .stdout
        .clone();

    let payload: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(payload["status"], "blocked");
    assert_eq!(payload["stop_reason"], "dirty_working_tree");
}

#[test]
#[serial]
fn run_loop_allows_initial_start_when_only_codexctl_files_are_dirty() {
    let fixture = CliFixture::new();
    fixture.init_git_repo();

    let task_path = fixture.write_task(
        "shapeup-bet-planning-only.yaml",
        r#"
type: codexctl-bet/v1
name: planning-only-bet
appetite: 1_week
objective: Allow unattended execution when only repo-local planning files are dirty
bounded_contexts:
  - Run Orchestration
success_signal: The run succeeds even though .codexctl files are still uncommitted
no_gos:
  - Do not ignore non-.codexctl dirty files.
acceptance_checks:
  - test -f planning-only.txt
agent:
  command:
    - bash
    - -lc
    - printf 'ok\n' > planning-only.txt
"#,
    );

    fixture
        .command()
        .args(["run-loop", "--task", task_path.to_str().unwrap(), "--json"])
        .assert()
        .success();

    assert!(fixture.repo_dir.join("planning-only.txt").exists());
}

#[test]
#[serial]
fn run_loop_records_detached_head_repo_state() {
    let fixture = CliFixture::new();
    fixture.init_git_repo();
    let task_path = fixture.write_task(
        "detached.yaml",
        r#"
type: codexctl-bet/v1
name: detached-bet
appetite: 1_week
objective: Record detached HEAD as an explicit repo-state warning
bounded_contexts:
  - Run Ledger
success_signal: The run succeeds and persists detached HEAD metadata
no_gos:
  - Do not mutate git refs automatically.
acceptance_checks:
  - true
agent:
  command:
    - bash
    - -lc
    - true
"#,
    );
    fixture.git(&["add", "."]);
    fixture.git(&["commit", "-m", "add detached bet"]);
    fixture.detach_head();

    fixture
        .command()
        .args(["run-loop", "--task", task_path.to_str().unwrap(), "--json"])
        .assert()
        .success();

    let latest_output = fixture
        .command()
        .args(["runs", "--latest", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let payload: Value = serde_json::from_slice(&latest_output).unwrap();
    assert_eq!(payload["run"]["repo_state"]["is_git_repo"], true);
    assert_eq!(payload["run"]["repo_state"]["is_detached_head"], true);
}

#[test]
#[serial]
fn run_loop_review_checks_can_pass() {
    let fixture = CliFixture::new();
    let task_path = fixture.write_task(
        "review-pass.yaml",
        r#"
type: codexctl-bet/v1
name: review-pass-bet
appetite: 1_week
objective: Pass both implementation and review gates
bounded_contexts:
  - Validation
success_signal: Review checks pass after acceptance passes
no_gos:
  - Do not skip the review phase.
acceptance_checks:
  - test -f review.txt
review_checks:
  - grep -q done review.txt
agent:
  command:
    - bash
    - -lc
    - printf 'done\n' > review.txt
"#,
    );

    let output = fixture
        .command()
        .args(["run-loop", "--task", task_path.to_str().unwrap(), "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let payload: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(payload["status"], "succeeded");
    assert_eq!(payload["implementation_status"], "passed");
    assert_eq!(payload["review_status"], "passed");
    assert!(
        fixture
            .last_run_report()
            .unwrap()
            .contains("Review gate: passed")
    );
}

#[test]
#[serial]
fn run_loop_review_failures_block_the_run() {
    let fixture = CliFixture::new();
    let task_path = fixture.write_task(
        "review-fail.yaml",
        r#"
type: codexctl-bet/v1
name: review-fail-bet
appetite: 1_week
objective: Block final success when review checks fail
bounded_contexts:
  - Validation
success_signal: Review failure is distinct from implementation success
no_gos:
  - Do not conflate acceptance and review phases.
acceptance_checks:
  - test -f review.txt
review_checks:
  - false
agent:
  command:
    - bash
    - -lc
    - printf 'done\n' > review.txt
"#,
    );

    let output = fixture
        .command()
        .args(["run-loop", "--task", task_path.to_str().unwrap(), "--json"])
        .assert()
        .failure()
        .code(20)
        .get_output()
        .stdout
        .clone();
    let payload: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(payload["status"], "blocked");
    assert_eq!(payload["implementation_status"], "passed");
    assert_eq!(payload["review_status"], "failed");
    assert_eq!(payload["stop_reason"], "review_checks_failed");

    let latest_output = fixture
        .command()
        .args(["runs", "--latest", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let latest_payload: Value = serde_json::from_slice(&latest_output).unwrap();
    assert_eq!(latest_payload["run"]["review_status"], "failed");
}

#[test]
#[serial]
fn run_loop_emits_notify_payload_without_masking_success() {
    let fixture = CliFixture::new();
    let task_path = fixture.write_task(
        "notify.yaml",
        r#"
type: codexctl-bet/v1
name: notify-bet
appetite: 1_week
objective: Emit a compact notification payload on terminal events
bounded_contexts:
  - Run Ledger
success_signal: The notify hook receives the final success payload
no_gos:
  - Do not build provider-specific integrations.
acceptance_checks:
  - true
agent:
  command:
    - bash
    - -lc
    - true
"#,
    );

    fixture
        .command()
        .args([
            "run-loop",
            "--task",
            task_path.to_str().unwrap(),
            "--notify-cmd",
            "cat > notify.json",
            "--json",
        ])
        .assert()
        .success();

    let notify_payload: Value =
        serde_json::from_slice(&fs::read(fixture.repo_dir.join("notify.json")).unwrap()).unwrap();
    assert_eq!(notify_payload["schema_version"], "notify/v1");
    assert_eq!(notify_payload["event"], "succeeded");
    assert_eq!(notify_payload["status"], "succeeded");
}

#[test]
#[serial]
fn run_loop_notify_failures_do_not_mask_success() {
    let fixture = CliFixture::new();
    let task_path = fixture.write_task(
        "notify-fail.yaml",
        r#"
type: codexctl-bet/v1
name: notify-fail-bet
appetite: 1_week
objective: Continue even when the notify hook fails
bounded_contexts:
  - Run Ledger
success_signal: The run succeeds and records notify failures in the event log
no_gos:
  - Do not let hook failures change the main run outcome.
acceptance_checks:
  - true
agent:
  command:
    - bash
    - -lc
    - true
"#,
    );

    fixture
        .command()
        .args([
            "run-loop",
            "--task",
            task_path.to_str().unwrap(),
            "--notify-cmd",
            "false",
            "--json",
        ])
        .assert()
        .success();

    let events = fixture.last_run_events().unwrap();
    assert!(events.contains("\"event_type\":\"notify_failed\""));
}

#[test]
#[serial]
fn run_loop_notify_timeout_does_not_mask_success() {
    let fixture = CliFixture::new();
    let task_path = fixture.write_task(
        "notify-timeout.yaml",
        r#"
type: codexctl-bet/v1
name: notify-timeout-bet
appetite: 1_week
objective: Continue even when the notify hook times out
bounded_contexts:
  - Run Ledger
success_signal: The run succeeds and records the notify timeout in the event log
no_gos:
  - Do not let stalled notification hooks block unattended completion.
acceptance_checks:
  - true
agent:
  command:
    - bash
    - -lc
    - true
"#,
    );

    fixture
        .command()
        .env("CODEXCTL_NOTIFY_TIMEOUT_SECONDS", "1")
        .args([
            "run-loop",
            "--task",
            task_path.to_str().unwrap(),
            "--notify-cmd",
            "sleep 2",
            "--json",
        ])
        .assert()
        .success();

    let events = fixture.last_run_events().unwrap();
    assert!(events.contains("\"event_type\":\"notify_failed\""));
    assert!(events.contains("timed out"));
}

#[test]
#[serial]
fn run_loop_times_out_slow_validation_within_runtime_budget() {
    let fixture = CliFixture::new();
    let task_path = fixture.write_task(
        "slow-validation.yaml",
        r#"
type: codexctl-bet/v1
name: slow-validation-bet
appetite: 1_week
objective: Time out a slow validation command inside the loop runtime budget
bounded_contexts:
  - Validation
success_signal: Slow validation is recorded as timed_out and the run exits on budget exhaustion
no_gos:
  - Do not let slow checks run indefinitely.
acceptance_checks:
  - sleep 61
agent:
  command:
    - bash
    - -lc
    - true
"#,
    );

    let output = fixture
        .command()
        .args([
            "run-loop",
            "--task",
            task_path.to_str().unwrap(),
            "--timeout-minutes",
            "1",
            "--max-consecutive-failures",
            "1",
            "--json",
        ])
        .assert()
        .failure()
        .code(21)
        .get_output()
        .stdout
        .clone();
    let payload: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(payload["status"], "budget_exhausted");
    assert_eq!(payload["implementation_status"], "timed_out");
    assert_eq!(payload["latest_validation"]["timed_out"], 1);
}

#[test]
#[serial]
fn run_loop_honors_preexisting_stop_file() {
    let fixture = CliFixture::new();
    let run_id = "20260414T010000Z-stop0001";
    let run_dir = fixture.config_dir.join("runs").join(run_id);
    fs::create_dir_all(run_dir.join("iterations")).unwrap();
    fs::create_dir_all(run_dir.join("logs")).unwrap();
    fs::write(
        run_dir.join("task.snapshot.yaml"),
        r#"
type: codexctl-bet/v1
name: stop-bet
appetite: 1_week
objective: Cancel immediately when a stop file already exists
bounded_contexts:
  - Run Orchestration
success_signal: The run transitions to cancelled before the first iteration
no_gos:
  - Do not ignore explicit operator stop requests.
acceptance_checks:
  - true
agent:
  command:
    - bash
    - -lc
    - true
"#,
    )
    .unwrap();
    fixture.write_run_record(
        run_id,
        json!({
            "schema_version": "runs/v1",
            "run_id": run_id,
            "status": "queued",
            "phase": "queued",
            "stop_reason": null,
            "task_name": "stop-bet",
            "task_path": fixture.repo_dir.join(".codexctl/tasks/stop.yaml").display().to_string(),
            "repo_root": fixture.repo_dir.display().to_string(),
            "profile": null,
            "auth_mode": null,
            "iteration_count": 0,
            "started_at": "2026-04-14T01:00:00Z",
            "updated_at": "2026-04-14T01:00:00Z",
            "finished_at": null,
            "implementation_status": "pending",
            "review_status": "pending",
            "repo_state": {
                "is_git_repo": false,
                "is_dirty": false,
                "is_detached_head": false
            },
            "latest_validation": {
                "status": null,
                "passed": 0,
                "failed": 0,
                "timed_out": 0,
                "errors": 0
            },
            "latest_review": {
                "status": null,
                "passed": 0,
                "failed": 0,
                "timed_out": 0,
                "errors": 0
            }
        }),
    );
    fs::write(run_dir.join(".stop"), "").unwrap();

    let output = fixture
        .command()
        .args(["run-loop", "--resume", run_id, "--json"])
        .assert()
        .failure()
        .code(22)
        .get_output()
        .stdout
        .clone();
    let payload: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(payload["status"], "cancelled");
    assert_eq!(payload["stop_reason"], "stop_requested_before_iteration");
}

struct CliFixture {
    temp: TempDir,
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
        fs::create_dir_all(config_dir.join("runs")).unwrap();
        fs::create_dir_all(&repo_dir).unwrap();

        Self {
            temp,
            home_dir,
            config_dir,
            repo_dir,
        }
    }

    fn command(&self) -> Command {
        self.command_in(&self.repo_dir)
    }

    fn command_in(&self, cwd: &PathBuf) -> Command {
        let mut cmd = Command::cargo_bin("codexctl").unwrap();
        cmd.current_dir(cwd)
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

    fn write_profile(&self, name: &str, auth_json: &[u8]) {
        let profile_dir = self.config_dir.join(name);
        fs::create_dir_all(&profile_dir).unwrap();
        fs::write(profile_dir.join("auth.json"), auth_json).unwrap();
    }

    fn write_live_auth(&self, auth_json: &[u8]) {
        fs::write(self.home_dir.join(".codex").join("auth.json"), auth_json).unwrap();
    }

    fn write_run_record(&self, run_id: &str, value: Value) {
        let run_dir = self.config_dir.join("runs").join(run_id);
        fs::create_dir_all(&run_dir).unwrap();
        fs::write(
            run_dir.join("run.json"),
            serde_json::to_vec_pretty(&value).unwrap(),
        )
        .unwrap();
    }

    fn latest_run_id(&self) -> Option<String> {
        let runs_dir = self.config_dir.join("runs");
        let mut entries: Vec<_> = fs::read_dir(runs_dir)
            .ok()?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_dir())
            .collect();
        entries.sort_by_key(|entry| entry.file_name());
        entries
            .last()
            .map(|entry| entry.file_name().to_string_lossy().to_string())
    }

    fn last_run_report(&self) -> Option<String> {
        let run_id = self.latest_run_id()?;
        fs::read_to_string(
            self.config_dir
                .join("runs")
                .join(run_id)
                .join("final-report.md"),
        )
        .ok()
    }

    fn last_run_events(&self) -> Option<String> {
        let run_id = self.latest_run_id()?;
        fs::read_to_string(
            self.config_dir
                .join("runs")
                .join(run_id)
                .join("events.jsonl"),
        )
        .ok()
    }

    fn init_git_repo(&self) {
        self.git(&["init"]);
        self.git(&["config", "user.email", "test@example.com"]);
        self.git(&["config", "user.name", "Test User"]);
        fs::write(self.repo_dir.join("README.md"), "repo").unwrap();
        self.git(&["add", "README.md"]);
        self.git(&["commit", "-m", "init"]);
    }

    fn detach_head(&self) {
        let output = StdCommand::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(&self.repo_dir)
            .output()
            .unwrap();
        assert!(output.status.success());
        let head = String::from_utf8(output.stdout).unwrap();
        self.git(&["checkout", head.trim()]);
    }

    fn git(&self, args: &[&str]) {
        let status = StdCommand::new("git")
            .args(args)
            .current_dir(&self.repo_dir)
            .status()
            .unwrap();
        assert!(status.success(), "git {:?} failed", args);
    }
}
