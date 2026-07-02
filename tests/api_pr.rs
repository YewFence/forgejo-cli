mod common;

use predicates::prelude::*;
use wiremock::matchers::{body_partial_json, method, path, path_regex, query_param};
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

/// Build a full Repository object for use in API response mocks.
///
/// The forgejo-api Repository struct has custom serde deserializers
/// (none_if_blank_url, deserialize_optional_ssh_url) that require URL
/// fields to be present even if empty/null. We also keep the object
/// minimal to avoid stack overflow when nested inside PullRequest
/// (head.repo / base.repo).
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

fn mock_pr_obj() -> serde_json::Value {
    serde_json::json!({
        "id": 1,
        "number": 1,
        "title": "Test PR",
        "body": "PR body",
        "state": "open",
        "draft": false,
        "html_url": "https://example.com/alice/repo/pulls/1",
        "diff_url": "https://example.com/alice/repo/pulls/1.diff",
        "patch_url": "https://example.com/alice/repo/pulls/1.patch",
        "url": "https://example.com/api/v1/repos/alice/repo/pulls/1",
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

/// PR search uses issue_list_issues with type=pulls, so the mock objects
/// are Issue-shaped (returned from the /issues endpoint).
fn mock_pr_as_issue_obj() -> serde_json::Value {
    serde_json::json!({
        "id": 1,
        "number": 1,
        "title": "Test PR",
        "body": "PR body",
        "state": "open",
        "html_url": "https://example.com/alice/repo/pulls/1",
        "url": "https://example.com/api/v1/repos/alice/repo/issues/1",
        "comments": 0,
        "created_at": "2024-01-15T10:00:00Z",
        "updated_at": "2024-01-15T12:00:00Z",
        "closed_at": null,
        "due_date": null,
        "user": mock_user_obj(),
        "labels": [],
        "assignees": [],
        "milestone": null,
        "assignee": null,
        "pull_request": {
            "merged": false,
            "merged_at": null,
            "draft": false,
            "html_url": "https://example.com/alice/repo/pulls/1"
        },
        "repository": null,
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

/// Issue object for close/reopen operations. The forgejo-api expects 201
/// from PATCH /issues/{index}.
fn mock_issue_obj_for_state(state: &str) -> serde_json::Value {
    serde_json::json!({
        "id": 1,
        "number": 1,
        "title": "Test PR",
        "body": "PR body",
        "state": state,
        "html_url": "https://example.com/alice/repo/issues/1",
        "url": "https://example.com/api/v1/repos/alice/repo/issues/1",
        "comments": 0,
        "created_at": "2024-01-15T10:00:00Z",
        "updated_at": "2024-01-15T12:00:00Z",
        "closed_at": null,
        "due_date": null,
        "user": mock_user_obj(),
        "labels": [],
        "assignees": [],
        "milestone": null,
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

// ===========================================================================
// 1. PR search (list)
// ===========================================================================

/// PR search uses the issues endpoint with type=pulls query param.
#[tokio::test]
async fn pr_search_json() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/issues"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_pr_as_issue_obj()]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "pr", "search", "--repo", "alice/repo"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"title\": \"Test PR\""));
}

// ===========================================================================
// 2. PR view
// ===========================================================================

#[tokio::test]
async fn pr_view_json() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/pulls/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_pr_obj()))
        .mount(&instance.server)
        .await;

    // --json must not fetch the repo for the archived warning.
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo"))
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "pr", "view", "alice/repo#1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"title\": \"Test PR\""))
        .stdout(predicate::str::contains("\"number\": 1"));
}

/// The archived-warning repo lookup is decorative; if it fails (scoped token,
/// transient error), the view must still print the PR.
#[tokio::test]
async fn pr_view_survives_failed_repo_lookup() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/pulls/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_pr_obj()))
        .mount(&instance.server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["pr", "view", "alice/repo#1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Test PR"));
}

/// --json pr review must emit machine-readable output, not the human renderer.
#[tokio::test]
async fn pr_review_list_json() {
    let instance = common::TestInstance::start().await;
    mount_pr_reviews(
        &instance,
        serde_json::json!([mock_review_obj(5, "bob", "APPROVED", false, 0)]),
    )
    .await;

    let assert = instance
        .fj()
        .args(["--json", "pr", "review", "alice/repo#1", "list"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("--json pr review output must parse as JSON");
    assert_eq!(parsed[0]["state"], "APPROVED");
}

/// A PR with only review-request entries has no submitted reviews; it must
/// not print the misleading stale/dismissed hint.
#[tokio::test]
async fn pr_review_list_request_review_only() {
    let instance = common::TestInstance::start().await;
    mount_pr_reviews(
        &instance,
        serde_json::json!([mock_review_obj(7, "dave", "REQUEST_REVIEW", false, 0)]),
    )
    .await;

    instance
        .fj()
        .args(["pr", "review", "alice/repo#1", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No reviews."))
        .stdout(predicate::str::contains("stale or dismissed").not());
}

/// Unknown label names on the --base/--head path must error instead of
/// silently widening the results.
#[tokio::test]
async fn pr_search_base_unknown_label_errors() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/labels"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([{
                    "id": 1,
                    "name": "bug",
                    "color": "ff0000",
                    "description": "",
                    "exclusive": false,
                    "is_archived": false,
                    "url": "https://example.com/api/v1/repos/alice/repo/labels/1"
                }]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    // The pulls endpoint must never be hit when label resolution fails.
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/pulls"))
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "pr", "search", "--repo", "alice/repo", "--base", "main", "--labels", "nosuch",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("label(s) not found: nosuch"));
}

#[tokio::test]
async fn pr_view_archived_warning() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/pulls/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_pr_obj()))
        .mount(&instance.server)
        .await;
    instance
        .mock_repo_archived("alice", "repo", "2024-01-15T12:00:00Z")
        .await;

    instance
        .fj()
        .args(["pr", "view", "alice/repo#1"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Repo archived since January 15, 2024",
        ))
        .stdout(predicate::str::contains("interactions are disabled"));
}

// ===========================================================================
// 2b. PR reviews
// ===========================================================================

fn mock_review_obj(
    id: i64,
    reviewer: &str,
    state: &str,
    stale: bool,
    comments_count: i64,
) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "user": {
            "id": 2,
            "login": reviewer,
            "full_name": reviewer,
            "email": format!("{reviewer}@example.com"),
            "avatar_url": "",
            "html_url": format!("https://example.com/{reviewer}"),
            "created": "2024-01-01T00:00:00Z",
            "last_login": "2024-01-01T00:00:00Z"
        },
        "team": null,
        "state": state,
        "body": "Review body",
        "commit_id": "abc123",
        "stale": stale,
        "official": true,
        "dismissed": false,
        "comments_count": comments_count,
        "submitted_at": "2024-01-15T10:00:00Z",
        "updated_at": "2024-01-15T10:00:00Z",
        "html_url": "https://example.com/alice/repo/pulls/1#issuecomment-1",
        "pull_request_url": "https://example.com/alice/repo/pulls/1"
    })
}

async fn mount_pr_reviews(instance: &common::TestInstance, reviews: serde_json::Value) {
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/pulls/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_pr_obj()))
        .mount(&instance.server)
        .await;

    let count = reviews.as_array().map(|a| a.len()).unwrap_or_default();
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/pulls/1/reviews"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(reviews)
                .insert_header("x-total-count", count.to_string().as_str()),
        )
        .expect(1)
        .mount(&instance.server)
        .await;
}

#[tokio::test]
async fn pr_review_list() {
    let instance = common::TestInstance::start().await;
    mount_pr_reviews(
        &instance,
        serde_json::json!([
            mock_review_obj(5, "bob", "APPROVED", false, 0),
            mock_review_obj(6, "carol", "REQUEST_CHANGES", true, 0),
        ]),
    )
    .await;

    // The stale review from carol is hidden by default.
    instance
        .fj()
        .args(["pr", "review", "alice/repo#1", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("APPROVED by bob"))
        .stdout(predicate::str::contains("carol").not());
}

#[tokio::test]
async fn pr_review_list_all_includes_stale() {
    let instance = common::TestInstance::start().await;
    mount_pr_reviews(
        &instance,
        serde_json::json!([mock_review_obj(6, "carol", "REQUEST_CHANGES", true, 0)]),
    )
    .await;

    instance
        .fj()
        .args(["pr", "review", "alice/repo#1", "list", "--all"])
        .assert()
        .success()
        .stdout(predicate::str::contains("CHANGES REQUESTED by carol"))
        .stdout(predicate::str::contains("(stale)"));
}

#[tokio::test]
async fn pr_review_list_only_stale_hint() {
    let instance = common::TestInstance::start().await;
    mount_pr_reviews(
        &instance,
        serde_json::json!([mock_review_obj(6, "carol", "REQUEST_CHANGES", true, 0)]),
    )
    .await;

    instance
        .fj()
        .args(["pr", "review", "alice/repo#1", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Only stale or dismissed reviews, use -a to display them.",
        ));
}

#[tokio::test]
async fn pr_review_list_empty() {
    let instance = common::TestInstance::start().await;
    mount_pr_reviews(&instance, serde_json::json!([])).await;

    instance
        .fj()
        .args(["pr", "review", "alice/repo#1", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No reviews."));
}

#[tokio::test]
async fn pr_review_list_with_comments() {
    let instance = common::TestInstance::start().await;
    mount_pr_reviews(
        &instance,
        serde_json::json!([mock_review_obj(5, "bob", "APPROVED", false, 1)]),
    )
    .await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/pulls/1/reviews/5/comments"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 9,
                "body": "Please rename this variable",
                "path": "src/main.rs",
                "position": 3,
                "original_position": 3,
                "diff_hunk": "@@ -1,3 +1,3 @@",
                "commit_id": "abc123",
                "original_commit_id": "abc123",
                "pull_request_review_id": 5,
                "user": {
                    "id": 2,
                    "login": "bob",
                    "full_name": "Bob",
                    "email": "bob@example.com",
                    "avatar_url": "",
                    "html_url": "https://example.com/bob",
                    "created": "2024-01-01T00:00:00Z",
                    "last_login": "2024-01-01T00:00:00Z"
                },
                "resolver": null,
                "created_at": "2024-01-15T10:00:00Z",
                "updated_at": "2024-01-15T10:00:00Z",
                "html_url": "https://example.com/alice/repo/pulls/1#discussion-9",
                "pull_request_url": "https://example.com/alice/repo/pulls/1"
            }
        ])))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["pr", "review", "alice/repo#1", "list", "--comments"])
        .assert()
        .success()
        .stdout(predicate::str::contains("In src/main.rs:3"))
        .stdout(predicate::str::contains("bob commented"))
        .stdout(predicate::str::contains("Please rename this variable"));
}

// ===========================================================================
// 3. PR create
// ===========================================================================

/// PR create with explicit --base, --head, title, and --body.
/// Requires mocking: repo_get, raw file lookups (template check),
/// compare diff, and the actual POST.
#[tokio::test]
async fn pr_create() {
    let instance = common::TestInstance::start().await;

    // repo_get
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_repo_obj("alice", "repo")))
        .mount(&instance.server)
        .await;

    // All PR template file lookups return 404
    Mock::given(method("GET"))
        .and(path_regex(r"/api/v1/repos/alice/repo/raw/.*"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "message": "not found",
            "url": ""
        })))
        .mount(&instance.server)
        .await;

    // Branch comparison
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/compare/main...feature"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "commits": [
                {
                    "sha": "abc123",
                    "url": "https://example.com/api/v1/repos/alice/repo/git/commits/abc123",
                    "html_url": "https://example.com/alice/repo/commit/abc123",
                    "commit": {
                        "message": "Add feature\n\nSome details",
                        "url": "https://example.com/api/v1/repos/alice/repo/git/commits/abc123",
                        "author": {
                            "name": "Alice",
                            "email": "a@example.com",
                            "date": "2024-01-15T10:00:00Z"
                        },
                        "committer": {
                            "name": "Alice",
                            "email": "a@example.com",
                            "date": "2024-01-15T10:00:00Z"
                        }
                    },
                    "created": "2024-01-15T10:00:00Z"
                }
            ],
            "total_commits": 1
        })))
        .mount(&instance.server)
        .await;

    // The actual PR creation (forgejo-api expects 201)
    Mock::given(method("POST"))
        .and(path("/api/v1/repos/alice/repo/pulls"))
        .respond_with(ResponseTemplate::new(201).set_body_json(mock_pr_obj()))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "pr",
            "create",
            "--repo",
            "alice/repo",
            "--base",
            "main",
            "--head",
            "feature",
            "Test PR",
            "--body",
            "PR body",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Created PR"));
}

/// PR create from an untracked branch with multiple remotes.
/// The branch has no tracking remote set, so `fj` must fall back to the
/// resolved remote (origin) for host validation and use the local branch
/// name as the head branch.
/// Regression test for issue #46.
#[tokio::test]
async fn pr_create_untracked_branch() {
    let instance = common::TestInstance::start().await;
    let server_url = instance.server.uri();

    // Create a repo with origin pointing at the mock server + a second remote
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let repo_path = tmp.path().to_path_buf();
    let repo = git2::Repository::init(&repo_path).expect("failed to init git repo");

    // Initial commit on main so HEAD exists
    {
        let sig = git2::Signature::now("Test", "test@example.com").unwrap();
        let tree_id = repo.index().unwrap().write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "initial commit", &tree, &[])
            .unwrap();
    }

    // Add two remotes -- origin at mock server, upstream elsewhere
    repo.remote("origin", &format!("{}/alice/repo.git", server_url))
        .unwrap();
    repo.remote(
        "upstream",
        "https://other-host.example.com/upstream/repo.git",
    )
    .unwrap();

    // Create a feature branch (no tracking remote)
    let head_commit = repo.head().unwrap().peel_to_commit().unwrap();
    repo.branch("feature-branch", &head_commit, false).unwrap();
    repo.set_head("refs/heads/feature-branch").unwrap();

    // Mock repo_get -- clone_url must match the mock server host
    let mut repo_obj = mock_repo_obj("alice", "repo");
    repo_obj["clone_url"] = serde_json::json!(format!("{}/alice/repo.git", server_url));
    repo_obj["ssh_url"] = serde_json::json!(format!("ssh://git@127.0.0.1/alice/repo.git"));

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&repo_obj))
        .mount(&instance.server)
        .await;

    // PR template lookups return 404
    Mock::given(method("GET"))
        .and(path_regex(r"/api/v1/repos/alice/repo/raw/.*"))
        .respond_with(
            ResponseTemplate::new(404)
                .set_body_json(serde_json::json!({"message": "not found", "url": ""})),
        )
        .mount(&instance.server)
        .await;

    // Branch comparison
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/compare/main...feature-branch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "commits": [{
                "sha": "abc123",
                "url": format!("{}/api/v1/repos/alice/repo/git/commits/abc123", server_url),
                "html_url": format!("{}/alice/repo/commit/abc123", server_url),
                "commit": {
                    "message": "Add feature",
                    "url": format!("{}/api/v1/repos/alice/repo/git/commits/abc123", server_url),
                    "author": {"name": "Alice", "email": "a@example.com", "date": "2024-01-15T10:00:00Z"},
                    "committer": {"name": "Alice", "email": "a@example.com", "date": "2024-01-15T10:00:00Z"}
                },
                "created": "2024-01-15T10:00:00Z"
            }],
            "total_commits": 1
        })))
        .mount(&instance.server)
        .await;

    // The actual PR creation -- verify the head branch is the local branch name
    Mock::given(method("POST"))
        .and(path("/api/v1/repos/alice/repo/pulls"))
        .and(body_partial_json(
            serde_json::json!({"head": "feature-branch"}),
        ))
        .respond_with(ResponseTemplate::new(201).set_body_json(mock_pr_obj()))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .current_dir(&repo_path)
        .args([
            "pr", "create", "--base", "main", "Test PR", "--body", "PR body",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Created PR"));
}

// ===========================================================================
// 3b. PR status
// ===========================================================================

/// Covers the forgejo-api 0.11 upgrade: the `skipped` commit status state
/// (adapted from upstream f8dbe99) and a relative `target_url`, which failed
/// to deserialize on forgejo-api 0.9 (target_url was url::Url; Forgejo
/// returns relative paths) and works on 0.11 (String).
#[tokio::test]
async fn pr_status_skipped_and_relative_target_url() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/pulls/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_pr_obj()))
        .mount(&instance.server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/pulls/1/commits"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([{
                    "sha": "abc123",
                    "url": "https://example.com/api/v1/repos/alice/repo/git/commits/abc123",
                    "html_url": "https://example.com/alice/repo/commit/abc123",
                    "commit": {
                        "message": "Add feature",
                        "url": "https://example.com/api/v1/repos/alice/repo/git/commits/abc123",
                        "author": {"name": "Alice", "email": "a@example.com", "date": "2024-01-15T10:00:00Z"},
                        "committer": {"name": "Alice", "email": "a@example.com", "date": "2024-01-15T10:00:00Z"}
                    },
                    "created": "2024-01-15T10:00:00Z"
                }]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/commits/abc123/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "state": "skipped",
            "sha": "abc123",
            "total_count": 1,
            "commit_url": "https://example.com/api/v1/repos/alice/repo/git/commits/abc123",
            "url": "https://example.com/api/v1/repos/alice/repo/commits/abc123/status",
            "statuses": [{
                "id": 1,
                "context": "ci/lint",
                "description": "skipped",
                "status": "skipped",
                "target_url": "/alice/repo/actions/runs/187/jobs/0",
                "url": "https://example.com/api/v1/repos/alice/repo/statuses/abc123",
                "created_at": "2024-01-15T10:00:00Z",
                "updated_at": "2024-01-15T10:00:00Z"
            }]
        })))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["pr", "status", "alice/repo#1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Skipped"))
        .stdout(predicate::str::contains("ci/lint"));
}

// ===========================================================================
// 4. PR close
// ===========================================================================

/// PR close delegates to issue_edit_issue with state=closed.
/// The forgejo-api expects 201 from PATCH /issues/{index}.
#[tokio::test]
async fn pr_close() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("PATCH"))
        .and(path("/api/v1/repos/alice/repo/issues/1"))
        .respond_with(ResponseTemplate::new(201).set_body_json(mock_issue_obj_for_state("closed")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["pr", "close", "alice/repo#1"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Closed issue #1"));
}

// ===========================================================================
// 5. PR reopen
// ===========================================================================

/// PR reopen delegates to issue_edit_issue with state=open.
/// The forgejo-api expects 201 from PATCH /issues/{index}.
#[tokio::test]
async fn pr_reopen() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("PATCH"))
        .and(path("/api/v1/repos/alice/repo/issues/1"))
        .respond_with(ResponseTemplate::new(201).set_body_json(mock_issue_obj_for_state("open")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["pr", "reopen", "alice/repo#1"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Reopened issue #1"));
}

// ===========================================================================
// 6. PR merge
// ===========================================================================

/// PR merge calls repo_get, repo_get_pull_request, then
/// repo_merge_pull_request (which expects 200).
#[tokio::test]
async fn pr_merge() {
    let instance = common::TestInstance::start().await;

    // repo_get (for default_merge_style)
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_repo_obj("alice", "repo")))
        .mount(&instance.server)
        .await;

    // repo_get_pull_request
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/pulls/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_pr_obj()))
        .mount(&instance.server)
        .await;

    // The actual merge (expects 200)
    Mock::given(method("POST"))
        .and(path("/api/v1/repos/alice/repo/pulls/1/merge"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["pr", "merge", "alice/repo#1", "--method", "merge"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Merged PR #1"));
}

// ===========================================================================
// 7. PR comment
// ===========================================================================

/// PR comment delegates to issue_create_comment via the issues API.
/// The forgejo-api expects 201 from POST /issues/{index}/comments.
#[tokio::test]
async fn pr_comment() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/repos/alice/repo/issues/1/comments"))
        .respond_with(ResponseTemplate::new(201).set_body_json(mock_comment_obj(10, "Looks good!")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["pr", "comment", "alice/repo#1", "Looks good!"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Added comment on issue #1"));
}

/// `pr search --base` switches to the pulls endpoint with server-side
/// base branch filtering; the issues endpoint must not be called.
#[tokio::test]
async fn pr_search_base_uses_pulls_endpoint() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/pulls"))
        .and(query_param("base", "main"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_pr_obj()]))
                .insert_header("x-total-count", "1"),
        )
        .expect(1)
        .mount(&instance.server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/issues"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .expect(0)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "--json", "pr", "search", "--repo", "alice/repo", "--base", "main",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"title\": \"Test PR\""));
}

/// `pr search --head` switches to the pulls endpoint with server-side
/// head branch filtering; the issues endpoint must not be called.
#[tokio::test]
async fn pr_search_head_uses_pulls_endpoint() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/pulls"))
        .and(query_param("head", "feature"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_pr_obj()]))
                .insert_header("x-total-count", "1"),
        )
        .expect(1)
        .mount(&instance.server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/issues"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .expect(0)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "--json", "pr", "search", "--repo", "alice/repo", "--head", "feature",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"title\": \"Test PR\""));
}

/// Verify that client-side filtering removes PRs not matching the query,
/// even if the server returns them (simulating broken server-side `q` param).
#[tokio::test]
async fn pr_search_filters_by_query_client_side() {
    let instance = common::TestInstance::start().await;

    let make_pr_issue = |id: u64, number: u64, title: &str, body: &str| {
        serde_json::json!({
            "id": id,
            "number": number,
            "title": title,
            "body": body,
            "state": "open",
            "html_url": format!("https://example.com/alice/repo/pulls/{number}"),
            "url": format!("https://example.com/api/v1/repos/alice/repo/issues/{number}"),
            "comments": 0,
            "created_at": "2024-01-15T10:00:00Z",
            "updated_at": "2024-01-15T12:00:00Z",
            "closed_at": null,
            "due_date": null,
            "user": mock_user_obj(),
            "labels": [],
            "assignees": [],
            "milestone": null,
            "assignee": null,
            "pull_request": {
                "merged": false,
                "merged_at": null,
                "draft": false,
                "html_url": format!("https://example.com/alice/repo/pulls/{number}")
            },
            "repository": null,
            "assets": [],
            "ref": "",
            "original_author": "",
            "original_author_id": 0,
            "pin_order": 0,
            "is_locked": false
        })
    };

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/issues"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([
                    make_pr_issue(1, 1, "Fix auth token refresh", "Token refresh was broken"),
                    make_pr_issue(2, 2, "Add dark mode", "Implements dark theme"),
                ]))
                .insert_header("x-total-count", "2"),
        )
        .mount(&instance.server)
        .await;

    let assert = instance
        .fj()
        .args(["--json", "pr", "search", "--repo", "alice/repo", "token"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(
        stdout.contains("Fix auth token refresh"),
        "matching PR should be in output:\n{stdout}"
    );
    assert!(
        !stdout.contains("Add dark mode"),
        "non-matching PR should be filtered out:\n{stdout}"
    );
}
