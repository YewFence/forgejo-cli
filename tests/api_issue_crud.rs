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
        "email": "a@example.com",
        "avatar_url": "",
        "html_url": "https://example.com/alice",
        "created": "2024-01-01T00:00:00Z",
        "last_login": "2024-01-01T00:00:00Z"
    })
}

fn mock_issue_obj() -> serde_json::Value {
    serde_json::json!({
        "id": 1,
        "number": 1,
        "title": "Bug report",
        "body": "Something broke",
        "state": "open",
        "html_url": "https://example.com/alice/repo/issues/1",
        "url": "https://example.com/api/v1/repos/alice/repo/issues/1",
        "user": mock_user_obj(),
        "labels": [],
        "assignees": [],
        "assignee": null,
        "milestone": null,
        "created_at": "2024-01-10T08:00:00Z",
        "updated_at": "2024-01-10T08:00:00Z",
        "closed_at": null,
        "due_date": null,
        "comments": 0,
        "pull_request": null,
        "repository": {"id": 1, "full_name": "alice/repo"},
        "assets": [],
        "ref": "",
        "original_author": "",
        "original_author_id": 0,
        "pin_order": 0,
        "is_locked": false
    })
}

fn mock_comment_obj(id: i64, body: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "body": body,
        "user": mock_user_obj(),
        "created_at": "2024-01-15T14:00:00Z",
        "updated_at": "2024-01-15T14:00:00Z",
        "html_url": "https://example.com/alice/repo/issues/1#issuecomment-10",
        "issue_url": "https://example.com/api/v1/repos/alice/repo/issues/1",
        "pull_request_url": "",
        "original_author": "",
        "original_author_id": 0,
        "assets": []
    })
}

// ===========================================================================
// 1. Issue create
// ===========================================================================

/// Issue create checks for templates (GET issue_templates + GET issue_config)
/// before calling POST /issues. We return 404 from issue_templates so the
/// CLI sees has_templates=false and skips the template requirement.
/// The forgejo-api expects 201 from POST /issues.
#[tokio::test]
async fn issue_create() {
    let instance = common::TestInstance::start().await;

    // repo_get_issue_templates returns 404 (no templates)
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/issue_templates"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "message": "not found",
            "url": ""
        })))
        .mount(&instance.server)
        .await;

    // repo_get_issue_config returns blank issues enabled
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/issue_config"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "blank_issues_enabled": true,
            "contact_links": []
        })))
        .mount(&instance.server)
        .await;

    // The actual issue creation (expects 201)
    Mock::given(method("POST"))
        .and(path("/api/v1/repos/alice/repo/issues"))
        .respond_with(ResponseTemplate::new(201).set_body_json(mock_issue_obj()))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "issue",
            "create",
            "--repo",
            "alice/repo",
            "Bug report",
            "--body",
            "Something broke",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Created issue #1"));
}

// ===========================================================================
// 2. Issue view
// ===========================================================================

#[tokio::test]
async fn issue_view_json() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/issues/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_issue_obj()))
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "issue", "view", "alice/repo#1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"title\": \"Bug report\""))
        .stdout(predicate::str::contains("\"number\": 1"));
}

// ===========================================================================
// 3. Issue close
// ===========================================================================

/// The forgejo-api expects 201 from PATCH /issues/{index}.
#[tokio::test]
async fn issue_close() {
    let instance = common::TestInstance::start().await;

    let mut closed_issue = mock_issue_obj();
    closed_issue["state"] = serde_json::json!("closed");
    closed_issue["closed_at"] = serde_json::json!("2024-01-15T13:00:00Z");

    Mock::given(method("PATCH"))
        .and(path("/api/v1/repos/alice/repo/issues/1"))
        .respond_with(ResponseTemplate::new(201).set_body_json(closed_issue))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["issue", "close", "alice/repo#1"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Closed issue #1"));
}

// ===========================================================================
// 4. Issue reopen
// ===========================================================================

/// The forgejo-api expects 201 from PATCH /issues/{index}.
#[tokio::test]
async fn issue_reopen() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("PATCH"))
        .and(path("/api/v1/repos/alice/repo/issues/1"))
        .respond_with(ResponseTemplate::new(201).set_body_json(mock_issue_obj()))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["issue", "reopen", "alice/repo#1"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Reopened issue #1"));
}

// ===========================================================================
// 5. Issue comment
// ===========================================================================

/// The forgejo-api expects 201 from POST /issues/{index}/comments.
#[tokio::test]
async fn issue_comment() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/repos/alice/repo/issues/1/comments"))
        .respond_with(
            ResponseTemplate::new(201).set_body_json(mock_comment_obj(10, "This is a comment")),
        )
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["issue", "comment", "alice/repo#1", "This is a comment"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Added comment on issue #1"));
}

// ===========================================================================
// 6. Issue edit title
// ===========================================================================

/// The forgejo-api expects 201 from PATCH /issues/{index}.
#[tokio::test]
async fn issue_edit_title() {
    let instance = common::TestInstance::start().await;

    let mut updated = mock_issue_obj();
    updated["title"] = serde_json::json!("New Title");

    Mock::given(method("PATCH"))
        .and(path("/api/v1/repos/alice/repo/issues/1"))
        .respond_with(ResponseTemplate::new(201).set_body_json(updated))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["issue", "edit", "alice/repo#1", "title", "New Title"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Updated title for issue #1"));
}

// ===========================================================================
// 7. Issue edit assignees
// ===========================================================================

/// Edit assignees first fetches the current issue to get existing assignees,
/// then PATCHes with the updated list. Both requests hit /issues/{index}.
/// The PATCH returns 201 per the forgejo-api convention.
#[tokio::test]
async fn issue_edit_assignees() {
    let instance = common::TestInstance::start().await;

    // GET issue to read current assignees
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/issues/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_issue_obj()))
        .mount(&instance.server)
        .await;

    // PATCH issue with updated assignees (expects 201)
    let mut updated = mock_issue_obj();
    updated["assignees"] = serde_json::json!([mock_user_obj()]);

    Mock::given(method("PATCH"))
        .and(path("/api/v1/repos/alice/repo/issues/1"))
        .respond_with(ResponseTemplate::new(201).set_body_json(updated))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "issue",
            "edit",
            "alice/repo#1",
            "assignees",
            "--add",
            "alice",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Updated assignees for issue #1"));
}

// ===========================================================================
// 8. Issue view comments
// ===========================================================================

#[tokio::test]
async fn issue_view_comments_json() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/issues/1/comments"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_comment_obj(10, "First comment")])),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "issue", "view", "alice/repo#1", "comments"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"body\": \"First comment\""));
}
