mod common;

use predicates::prelude::*;
use rstest::rstest;

#[rstest]
#[case(&["repo", "delete", "alice/repo", "--dry-run"])]
#[case(&["tag", "--repo", "alice/repo", "delete", "v1.0", "--dry-run"])]
#[case(&["release", "--repo", "alice/repo", "delete", "v1.0", "--dry-run"])]
#[case(&["release", "--repo", "alice/repo", "asset", "delete", "v1.0", "my-asset", "--dry-run"])]
#[case(&["milestone", "--repo", "alice/repo", "delete", "sprint-1", "--dry-run"])]
#[case(&["actions", "--repo", "alice/repo", "variables", "delete", "MY_VAR", "--dry-run"])]
#[case(&["actions", "--repo", "alice/repo", "secrets", "delete", "MY_SECRET", "--dry-run"])]
#[case(&["actions", "--repo", "alice/repo", "run", "delete", "42", "--dry-run"])]
#[case(&["actions", "--repo", "alice/repo", "artifact", "delete", "my-artifact", "--dry-run"])]
#[case(&["repo", "labels", "--repo", "alice/repo", "delete", "bug", "--dry-run"])]
#[case(&["org", "label", "rm", "my-org", "my-label", "--dry-run"])]
#[case(&["org", "team", "delete", "my-org", "my-team", "--dry-run"])]
#[case(&["org", "team", "repo", "rm", "my-org", "my-team", "my-repo", "--dry-run"])]
#[case(&["org", "team", "member", "rm", "my-org", "my-team", "alice", "--dry-run"])]
#[case(&["user", "key", "delete", "1", "--dry-run"])]
#[case(&["user", "gpg", "delete", "1", "--dry-run"])]
#[tokio::test]
async fn dry_run_prints_preview_and_exits_zero(#[case] args: &[&str]) {
    let instance = common::TestInstance::start().await;

    instance
        .fj()
        .args(args)
        .assert()
        .success()
        .stderr(predicate::str::contains("dry-run"));
}
