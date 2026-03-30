mod common;

use predicates::prelude::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Create a bare git repo with an initial commit and return the tempdir + file:// URL.
fn create_bare_repo_with_commit() -> (tempfile::TempDir, String) {
    let dir = tempfile::tempdir().unwrap();
    let repo = git2::Repository::init_bare(dir.path()).unwrap();

    // Create an initial commit so the repo is cloneable
    let sig = git2::Signature::now("Test", "test@example.com").unwrap();
    let tree_id = repo.treebuilder(None).unwrap().write().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    repo.commit(
        Some("refs/heads/main"),
        &sig,
        &sig,
        "Initial commit",
        &tree,
        &[],
    )
    .unwrap();

    // Convert path to URL with forward slashes for git2 compatibility on Windows
    let path_str = dir.path().to_str().unwrap().replace('\\', "/");
    let url = format!("file:///{path_str}");
    (dir, url)
}

/// Build a mock repo JSON response with a custom clone_url.
fn mock_repo_json_with_clone_url(owner: &str, name: &str, clone_url: &str) -> serde_json::Value {
    serde_json::json!({
        "id": 1,
        "name": name,
        "full_name": format!("{owner}/{name}"),
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
        "description": "",
        "avatar_url": "",
        "html_url": format!("https://example.com/{owner}/{name}"),
        "ssh_url": format!("ssh://git@example.com/{owner}/{name}.git"),
        "clone_url": clone_url,
        "original_url": "",
        "languages_url": "",
        "url": format!("https://example.com/api/v1/repos/{owner}/{name}"),
        "default_branch": "main",
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

// ===========================================================================
// 1. repo clone into an explicit path
// ===========================================================================

#[tokio::test]
async fn repo_clone_to_temp_dir() {
    let instance = common::TestInstance::start().await;
    let (_bare_dir, bare_url) = create_bare_repo_with_commit();

    // Mock the repo API to return our file:// clone_url
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(mock_repo_json_with_clone_url("alice", "repo", &bare_url)),
        )
        .mount(&instance.server)
        .await;

    let clone_dest = tempfile::tempdir().unwrap();
    let dest_path = clone_dest.path().join("cloned-repo");

    instance
        .fj()
        .args(["repo", "clone", "alice/repo", dest_path.to_str().unwrap()])
        .assert()
        .success()
        .stderr(predicate::str::contains("Cloned alice/repo"));

    // Verify the clone produced a valid git repository
    assert!(
        dest_path.join(".git").exists(),
        ".git directory should exist in cloned repo"
    );
}

// ===========================================================================
// 2. repo clone defaults to ./<repo_name> when no path given
// ===========================================================================

#[tokio::test]
async fn repo_clone_default_path() {
    let instance = common::TestInstance::start().await;
    let (_bare_dir, bare_url) = create_bare_repo_with_commit();

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/myproject"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(mock_repo_json_with_clone_url(
                "alice",
                "myproject",
                &bare_url,
            )),
        )
        .mount(&instance.server)
        .await;

    // Run from an isolated temp directory so the default ./myproject lands there
    let work_dir = tempfile::tempdir().unwrap();

    instance
        .fj()
        .current_dir(work_dir.path())
        .args(["repo", "clone", "alice/myproject"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Cloned alice/myproject"));

    let expected = work_dir.path().join("myproject");
    assert!(
        expected.join(".git").exists(),
        ".git directory should exist at default clone path"
    );
}
