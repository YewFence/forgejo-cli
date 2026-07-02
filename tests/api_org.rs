mod common;

use predicates::prelude::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn mock_org_obj(id: i64, name: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "name": name,
        "username": name,
        "full_name": format!("{name} Foundation"),
        "description": "An example organization",
        "avatar_url": "",
        "email": format!("{name}@example.com"),
        "location": "Internet",
        "website": "https://example.com",
        "visibility": "public",
        "repo_admin_change_team_access": false,
        // forgejo-api 0.11 requires this key to be present (no serde default)
        "created": "2024-01-01T00:00:00Z"
    })
}

fn mock_team_obj(id: i64, name: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "name": name,
        "description": "A test team",
        "permission": "write",
        "units": ["repo.code", "repo.issues", "repo.pulls"],
        "units_map": {
            "repo.code": "write",
            "repo.issues": "write",
            "repo.pulls": "write"
        },
        "includes_all_repositories": false,
        "can_create_org_repo": false,
        "organization": mock_org_obj(1, "my-org")
    })
}

fn mock_user_obj(id: i64, login: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "login": login,
        "full_name": login,
        "email": format!("{login}@example.com"),
        "avatar_url": "",
        "html_url": format!("https://example.com/{login}"),
        "created": "2024-01-01T00:00:00Z",
        "last_login": "2024-01-01T00:00:00Z"
    })
}

fn mock_repo_obj(owner: &str, name: &str) -> serde_json::Value {
    serde_json::json!({
        "id": 1,
        "owner": mock_user_obj(1, owner),
        "name": name,
        "full_name": format!("{owner}/{name}"),
        "description": "",
        "avatar_url": "",
        "html_url": format!("https://example.com/{owner}/{name}"),
        "ssh_url": format!("ssh://git@example.com/{owner}/{name}.git"),
        "clone_url": format!("https://example.com/{owner}/{name}.git"),
        "original_url": "",
        "languages_url": "",
        "url": format!("https://example.com/api/v1/repos/{owner}/{name}"),
        "default_branch": "main",
        "archived_at": null,
        "mirror_updated": null,
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z"
    })
}

fn mock_label_obj(id: i64, name: &str, color: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "name": name,
        "color": color,
        "description": "A test label",
        "exclusive": false,
        "is_archived": false,
        "url": ""
    })
}

// ===========================================================================
// 1. org list --json (via --only-member-of -> GET /api/v1/user/orgs)
// ===========================================================================

#[tokio::test]
async fn org_list_json() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/user/orgs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            mock_org_obj(1, "alpha-org"),
            mock_org_obj(2, "beta-org")
        ])))
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "org", "list", "--only-member-of"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alpha-org"))
        .stdout(predicate::str::contains("beta-org"));
}

// ===========================================================================
// 2. org view --json
// ===========================================================================

#[tokio::test]
async fn org_view_json() {
    let instance = common::TestInstance::start().await;

    // Main org endpoint
    Mock::given(method("GET"))
        .and(path("/api/v1/orgs/my-org"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_org_obj(1, "my-org")))
        .mount(&instance.server)
        .await;

    // view_org fetches member count via org_list_members (page 1, page_size 1)
    Mock::given(method("GET"))
        .and(path("/api/v1/orgs/my-org/members"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([]))
                .insert_header("x-total-count", "5"),
        )
        .mount(&instance.server)
        .await;

    // view_org fetches team count via org_list_teams (page 1, page_size 1)
    Mock::given(method("GET"))
        .and(path("/api/v1/orgs/my-org/teams"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([]))
                .insert_header("x-total-count", "3"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "org", "view", "my-org"])
        .assert()
        .success()
        .stdout(predicate::str::contains("my-org"));
}

// ===========================================================================
// 3. org team list --json
// ===========================================================================

#[tokio::test]
async fn org_team_list_json() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/orgs/my-org/teams"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([
                    mock_team_obj(10, "dev-team"),
                    mock_team_obj(11, "ops-team")
                ]))
                .insert_header("x-total-count", "2"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "org", "team", "list", "my-org"])
        .assert()
        .success()
        .stdout(predicate::str::contains("dev-team"))
        .stdout(predicate::str::contains("ops-team"));
}

// ===========================================================================
// 4. org team delete --force
// ===========================================================================

#[tokio::test]
async fn org_team_delete_force() {
    let instance = common::TestInstance::start().await;

    // find_team_by_name streams GET /api/v1/orgs/my-org/teams to find the
    // team by name, then uses its ID for the DELETE call.
    Mock::given(method("GET"))
        .and(path("/api/v1/orgs/my-org/teams"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_team_obj(42, "dev-team")]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    // DELETE /api/v1/teams/{id} -- note: the endpoint uses /teams/{id}, not
    // /orgs/{org}/teams/{id}.
    Mock::given(method("DELETE"))
        .and(path("/api/v1/teams/42"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["org", "team", "delete", "my-org", "dev-team", "--force"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Deleted team"));
}

// ===========================================================================
// 5. org label list
// ===========================================================================

#[tokio::test]
async fn org_label_list() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/orgs/my-org/labels"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([
                    mock_label_obj(1, "bug", "ee0701"),
                    mock_label_obj(2, "feature", "0075ca")
                ]))
                .insert_header("x-total-count", "2"),
        )
        .mount(&instance.server)
        .await;

    // render_label_list does not support --json; it prints label names to
    // stdout in plain text when the terminal is not fancy.
    instance
        .fj()
        .args(["org", "label", "list", "my-org"])
        .assert()
        .success()
        .stdout(predicate::str::contains("bug"))
        .stdout(predicate::str::contains("feature"));
}

// ===========================================================================
// 6. org create
// ===========================================================================

#[tokio::test]
async fn org_create() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/orgs"))
        .respond_with(ResponseTemplate::new(201).set_body_json(mock_org_obj(1, "my-org")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["org", "create", "my-org"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Created"));
}

// ===========================================================================
// 7. org edit
// ===========================================================================

#[tokio::test]
async fn org_edit() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("PATCH"))
        .and(path("/api/v1/orgs/my-org"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_org_obj(1, "my-org")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["org", "edit", "my-org", "--description", "Updated"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Updated org my-org"));
}

// ===========================================================================
// 8. org activity
// ===========================================================================

#[tokio::test]
async fn org_activity() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/orgs/my-org/activities/feeds"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 1,
                "op_type": "create_repo",
                "act_user": mock_user_obj(1, "alice"),
                "repo": mock_repo_obj("my-org", "new-repo"),
                "content": "",
                "ref_name": "",
                "created": "2024-01-15T10:00:00Z"
            }
        ])))
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["org", "activity", "my-org"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alice"));
}

// ===========================================================================
// 9. org members
// ===========================================================================

#[tokio::test]
async fn org_members() {
    let instance = common::TestInstance::start().await;

    // list_org_members calls user_get_current() first
    instance.mock_current_user("alice").await;

    // org_is_member check: GET /api/v1/orgs/my-org/members/alice -> 204 means member
    Mock::given(method("GET"))
        .and(path("/api/v1/orgs/my-org/members/alice"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&instance.server)
        .await;

    // The actual member list
    Mock::given(method("GET"))
        .and(path("/api/v1/orgs/my-org/members"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([
                    mock_user_obj(1, "alice"),
                    mock_user_obj(2, "bob")
                ]))
                .insert_header("x-total-count", "2"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["org", "members", "my-org"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alice"))
        .stdout(predicate::str::contains("bob"));
}

// ===========================================================================
// 10. org team view --json
// ===========================================================================

#[tokio::test]
async fn org_team_view_json() {
    let instance = common::TestInstance::start().await;

    // find_team_by_name streams GET /api/v1/orgs/my-org/teams
    Mock::given(method("GET"))
        .and(path("/api/v1/orgs/my-org/teams"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_team_obj(10, "dev-team")]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "org", "team", "view", "my-org", "dev-team"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"name\": \"dev-team\""));
}

// ===========================================================================
// 11. org team create
// ===========================================================================

#[tokio::test]
async fn org_team_create() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/orgs/my-org/teams"))
        .respond_with(ResponseTemplate::new(201).set_body_json(mock_team_obj(20, "new-team")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["org", "team", "create", "my-org", "new-team"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Created team"));
}

// ===========================================================================
// 12. org team edit
// ===========================================================================

#[tokio::test]
async fn org_team_edit() {
    let instance = common::TestInstance::start().await;

    // find_team_by_name streams GET /api/v1/orgs/my-org/teams
    Mock::given(method("GET"))
        .and(path("/api/v1/orgs/my-org/teams"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_team_obj(10, "dev-team")]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    // PATCH /api/v1/teams/{id}
    Mock::given(method("PATCH"))
        .and(path("/api/v1/teams/10"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_team_obj(10, "dev-team")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "org",
            "team",
            "edit",
            "my-org",
            "dev-team",
            "--description",
            "Updated",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Updated team"));
}

// ===========================================================================
// 13. org team repo list
// ===========================================================================

#[tokio::test]
async fn org_team_repo_list() {
    let instance = common::TestInstance::start().await;

    // find_team_by_name streams GET /api/v1/orgs/my-org/teams
    Mock::given(method("GET"))
        .and(path("/api/v1/orgs/my-org/teams"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_team_obj(10, "dev-team")]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    // GET /api/v1/teams/{id}/repos
    Mock::given(method("GET"))
        .and(path("/api/v1/teams/10/repos"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_repo_obj("my-org", "my-repo")]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["org", "team", "repo", "list", "my-org", "dev-team"])
        .assert()
        .success()
        .stdout(predicate::str::contains("my-org/my-repo"));
}

// ===========================================================================
// 14. org team member list
// ===========================================================================

#[tokio::test]
async fn org_team_member_list() {
    let instance = common::TestInstance::start().await;

    // find_team_by_name streams GET /api/v1/orgs/my-org/teams
    Mock::given(method("GET"))
        .and(path("/api/v1/orgs/my-org/teams"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_team_obj(10, "dev-team")]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    // GET /api/v1/teams/{id}/members
    Mock::given(method("GET"))
        .and(path("/api/v1/teams/10/members"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_user_obj(1, "alice")]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["org", "team", "member", "list", "my-org", "dev-team"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alice"));
}

// ===========================================================================
// 15. org label add (create)
// ===========================================================================

#[tokio::test]
async fn org_label_create() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/orgs/my-org/labels"))
        .respond_with(ResponseTemplate::new(201).set_body_json(mock_label_obj(5, "bug", "ee0701")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["org", "label", "add", "my-org", "bug", "ee0701"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Created label"));
}

// ===========================================================================
// 16. org label edit
// ===========================================================================

#[tokio::test]
async fn org_label_edit() {
    let instance = common::TestInstance::start().await;

    // find_label_by_name streams GET /api/v1/orgs/my-org/labels
    Mock::given(method("GET"))
        .and(path("/api/v1/orgs/my-org/labels"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_label_obj(5, "bug", "ee0701")]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    // PATCH /api/v1/orgs/my-org/labels/{id}
    Mock::given(method("PATCH"))
        .and(path("/api/v1/orgs/my-org/labels/5"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_label_obj(5, "Bug", "ff0000")))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "org",
            "label",
            "edit",
            "my-org",
            "bug",
            "--new-name",
            "Bug",
            "--color",
            "ff0000",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Changed label"));
}

// ===========================================================================
// 17. org label rm --force (delete)
// ===========================================================================

#[tokio::test]
async fn org_label_delete_force() {
    let instance = common::TestInstance::start().await;

    // find_label_by_name streams GET /api/v1/orgs/my-org/labels
    Mock::given(method("GET"))
        .and(path("/api/v1/orgs/my-org/labels"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_label_obj(5, "bug", "ee0701")]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    // DELETE /api/v1/orgs/my-org/labels/{id}
    Mock::given(method("DELETE"))
        .and(path("/api/v1/orgs/my-org/labels/5"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["org", "label", "rm", "my-org", "bug", "--force"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Removed label"));
}
