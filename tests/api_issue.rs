mod common;

use predicates::prelude::*;

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
