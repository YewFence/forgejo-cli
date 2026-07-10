pub fn generate_cli_reference() -> String {
    let markdown = clap_markdown::help_markdown_command(&forgejo_cli_plus::cli_command());
    let mut normalized = String::with_capacity(markdown.len());

    for line in markdown.lines() {
        let line = line.trim_end();
        let line = line.strip_suffix(" —").unwrap_or(line);
        normalized.push_str(line);
        normalized.push('\n');
    }

    normalized
}
