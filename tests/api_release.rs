mod common;

use predicates::prelude::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

fn mock_release_with_assets(
    id: i64,
    name: &str,
    tag: &str,
    assets: serde_json::Value,
) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "tag_name": tag,
        "name": name,
        "body": "Release notes",
        "draft": false,
        "prerelease": false,
        "created_at": "2024-07-01T12:00:00Z",
        "published_at": "2024-07-01T12:00:00Z",
        "html_url": format!("https://example.com/alice/repo/releases/tag/{tag}"),
        "tarball_url": format!("https://example.com/alice/repo/archive/{tag}.tar.gz"),
        "zipball_url": format!("https://example.com/alice/repo/archive/{tag}.zip"),
        "url": format!("https://example.com/api/v1/repos/alice/repo/releases/{id}"),
        "upload_url": format!("https://example.com/api/v1/repos/alice/repo/releases/{id}/assets"),
        "assets": assets,
        "author": {
            "id": 1,
            "login": "alice",
            "full_name": "Alice",
            "email": "alice@example.com",
            "avatar_url": "",
            "html_url": "https://example.com/alice",
            "created": "2024-01-01T00:00:00Z",
            "last_login": "2024-01-01T00:00:00Z"
        }
    })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn mock_release_obj(id: i64, name: &str, tag: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "tag_name": tag,
        "name": name,
        "body": "Release notes",
        "draft": false,
        "prerelease": false,
        "created_at": "2024-07-01T12:00:00Z",
        "published_at": "2024-07-01T12:00:00Z",
        "html_url": format!("https://example.com/alice/repo/releases/tag/{tag}"),
        "tarball_url": format!("https://example.com/alice/repo/archive/{tag}.tar.gz"),
        "zipball_url": format!("https://example.com/alice/repo/archive/{tag}.zip"),
        "url": format!("https://example.com/api/v1/repos/alice/repo/releases/{id}"),
        "upload_url": format!("https://example.com/api/v1/repos/alice/repo/releases/{id}/assets"),
        "assets": [],
        "author": {
            "id": 1,
            "login": "alice",
            "full_name": "Alice",
            "email": "alice@example.com",
            "avatar_url": "",
            "html_url": "https://example.com/alice",
            "created": "2024-01-01T00:00:00Z",
            "last_login": "2024-01-01T00:00:00Z"
        }
    })
}

// ===========================================================================
// 1. release create (using --tag to reference an existing tag)
// ===========================================================================

#[tokio::test]
async fn release_create() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/repos/alice/repo/releases"))
        .respond_with(ResponseTemplate::new(201).set_body_json(mock_release_obj(
            1,
            "Release 1.0",
            "v1.0.0",
        )))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "release",
            "--repo",
            "alice/repo",
            "create",
            "Release 1.0",
            "--tag",
            "v1.0.0",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Created release Release 1.0"));
}

// ===========================================================================
// 2. release view --json (find by name via paginated list)
// ===========================================================================

#[tokio::test]
async fn release_view_json() {
    let instance = common::TestInstance::start().await;

    // find_release streams GET /api/v1/repos/alice/repo/releases to match name
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/releases"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_release_obj(
                    1,
                    "Release 1.0",
                    "v1.0.0"
                )]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "--json",
            "release",
            "--repo",
            "alice/repo",
            "view",
            "Release 1.0",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"name\": \"Release 1.0\""));
}

// ===========================================================================
// 3. release edit (find by name, then PATCH)
// ===========================================================================

#[tokio::test]
async fn release_edit() {
    let instance = common::TestInstance::start().await;

    // find_release streams GET /api/v1/repos/alice/repo/releases
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/releases"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_release_obj(
                    1,
                    "Release 1.0",
                    "v1.0.0"
                )]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    // PATCH /api/v1/repos/alice/repo/releases/{id}
    Mock::given(method("PATCH"))
        .and(path("/api/v1/repos/alice/repo/releases/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_release_obj(
            1,
            "Release 1.0.1",
            "v1.0.0",
        )))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "release",
            "--repo",
            "alice/repo",
            "edit",
            "Release 1.0",
            "--rename",
            "Release 1.0.1",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Updated release Release 1.0"));
}

// ===========================================================================
// 4. release delete --force (find by name, then DELETE)
// ===========================================================================

#[tokio::test]
async fn release_delete_force() {
    let instance = common::TestInstance::start().await;

    // find_release streams GET /api/v1/repos/alice/repo/releases
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/releases"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([mock_release_obj(
                    1,
                    "Release 1.0",
                    "v1.0.0"
                )]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    // DELETE /api/v1/repos/alice/repo/releases/{id}
    Mock::given(method("DELETE"))
        .and(path("/api/v1/repos/alice/repo/releases/1"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args([
            "release",
            "--repo",
            "alice/repo",
            "delete",
            "Release 1.0",
            "--force",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Deleted release Release 1.0"));
}

// ===========================================================================
// 5. release asset download (named asset, not source archive)
// ===========================================================================

#[tokio::test]
async fn release_asset_download() {
    let instance = common::TestInstance::start().await;

    let asset_uuid = "abc-123-def-456";

    // 1. find_release streams GET /releases to match by name
    let release = mock_release_with_assets(
        1,
        "Release 1.0",
        "v1.0.0",
        serde_json::json!([{
            "id": 42,
            "name": "binary.tar.gz",
            "size": 13,
            "download_count": 5,
            "created_at": "2024-07-01T12:00:00Z",
            "uuid": asset_uuid,
            "browser_download_url": "https://example.com/dl/binary.tar.gz"
        }]),
    );

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/releases"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([release]))
                .insert_header("x-total-count", "1"),
        )
        .mount(&instance.server)
        .await;

    // 2. download_release_attachment first calls repo_get_release_attachment
    //    GET /api/v1/repos/{owner}/{repo}/releases/{id}/assets/{attachment_id}
    //    which returns the Attachment JSON (needs uuid field)
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/releases/1/assets/42"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 42,
            "name": "binary.tar.gz",
            "size": 13,
            "download_count": 5,
            "created_at": "2024-07-01T12:00:00Z",
            "uuid": asset_uuid,
            "browser_download_url": "https://example.com/dl/binary.tar.gz"
        })))
        .expect(1)
        .mount(&instance.server)
        .await;

    // 3. Then it downloads the raw bytes from /attachments/{uuid}
    Mock::given(method("GET"))
        .and(path(format!("/attachments/{asset_uuid}")))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(b"file content!"))
        .expect(1)
        .mount(&instance.server)
        .await;

    let output_dir = tempfile::tempdir().unwrap();
    let output_path = output_dir.path().join("binary.tar.gz");

    instance
        .fj()
        .args([
            "release",
            "--repo",
            "alice/repo",
            "asset",
            "download",
            "Release 1.0",
            "binary.tar.gz",
            "--output",
            output_path.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Downloaded binary.tar.gz"));

    let content = std::fs::read_to_string(&output_path).expect("output file should exist");
    assert_eq!(content, "file content!");
}
