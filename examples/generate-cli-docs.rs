use std::path::PathBuf;

mod support;

fn main() -> std::io::Result<()> {
    let output = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/cli.md");
    let markdown = support::generate_cli_reference();

    std::fs::create_dir_all(output.parent().expect("output path should have a parent"))?;
    std::fs::write(&output, markdown)?;
    eprintln!("Generated {}", output.display());

    Ok(())
}
