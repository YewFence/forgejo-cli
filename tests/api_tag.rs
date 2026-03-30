mod common;

use predicates::prelude::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn tag_list_json() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/tags"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([
                    {
                        "name": "v1.0.0",
                        "id": "abc123",
                        "message": "Release 1.0",
                        "commit": {
                            "sha": "deadbeef",
                            "created": "2024-01-15T10:00:00Z",
                            "url": "https://example.com/commit/deadbeef"
                        },
                        "tarball_url": "",
                        "zipball_url": ""
                    }
                ]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "tag", "--repo", "alice/repo", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"name\": \"v1.0.0\""));
}

#[tokio::test]
async fn tag_delete_dry_run() {
    let instance = common::TestInstance::start().await;

    instance
        .fj()
        .args([
            "tag",
            "--repo",
            "alice/repo",
            "delete",
            "v1.0.0",
            "--dry-run",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run]"));
}

#[tokio::test]
async fn tag_delete_force() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("DELETE"))
        .and(path("/api/v1/repos/alice/repo/tags/v1.0.0"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["tag", "--repo", "alice/repo", "delete", "v1.0.0", "--force"])
        .assert()
        .success();
}

fn mock_tag_obj(name: &str) -> serde_json::Value {
    serde_json::json!({
        "name": name,
        "id": "abc123",
        "message": "Release 1.0",
        "commit": {
            "sha": "deadbeef",
            "created": "2024-01-15T10:00:00Z",
            "url": "https://example.com/commit/deadbeef"
        },
        "tarball_url": "",
        "zipball_url": ""
    })
}

#[tokio::test]
async fn tag_create() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/repos/alice/repo/tags"))
        .respond_with(ResponseTemplate::new(201).set_body_json(mock_tag_obj("v2.0.0")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["tag", "--repo", "alice/repo", "create", "v2.0.0"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Created tag v2.0.0"));
}

#[tokio::test]
async fn tag_view_json() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/tags/v1.0.0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_tag_obj("v1.0.0")))
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "tag", "--repo", "alice/repo", "view", "v1.0.0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"name\": \"v1.0.0\""));
}
