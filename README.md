<h1 align="center"><img src="logo.png" alt="" width="128"><br>Forgejo CLI</h1>

<p align="center">
  <a href="LICENSE-APACHE"><img src="https://img.shields.io/badge/license-Apache--2.0%20%2F%20MIT-blue" alt="License"></a>
  <img src="https://img.shields.io/badge/lang-Rust-F74C00?logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/Forgejo-compatible-478061?logo=forgejo&logoColor=white" alt="Forgejo">
  <a href="https://github.com/YewFence/forgejo-cli/releases/latest"><img src="https://img.shields.io/github/v/release/YewFence/forgejo-cli?sort=semver" alt="Latest release"></a>
  <a href="https://YewFence.github.io/forgejo-cli/"><img src="https://img.shields.io/badge/docs-online-blue" alt="Documentation"></a>
</p>

> Like `gh` for GitHub, but for [Forgejo](https://forgejo.org).

An independently maintained hard fork of [`forgejo-cli-plus`](https://codeberg.org/stalecontext/forgejo-cli-plus), which is itself based on the original [`forgejo-cli`](https://codeberg.org/forgejo-contrib/forgejo-cli).
It works alongside `git` to handle Forgejo-specific tasks: issues, pull requests, milestones, releases, organizations, wikis, CI, and more.

## Highlights

| | |
|---|---|
| **Issues & PRs** | Create, search, edit, comment, close, and merge, with label, assignee, and milestone support |
| **Milestones** | List, view, create, edit, and delete milestones; filter issues and PRs by milestone |
| **Releases & Tags** | Create releases, attach assets, and manage tags |
| **Organizations** | Manage teams, members, repositories, labels, and visibility |
| **CI / Actions** | Dispatch workflows and manage secrets and variables |
| **User Management** | Edit profiles, manage SSH/GPG keys, and follow or block users |
| **Wiki** | View pages and clone wiki repositories |
| **Auth** | OAuth login, token-based authentication, and multiple instances |
| **Automation** | `--json`, `--yes`, `--verbose`, and `--force` / `--dry-run` on destructive operations |

### Differences from the original forgejo-cli

This fork retains the command shape, binary name (`fj`), and configuration format while extending the original project with:

- milestone management and milestone filters for issues and pull requests;
- issue assignee editing;
- aligned, colorized table output;
- machine-readable JSON output on supported commands;
- non-interactive and diagnostic flags for automation;
- dry-run and force modes for destructive operations;
- PKCE S256 for OAuth login;
- pagination, README detection, and relative-date fixes;
- unit, CLI, and wiremock integration tests;
- mise-based local tooling and thin GitHub Actions workflows;
- generated CLI and VitePress documentation.

## Installation

### From a local checkout

```sh
cargo install --path .
```

### Directly from the Git repository

```sh
cargo install --git https://github.com/YewFence/forgejo-cli.git
```

### From GitHub Releases with mise

```sh
mise use --global github:YewFence/forgejo-cli
fj version
```

For one-off execution without a global installation:

```sh
mise exec github:YewFence/forgejo-cli -- fj version
```

## Getting started

Create a token at `https://<your-instance>/user/settings/applications`, then add it to `fj`. Use `-H` when the current repository does not have a remote for that instance.

```sh
fj -H codeberg.org auth add-key
# Paste the token when prompted.

fj whoami
```

Once authenticated, run commands from a repository with a Forgejo remote:

```sh
fj repo clone owner/repo
fj issue create "title goes here"
fj pr create "fix the thing"
fj pr status --wait
fj pr merge 42
```

`fj` normally determines the Forgejo instance from Git remotes. Outside a repository, pass `-H <host>` or set `FJ_FALLBACK_HOST`.

Output is colorized in terminals and plain when piped. Use `--style minimal` to force plain output.

### Automation and scripting

Supported list and view commands accept the top-level `--json` flag. Place it before the command and combine it with `--yes` or `--verbose` as needed.

```sh
fj --yes --json issue search
fj --verbose issue view 42
fj repo delete owner/repo --dry-run
fj --yes repo delete owner/repo --force
```

## Documentation

- [Documentation site](https://YewFence.github.io/forgejo-cli/)
- [Generated CLI reference](docs/cli.md)

After changing commands, arguments, or help text, regenerate and verify the CLI reference:

```sh
mise run docs:generate
mise run docs:check
```

Do not edit `docs/cli.md` manually.

### Shell completion

For example, install Bash completions with:

```sh
fj completion bash > ~/.local/share/bash-completion/completions/fj
```

## Development

```sh
mise run hooks:install
mise run check
mise run test
mise -E ci run audit
```

See the [Contributing Guide](CONTRIBUTING.md) for the complete development workflow.

## Contributing

Issues and pull requests are welcome in the [GitHub repository](https://github.com/YewFence/forgejo-cli).

## License

[Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
Contributions are dual-licensed under both licenses unless explicitly stated otherwise.

## Credits

Originally created by Cyborus and contributors as [`forgejo-cli`](https://codeberg.org/forgejo-contrib/forgejo-cli), with subsequent work from the [`forgejo-cli-plus`](https://codeberg.org/stalecontext/forgejo-cli-plus) maintainers and contributors.
