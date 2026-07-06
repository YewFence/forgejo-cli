mod common;

use predicates::prelude::*;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

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

    // Add a key for the mock server host.
    instance
        .fj()
        .args(["auth", "add-key", "mytoken123"])
        .assert()
        .success();

    // After adding a key, auth list should show the login's host.
    instance
        .fj()
        .args(["auth", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("127.0.0.1"));

    // Trying to add a key again should report it already exists.
    instance
        .fj()
        .args(["auth", "add-key", "othertoken"])
        .assert()
        .success()
        .stderr(predicate::str::contains("already exists"));
}

#[tokio::test]
async fn auth_add_key_then_logout() {
    let instance = common::TestInstance::start().await;

    // Add a key first.
    instance
        .fj()
        .args(["auth", "add-key", "mytoken123"])
        .assert()
        .success();

    // Verify the login exists.
    instance
        .fj()
        .args(["auth", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("127.0.0.1"));

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
        .stderr(predicate::str::contains("Signed out of "));

    // After logout, list should be empty again.
    instance
        .fj()
        .args(["auth", "list"])
        .assert()
        .success()
        .stderr(predicate::str::contains("No logins"));
}

#[tokio::test]
async fn forgejo_token_env_authenticates_api_requests() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/my-repo"))
        .and(header("Authorization", "token env-token"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(common::TestInstance::repo_json("alice", "my-repo")),
        )
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .env("FORGEJO_TOKEN", "env-token")
        .args(["repo", "view", "alice/my-repo"])
        .assert()
        .success();
}

#[tokio::test]
async fn forgejo_host_env_selects_api_host() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/my-repo"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(common::TestInstance::repo_json("alice", "my-repo")),
        )
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj_with_host_env()
        .args(["repo", "view", "alice/my-repo"])
        .assert()
        .success();
}

#[tokio::test]
async fn config_flag_overrides_data_dir() {
    let instance = common::TestInstance::start().await;
    let config_dir = tempfile::tempdir().expect("failed to create config dir");

    instance
        .fj()
        .args([
            "--config",
            config_dir
                .path()
                .to_str()
                .expect("config path is not UTF-8"),
            "auth",
            "add-key",
            "mytoken123",
        ])
        .assert()
        .success();

    instance
        .fj()
        .args([
            "--config",
            config_dir
                .path()
                .to_str()
                .expect("config path is not UTF-8"),
            "auth",
            "list",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("127.0.0.1"));
}
