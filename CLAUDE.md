# forgejo-cli-plus

Community-maintained fork of [forgejo-cli](https://codeberg.org/forgejo-contrib/forgejo-cli) adding milestone support, project boards, and other features not present upstream.

## Build & run

```sh
cargo build              # debug
cargo build --release    # release — binary at target/release/fj
cargo test               # no tests exist yet
```

Binary name is `fj` (set in Cargo.toml `[[bin]]`).

## Key things to know

- **Milestone fields are hardcoded to `None`** throughout issues.rs and prs.rs — in create options, edit options, and search queries. This is the first thing to fix.
- **No test suite exists.** Any new feature should ideally add tests, but the upstream never had them either.
- **`SpecialRender` is a global singleton** initialized once in `main()`. Access it via `crate::special_render()`. It controls all colored/unicode output. Don't construct your own ANSI codes.
- **Editor integration** uses `$EDITOR` env var. VS Code gets `--wait` appended automatically (see `get_editor_flags()` in main.rs).
- **Cargo.toml `repository` field and flake.nix `homepage`** still point to upstream — update when publishing.

## Adding a new command

Follow `src/tag.rs` as the template — it's the simplest complete command.

1. Create `src/<command>.rs`
2. Define `<Command>Command` struct with `#[derive(Args)]`, include `remote: Option<String>` and `repo: Option<RepoArg>` fields
3. Define `<Command>Subcommand` enum with `#[derive(Subcommand)]`
4. Implement `run()` — call `RepoInfo::get_current()` then `keys.get_api()`
5. Add `mod <command>;` and the enum variant in `main.rs`

## Conventions

- `eyre` for errors, `OptionExt` for `.ok_or_eyre()` on Options
- `crate::special_render()` for colors and symbols — never raw ANSI
- `crate::markdown()` to render markdown bodies
- `crate::editor()` to open user's editor for text input
- `crate::readline()` and `crate::prompt_bool()` for interactive prompts
- Naming is inconsistent: org labels/team-repos/team-members use `Rm`, but org teams and everything else (repo labels, tags, releases) use `Delete`. Match the existing pattern in whichever module you're editing.

## Upstream sync

```sh
git remote add upstream https://codeberg.org/forgejo-contrib/forgejo-cli.git
git fetch upstream
git cherry-pick <sha>    # cherry-pick individual fixes as needed
```

Don't merge upstream wholesale — we'll diverge intentionally on features.

## License

Apache-2.0 OR MIT, dual-licensed.
