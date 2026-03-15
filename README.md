# `fj` — Forgejo CLI Plus

Fork of [`forgejo-cli`](https://codeberg.org/forgejo-contrib/forgejo-cli). Like `gh` for GitHub, but for Forgejo.

Works alongside `git` to handle the Forgejo-specific stuff: issues, PRs, releases, orgs, wikis, CI, etc.

## What's different from upstream?

- Milestone support — planned (create, view, edit, delete, filter by)
- Project board support — planned
- Full command reference in the README instead of a wiki
- Open to contributions

## Install

**Pre-built binaries** — grab from the [releases page](https://codeberg.org/stalecontext/forgejo-cli-plus/releases/latest) (x86_64 Windows + Linux).

**Cargo:**
```sh
cargo install --git https://codeberg.org/stalecontext/forgejo-cli-plus.git
```

**Nix:**
```sh
nix run codeberg:stalecontext/forgejo-cli-plus
```

**From source:**
```sh
git clone https://codeberg.org/stalecontext/forgejo-cli-plus.git
cd forgejo-cli-plus
cargo build --release
# binary: target/release/fj
```

## Getting started

```sh
fj auth login                        # OAuth via browser
fj whoami                            # verify it worked
fj repo clone owner/repo             # clone something
fj issue create "title goes here"    # file an issue
fj pr create "fix the thing"         # open a PR from current branch
fj pr status --wait                  # watch CI
fj pr merge 42                       # merge it
```

`fj` figures out which Forgejo instance to talk to from your git remotes. Override with `--host` or `--repo`. Set `FJ_FALLBACK_HOST` for when you're not in a repo.

Output is colorized in terminals, plain when piped. Force plain with `--style minimal`.

## Command reference

### Global flags

| Flag | |
|---|---|
| `-H, --host <HOST>` | Target Forgejo instance |
| `--style <fancy\|minimal>` | Output style (auto-detected) |

### `fj repo`

```
fj repo create <name>           Create repo (-d description, -P private, -r remote name, --push, -S ssh)
fj repo fork <repo>             Fork (--name for custom name)
fj repo clone <repo> [path]     Clone (-S ssh, -I identity file)
fj repo migrate <url> <name>    Migrate/mirror (-m mirror, -p private, -i include, -L lfs-endpoint, -s service, --token, --login)
fj repo view [repo]             View info
fj repo readme [repo]           View README
fj repo star [repo]             Star
fj repo unstar [repo]           Unstar
fj repo delete <repo>           Delete (irreversible)
fj repo browse [repo]           Open in browser
fj repo labels view             List labels
fj repo labels create <n> <c>   Create label
fj repo labels edit <id>        Edit label
fj repo labels delete <id>      Delete label
```

### `fj issue`

```
fj issue create [title]         Create (--body, --body-file, --template, --no-template, --web)
fj issue view <id>              View issue
fj issue view <id> comments     List comments
fj issue view <id> comment <n>  View nth comment
fj issue search [query]         Search (-l labels, -c creator, -a assignee, -s state)
fj issue edit <id> title        Edit title
fj issue edit <id> body         Edit body
fj issue edit <id> labels       Edit labels (--add, --rm)
fj issue edit <id> comment <n>  Edit comment
fj issue comment <id> [body]    Comment (--body-file)
fj issue close <id>             Close (-w [msg] optional closing comment)
fj issue browse <id>            Open in browser
fj issue templates              List templates
```

### `fj pr`

```
fj pr create [title]            Create (--base, --head, --body, --body-file, -A autofill, --agit, --web)
fj pr view [id]                 View PR
fj pr view [id] comments        List comments
fj pr view [id] commits         List commits
fj pr view [id] files           List changed files
fj pr view [id] labels          List labels
fj pr view [id] diff            View diff (--patch for patch format, --editor to open in editor)
fj pr search [query]            Search (-l labels, -c creator, -a assignee, -s state)
fj pr checkout <num>            Checkout locally (--branch-name, -S ssh, -I identity file)
fj pr status [id]               CI/merge status (--wait to block)
fj pr edit [id] title           Edit title
fj pr edit [id] body            Edit body
fj pr edit [id] labels          Edit labels (--add, --rm)
fj pr comment [id] [body]       Comment (--body-file)
fj pr close [id]                Close without merge (-w [msg] optional comment)
fj pr merge [id]                Merge (-M method, --delete branch, -t title, -m message)
fj pr browse [id]               Open in browser
```

### `fj release`

```
fj release create <name>        Create (-t tag, -T create tag, --attach, -b body, -B branch, --draft, --prerelease)
fj release edit <name>          Edit
fj release delete <name>        Delete (-t by tag name)
fj release list                 List (--include-prerelease, --include-draft)
fj release view <name>          View (-t by tag name)
fj release browse [name]        Open in browser
fj release asset create <r> <f> Attach file
fj release asset delete <r> <a> Remove attachment
fj release asset download <r> <a>  Download attachment (-o output path)
```

### `fj tag`

```
fj tag create <name>            Create (-b message, -B branch)
fj tag delete <name>            Delete
fj tag list                     List (-p page)
fj tag view <name>              View
```

### `fj wiki`

```
fj wiki contents                List pages
fj wiki view <page>             View page (rendered)
fj wiki clone                   Clone wiki repo (-p path, -S ssh, -I identity file)
fj wiki browse <page>           Open in browser
```

### `fj actions`

```
fj actions tasks                List workflow tasks (-p page)
fj actions dispatch <wf> <ref>  Dispatch workflow (-I key=value inputs)
fj actions variables list       List variables (-v verbose)
fj actions variables create <n> [data]  Create variable (--force to overwrite, opens editor if no data)
fj actions variables delete <n> Delete variable
fj actions secrets list         List secrets
fj actions secrets create <n> <d>  Create secret
fj actions secrets delete <n>   Delete secret
```

### `fj org`

```
fj org create <name>            Create org
fj org edit <name>              Edit
fj org view <name>              View
fj org list                     List orgs (-o only orgs you're a member of)
fj org members <org>            List members
fj org activity <org>           Activity feed
fj org visibility <org>         View/set your visibility (-s public|private)

fj org team list <org>                    List teams
fj org team view <org> <team>             View team
fj org team create <org> <team>           Create team
fj org team edit <org> <team>             Edit team
fj org team delete <org> <team>           Delete team
fj org team repo list <org> <team>        Team repos
fj org team repo add <org> <team> <repo>  Add repo to team
fj org team repo rm <org> <team> <repo>   Remove repo from team
fj org team member list <org> <team>      Team members
fj org team member add <org> <team> <u>   Add member
fj org team member rm <org> <team> <u>    Remove member

fj org label list <org>         List org labels
fj org label add <org> <n> <c>  Add label
fj org label edit <org> <n>     Edit label
fj org label rm <org> <n>       Remove label

fj org repo list <org>          List repos
fj org repo create <org> <name> Create repo in org
```

### `fj user`

```
fj user search <query>          Search users
fj user view [user]             View profile (omit for self)
fj user browse [user]           Open in browser
fj user follow <user>           Follow
fj user unfollow <user>         Unfollow
fj user following [user]        List follows
fj user followers [user]        List followers
fj user block <user>            Block
fj user unblock <user>          Unblock
fj user repos [user]            List repos (--starred, --sort)
fj user orgs [user]             List orgs
fj user activity [user]         Activity feed

fj user edit bio [text]         Set bio
fj user edit name [name]        Set display name
fj user edit pronouns [text]    Set pronouns
fj user edit location [text]    Set location
fj user edit website [url]      Set website
fj user edit email              Manage emails
fj user edit activity           Activity visibility

fj user key list                SSH keys
fj user key upload [file]       Upload SSH key (-t title, --force, -r read-only)
fj user key view <id>           View key
fj user key delete <id>         Delete key

fj user gpg list                GPG keys
fj user gpg upload <key>        Upload GPG key (--no-verify to skip verification)
fj user gpg view <id>           View key
fj user gpg delete <id>         Delete key
fj user gpg verify <id>         Verify a GPG key
```

### `fj auth`

```
fj auth login                   Log in (OAuth, opens browser)
fj auth logout <host>           Log out
fj auth add-key <user> [key]    Add API token directly
fj auth use-ssh [bool]          Toggle SSH as default
fj auth list                    List instances
```

### Other

```
fj whoami                       Show current user@instance
fj version                      Version (-v verbose; --check if built with update-check feature)
fj completion <shell>           Shell completions (bash/zsh/fish/powershell/elvish/nushell)
```

Shell completion install example:
```sh
fj completion bash > ~/.local/share/bash-completion/completions/fj
```

## Roadmap

- [ ] Milestone management (list, create, view, edit, delete)
- [ ] `--milestone` flag on issue/PR create and edit
- [ ] Milestone filtering in search
- [ ] Milestone info in issue/PR view output
- [ ] Project board support

## Contributing

PRs welcome. Keep them focused, make sure `cargo build` passes, test against a real instance.

## License

[Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), your choice.

Contributions are dual-licensed under both unless you state otherwise.

## Credits

Originally created by Cyborus and contributors as [forgejo-cli](https://codeberg.org/forgejo-contrib/forgejo-cli). This fork contains modifications — see git history for details.
