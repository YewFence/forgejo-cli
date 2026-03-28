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
- `--json` flag for machine-readable output on all list and view commands
- `--yes` / `--verbose` global flags for scripting and automation
- `--force` / `--dry-run` on all 14 destructive commands

**Bug fixes:**
- PKCE S256 code challenge for OAuth login
- PR search pagination
- Lowercase and extensionless README file detection
- Relative time display for dates under 1 year

**Project:**
- Full command reference in the README
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
fj -H codeberg.org auth add-key <your-username>
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
> Every command supports `--json` for machine-readable output. Combine with `--yes` and `--verbose` for fully non-interactive scripting.

```sh
fj --yes --json issue list                   # JSON output, no prompts
fj --verbose issue view 42                   # print API calls to stderr
```

```sh
fj repo delete owner/repo --dry-run          # preview without executing
fj --yes repo delete owner/repo --force      # non-interactive delete
```

---

## Command reference

### Global flags

| Flag | Description |
|---|---|
| `-H, --host <HOST>` | Target Forgejo instance |
| `--style <fancy\|minimal>` | Output style (auto-detected) |
| `--json` | Machine-readable JSON output for list and view commands |
| `-y, --yes` | Skip all confirmation prompts (auto-confirm) |
| `-v, --verbose` | Print API calls and resolution steps to stderr |

---

### `fj repo`

```
fj repo create <name>                       Create repo (-d description, -P private, -r remote, --push, -S ssh)
fj repo fork <repo>                         Fork (--name for custom name)
fj repo clone <repo> [path]                 Clone (-S ssh, -I identity file)
fj repo migrate <url> <name>                Migrate/mirror (-m mirror, -p private, -i include, -s service)
fj repo view [repo]                         View info
fj repo readme [repo]                       View README
fj repo star [repo]                         Star
fj repo unstar [repo]                       Unstar
fj repo delete <repo>                       Delete (-f force, --dry-run)
fj repo browse [repo]                       Open in browser
fj repo labels view                         List labels
fj repo labels create <n> <c>               Create label
fj repo labels edit <id>                    Edit label
fj repo labels delete <id>                  Delete label (-f force, --dry-run)
```

---

### `fj issue`

```
fj issue create [title]                     Create (--body, --body-file, --template, -M milestone, --web)
fj issue view <id>                          View issue (includes milestone info)
fj issue view <id> comments                 List comments
fj issue view <id> comment <n>              View nth comment
fj issue search [query]                     Search (-l labels, -c creator, -a assignee, -M milestone, -s state)
fj issue edit <id> title                    Edit title
fj issue edit <id> body                     Edit body
fj issue edit <id> labels                   Edit labels (--add, --rm)
fj issue edit <id> assignees                Edit assignees (--add, --rm)
fj issue edit <id> comment <n>              Edit comment
fj issue comment <id> [body]                Comment (--body-file)
fj issue close <id>                         Close (-w [msg] optional closing comment)
fj issue reopen <id>                        Reopen (-w [msg] optional comment)
fj issue browse <id>                        Open in browser
fj issue templates                          List templates
```

---

### `fj pr`

```
fj pr create [title]                        Create (--base, --head, --body, -A autofill, -M milestone, --web)
fj pr view [id]                             View PR (includes milestone info)
fj pr view [id] comments                    List comments
fj pr view [id] commits                     List commits
fj pr view [id] files                       List changed files
fj pr view [id] labels                      List labels
fj pr view [id] diff                        View diff (--patch, --editor)
fj pr search [query]                        Search (-l labels, -c creator, -a assignee, -M milestone, -s state)
fj pr checkout <num>                        Checkout locally (--branch-name, -S ssh, -I identity file)
fj pr status [id]                           CI/merge status (--wait to block)
fj pr edit [id] title                       Edit title
fj pr edit [id] body                        Edit body
fj pr edit [id] labels                      Edit labels (--add, --rm)
fj pr edit [id] assignees                   Edit assignees (--add, --rm)
fj pr edit [id] comment <n>                 Edit comment
fj pr comment [id] [body]                   Comment (--body-file)
fj pr close [id]                            Close without merge (-w [msg] optional comment)
fj pr reopen [id]                           Reopen (-w [msg] optional comment)
fj pr merge [id]                            Merge (-M method, --delete branch, -t title, -m message)
fj pr browse [id]                           Open in browser
```

---

### `fj milestone`

```
fj milestone list                           List milestones (-s state: open/closed/all)
fj milestone view <name>                    View details (by title or numeric ID)
fj milestone create <title>                 Create (--body, --due YYYY-MM-DD)
fj milestone edit <name>                    Edit (--title, --body, --due, --state)
fj milestone delete <name>                  Delete (-f force, --dry-run)
```

---

### `fj release`

```
fj release create <name>                    Create (-t tag, -T create tag, --attach, -b body, --draft, --prerelease)
fj release edit <name>                      Edit
fj release delete <name>                    Delete (-t by tag name, -f force, --dry-run)
fj release list                             List (--include-prerelease, --include-draft)
fj release view <name>                      View (-t by tag name)
fj release browse [name]                    Open in browser
fj release asset create <r> <f>             Attach file
fj release asset delete <r> <a>             Remove attachment (-f force, --dry-run)
fj release asset download <r> <a>           Download attachment (-o output path)
```

---

### `fj tag`

```
fj tag create <name>                        Create (-b message, -B branch)
fj tag delete <name>                        Delete (-f force, --dry-run)
fj tag list                                 List (-p page)
fj tag view <name>                          View
```

---

### `fj wiki`

```
fj wiki contents                            List pages
fj wiki view <page>                         View page (rendered)
fj wiki clone                               Clone wiki repo (-p path, -S ssh, -I identity file)
fj wiki browse <page>                       Open in browser
```

---

### `fj actions`

```
fj actions tasks                            List workflow tasks (-p page)
fj actions dispatch <wf> <ref>              Dispatch workflow (-I key=value inputs)
fj actions variables list                   List variables (-v verbose)
fj actions variables create <n> [data]      Create variable (--force to overwrite, opens editor if no data)
fj actions variables delete <n>             Delete variable (-f force, --dry-run)
fj actions secrets list                     List secrets
fj actions secrets create <n> <d>           Create secret
fj actions secrets delete <n>               Delete secret (-f force, --dry-run)
```

---

### `fj org`

```
fj org create <name>                        Create org
fj org edit <name>                          Edit
fj org view <name>                          View
fj org list                                 List orgs (-o only orgs you're a member of)
fj org members <org>                        List members
fj org activity <org>                       Activity feed
fj org visibility <org>                     View/set visibility (-s public|private)
```

**Teams:**
```
fj org team list <org>                      List teams
fj org team view <org> <team>               View team
fj org team create <org> <team>             Create team
fj org team edit <org> <team>               Edit team
fj org team delete <org> <team>             Delete team (-f force, --dry-run)
fj org team repo list <org> <team>          Team repos
fj org team repo add <org> <team> <repo>    Add repo to team
fj org team repo rm <org> <team> <repo>     Remove repo from team (-f force, --dry-run)
fj org team member list <org> <team>        Team members
fj org team member add <org> <team> <u>     Add member
fj org team member rm <org> <team> <u>      Remove member (-f force, --dry-run)
```

**Labels:**
```
fj org label list <org>                     List org labels
fj org label add <org> <n> <c>              Add label
fj org label edit <org> <n>                 Edit label
fj org label rm <org> <n>                   Remove label (-f force, --dry-run)
```

**Repos:**
```
fj org repo list <org>                      List repos
fj org repo create <org> <name>             Create repo in org
```

---

### `fj user`

```
fj user search <query>                      Search users
fj user view [user]                         View profile (omit for self)
fj user browse [user]                       Open in browser
fj user follow <user>                       Follow
fj user unfollow <user>                     Unfollow
fj user following [user]                    List follows
fj user followers [user]                    List followers
fj user block <user>                        Block
fj user unblock <user>                      Unblock
fj user repos [user]                        List repos (--starred, --sort)
fj user orgs [user]                         List orgs
fj user activity [user]                     Activity feed
```

**Profile editing:**
```
fj user edit bio [text]                     Set bio
fj user edit name [name]                    Set display name
fj user edit pronouns [text]                Set pronouns
fj user edit location [text]                Set location
fj user edit website [url]                  Set website
fj user edit email                          Manage emails
fj user edit activity                       Activity visibility
```

**SSH keys:**
```
fj user key list                            List keys
fj user key upload [file]                   Upload key (-t title, --force, -r read-only)
fj user key view <id>                       View key
fj user key delete <id>                     Delete key (-f force, --dry-run)
```

**GPG keys:**
```
fj user gpg list                            List keys
fj user gpg upload <key>                    Upload key (--no-verify to skip verification)
fj user gpg view <id>                       View key
fj user gpg delete <id>                     Delete key (-f force, --dry-run)
fj user gpg verify <id>                     Verify key
```

---

### `fj auth`

```
fj auth login                               Log in (OAuth, opens browser)
fj auth logout <host>                       Log out
fj auth add-key <user> [key]                Add API token directly
fj auth use-ssh [bool]                      Toggle SSH as default
fj auth list                                List instances
```

---

### Other

```
fj whoami                                   Show current user@instance
fj version                                  Version (-v verbose; --check with update-check feature)
fj completion <shell>                       Shell completions (bash/zsh/fish/powershell/elvish/nushell)
```

Shell completion install example:
```sh
fj completion bash > ~/.local/share/bash-completion/completions/fj
```


---

## Contributing

PRs welcome. Keep them focused, make sure `cargo build` passes, test against a real instance.

## License

[Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), your choice.
Contributions are dual-licensed under both unless you state otherwise.

## Credits

Originally created by Cyborus and contributors as [`forgejo-cli`](https://codeberg.org/forgejo-contrib/forgejo-cli).
This fork contains modifications -- see git history for details.
