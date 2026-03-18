# forgejo-cli-plus

Community-maintained fork of [forgejo-cli](https://codeberg.org/forgejo-contrib/forgejo-cli).

## Build & test

```sh
cargo build              # debug
cargo build --release    # binary at target/release/fj
cargo test               # PKCE S256 tests
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

## Upstream sync

```sh
git remote add upstream https://codeberg.org/forgejo-contrib/forgejo-cli.git
git fetch upstream
git cherry-pick <sha>
```

Cherry-pick only. Don't merge upstream wholesale.
