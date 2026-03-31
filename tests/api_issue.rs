mod common;

use predicates::prelude::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn issue_search_json() {
    let instance = common::TestInstance::start().await;

    instance
        .mock_issues_list(
            "alice",
            "repo",
            serde_json::json!([
                {
                    "id": 1,
                    "number": 42,
                    "title": "Test issue",
                    "body": "This is a test issue body",
                    "state": "open",
                    "html_url": "https://example.com/alice/repo/issues/42",
                    "url": "https://example.com/api/v1/repos/alice/repo/issues/42",
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
                }
            ]),
        )
        .await;

    instance
        .fj()
        .args(["--json", "issue", "search", "--repo", "alice/repo"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"title\": \"Test issue\""));
}

/// Verify that client-side filtering removes issues not matching the query,
/// even if the server returns them (simulating broken server-side `q` param).
#[tokio::test]
async fn issue_search_filters_by_query_client_side() {
    let instance = common::TestInstance::start().await;

    let issues = serde_json::json!([
        {
            "id": 1,
            "number": 1,
            "title": "Fix login bug",
            "body": "The login page crashes on submit",
            "state": "open",
            "html_url": "https://example.com/alice/repo/issues/1",
            "url": "https://example.com/api/v1/repos/alice/repo/issues/1",
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
        },
        {
            "id": 2,
            "number": 2,
            "title": "Unrelated feature request",
            "body": "Add dark mode support",
            "state": "open",
            "html_url": "https://example.com/alice/repo/issues/2",
            "url": "https://example.com/api/v1/repos/alice/repo/issues/2",
            "comments": 0,
            "created_at": "2024-01-16T10:00:00Z",
            "updated_at": "2024-01-16T10:00:00Z",
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
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/issues"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(issues)
                .insert_header("x-total-count", "2"),
        )
        .mount(&instance.server)
        .await;

    let assert = instance
        .fj()
        .args(["--json", "issue", "search", "--repo", "alice/repo", "login"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(
        stdout.contains("Fix login bug"),
        "matching issue should be in output:\n{stdout}"
    );
    assert!(
        !stdout.contains("Unrelated feature request"),
        "non-matching issue should be filtered out:\n{stdout}"
    );
}
