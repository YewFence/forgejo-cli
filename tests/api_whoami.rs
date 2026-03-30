mod common;

use predicates::prelude::*;

#[tokio::test]
async fn whoami_after_add_key() {
    let instance = common::TestInstance::start().await;

    // First add a key so we have a login stored.
    instance
        .fj()
        .args(["auth", "add-key", "alice", "mytoken123"])
        .assert()
        .success();

    // Now whoami should report the current user.
    instance
        .fj()
        .args(["whoami"])
        .assert()
        .success()
        .stdout(predicate::str::contains("currently signed in to alice@"));
}

#[tokio::test]
async fn whoami_not_logged_in() {
    let instance = common::TestInstance::start().await;

    // With no key stored, whoami should fail with "not logged in".
    instance
        .fj()
        .args(["whoami"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not logged in"));
}
