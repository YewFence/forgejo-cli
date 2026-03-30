mod common;

use assert_cmd::Command;
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

fn mock_issue_obj() -> serde_json::Value {
    serde_json::json!({
        "id": 1,
        "number": 1,
        "title": "Editor Title",
        "body": "fake editor body",
        "state": "open",
        "html_url": "https://example.com/alice/repo/issues/1",
        "url": "https://example.com/api/v1/repos/alice/repo/issues/1",
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

/// Create a fake editor script that writes known content into the file it
/// receives as its first argument.
///
/// On Windows this is a .bat file; on Unix a shell script.
fn create_fake_editor(dir: &std::path::Path, content: &str) -> std::path::PathBuf {
    #[cfg(windows)]
    {
        let script = dir.join("fake_editor.bat");
        // %~1 strips surrounding quotes from the first argument
        std::fs::write(&script, format!("@echo off\r\necho {content}> %~1\r\n")).unwrap();
        script
    }
    #[cfg(not(windows))]
    {
        let script = dir.join("fake_editor.sh");
        std::fs::write(&script, format!("#!/bin/sh\necho '{content}' > \"$1\"\n")).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
        }
        script
    }
}

// ===========================================================================
// 1. issue create with editor
// ===========================================================================

/// When `issue create` is called with a title but no --body, the CLI opens
/// the editor to get the body text. We set EDITOR to a fake script that
/// writes known content, then verify the issue is created successfully.
///
/// We do NOT pass --yes because that would block the editor path (the editor
/// function bails when --yes is set and content is empty).
#[tokio::test]
async fn issue_create_with_editor() {
    let instance = common::TestInstance::start().await;
    let data_dir = tempfile::tempdir().expect("failed to create temp dir for editor test");

    let editor_path = create_fake_editor(data_dir.path(), "fake editor body");

    // repo_get_issue_templates returns 404 (no templates)
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/issue_templates"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "message": "not found",
            "url": ""
        })))
        .mount(&instance.server)
        .await;

    // repo_get_issue_config returns blank issues enabled
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/issue_config"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "blank_issues_enabled": true,
            "contact_links": []
        })))
        .mount(&instance.server)
        .await;

    // The actual issue creation (expects 201)
    Mock::given(method("POST"))
        .and(path("/api/v1/repos/alice/repo/issues"))
        .respond_with(ResponseTemplate::new(201).set_body_json(mock_issue_obj()))
        .expect(1)
        .mount(&instance.server)
        .await;

    // Build the command manually (no --yes, so the editor path is taken)
    let mut cmd = Command::cargo_bin("fj").unwrap();
    cmd.args(["-H", &instance.server.uri()])
        .env("FJ_DATA_DIR", data_dir.path())
        .env("EDITOR", &editor_path)
        .args(["issue", "create", "--repo", "alice/repo", "Editor Title"]);

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Created issue #1"));
}

// ===========================================================================
// 2. editor blocked by --yes with no body
// ===========================================================================

/// When --yes is set and no --body is provided, the editor function bails
/// with an error message telling the user to provide content explicitly.
#[tokio::test]
async fn issue_create_editor_blocked_by_yes() {
    let instance = common::TestInstance::start().await;

    // repo_get_issue_templates returns 404 (no templates)
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/issue_templates"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "message": "not found",
            "url": ""
        })))
        .mount(&instance.server)
        .await;

    // repo_get_issue_config returns blank issues enabled
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/issue_config"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "blank_issues_enabled": true,
            "contact_links": []
        })))
        .mount(&instance.server)
        .await;

    // With --yes and no --body, the editor path should be blocked
    instance
        .fj()
        .args([
            "issue",
            "create",
            "--repo",
            "alice/repo",
            "Title Without Body",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--body").or(predicate::str::contains("--body-file")));
}
