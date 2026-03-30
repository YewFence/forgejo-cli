use assert_cmd::Command;
use wiremock::matchers::{method, path};
use wiremock::MockServer;
use wiremock::{Mock, ResponseTemplate};

/// A test Forgejo instance backed by wiremock.
#[allow(dead_code)]
pub struct TestInstance {
    pub server: MockServer,
    /// Private temp dir so each test gets its own keys.json.
    _data_dir: tempfile::TempDir,
}

#[allow(dead_code)]
impl TestInstance {
    pub async fn start() -> Self {
        let server = MockServer::start().await;
        let data_dir = tempfile::tempdir().expect("failed to create temp data dir");
        Self {
            server,
            _data_dir: data_dir,
        }
    }

    /// Get a Command preconfigured to talk to this mock server.
    ///
    /// Uses `--host` to point at the wiremock server and `--yes` to skip
    /// confirmation prompts.  Sets `FJ_DATA_DIR` to an isolated temp
    /// directory so concurrent tests don't race on the same keys.json.
    pub fn fj(&self) -> Command {
        let mut cmd = Command::cargo_bin("fj").unwrap();
        cmd.args(["-H", &self.server.uri()]);
        // Skip all interactive prompts
        cmd.arg("--yes");
        // Isolate the data directory to avoid concurrent file access races.
        cmd.env("FJ_DATA_DIR", self._data_dir.path());
        cmd
    }

    /// Mount a mock for GET /api/v1/user (current user).
    pub async fn mock_current_user(&self, username: &str) {
        Mock::given(method("GET"))
            .and(path("/api/v1/user"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1,
                "login": username,
                "full_name": username,
                "email": format!("{username}@example.com"),
                "avatar_url": "",
                "html_url": format!("https://example.com/{username}"),
                "created": "2024-01-01T00:00:00Z",
                "last_login": "2024-01-01T00:00:00Z"
            })))
            .mount(&self.server)
            .await;
    }

    /// Mount a mock for GET /api/v1/repos/:owner/:repo
    ///
    /// The response includes all fields that have custom serde deserializers
    /// (`none_if_blank_url`, `deserialize_optional_ssh_url`) since those fields
    /// must be present in the JSON even if null.
    pub async fn mock_repo(&self, owner: &str, name: &str) {
        Mock::given(method("GET"))
            .and(path(format!("/api/v1/repos/{owner}/{name}")))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
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
            })))
            .mount(&self.server)
            .await;
    }

    /// Mount a mock for GET /api/v1/repos/:owner/:repo/issues
    pub async fn mock_issues_list(&self, owner: &str, repo: &str, issues: serde_json::Value) {
        Mock::given(method("GET"))
            .and(path(format!("/api/v1/repos/{owner}/{repo}/issues")))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(issues)
                    .insert_header("x-total-count", "1"),
            )
            .mount(&self.server)
            .await;
    }

    /// Mount a mock for DELETE that returns 204 No Content.
    pub async fn mock_delete(&self, api_path: &str) {
        Mock::given(method("DELETE"))
            .and(path(api_path))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }
}
