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

fn mock_repo_obj(owner: &str, name: &str) -> serde_json::Value {
    serde_json::json!({
        "id": 1,
        "owner": {
            "login": owner,
            "id": 1,
            "avatar_url": "",
            "html_url": format!("https://example.com/{owner}"),
            "full_name": owner,
            "email": format!("{owner}@example.com"),
            "created": "2024-01-01T00:00:00Z",
            "last_login": "2024-01-01T00:00:00Z"
        },
        "name": name,
        "full_name": format!("{owner}/{name}"),
        "description": "A test repo",
        "avatar_url": "",
        "html_url": format!("https://example.com/{owner}/{name}"),
        "ssh_url": format!("ssh://git@example.com/{owner}/{name}.git"),
        "clone_url": format!("https://example.com/{owner}/{name}.git"),
        "original_url": "",
        "languages_url": "",
        "url": format!("https://example.com/api/v1/repos/{owner}/{name}"),
        "default_branch": "main",
        "default_merge_style": "merge",
        "stars_count": 0,
        "forks_count": 0,
        "watchers_count": 0,
        "open_issues_count": 0,
        "open_pr_counter": 0,
        "release_counter": 0,
        "private": false,
        "fork": false,
        "archived": false,
        "archived_at": null,
        "mirror_updated": null,
        "has_issues": true,
        "has_pull_requests": true,
        "has_releases": true,
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z"
    })
}

fn mock_issue_obj() -> serde_json::Value {
    serde_json::json!({
        "id": 1,
        "number": 1,
        "title": "Bug",
        "body": "",
        "state": "open",
        "html_url": "https://example.com/alice/repo/issues/1",
        "url": "",
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

fn mock_pr_obj() -> serde_json::Value {
    serde_json::json!({
        "id": 1,
        "number": 1,
        "title": "Test PR",
        "body": "",
        "state": "open",
        "draft": false,
        "html_url": "https://example.com/alice/repo/pulls/1",
        "diff_url": "https://example.com/alice/repo/pulls/1.diff",
        "patch_url": "https://example.com/alice/repo/pulls/1.patch",
        "url": "",
        "mergeable": true,
        "merged": false,
        "head": {
            "label": "feature",
            "ref": "feature",
            "sha": "abc123",
            "repo": mock_repo_obj("alice", "repo")
        },
        "base": {
            "label": "main",
            "ref": "main",
            "sha": "def456",
            "repo": mock_repo_obj("alice", "repo")
        },
        "user": mock_user_obj(),
        "labels": [],
        "assignees": [],
        "milestone": null,
        "created_at": "2024-01-15T10:00:00Z",
        "updated_at": "2024-01-15T12:00:00Z",
        "closed_at": null,
        "merged_at": null,
        "due_date": null,
        "comments": 0,
        "additions": 10,
        "deletions": 5,
        "changed_files": 2,
        "requested_reviewers": []
    })
}

// ===========================================================================
// 1. issue browse
// ===========================================================================

/// `fj issue browse alice/repo#1` fetches the issue and opens its html_url.
/// With FJ_BROWSER="" the URL is printed to stderr instead of opening a browser.
#[tokio::test]
async fn issue_browse() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/issues/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_issue_obj()))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .env("FJ_BROWSER", "")
        .args(["issue", "browse", "alice/repo#1"])
        .assert()
        .success()
        .stderr(predicate::str::contains("URL:"))
        .stderr(predicate::str::contains(
            "https://example.com/alice/repo/issues/1",
        ));
}

// ===========================================================================
// 2. pr browse
// ===========================================================================

/// `fj pr browse alice/repo#1` fetches the PR and opens its html_url.
#[tokio::test]
async fn pr_browse() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/pulls/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_pr_obj()))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .env("FJ_BROWSER", "")
        .args(["pr", "browse", "alice/repo#1"])
        .assert()
        .success()
        .stderr(predicate::str::contains("URL:"))
        .stderr(predicate::str::contains(
            "https://example.com/alice/repo/pulls/1",
        ));
}

// ===========================================================================
// 3. repo browse
// ===========================================================================

/// `fj repo browse alice/repo` constructs the URL from host + owner/name.
/// No API call is needed; the URL is derived from the --host flag and the
/// repo argument.
#[tokio::test]
async fn repo_browse() {
    let instance = common::TestInstance::start().await;

    instance
        .fj()
        .env("FJ_BROWSER", "")
        .args(["repo", "browse", "alice/repo"])
        .assert()
        .success()
        .stderr(predicate::str::contains("URL:"))
        .stderr(predicate::str::contains("alice/repo"));
}

// ===========================================================================
// 4. release browse (no name -- opens releases page)
// ===========================================================================

/// `fj release --repo alice/repo browse` (without a release name) fetches
/// the repository info to get its html_url, then appends /releases.
#[tokio::test]
async fn release_browse_list() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_repo_obj("alice", "repo")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .env("FJ_BROWSER", "")
        .args(["release", "--repo", "alice/repo", "browse"])
        .assert()
        .success()
        .stderr(predicate::str::contains("URL:"))
        .stderr(predicate::str::contains("releases"));
}

// ===========================================================================
// 5. user browse
// ===========================================================================

/// `fj user browse alice` constructs the URL from host + username.
/// When a username is provided, no API call to /user is needed.
#[tokio::test]
async fn user_browse() {
    let instance = common::TestInstance::start().await;

    instance
        .fj()
        .env("FJ_BROWSER", "")
        .args(["user", "browse", "alice"])
        .assert()
        .success()
        .stderr(predicate::str::contains("URL:"))
        .stderr(predicate::str::contains("alice"));
}

// ===========================================================================
// 6. wiki browse
// ===========================================================================

/// `fj wiki --repo alice/repo browse Home` fetches the wiki page and opens
/// its html_url.
#[tokio::test]
async fn wiki_browse() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/wiki/page/Home"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "title": "Home",
            "sub_url": "Home",
            "content_base64": "SGVsbG8gd2lraQ==",
            "html_url": "https://example.com/alice/repo/wiki/Home",
            "commit_count": 1,
            "footer": "",
            "sidebar": "",
            "last_commit": null
        })))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .env("FJ_BROWSER", "")
        .args(["wiki", "--repo", "alice/repo", "browse", "Home"])
        .assert()
        .success()
        .stderr(predicate::str::contains("URL:"));
}
