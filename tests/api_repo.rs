mod common;

use predicates::prelude::*;
use wiremock::matchers::{body_partial_json, method, path};
use wiremock::{Mock, ResponseTemplate};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn mock_repo_json(owner: &str, name: &str) -> serde_json::Value {
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

fn mock_label_json(id: i64, name: &str, color: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "name": name,
        "color": color,
        "description": format!("The {name} label"),
        "exclusive": false,
        "is_archived": false,
        "url": format!("https://example.com/api/v1/repos/alice/repo/labels/{id}")
    })
}

// ===========================================================================
// View
// ===========================================================================

#[tokio::test]
async fn repo_view_json() {
    let instance = common::TestInstance::start().await;
    instance.mock_repo("alice", "my-repo").await;

    instance
        .fj()
        .args(["--json", "repo", "view", "alice/my-repo"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"full_name\": \"alice/my-repo\""));
}

#[tokio::test]
async fn repo_delete_dry_run() {
    let instance = common::TestInstance::start().await;

    instance
        .fj()
        .args(["repo", "delete", "alice/my-repo", "--dry-run"])
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run]"));
}

#[tokio::test]
async fn repo_delete_with_force() {
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
}

// ===========================================================================
// Create
// ===========================================================================

#[tokio::test]
async fn repo_create() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/user/repos"))
        .respond_with(
            ResponseTemplate::new(201).set_body_json(mock_repo_json("alice", "my-new-repo")),
        )
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["repo", "create", "my-new-repo"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Created repo at"));
}

// ===========================================================================
// Fork
// ===========================================================================

#[tokio::test]
async fn repo_fork() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/repos/alice/upstream-repo/forks"))
        .respond_with(
            ResponseTemplate::new(202).set_body_json(mock_repo_json("bob", "upstream-repo")),
        )
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["repo", "fork", "alice/upstream-repo"])
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "Forked alice/upstream-repo into bob/upstream-repo",
        ));
}

// ===========================================================================
// Migrate
// ===========================================================================

#[tokio::test]
async fn repo_migrate() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/repos/migrate"))
        .respond_with(
            ResponseTemplate::new(201).set_body_json(mock_repo_json("alice", "my-mirror")),
        )
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "repo",
            "migrate",
            "https://github.com/example/repo",
            "my-mirror",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Done! View online at"));
}

#[tokio::test]
async fn repo_migrate_with_owner() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/repos/migrate"))
        .and(body_partial_json(serde_json::json!({
            "repo_owner": "someorg",
            "repo_name": "my-mirror"
        })))
        .respond_with(
            ResponseTemplate::new(201).set_body_json(mock_repo_json("someorg", "my-mirror")),
        )
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "repo",
            "migrate",
            "https://github.com/example/repo",
            "someorg/my-mirror",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Done! View online at"));
}

// ===========================================================================
// Edit / Units
// ===========================================================================

#[tokio::test]
async fn repo_edit_description() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("PATCH"))
        .and(path("/api/v1/repos/alice/my-repo"))
        .and(body_partial_json(serde_json::json!({
            "description": "new desc",
            "private": null,
            "name": null
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_repo_json("alice", "my-repo")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "repo",
            "edit",
            "--repo",
            "alice/my-repo",
            "--description",
            "new desc",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Edited repository alice/my-repo"));
}

#[tokio::test]
async fn repo_units_disable_issues() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("PATCH"))
        .and(path("/api/v1/repos/alice/my-repo"))
        .and(body_partial_json(serde_json::json!({
            "has_issues": false,
            "has_wiki": null
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_repo_json("alice", "my-repo")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "repo",
            "units",
            "--repo",
            "alice/my-repo",
            "issues",
            "--enable",
            "false",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "Updated issues unit for alice/my-repo",
        ));
}

// ===========================================================================
// Readme
// ===========================================================================

#[tokio::test]
async fn repo_readme() {
    let instance = common::TestInstance::start().await;

    // The readme command tries "README.md" first via
    // GET /api/v1/repos/{owner}/{repo}/raw/{filepath}
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/my-repo/raw/README.md"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string("# Hello World\n\nThis is a test readme."),
        )
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["repo", "readme", "alice/my-repo"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello World"));
}

// ===========================================================================
// Star / Unstar
// ===========================================================================

#[tokio::test]
async fn repo_star() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("PUT"))
        .and(path("/api/v1/user/starred/alice/my-repo"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["repo", "star", "alice/my-repo"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Starred alice/my-repo"));
}

#[tokio::test]
async fn repo_unstar() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("DELETE"))
        .and(path("/api/v1/user/starred/alice/my-repo"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["repo", "unstar", "alice/my-repo"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Removed star from alice/my-repo"));
}

// ===========================================================================
// Labels - Delete
// ===========================================================================

#[tokio::test]
async fn label_delete_force() {
    let instance = common::TestInstance::start().await;

    // find_user_label fetches all labels to resolve name -> ID
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/labels"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([
                    mock_label_json(1, "bug", "ee0701"),
                    mock_label_json(2, "feature", "84b6eb"),
                ]))
                .insert_header("x-total-count", "2"),
        )
        .mount(&instance.server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/api/v1/repos/alice/repo/labels/1"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "repo",
            "labels",
            "--repo",
            "alice/repo",
            "delete",
            "bug",
            "--force",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Deleted label bug"));
}

// ===========================================================================
// Labels - Edit
// ===========================================================================

#[tokio::test]
async fn label_edit() {
    let instance = common::TestInstance::start().await;

    // find_user_label fetches all labels to resolve name -> ID
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/labels"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([
                    mock_label_json(1, "bug", "ee0701"),
                    mock_label_json(2, "feature", "84b6eb"),
                ]))
                .insert_header("x-total-count", "2"),
        )
        .mount(&instance.server)
        .await;

    // PATCH returns the updated label
    Mock::given(method("PATCH"))
        .and(path("/api/v1/repos/alice/repo/labels/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_label_json(1, "Bug", "ff0000")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "repo",
            "labels",
            "--repo",
            "alice/repo",
            "edit",
            "bug",
            "--name",
            "Bug",
            "--color",
            "ff0000",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Edited label"));
}
