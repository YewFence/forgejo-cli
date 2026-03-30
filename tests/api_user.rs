mod common;

use predicates::prelude::*;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn alice_user_json() -> serde_json::Value {
    serde_json::json!({
        "id": 1,
        "login": "alice",
        "full_name": "Alice Smith",
        "email": "alice@example.com",
        "avatar_url": "",
        "html_url": "https://example.com/alice",
        "created": "2024-01-01T00:00:00Z",
        "last_login": "2024-01-01T00:00:00Z",
        "description": "A developer",
        "location": "Internet",
        "website": "https://alice.example.com",
        "pronouns": "she/her",
        "followers_count": 10,
        "following_count": 5,
        "starred_repos_count": 20,
        "visibility": "public",
        "is_admin": false,
        "restricted": false,
        "prohibit_login": false
    })
}

fn bob_user_json() -> serde_json::Value {
    serde_json::json!({
        "id": 2,
        "login": "bob",
        "full_name": "Bob Jones",
        "email": "bob@example.com",
        "avatar_url": "",
        "html_url": "https://example.com/bob",
        "created": "2024-02-01T00:00:00Z",
        "last_login": "2024-06-01T00:00:00Z",
        "description": "",
        "location": "",
        "website": "",
        "pronouns": "",
        "followers_count": 3,
        "following_count": 7,
        "starred_repos_count": 5,
        "visibility": "public",
        "is_admin": false,
        "restricted": false,
        "prohibit_login": false
    })
}

fn repo_json() -> serde_json::Value {
    serde_json::json!({
        "id": 1,
        "name": "my-project",
        "full_name": "alice/my-project",
        "description": "A project",
        "private": false,
        "fork": false,
        "html_url": "https://example.com/alice/my-project",
        "ssh_url": "",
        "clone_url": "",
        "original_url": "",
        "avatar_url": "",
        "languages_url": "",
        "url": "",
        "default_branch": "main",
        "stars_count": 5,
        "forks_count": 2,
        "watchers_count": 3,
        "open_issues_count": 1,
        "open_pr_counter": 0,
        "release_counter": 0,
        "archived": false,
        "archived_at": null,
        "mirror_updated": null,
        "has_issues": true,
        "has_pull_requests": true,
        "has_releases": true,
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-06-01T00:00:00Z"
    })
}

fn ssh_key_json() -> serde_json::Value {
    serde_json::json!({
        "id": 1,
        "key": "ssh-ed25519 AAAAC3...",
        "title": "laptop",
        "url": "https://example.com/api/v1/user/keys/1",
        "created_at": "2024-01-01T00:00:00Z",
        "read_only": false,
        "fingerprint": "SHA256:abc123...",
        "key_type": "ssh-ed25519"
    })
}

fn gpg_key_json() -> serde_json::Value {
    serde_json::json!({
        "id": 1,
        "primary_key_id": "",
        "key_id": "ABC123DEF456",
        "public_key": "-----BEGIN PGP PUBLIC KEY BLOCK-----...",
        "emails": [{"email": "alice@example.com", "verified": true}],
        "subkeys": [],
        "can_sign": true,
        "can_encrypt_comms": false,
        "can_encrypt_storage": false,
        "can_certify": true,
        "created_at": "2024-01-01T00:00:00Z",
        "expires_at": null,
        "verified": true
    })
}

fn user_settings_json() -> serde_json::Value {
    serde_json::json!({
        "description": "A developer",
        "diff_view_style": "unified",
        "enable_repo_unit_hints": true,
        "full_name": "Alice Smith",
        "hide_activity": false,
        "hide_email": false,
        "language": "en-US",
        "location": "Internet",
        "pronouns": "she/her",
        "theme": "gitea-auto",
        "website": "https://alice.example.com"
    })
}

// ---------------------------------------------------------------------------
// 1. user search
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_search() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/users/search"))
        .and(query_param("q", "alice"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [alice_user_json()],
            "ok": true
        })))
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "search", "alice"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alice"));
}

// ---------------------------------------------------------------------------
// 2. user view (another user, JSON mode)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_view_other_json() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/users/alice"))
        .respond_with(ResponseTemplate::new(200).set_body_json(alice_user_json()))
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "user", "view", "alice"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"login\": \"alice\""));
}

// ---------------------------------------------------------------------------
// 2b. user view (self, JSON mode)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_view_self_json() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/user"))
        .respond_with(ResponseTemplate::new(200).set_body_json(alice_user_json()))
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "user", "view"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"login\": \"alice\""));
}

// ---------------------------------------------------------------------------
// 3. user follow
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_follow() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("PUT"))
        .and(path("/api/v1/user/following/bob"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "follow", "bob"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Followed bob"));
}

// ---------------------------------------------------------------------------
// 4. user unfollow
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_unfollow() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("DELETE"))
        .and(path("/api/v1/user/following/bob"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "unfollow", "bob"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Unfollowed bob"));
}

// ---------------------------------------------------------------------------
// 5. user following (list who alice follows)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_following() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/users/alice/following"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([bob_user_json()]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "following", "alice"])
        .assert()
        .success()
        .stdout(predicate::str::contains("bob"));
}

// ---------------------------------------------------------------------------
// 6. user followers (list alice's followers)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_followers() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/users/alice/followers"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([bob_user_json()]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "followers", "alice"])
        .assert()
        .success()
        .stdout(predicate::str::contains("bob"));
}

// ---------------------------------------------------------------------------
// 7. user block
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_block() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("PUT"))
        .and(path("/api/v1/user/block/bob"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "block", "bob"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Blocked bob"));
}

// ---------------------------------------------------------------------------
// 8. user unblock
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_unblock() {
    let instance = common::TestInstance::start().await;

    // Note: the forgejo-api crate uses PUT for unblock, not DELETE.
    Mock::given(method("PUT"))
        .and(path("/api/v1/user/unblock/bob"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "unblock", "bob"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Unblocked bob"));
}

// ---------------------------------------------------------------------------
// 9. user repos (JSON mode)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_repos_json() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/users/alice/repos"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([repo_json()]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "repos", "alice"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alice/my-project"));
}

// ---------------------------------------------------------------------------
// 10. user repos --starred
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_repos_starred() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/users/alice/starred"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([repo_json()]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "repos", "alice", "--starred"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alice/my-project"));
}

// ---------------------------------------------------------------------------
// 11. user orgs
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_orgs() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/users/alice/orgs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 1,
                "name": "my-org",
                "full_name": "My Organization",
                "avatar_url": "",
                "description": "An org",
                "website": "",
                "location": "",
                "visibility": "public"
            }
        ])))
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "orgs", "alice"])
        .assert()
        .success()
        .stdout(predicate::str::contains("my-org"));
}

// ---------------------------------------------------------------------------
// 12. user activity
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_activity() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/users/alice/activities/feeds"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([
                    {
                        "id": 1,
                        "op_type": "create_repo",
                        "act_user_id": 1,
                        "act_user": alice_user_json(),
                        "repo": {
                            "id": 1,
                            "name": "my-project",
                            "full_name": "alice/my-project",
                            "owner": alice_user_json(),
                            "html_url": "https://example.com/alice/my-project",
                            "ssh_url": "",
                            "clone_url": "",
                            "original_url": "",
                            "avatar_url": "",
                            "languages_url": "",
                            "url": "",
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
                        },
                        "content": "",
                        "ref_name": "",
                        "is_private": false,
                        "created": "2024-06-01T00:00:00Z"
                    }
                ]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "activity", "alice"])
        .assert()
        .success()
        .stdout(predicate::str::contains("created repository"));
}

// ---------------------------------------------------------------------------
// 13. user edit bio (inline argument)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_edit_bio() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("PATCH"))
        .and(path("/api/v1/user/settings"))
        .respond_with(ResponseTemplate::new(200).set_body_json(user_settings_json()))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "edit", "bio", "new bio"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Updated bio"));
}

// ---------------------------------------------------------------------------
// 14. user edit name (inline argument)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_edit_name() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("PATCH"))
        .and(path("/api/v1/user/settings"))
        .respond_with(ResponseTemplate::new(200).set_body_json(user_settings_json()))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "edit", "name", "New Name"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Updated name"));
}

// ---------------------------------------------------------------------------
// 14b. user edit name --unset
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_edit_name_unset() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("PATCH"))
        .and(path("/api/v1/user/settings"))
        .respond_with(ResponseTemplate::new(200).set_body_json(user_settings_json()))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "edit", "name", "--unset"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Removed name"));
}

// ---------------------------------------------------------------------------
// 14c. user edit pronouns (inline argument)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_edit_pronouns() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("PATCH"))
        .and(path("/api/v1/user/settings"))
        .respond_with(ResponseTemplate::new(200).set_body_json(user_settings_json()))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "edit", "pronouns", "they/them"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Updated pronouns"));
}

// ---------------------------------------------------------------------------
// 14d. user edit location (inline argument)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_edit_location() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("PATCH"))
        .and(path("/api/v1/user/settings"))
        .respond_with(ResponseTemplate::new(200).set_body_json(user_settings_json()))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "edit", "location", "Mars"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Updated location"));
}

// ---------------------------------------------------------------------------
// 14e. user edit activity --visibility hidden
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_edit_activity_hidden() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("PATCH"))
        .and(path("/api/v1/user/settings"))
        .respond_with(ResponseTemplate::new(200).set_body_json(user_settings_json()))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "edit", "activity", "--visibility", "hidden"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Activity is now hidden"));
}

// ---------------------------------------------------------------------------
// 14f. user edit website (inline argument)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_edit_website() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("PATCH"))
        .and(path("/api/v1/user/settings"))
        .respond_with(ResponseTemplate::new(200).set_body_json(user_settings_json()))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "edit", "website", "https://example.com"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Updated website"));
}

// ---------------------------------------------------------------------------
// 15. user key list
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_key_list() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/user/keys"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([ssh_key_json()]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "key", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("laptop"));
}

// ---------------------------------------------------------------------------
// 16. user key view
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_key_view() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/user/keys/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ssh_key_json()))
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "user", "key", "view", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"title\": \"laptop\""));
}

// ---------------------------------------------------------------------------
// 17. user key delete --force
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_key_delete_force() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("DELETE"))
        .and(path("/api/v1/user/keys/1"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "key", "delete", "1", "--force"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Deleted key with ID 1"));
}

// ---------------------------------------------------------------------------
// 17b. user key delete --dry-run
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_key_delete_dry_run() {
    let instance = common::TestInstance::start().await;

    instance
        .fj()
        .args(["user", "key", "delete", "1", "--dry-run"])
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run]"));
}

// ---------------------------------------------------------------------------
// 18. user gpg list
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_gpg_list() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/user/gpg_keys"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([gpg_key_json()]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "gpg", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ABC123DEF456"));
}

// ---------------------------------------------------------------------------
// 18b. user gpg view
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_gpg_view() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/user/gpg_keys/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(gpg_key_json()))
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["--json", "user", "gpg", "view", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"key_id\": \"ABC123DEF456\""));
}

// ---------------------------------------------------------------------------
// 19. user gpg delete --force
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_gpg_delete_force() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("DELETE"))
        .and(path("/api/v1/user/gpg_keys/1"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["user", "gpg", "delete", "1", "--force"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Deleted GPG key with ID 1"));
}

// ---------------------------------------------------------------------------
// 19b. user gpg delete --dry-run
// ---------------------------------------------------------------------------

#[tokio::test]
async fn user_gpg_delete_dry_run() {
    let instance = common::TestInstance::start().await;

    instance
        .fj()
        .args(["user", "gpg", "delete", "1", "--dry-run"])
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run]"));
}
