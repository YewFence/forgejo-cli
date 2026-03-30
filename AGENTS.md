# forgejo-cli-plus

Community-maintained fork of [forgejo-cli](https://codeberg.org/forgejo-contrib/forgejo-cli).

## Build & test

```sh
cargo build              # debug
cargo build --release    # binary at target/release/fj
cargo test --all-targets # full suite (93 tests)
cargo insta review       # review changed snapshots
```

Binary name is `fj` (set in Cargo.toml `[[bin]]`).

## Gotchas

- **`SpecialRender` is a global singleton** -- access via `crate::special_render()`. Never construct raw ANSI codes.
- **`--repo` flag is inconsistent** across commands. `search`, `create`, `milestone` have `--repo`; `view`, `edit` only have `--remote` (local git remote name). Pre-existing upstream issue.
- **Cargo.toml `repository` and flake.nix `homepage`** still point to upstream.

## Adding a new command

Follow `src/tag.rs` as the template.

1. Create `src/<command>.rs`
2. `<Command>Command` struct with `#[derive(Args)]`, include `remote: Option<String>` and `repo: Option<RepoArg>`
3. `<Command>Subcommand` enum with `#[derive(Subcommand)]`
4. `run()` calls `RepoInfo::get_current()` then `keys.get_api()`
5. Add `mod <command>;` and enum variant in `main.rs`

## Conventions

- `eyre` for errors, `OptionExt` for `.ok_or_eyre()` on Options
- `crate::special_render()` for colors/symbols
- `crate::markdown()` for markdown bodies, `crate::editor()` for editor input
- `crate::readline()` and `crate::prompt_bool()` for interactive prompts
- Every mutation command prints a confirmation message
- `.all()` for paginated lists, `.stream()` for find-by-name searches
- Name-to-ID resolution: try numeric ID via direct API first, fall back to name search with server-side filter
- Naming: org labels/team-repos/team-members use `Rm`; everything else uses `Delete`. Match existing pattern per module.

## Agentic / non-interactive flags

Global flags (on every command):
- `--yes` / `-y` -- skip all confirmation prompts, auto-confirm
- `--verbose` / `-v` -- print API calls and resolution steps to stderr
- `--json` -- machine-readable JSON output

Per-command flags (on destructive operations only):
- `--force` / `-f` -- skip confirmation prompt for this operation
- `--dry-run` -- preview what would happen without executing

Example agent invocation:
```sh
fj --yes --json issue list
fj --yes --json repo delete owner/repo --force
fj repo delete owner/repo --dry-run
```

## Adding a destructive command

Every delete/remove operation must follow this pattern:

1. Add `force: bool` (`--force`/`-f`) and `dry_run: bool` (`--dry-run`) to the subcommand variant
2. In the handler function, check dry-run first, then confirmation, then execute:

```rust
if dry_run {
    crate::output::dry_run(&format!("delete thing {name}"));
    return Ok(());
}
if !force && !crate::yes_mode() {
    if !crate::prompt_bool(&format!("Delete '{name}'?"), false).await? {
        crate::output::info("Not deleted");
        return Ok(());
    }
}
crate::verbose_log!("Deleting {name}");
// ... API call ...
crate::output::success(&format!("Deleted {name}"));
```

## Testing

Run `cargo test --all-targets` before committing. All tests must pass.

**Test structure:**
- `src/*.rs` -- in-module `#[cfg(test)]` unit tests for pure functions
- `tests/cli_help.rs` -- `insta` snapshot tests for all `--help` output
- `tests/cli_errors.rs` -- CLI error handling and exit code tests
- `tests/api_repo.rs`, `tests/api_tag.rs`, `tests/api_issue.rs` -- wiremock integration tests
- `tests/dry_run.rs` -- parameterized `--dry-run` test for all 14 destructive commands
- `tests/common/mod.rs` -- shared `TestInstance` helper (wiremock + assert_cmd)

**When adding a new command:**
- Add unit tests for any pure parsing/validation functions in the module's `#[cfg(test)]` block
- Add a `--help` snapshot: add a `#[case("newcommmand")]` line to `tests/cli_help.rs`, run `cargo insta test --accept` to generate it
- If the command is destructive, add a `#[case]` to `tests/dry_run.rs`

**When adding a destructive command:**
- Add a `#[case(&["...", "--dry-run"])]` line to `tests/dry_run.rs`
- Verify the `--dry-run` check is the very first thing in the handler (before any API calls)

**Snapshots:**
- `cargo insta test` runs tests and shows pending snapshot changes
- `cargo insta review` interactively accepts/rejects changes
- `cargo insta test --accept` accepts all changes (use after intentional help text updates)
- Snapshots normalize `fj.exe` to `fj` for cross-platform portability

**wiremock integration tests:**
- `TestInstance::start()` creates an isolated mock server + temp data dir
- `instance.fj()` returns a preconfigured `Command` pointing at the mock server
- Use `.expect(1)` on mocks to verify API calls are actually made
- `FJ_DATA_DIR` env var isolates key storage per test to prevent races

## Upstream sync

```sh
git remote add upstream https://codeberg.org/forgejo-contrib/forgejo-cli.git
git fetch upstream
git cherry-pick <sha>
```

Cherry-pick only. Don't merge upstream wholesale.
