mod common;

use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

/// Helper: full repo JSON with all fields required by custom serde deserializers.
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

/// Helper: tag JSON matching the shape expected by the Tag deserializer.
fn tag_json(name: &str, sha: &str) -> serde_json::Value {
    serde_json::json!({
        "name": name,
        "id": sha,
        "message": format!("Release {name}"),
        "commit": {
            "sha": sha,
            "created": "2024-01-15T10:00:00Z",
            "url": format!("https://example.com/commit/{sha}")
        },
        "tarball_url": "",
        "zipball_url": ""
    })
}

// ---------------------------------------------------------------------------
// Test 1: --json produces valid JSON
// ---------------------------------------------------------------------------
#[tokio::test]
async fn json_flag_produces_valid_json() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/my-repo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(repo_json("alice", "my-repo")))
        .mount(&instance.server)
        .await;

    let output = instance
        .fj()
        .args(["--json", "repo", "view", "alice/my-repo"])
        .output()
        .expect("failed to run fj");

    assert!(output.status.success(), "command failed: {:?}", output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
    assert!(parsed.is_ok(), "stdout is not valid JSON: {stdout}");
}

// ---------------------------------------------------------------------------
// Test 2: --verbose output goes to stderr, stdout stays clean JSON
// ---------------------------------------------------------------------------
#[tokio::test]
async fn verbose_output_goes_to_stderr() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/my-repo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(repo_json("alice", "my-repo")))
        .mount(&instance.server)
        .await;

    let output = instance
        .fj()
        .args(["--verbose", "--json", "repo", "view", "alice/my-repo"])
        .output()
        .expect("failed to run fj");

    assert!(output.status.success(), "command failed: {:?}", output);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("[verbose]"),
        "expected stderr to contain '[verbose]', got: {stderr}"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("[verbose]"),
        "verbose output leaked into stdout: {stdout}"
    );

    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
    assert!(
        parsed.is_ok(),
        "stdout is not valid JSON when --verbose is set: {stdout}"
    );
}

// ---------------------------------------------------------------------------
// Test 3: --force skips prompts on destructive commands
// ---------------------------------------------------------------------------
#[tokio::test]
async fn force_flag_skips_prompt_on_delete() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("DELETE"))
        .and(path("/api/v1/repos/alice/my-repo"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["repo", "delete", "alice/my-repo", "--force"])
        .assert()
        .success();

    // wiremock will panic on drop if the mock was not called exactly once,
    // so reaching this point means the DELETE was issued without prompting.
}

// ---------------------------------------------------------------------------
// Test 4: --json on list commands produces a JSON array
// ---------------------------------------------------------------------------
#[tokio::test]
async fn json_list_produces_array() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/tags"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([
                    tag_json("v1.0.0", "aaa111"),
                    tag_json("v2.0.0", "bbb222")
                ]))
                .insert_header("x-total-count", "2"),
        )
        .mount(&instance.server)
        .await;

    let output = instance
        .fj()
        .args(["--json", "tag", "--repo", "alice/repo", "list"])
        .output()
        .expect("failed to run fj");

    assert!(output.status.success(), "command failed: {:?}", output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|_| panic!("stdout is not valid JSON: {stdout}"));

    assert!(parsed.is_array(), "expected JSON array, got: {parsed}");
    assert_eq!(
        parsed.as_array().unwrap().len(),
        2,
        "expected 2 items in array, got: {parsed}"
    );
}
