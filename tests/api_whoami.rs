mod common;

use predicates::prelude::*;

#[tokio::test]
async fn whoami_after_add_key() {
    let instance = common::TestInstance::start().await;

    // whoami now queries the API for the current user instead of reading a
    // stored username, so mock GET /api/v1/user.
    instance.mock_current_user("alice").await;

    // First add a key so we have a login stored.
    instance
        .fj()
        .args(["auth", "add-key", "mytoken123"])
        .assert()
        .success();

    // Now whoami should report the current user from the API.
    instance
        .fj()
        .args(["whoami"])
        .assert()
        .success()
        .stdout(predicate::str::contains("currently signed in to alice@"));
}

#[tokio::test]
async fn whoami_with_token_and_host_env() {
    let instance = common::TestInstance::start().await;

    instance.mock_current_user("alice").await;

    instance
        .fj_with_host_env()
        .env("FORGEJO_TOKEN", "mytoken123")
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
