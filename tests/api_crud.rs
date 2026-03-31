mod common;

use predicates::prelude::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn mock_user_obj() -> serde_json::Value {
    serde_json::json!({
        "id": 1,
        "login": "alice",
        "full_name": "Alice",
        "email": "alice@example.com",
        "avatar_url": "https://example.com/avatars/alice",
        "html_url": "https://example.com/alice",
        "created": "2024-01-01T00:00:00Z",
        "last_login": "2024-01-01T00:00:00Z"
    })
}

fn mock_milestone_obj(id: i64, title: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "title": title,
        "description": "A test milestone",
        "state": "open",
        "open_issues": 3,
        "closed_issues": 1,
        "created_at": "2024-06-01T00:00:00Z",
        "updated_at": "2024-06-15T00:00:00Z",
        "closed_at": null,
        "due_on": null
    })
}

fn mock_release_obj(id: i64, name: &str, tag: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "tag_name": tag,
        "name": name,
        "body": "Release notes for this version",
        "draft": false,
        "prerelease": false,
        "created_at": "2024-07-01T12:00:00Z",
        "published_at": "2024-07-01T12:00:00Z",
        "html_url": "https://example.com/alice/repo/releases/tag/v1.0.0",
        "tarball_url": "https://example.com/alice/repo/archive/v1.0.0.tar.gz",
        "zipball_url": "https://example.com/alice/repo/archive/v1.0.0.zip",
        "url": "https://example.com/api/v1/repos/alice/repo/releases/1",
        "upload_url": "https://example.com/api/v1/repos/alice/repo/releases/1/assets",
        "assets": [],
        "author": mock_user_obj()
    })
}

fn mock_label_obj(id: i64, name: &str, color: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "name": name,
        "color": color,
        "description": "A test label",
        "exclusive": false,
        "is_archived": false,
        "url": "https://example.com/api/v1/repos/alice/repo/labels/1"
    })
}

// ===========================================================================
// 1. Milestone CRUD
// ===========================================================================

#[tokio::test]
async fn milestone_create() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/repos/alice/repo/milestones"))
        .respond_with(ResponseTemplate::new(201).set_body_json(mock_milestone_obj(1, "Sprint 1")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["milestone", "--repo", "alice/repo", "create", "Sprint 1"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Created milestone"));
}

#[tokio::test]
async fn milestone_list_json() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/milestones"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([
                    mock_milestone_obj(1, "Sprint 1"),
                    mock_milestone_obj(2, "Sprint 2")
                ]))
                .insert_header("x-total-count", "2"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "milestone", "--repo", "alice/repo", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Sprint 1"))
        .stdout(predicate::str::contains("Sprint 2"));
}

/// Delete resolves the milestone name to an ID via GET milestones (with
/// name filter + stream), then issues DELETE /milestones/{id}.
#[tokio::test]
async fn milestone_delete_force() {
    let instance = common::TestInstance::start().await;

    // find_milestone calls GET /milestones with ?name=Sprint+1&state=all
    // and streams through pages looking for an exact title match.
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/milestones"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_milestone_obj(7, "Sprint 1")]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/api/v1/repos/alice/repo/milestones/7"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "milestone",
            "--repo",
            "alice/repo",
            "delete",
            "Sprint 1",
            "--force",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Deleted milestone"));
}

// ===========================================================================
// 2. Release list
// ===========================================================================

#[tokio::test]
async fn release_list_json() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/releases"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([
                    mock_release_obj(1, "First Release", "v1.0.0"),
                    mock_release_obj(2, "Second Release", "v2.0.0")
                ]))
                .insert_header("x-total-count", "2"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "release", "--repo", "alice/repo", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("First Release"))
        .stdout(predicate::str::contains("Second Release"));
}

// ===========================================================================
// 3. Label CRUD
// ===========================================================================

#[tokio::test]
async fn label_create() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/repos/alice/repo/labels"))
        .respond_with(ResponseTemplate::new(201).set_body_json(mock_label_obj(1, "bug", "ee0701")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "repo",
            "labels",
            "--repo",
            "alice/repo",
            "create",
            "bug",
            "ee0701",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Created label"));
}

// ===========================================================================
// 4. Milestone view (by numeric ID)
// ===========================================================================

/// View with a numeric ID calls GET /milestones/{id} directly.
#[tokio::test]
async fn milestone_view_json() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/milestones/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_milestone_obj(1, "Sprint 1")))
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "milestone", "--repo", "alice/repo", "view", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"title\": \"Sprint 1\""));
}

// ===========================================================================
// 5. Milestone edit (resolve by name, then PATCH)
// ===========================================================================

/// Edit resolves the milestone name via GET milestones (stream), then
/// PATCHes /milestones/{id} with the new values.
#[tokio::test]
async fn milestone_edit() {
    let instance = common::TestInstance::start().await;

    // find_milestone name search
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/milestones"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_milestone_obj(5, "Sprint 1")]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    let mut updated = mock_milestone_obj(5, "Sprint 1a");
    updated["description"] = serde_json::json!("Updated description");

    Mock::given(method("PATCH"))
        .and(path("/api/v1/repos/alice/repo/milestones/5"))
        .respond_with(ResponseTemplate::new(200).set_body_json(updated))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "milestone",
            "--repo",
            "alice/repo",
            "edit",
            "Sprint 1",
            "--title",
            "Sprint 1a",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Updated milestone"));
}

// ===========================================================================
// Label CRUD
// ===========================================================================

/// The label list (view) command does not support --json, so we check the
/// human-readable output for the label name and ID.
#[tokio::test]
async fn label_list() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/labels"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([
                    mock_label_obj(1, "bug", "ee0701"),
                    mock_label_obj(2, "enhancement", "84b6eb")
                ]))
                .insert_header("x-total-count", "2"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["repo", "labels", "--repo", "alice/repo", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("bug"))
        .stdout(predicate::str::contains("enhancement"));
}
