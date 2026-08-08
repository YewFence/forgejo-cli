#[test]
fn cli_definition_is_valid() {
    let command = forgejo_cli::cli_command();
    assert_eq!(command.get_name(), "fj");
    command.debug_assert();
}

#[test]
fn every_command_renders_long_help() {
    let mut command = forgejo_cli::cli_command();
    let mut path = vec![command.get_name().to_owned()];
    assert_help_renders(&mut command, &mut path);
}

fn assert_help_renders(command: &mut clap::Command, path: &mut Vec<String>) {
    let command_path = path.join(" ");
    let mut output = Vec::new();
    command
        .write_long_help(&mut output)
        .unwrap_or_else(|error| panic!("failed to render help for `{command_path}`: {error}"));
    let output = String::from_utf8(output)
        .unwrap_or_else(|error| panic!("help for `{command_path}` is not UTF-8: {error}"));
    assert!(
        !output.trim().is_empty(),
        "rendered empty help for `{command_path}`"
    );

    for subcommand in command.get_subcommands_mut() {
        path.push(subcommand.get_name().to_owned());
        assert_help_renders(subcommand, path);
        path.pop();
    }
}
