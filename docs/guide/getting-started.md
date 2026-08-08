# Getting Started

## Installation

Install from a local checkout:

```sh
cargo install --path .
```

Install directly from the Git repository:

```sh
cargo install --git https://github.com/YewFence/forgejo-cli.git
```

Install a binary from the latest GitHub Release with mise:

```sh
mise use --global github:YewFence/forgejo-cli
fj version
```

Or run it once without installing globally:

```sh
mise exec github:YewFence/forgejo-cli -- fj version
```

## Authentication

Create an access token at `https://<your-instance>/user/settings/applications`, then add it to `fj`:

```sh
fj -H codeberg.org auth add-key
fj whoami
```

Inside a Git repository, `fj` normally selects the Forgejo instance from the configured remotes. Outside a repository, pass `-H <host>` or set `FJ_FALLBACK_HOST`.

## First commands

```sh
fj repo clone owner/repo
fj issue create "title goes here"
fj pr create "fix the thing"
fj pr status --wait
```

Run `fj help` to inspect the command tree or open the [generated CLI reference](/cli) for every command and option.

## Automation

Place global flags before the command:

```sh
fj --yes --json issue search
fj --verbose issue view 42
```

Destructive operations support previews and explicit confirmation bypasses:

```sh
fj repo delete owner/repo --dry-run
fj --yes repo delete owner/repo --force
```
