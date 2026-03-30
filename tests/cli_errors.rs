use assert_cmd::Command;
use predicates::prelude::*;

fn fj() -> Command {
    Command::cargo_bin("fj").unwrap()
}

#[test]
fn unknown_subcommand_fails() {
    fj().arg("nonexistent")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn repo_delete_no_args_fails() {
    fj().args(["repo", "delete"]).assert().failure();
}

#[test]
fn issue_view_no_remote_gives_helpful_error() {
    // When run outside a git repo or without a remote, should explain how to set up
    fj().args(["issue", "view", "1"])
        .current_dir(tempfile::tempdir().unwrap().path())
        .env_remove("GIT_DIR")
        .assert()
        .failure()
        .stderr(predicate::str::contains("repo").or(predicate::str::contains("remote")));
}

#[test]
fn completion_generates_output() {
    fj().args(["completion", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("complete"));
}

#[test]
fn version_shows_version() {
    fj().arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("fj"));
}
