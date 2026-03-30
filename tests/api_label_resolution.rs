mod common;

use predicates::prelude::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn mock_label_obj(id: i64, name: &str, color: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "name": name,
        "color": color,
        "description": format!("The {name} label"),
        "url": format!("https://example.com/api/v1/repos/alice/repo/labels/{id}"),
        "exclusive": false,
        "is_archived": false
    })
}

// mock_issue_obj is kept for future issue-related label resolution tests.
#[allow(dead_code)]
fn mock_issue_obj(number: i64, title: &str) -> serde_json::Value {
    serde_json::json!({
        "id": number,
        "number": number,
        "title": title,
        "body": "test body",
        "state": "open",
        "html_url": format!("https://example.com/alice/repo/issues/{number}"),
        "url": format!("https://example.com/api/v1/repos/alice/repo/issues/{number}"),
        "comments": 0,
        "created_at": "2024-01-15T10:00:00Z",
        "updated_at": "2024-01-15T10:00:00Z",
        "closed_at": null,
        "due_date": null,
        "user": {
            "id": 1,
            "login": "alice",
            "full_name": "Alice",
            "email": "alice@example.com",
            "avatar_url": "https://example.com/avatars/alice",
            "html_url": "https://example.com/alice",
            "created": "2024-01-01T00:00:00Z",
            "last_login": "2024-01-01T00:00:00Z"
        },
        "labels": [],
        "assignees": [],
        "milestone": null,
        "assignee": null,
        "pull_request": null,
        "repository": null,
        "assets": [],
        "ref": "",
        "original_author": "",
        "original_author_id": 0,
        "pin_order": 0,
        "is_locked": false
    })
}

fn mock_tag_obj(name: &str) -> serde_json::Value {
    serde_json::json!({
        "name": name,
        "id": "abc123",
        "message": format!("Release {name}"),
        "commit": {
            "sha": "deadbeef",
            "created": "2024-01-15T10:00:00Z",
            "url": "https://example.com/commit/deadbeef"
        },
        "tarball_url": "",
        "zipball_url": ""
    })
}

// ===========================================================================
// 1. Label name-to-ID resolution via issue edit labels
// ===========================================================================
//
// The `fj issue edit labels --add bug --add feature` command resolves label
// names to IDs by fetching all repo labels and org labels, then posting the
// resolved IDs to the issue. This exercises the same label name-to-ID
// resolution path used throughout the codebase (edit_labels in main.rs).

#[tokio::test]
async fn label_name_resolution_via_issue_edit() {
    let instance = common::TestInstance::start().await;

    // Mock: GET /repos/alice/repo/labels returns two labels
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/labels"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([
                    mock_label_obj(1, "bug", "ee0701"),
                    mock_label_obj(2, "feature", "84b6eb"),
                ]))
                .insert_header("x-total-count", "2"),
        )
        .mount(&instance.server)
        .await;

    // Mock: GET /orgs/alice/labels returns empty (no org-level labels)
    Mock::given(method("GET"))
        .and(path("/api/v1/orgs/alice/labels"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([]))
                .insert_header("x-total-count", "0"),
        )
        .mount(&instance.server)
        .await;

    // Mock: POST /repos/alice/repo/issues/1/labels (add resolved label IDs)
    Mock::given(method("POST"))
        .and(path("/api/v1/repos/alice/repo/issues/1/labels"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            mock_label_obj(1, "bug", "ee0701"),
            mock_label_obj(2, "feature", "84b6eb"),
        ])))
        .expect(1)
        .mount(&instance.server)
        .await;

    // Run: fj issue edit alice/repo#1 labels --add bug --add feature
    instance
        .fj()
        .args([
            "issue",
            "edit",
            "alice/repo#1",
            "labels",
            "--add",
            "bug",
            "--add",
            "feature",
        ])
        .assert()
        .success();
}

// ===========================================================================
// 2. --repo flag resolution
// ===========================================================================
//
// The `--repo owner/name` flag bypasses git remote detection and directly
// specifies the target repository. Verify that it works by running a command
// against the mock server with --repo and checking the output.

#[tokio::test]
async fn repo_flag_resolution_with_tag_list() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/bob/project/tags"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_tag_obj("v2.0.0"),]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "tag", "--repo", "bob/project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"name\": \"v2.0.0\""));
}

// ===========================================================================
// 3. Milestone name resolution - not found
// ===========================================================================
//
// When milestone delete receives a name that doesn't match any milestone on
// the server, find_milestone should return an error. Verify that the command
// fails gracefully with a "not found" message.

#[tokio::test]
async fn milestone_delete_not_found() {
    let instance = common::TestInstance::start().await;

    // Mock: GET /repos/alice/repo/milestones returns an empty list.
    // find_milestone first tries numeric ID parse (fails for "nonexistent"),
    // then falls back to name search which returns no matches.
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/milestones"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([]))
                .insert_header("x-total-count", "0"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "milestone",
            "--repo",
            "alice/repo",
            "delete",
            "nonexistent",
            "--force",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}
