use assert_cmd::Command;
use rstest::rstest;

fn fj() -> Command {
    Command::cargo_bin("fj").unwrap()
}

/// Normalize platform-specific binary name so snapshots are portable.
fn normalize_bin_name(s: &str) -> String {
    s.replace("fj.exe", "fj")
}

#[test]
fn help_top_level() {
    let output = fj().arg("--help").output().unwrap();
    let stdout = normalize_bin_name(&String::from_utf8_lossy(&output.stdout));
    insta::assert_snapshot!("help_top_level", stdout);
}

#[rstest]
#[case("repo")]
#[case("issue")]
#[case("pr")]
#[case("tag")]
#[case("release")]
#[case("milestone")]
#[case("org")]
#[case("user")]
#[case("actions")]
#[case("auth")]
#[case("wiki")]
#[case("version")]
fn help_subcommand(#[case] cmd: &str) {
    let output = fj().args([cmd, "--help"]).output().unwrap();
    let stdout = normalize_bin_name(&String::from_utf8_lossy(&output.stdout));
    insta::assert_snapshot!(format!("help_{cmd}"), stdout);
}

#[rstest]
#[case("actions run")]
#[case("actions artifact")]
fn help_nested_subcommand(#[case] cmd: &str) {
    let mut args: Vec<&str> = cmd.split(' ').collect();
    args.push("--help");
    let output = fj().args(&args).output().unwrap();
    let stdout = normalize_bin_name(&String::from_utf8_lossy(&output.stdout));
    insta::assert_snapshot!(format!("help_{}", cmd.replace(' ', "_")), stdout);
}
