mod common;

use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn repo_view_404_not_found() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/nonexistent"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "message": "not found",
            "url": ""
        })))
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "repo", "view", "alice/nonexistent"])
        .assert()
        .failure();
}

#[tokio::test]
async fn repo_delete_403_forbidden() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("DELETE"))
        .and(path("/api/v1/repos/alice/repo"))
        .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
            "message": "forbidden"
        })))
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["repo", "delete", "alice/repo", "--force"])
        .assert()
        .failure();
}

#[tokio::test]
async fn repo_view_500_server_error() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "repo", "view", "alice/repo"])
        .assert()
        .failure();
}

#[tokio::test]
async fn repo_view_invalid_json() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw("this is not json", "application/json"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "repo", "view", "alice/repo"])
        .assert()
        .failure();
}
