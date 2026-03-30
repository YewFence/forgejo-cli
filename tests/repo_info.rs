mod common;

use predicates::prelude::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

/// Full repo JSON with all fields required by custom serde deserializers.
fn repo_json(owner: &str, name: &str) -> serde_json::Value {
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

/// Create a temp dir with a git repo and a single remote.
///
/// Returns the `TempDir` (must be kept alive for the duration of the test)
/// and the path to the repo.
fn init_repo_with_remote(
    remote_name: &str,
    remote_url: &str,
) -> (tempfile::TempDir, std::path::PathBuf) {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let repo_path = tmp.path().to_path_buf();

    let repo = git2::Repository::init(&repo_path).expect("failed to init git repo");

    // Create an initial commit so HEAD exists (required for branch tracking).
    {
        let sig = git2::Signature::now("Test", "test@example.com").unwrap();
        let tree_id = repo.index().unwrap().write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "initial commit", &tree, &[])
            .unwrap();
    }

    repo.remote(remote_name, remote_url)
        .expect("failed to add remote");

    (tmp, repo_path)
}

// ---------------------------------------------------------------------------
// Test 1: HTTPS remote detection
//
// Creates a temp git repo whose "origin" points at the wiremock server.
// Runs `fj repo view` with no explicit repo arg.  fj should discover the
// remote, extract owner/name from the URL, and query the mock API.
// ---------------------------------------------------------------------------
#[tokio::test]
async fn https_remote_detected_from_origin() {
    let instance = common::TestInstance::start().await;
    let server_url = instance.server.uri(); // e.g. http://127.0.0.1:PORT

    let remote_url = format!("{}/alice/my-repo.git", server_url);
    let (_tmp, repo_path) = init_repo_with_remote("origin", &remote_url);

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/my-repo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(repo_json("alice", "my-repo")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .current_dir(&repo_path)
        .args(["repo", "view"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alice/my-repo"));
}

// ---------------------------------------------------------------------------
// Test 2: .git suffix is stripped from the remote URL
//
// The remote ends in ".git".  fj should strip it and resolve the repo name
// without the suffix.
// ---------------------------------------------------------------------------
#[tokio::test]
async fn git_suffix_stripped_from_remote() {
    let instance = common::TestInstance::start().await;
    let server_url = instance.server.uri();

    let remote_url = format!("{}/bob/some-project.git", server_url);
    let (_tmp, repo_path) = init_repo_with_remote("origin", &remote_url);

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/bob/some-project"))
        .respond_with(ResponseTemplate::new(200).set_body_json(repo_json("bob", "some-project")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .current_dir(&repo_path)
        .args(["repo", "view"])
        .assert()
        .success()
        .stdout(predicate::str::contains("bob/some-project"));
}

// ---------------------------------------------------------------------------
// Test 3: Remote URL without .git suffix
//
// Some remotes omit the .git suffix entirely.  fj should still parse them.
// ---------------------------------------------------------------------------
#[tokio::test]
async fn remote_without_git_suffix() {
    let instance = common::TestInstance::start().await;
    let server_url = instance.server.uri();

    let remote_url = format!("{}/carol/bare-url", server_url);
    let (_tmp, repo_path) = init_repo_with_remote("origin", &remote_url);

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/carol/bare-url"))
        .respond_with(ResponseTemplate::new(200).set_body_json(repo_json("carol", "bare-url")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .current_dir(&repo_path)
        .args(["repo", "view"])
        .assert()
        .success()
        .stdout(predicate::str::contains("carol/bare-url"));
}

// ---------------------------------------------------------------------------
// Test 4: Custom remote name via --remote / -R
//
// The repo has a remote named "upstream" (not "origin").  Using --remote
// upstream should make fj read that remote's URL.
// ---------------------------------------------------------------------------
#[tokio::test]
async fn custom_remote_name() {
    let instance = common::TestInstance::start().await;
    let server_url = instance.server.uri();

    let remote_url = format!("{}/dave/other-repo.git", server_url);
    let (_tmp, repo_path) = init_repo_with_remote("upstream", &remote_url);

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/dave/other-repo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(repo_json("dave", "other-repo")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .current_dir(&repo_path)
        .args(["repo", "view", "--remote", "upstream"])
        .assert()
        .success()
        .stdout(predicate::str::contains("dave/other-repo"));
}

// ---------------------------------------------------------------------------
// Test 5: Single remote auto-detection (non-origin name)
//
// When there is exactly one remote (with any name), fj should use it even
// without --remote.
// ---------------------------------------------------------------------------
#[tokio::test]
async fn single_remote_auto_detected() {
    let instance = common::TestInstance::start().await;
    let server_url = instance.server.uri();

    let remote_url = format!("{}/eve/solo-remote.git", server_url);
    // Remote is named "myfork", not "origin"
    let (_tmp, repo_path) = init_repo_with_remote("myfork", &remote_url);

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/eve/solo-remote"))
        .respond_with(ResponseTemplate::new(200).set_body_json(repo_json("eve", "solo-remote")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .current_dir(&repo_path)
        .args(["repo", "view"])
        .assert()
        .success()
        .stdout(predicate::str::contains("eve/solo-remote"));
}

// ---------------------------------------------------------------------------
// Test 6: Explicit repo arg overrides git remote
//
// Even if the git remote points at one repo, passing an explicit repo arg
// should take precedence.
// ---------------------------------------------------------------------------
#[tokio::test]
async fn explicit_repo_arg_overrides_remote() {
    let instance = common::TestInstance::start().await;
    let server_url = instance.server.uri();

    // Remote points at alice/my-repo
    let remote_url = format!("{}/alice/my-repo.git", server_url);
    let (_tmp, repo_path) = init_repo_with_remote("origin", &remote_url);

    // But we ask for frank/other-repo explicitly
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/frank/other-repo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(repo_json("frank", "other-repo")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .current_dir(&repo_path)
        .args(["repo", "view", "frank/other-repo"])
        .assert()
        .success()
        .stdout(predicate::str::contains("frank/other-repo"));
}

// ---------------------------------------------------------------------------
// Test 7: No git repo gives a helpful error
//
// Running fj in a directory with no git repo and no repo arg should fail
// with a helpful message.
// ---------------------------------------------------------------------------
#[tokio::test]
async fn no_git_repo_gives_error() {
    let instance = common::TestInstance::start().await;

    let tmp = tempfile::tempdir().expect("failed to create temp dir");

    instance
        .fj()
        .current_dir(tmp.path())
        .args(["repo", "view"])
        .env_remove("GIT_DIR")
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("repo")
                .or(predicate::str::contains("couldn't"))
                .or(predicate::str::contains("specify")),
        );
}

// ---------------------------------------------------------------------------
// Test 8: JSON output includes repo data detected from remote
//
// Verifies that the full pipeline (remote detection -> API call -> JSON
// output) works end-to-end.
// ---------------------------------------------------------------------------
#[tokio::test]
async fn json_output_from_detected_remote() {
    let instance = common::TestInstance::start().await;
    let server_url = instance.server.uri();

    let remote_url = format!("{}/grace/json-test.git", server_url);
    let (_tmp, repo_path) = init_repo_with_remote("origin", &remote_url);

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/grace/json-test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(repo_json("grace", "json-test")))
        .expect(1)
        .mount(&instance.server)
        .await;

    let output = instance
        .fj()
        .current_dir(&repo_path)
        .args(["--json", "repo", "view"])
        .output()
        .expect("failed to run fj");

    assert!(output.status.success(), "command failed: {:?}", output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|_| panic!("stdout is not valid JSON: {stdout}"));

    assert_eq!(parsed["full_name"], "grace/json-test");
    assert_eq!(parsed["name"], "json-test");
}

// ---------------------------------------------------------------------------
// Test 9: Verbose output shows resolution steps
//
// With --verbose, stderr should contain resolution info showing the
// discovered host and repo.
// ---------------------------------------------------------------------------
#[tokio::test]
async fn verbose_shows_repo_resolution() {
    let instance = common::TestInstance::start().await;
    let server_url = instance.server.uri();

    let remote_url = format!("{}/heidi/verbose-test.git", server_url);
    let (_tmp, repo_path) = init_repo_with_remote("origin", &remote_url);

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/heidi/verbose-test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(repo_json("heidi", "verbose-test")))
        .mount(&instance.server)
        .await;

    let output = instance
        .fj()
        .current_dir(&repo_path)
        .args(["--verbose", "repo", "view"])
        .output()
        .expect("failed to run fj");

    assert!(output.status.success(), "command failed: {:?}", output);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Resolved repo"),
        "expected verbose output to contain 'Resolved repo', got: {stderr}"
    );
    assert!(
        stderr.contains("heidi/verbose-test"),
        "expected verbose output to contain repo name, got: {stderr}"
    );
}
