mod common;

use predicates::prelude::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn actions_tasks_list() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/actions/tasks"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "workflow_runs": [
                {
                    "id": 1,
                    "name": "CI",
                    "run_number": 42,
                    "status": "success",
                    "display_title": "Build",
                    "head_sha": "abc123def456",
                    "event": "push",
                    "run_started_at": "2024-07-01T12:00:00Z",
                    "updated_at": "2024-07-01T12:05:00Z",
                    "url": "",
                    "created_at": "2024-07-01T11:55:00Z"
                }
            ],
            "total_count": 1
        })))
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["actions", "--repo", "alice/repo", "tasks"])
        .assert()
        .success()
        .stdout(predicate::str::contains("CI"))
        .stdout(predicate::str::contains("42"))
        .stdout(predicate::str::contains("Build"))
        .stdout(predicate::str::contains("push"));
}

#[tokio::test]
async fn actions_variables_list() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/actions/variables"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([
                    {
                        "name": "CI_TOKEN",
                        "data": "value123",
                        "owner_id": 1,
                        "repo_id": 1
                    }
                ]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["actions", "--repo", "alice/repo", "variables", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("CI_TOKEN"))
        .stdout(predicate::str::contains("value123"));
}

#[tokio::test]
async fn actions_variables_create() {
    let instance = common::TestInstance::start().await;

    // create_repo_variable is POST to /api/v1/repos/:owner/:repo/actions/variables/:variablename
    // and returns 201 on success
    Mock::given(method("POST"))
        .and(path("/api/v1/repos/alice/repo/actions/variables/MY_VAR"))
        .respond_with(ResponseTemplate::new(201))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "actions",
            "--repo",
            "alice/repo",
            "variables",
            "create",
            "MY_VAR",
            "my_value",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Created variable MY_VAR"));
}

#[tokio::test]
async fn actions_secrets_list() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/actions/secrets"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([
                    {
                        "name": "SECRET_KEY",
                        "created_at": "2024-01-01T00:00:00Z"
                    }
                ]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["actions", "--repo", "alice/repo", "secrets", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("SECRET_KEY"));
}

#[tokio::test]
async fn actions_secrets_create() {
    let instance = common::TestInstance::start().await;

    // update_repo_secret is PUT to /api/v1/repos/:owner/:repo/actions/secrets/:secretname
    // and returns 201 or 204 on success
    Mock::given(method("PUT"))
        .and(path("/api/v1/repos/alice/repo/actions/secrets/MY_SECRET"))
        .respond_with(ResponseTemplate::new(201))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "actions",
            "--repo",
            "alice/repo",
            "secrets",
            "create",
            "MY_SECRET",
            "secret_value",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Created secret MY_SECRET"));
}

#[tokio::test]
async fn actions_dispatch() {
    let instance = common::TestInstance::start().await;

    // dispatch_workflow is POST to
    // /api/v1/repos/:owner/:repo/actions/workflows/:workflowfilename/dispatches
    // and returns 204 on success (no body)
    Mock::given(method("POST"))
        .and(path(
            "/api/v1/repos/alice/repo/actions/workflows/ci.yml/dispatches",
        ))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "actions",
            "--repo",
            "alice/repo",
            "dispatch",
            "ci.yml",
            "main",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Dispatched workflow ci.yml"));
}
