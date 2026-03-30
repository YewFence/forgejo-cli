mod common;

use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

fn make_issue(id: u64, number: u64, title: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "number": number,
        "title": title,
        "body": "",
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

/// Test that `.all()` pagination collects results across multiple pages.
///
/// The forgejo-api crate paginates by:
/// 1. Sending the first request (no `page` query param)
/// 2. Reading the `x-total-count` response header
/// 3. Incrementing the `page` query param for subsequent requests
/// 4. Stopping when total items seen >= x-total-count
///
/// Issue search uses `.all()`, so we mock two pages of issues and verify
/// all four titles appear in the JSON output.
#[tokio::test]
async fn issue_search_paginates_across_pages() {
    let instance = common::TestInstance::start().await;

    let page1 = serde_json::json!([
        make_issue(1, 1, "First issue"),
        make_issue(2, 2, "Second issue"),
    ]);
    let page2 = serde_json::json!([
        make_issue(3, 3, "Third issue"),
        make_issue(4, 4, "Fourth issue"),
    ]);

    // Page 1 (catch-all): first request has no `page` query param.
    // Registered first so it has lower priority in wiremock's LIFO order.
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/issues"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(page1)
                .insert_header("x-total-count", "4"),
        )
        .up_to_n_times(1)
        .expect(1)
        .mount(&instance.server)
        .await;

    // Page 2 (specific): matches when page=2 query param is present.
    // Registered second so wiremock checks it first (LIFO).
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/issues"))
        .and(query_param("page", "2"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(page2)
                .insert_header("x-total-count", "4"),
        )
        .expect(1)
        .mount(&instance.server)
        .await;

    let assert = instance
        .fj()
        .args(["--json", "issue", "search", "--repo", "alice/repo"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(
        stdout.contains("First issue"),
        "missing 'First issue' in output:\n{stdout}"
    );
    assert!(
        stdout.contains("Second issue"),
        "missing 'Second issue' in output:\n{stdout}"
    );
    assert!(
        stdout.contains("Third issue"),
        "missing 'Third issue' in output:\n{stdout}"
    );
    assert!(
        stdout.contains("Fourth issue"),
        "missing 'Fourth issue' in output:\n{stdout}"
    );
}
