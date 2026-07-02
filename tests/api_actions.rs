mod common;

use predicates::prelude::*;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

/// An ActionRun fixture.
///
/// `created`/`started`/`stopped`/`updated` use `time::serde::rfc3339::option`
/// and `html_url` uses `none_if_blank_url`, so those keys must be present.
fn mock_run_obj(id: i64) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "title": "Build stuff",
        "workflow_id": "ci.yml",
        "status": "success",
        "prettyref": "main",
        "event": "push",
        "trigger_event": "push",
        "commit_sha": "abc123def456",
        "html_url": format!("https://example.com/alice/repo/actions/runs/{id}"),
        "created": "2024-07-01T11:55:00Z",
        "started": "2024-07-01T12:00:00Z",
        "stopped": "2024-07-01T12:05:00Z",
        "updated": "2024-07-01T12:05:00Z"
    })
}

/// An ActionArtifact fixture.
///
/// `created_at`/`expires_at`/`updated_at` use `time::serde::rfc3339::option`,
/// so those keys must be present.
fn mock_artifact_obj(id: i64, name: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "name": name,
        "size_in_bytes": 2048,
        "run_id": 42,
        "archive_download_url": format!("https://example.com/alice/repo/actions/artifacts/{id}/zip"),
        "expired": false,
        "created_at": "2024-07-01T12:00:00Z",
        "expires_at": "2024-10-01T12:00:00Z",
        "updated_at": "2024-07-01T12:00:00Z"
    })
}

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
async fn actions_tasks_status_filter() {
    let instance = common::TestInstance::start().await;

    // Multiple --status values are comma-joined into a single query param.
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/actions/tasks"))
        .and(query_param("status", "failure,cancelled"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "workflow_runs": [],
            "total_count": 0
        })))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "actions",
            "--repo",
            "alice/repo",
            "tasks",
            "--status",
            "failure",
            "--status",
            "cancelled",
        ])
        .assert()
        .success();
}

#[tokio::test]
async fn actions_run_list() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/actions/runs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "workflow_runs": [mock_run_obj(42)],
            "total_count": 1
        })))
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["actions", "--repo", "alice/repo", "run", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("42"))
        .stdout(predicate::str::contains("ci.yml"))
        .stdout(predicate::str::contains("main"));
}

#[tokio::test]
async fn actions_run_list_filters() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/actions/runs"))
        .and(query_param("ref", "refs/heads/main"))
        .and(query_param("workflow_id", "ci.yml"))
        .and(query_param("status", "success"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "workflow_runs": [],
            "total_count": 0
        })))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "actions",
            "--repo",
            "alice/repo",
            "run",
            "list",
            "--ref",
            "refs/heads/main",
            "--workflow-id",
            "ci.yml",
            "--status",
            "success",
        ])
        .assert()
        .success();
}

#[tokio::test]
async fn actions_run_view() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/actions/runs/42"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_run_obj(42)))
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["actions", "--repo", "alice/repo", "run", "view", "42"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Build stuff"))
        .stdout(predicate::str::contains("ci.yml"))
        .stdout(predicate::str::contains("push"));
}

#[tokio::test]
async fn actions_run_jobs() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/actions/runs/42/jobs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 7,
                "name": "build",
                "status": "success",
                "run_id": 42,
                "attempt": 1
            }
        ])))
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["actions", "--repo", "alice/repo", "run", "jobs", "42"])
        .assert()
        .success()
        .stdout(predicate::str::contains("7"))
        .stdout(predicate::str::contains("build"));
}

#[tokio::test]
async fn actions_run_cancel() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/repos/alice/repo/actions/runs/42/cancel"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["actions", "--repo", "alice/repo", "run", "cancel", "42"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Cancelled run 42"));
}

#[tokio::test]
async fn actions_run_delete_force() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("DELETE"))
        .and(path("/api/v1/repos/alice/repo/actions/runs/42"))
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
            "run",
            "delete",
            "42",
            "--force",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Deleted run 42"));
}

#[tokio::test]
async fn actions_run_job_logs() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/actions/jobs/7/logs"))
        .respond_with(ResponseTemplate::new(200).set_body_string("line one\nline two\n"))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "actions",
            "--repo",
            "alice/repo",
            "run",
            "logs",
            "42",
            "--job",
            "7",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("line one"))
        .stdout(predicate::str::contains("line two"));
}

#[tokio::test]
async fn actions_run_logs_archive() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/actions/runs/42/logs"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(b"FAKEZIPBYTES".as_slice()))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["actions", "--repo", "alice/repo", "run", "logs", "42"])
        .assert()
        .success()
        .stdout(predicate::str::contains("FAKEZIPBYTES"));
}

#[tokio::test]
async fn actions_artifact_list() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/actions/artifacts"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_artifact_obj(5, "build-output")])),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["actions", "--repo", "alice/repo", "artifact", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("build-output"))
        .stdout(predicate::str::contains("2.0 KiB"))
        .stdout(predicate::str::contains("2024-10-01"));
}

#[tokio::test]
async fn actions_artifact_list_by_run() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/actions/runs/42/artifacts"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_artifact_obj(5, "build-output")])),
        )
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "actions",
            "--repo",
            "alice/repo",
            "artifact",
            "list",
            "--run",
            "42",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("build-output"));
}

#[tokio::test]
async fn actions_artifact_download_by_name() {
    let instance = common::TestInstance::start().await;

    // Name resolution goes through the server-side name filter...
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/actions/artifacts"))
        .and(query_param("name", "build-output"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_artifact_obj(5, "build-output")])),
        )
        .expect(1)
        .mount(&instance.server)
        .await;

    // ...then downloads the resolved id.
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/actions/artifacts/5/zip"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(b"FAKEZIP".as_slice()))
        .expect(1)
        .mount(&instance.server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let output_path = dir.path().join("out.zip");

    instance
        .fj()
        .args([
            "actions",
            "--repo",
            "alice/repo",
            "artifact",
            "download",
            "build-output",
            "--output",
            output_path.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Downloaded build-output"));

    let contents = std::fs::read(&output_path).unwrap();
    assert_eq!(contents, b"FAKEZIP");
}

#[tokio::test]
async fn actions_artifact_delete_force() {
    let instance = common::TestInstance::start().await;

    // Numeric args resolve by id first.
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/actions/artifacts/5"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(mock_artifact_obj(5, "build-output")),
        )
        .mount(&instance.server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/api/v1/repos/alice/repo/actions/artifacts/5"))
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
            "artifact",
            "delete",
            "5",
            "--force",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Deleted artifact 5"));
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
