#[path = "../examples/support/mod.rs"]
mod support;

#[cfg(not(feature = "update-check"))]
const CLI_REFERENCE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/docs/cli.md");

#[cfg(not(feature = "update-check"))]
#[test]
fn generated_cli_reference_is_current() {
    let generated = support::generate_cli_reference();
    let committed = std::fs::read_to_string(CLI_REFERENCE)
        .expect("docs/cli.md is missing; run `mise run docs:generate`");
    let committed = committed.replace("\r\n", "\n");

    assert!(
        committed == generated,
        "docs/cli.md is stale; run `mise run docs:generate`"
    );
}

#[cfg(feature = "update-check")]
#[test]
fn update_check_reference_renders() {
    let generated = support::generate_cli_reference();
    assert!(
        generated.contains("`--check`"),
        "the update-check feature should add `fj version --check` to the generated reference"
    );
}
