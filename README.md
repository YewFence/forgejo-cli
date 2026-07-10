<h1 align="center"><img src="logo.png" alt="" width="128"><br>Forgejo CLI <b><i>Plus</i></b></h1>

<p align="center">
  <a href="LICENSE-APACHE"><img src="https://img.shields.io/badge/license-Apache--2.0%20%2F%20MIT-blue" alt="License"></a>
  <img src="https://img.shields.io/badge/lang-Rust-F74C00?logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/Forgejo-compatible-478061?logo=forgejo&logoColor=white" alt="Forgejo">
  <a href="https://codeberg.org/stalecontext/forgejo-cli-plus/releases/latest"><img src="https://img.shields.io/badge/download-latest%20release-brightgreen" alt="Latest Release"></a>
</p>

> Like `gh` for GitHub, but for [Forgejo](https://forgejo.org).

A community-maintained fork of [`forgejo-cli`](https://codeberg.org/forgejo-contrib/forgejo-cli).
Works alongside `git` to handle the Forgejo-specific stuff: issues, PRs, milestones, releases, orgs, wikis, CI, and more.

---

## Highlights

| | |
|---|---|
| **Issues & PRs** | Create, search, edit, comment, close, merge -- with label, assignee, and milestone support |
| **Milestones** | Full CRUD: list, view, create, edit, delete. Filter issues/PRs by milestone |
| **Releases & Tags** | Create releases, attach assets, manage tags |
| **Organizations** | Teams, members, repos, labels, visibility |
| **CI / Actions** | Dispatch workflows, manage secrets and variables |
| **User Management** | Profile editing, SSH/GPG keys, follow/block |
| **Wiki** | View pages, clone wiki repos |
| **Auth** | OAuth login, token-based auth, multi-instance |
| **Automation** | `--json` output, `--yes` auto-confirm, `--verbose` diagnostics, `--force` / `--dry-run` on destructive ops |

### What's different from [upstream](https://codeberg.org/forgejo-contrib/forgejo-cli)?

> [!IMPORTANT]
> This fork adds features and fixes on top of the original. It's a drop-in replacement -- same binary name (`fj`), same config.

**New features:**
- Milestone management -- list, view, create, edit, delete
- `--milestone` flag on issue/PR create and search
- Issue assignee editing (`--add` / `--rm`)
- Tabular list output with aligned columns and colored status indicators
- `--json` flag for machine-readable output on supported list and view commands
- `--yes` / `--verbose` global flags for scripting and automation
- `--force` / `--dry-run` on destructive commands

**Bug fixes:**
- PKCE S256 code challenge for OAuth login
- PR search pagination
- Lowercase and extensionless README file detection
- Relative time display for dates under 1 year

**Project:**
- Comprehensive unit, CLI, and integration test suite
- CI via GitHub Actions and mise
- Full command reference generated directly from the Clap schema
- Open to contributions

---

## Install

### Quick install (Linux / macOS)

```sh
curl -fsSL https://codeberg.org/stalecontext/forgejo-cli-plus/raw/branch/main/install.sh | sh
```

Set `INSTALL_DIR` to change the install location (default: `/usr/local/bin`).

### Homebrew (macOS / Linux)

```sh
brew tap stalecontext/forgejo-cli-plus https://codeberg.org/stalecontext/homebrew-forgejo-cli-plus.git
brew install forgejo-cli-plus
```

### Pre-built binaries

Grab the right archive from the [latest release](https://codeberg.org/stalecontext/forgejo-cli-plus/releases/latest):

| Platform | Archive |
|---|---|
| Linux x86_64 | `forgejo-cli-plus-linux-x86_64.tar.gz` |
| macOS x86_64 (Intel) | `forgejo-cli-plus-macos-x86_64.tar.gz` |
| macOS aarch64 (Apple Silicon) | `forgejo-cli-plus-macos-aarch64.tar.gz` |
| Windows x86_64 | `forgejo-cli-plus-windows-x86_64.zip` |

Extract and put `fj` (or `fj.exe`) somewhere on your `PATH`.

### Cargo

```sh
cargo install forgejo-cli-plus
```

Or install from git for the latest unreleased changes:

```sh
cargo install --git https://codeberg.org/stalecontext/forgejo-cli-plus.git
```

### Nix

```sh
nix run codeberg:stalecontext/forgejo-cli-plus
```

### Build from source

```sh
git clone https://codeberg.org/stalecontext/forgejo-cli-plus.git
cd forgejo-cli-plus
cargo build --release
# binary: target/release/fj
```

### Replacing upstream forgejo-cli

> [!WARNING]
> If you previously installed the upstream `forgejo-cli`, uninstall it first to avoid conflicts -- both install a binary named `fj`.

```sh
# If installed via cargo
cargo uninstall forgejo-cli

# If installed via Homebrew
brew uninstall forgejo-cli

# If installed via Nix
nix profile remove forgejo-cli

# If installed from a pre-built binary
rm "$(which fj)"
```

Then install forgejo-cli-plus using any method above.

---

## Getting started

> [!TIP]
> Create a token at `https://<your-instance>/user/settings/applications`, then add it with the command below. Use `-H` if you're not inside a repo that points to your instance.

```sh
fj -H codeberg.org auth add-key
# paste the token when prompted

fj whoami                                    # verify it worked
```

```sh
# Then use it from any repo with a Forgejo remote
fj repo clone owner/repo                     # clone something
fj issue create "title goes here"            # file an issue
fj pr create "fix the thing"                 # open a PR from current branch
fj pr status --wait                          # watch CI
fj pr merge 42                               # merge it
```

> [!NOTE]
> `fj` figures out which Forgejo instance to talk to from your git remotes. If you're not in a repo, use `-H <host>` or set `FJ_FALLBACK_HOST`.
>
> Output is colorized in terminals, plain when piped. Force plain with `--style minimal`.

### Automation / scripting

> [!TIP]
> Supported list and view commands accept the top-level `--json` flag. Place it before the command, and combine it with `--yes` and `--verbose` for non-interactive scripting.

```sh
fj --yes --json issue search                 # JSON output, no prompts
fj --verbose issue view 42                   # print API calls to stderr
```

```sh
fj repo delete owner/repo --dry-run          # preview without executing
fj --yes repo delete owner/repo --force      # non-interactive delete
```

---

## Command reference

The complete CLI reference is generated from the Clap command schema:

- [CLI reference](docs/cli.md)

After changing commands, arguments, or help text, regenerate and verify it with:

```sh
mise run docs:generate
mise run docs:check
```

Do not edit `docs/cli.md` by hand.

### Shell completion

For example, install Bash completions with:

```sh
fj completion bash > ~/.local/share/bash-completion/completions/fj
```

## Contributing

PRs welcome. Keep them focused, run `mise run check`, and test against a real instance when touching API interactions. After changing CLI commands, arguments, or help text, run `mise run docs:generate` and commit the generated reference.

## License

[Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), your choice.
Contributions are dual-licensed under both unless you state otherwise.

## Credits

Originally created by Cyborus and contributors as [`forgejo-cli`](https://codeberg.org/forgejo-contrib/forgejo-cli).
This fork contains modifications -- see git history for details.
