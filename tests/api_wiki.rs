mod common;

use predicates::prelude::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn wiki_contents() {
    let instance = common::TestInstance::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/wiki/pages"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([
                    {
                        "title": "Home",
                        "sub_url": "Home",
                        "html_url": "https://example.com/alice/repo/wiki/Home",
                        "last_commit": null
                    },
                    {
                        "title": "Getting Started",
                        "sub_url": "Getting-Started",
                        "html_url": "https://example.com/alice/repo/wiki/Getting-Started",
                        "last_commit": null
                    }
                ]))
                .insert_header("x-total-count", "2"),
        )
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["wiki", "--repo", "alice/repo", "contents"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Home"))
        .stdout(predicate::str::contains("Getting Started"));
}

#[tokio::test]
async fn wiki_view() {
    let instance = common::TestInstance::start().await;

    // "Hello wiki" base64-encoded is "SGVsbG8gd2lraQ=="
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/repo/wiki/page/Home"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "title": "Home",
            "sub_url": "Home",
            "content_base64": "SGVsbG8gd2lraQ==",
            "html_url": "https://example.com/alice/repo/wiki/Home",
            "commit_count": 1,
            "footer": "",
            "sidebar": "",
            "last_commit": null
        })))
        .mount(&instance.server)
        .await;

    instance
        .fj()
        .args(["wiki", "--repo", "alice/repo", "view", "Home"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Home"))
        .stdout(predicate::str::contains("Hello wiki"));
}
