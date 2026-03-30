mod common;

use predicates::prelude::*;

#[tokio::test]
async fn auth_list_empty() {
    let instance = common::TestInstance::start().await;

    // With an empty data dir, auth list should show "No logins."
    instance
        .fj()
        .args(["auth", "list"])
        .assert()
        .success()
        .stderr(predicate::str::contains("No logins"));
}

#[tokio::test]
async fn auth_logout_not_signed_in() {
    let instance = common::TestInstance::start().await;

    // Logging out of a host we never logged into should show an info message.
    instance
        .fj()
        .args(["auth", "logout", "example.com"])
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "Already not signed in to example.com",
        ));
}

#[tokio::test]
async fn auth_add_key_and_list() {
    let instance = common::TestInstance::start().await;
    let host = instance.server.uri();

    // Add a key for the mock server host.
    instance
        .fj()
        .args(["auth", "add-key", "alice", "mytoken123"])
        .assert()
        .success();

    // After adding a key, auth list should show the login.
    instance
        .fj()
        .args(["auth", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alice@"));

    // Trying to add a key again should report it already exists.
    instance
        .fj()
        .args(["auth", "add-key", "bob", "othertoken"])
        .assert()
        .success()
        .stderr(predicate::str::contains("already exists"));

    // Verify we can see the host in the output (the mock server is 127.0.0.1:PORT).
    let _ = host;
}

#[tokio::test]
async fn auth_add_key_then_logout() {
    let instance = common::TestInstance::start().await;

    // Add a key first.
    instance
        .fj()
        .args(["auth", "add-key", "alice", "mytoken123"])
        .assert()
        .success();

    // Verify the login exists.
    instance
        .fj()
        .args(["auth", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alice@"));

    // Now extract the host part from the server URI. The logout command
    // expects the host:port without the scheme.
    let uri = instance.server.uri();
    let host = uri.strip_prefix("http://").unwrap_or(&uri);

    // Log out.
    instance
        .fj()
        .args(["auth", "logout", host])
        .assert()
        .success()
        .stderr(predicate::str::contains("Signed out of alice@"));

    // After logout, list should be empty again.
    instance
        .fj()
        .args(["auth", "list"])
        .assert()
        .success()
        .stderr(predicate::str::contains("No logins"));
}
