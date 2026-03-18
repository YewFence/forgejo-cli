---
name: forgejo-cli
description: Use when interacting with Forgejo or Gitea instances - creating issues, pull requests, managing repos, releases, milestones, orgs, users, CI/CD actions, or any git forge operation via the fj CLI
---

# forgejo-cli-plus (`fj`)

CLI for Forgejo/Gitea instances. Binary name: `fj`.

## Agentic Usage

Always use these global flags for non-interactive automation:

```bash
fj --yes --json <command>        # Skip prompts + machine-readable output
fj --yes --verbose <command>     # Skip prompts + debug API calls to stderr
```

| Global Flag | Effect |
|-------------|--------|
| `--yes, -y` | Skip all confirmation prompts |
| `--json` | JSON output (list/view commands) |
| `--verbose, -v` | Print API calls to stderr |
| `-H, --host <url>` | Target instance (overrides git remote detection) |

Destructive commands also accept:
- `--force, -f` - Skip per-command confirmation
- `--dry-run` - Preview without executing

## Authentication

```bash
fj auth login                    # OAuth browser flow
fj auth add-key <user> <token>   # Direct token (stdin if omitted)
fj auth logout <host>            # Log out from instance
fj auth list                     # Show logged-in instances
fj auth use-ssh [true|false]     # Toggle SSH as default for current instance
fj whoami                        # Current user@instance
```

## Common Workflows

### Issues

```bash
# Create
fj issue create "Title" --body "Description" -r owner/repo
fj issue create --template bug              # Use issue template

# Search/list
fj --json issue search -r owner/repo -s open
fj --json issue search -r owner/repo -l "bug,priority"
fj --json issue search -r owner/repo -a username -s all

# View
fj --json issue view 42
fj issue view 42 comments                  # All comments
fj issue view 42 comment 3                 # Specific comment

# Edit
fj issue edit 42 title "New title"
fj issue edit 42 body "New body"
fj issue edit 42 labels -a bug -a urgent -r wontfix
fj issue edit 42 assignees -a user1 -r user2

# State
fj --yes issue close 42
fj --yes issue close 42 -w "Fixed in abc123"
fj --yes issue reopen 42

# Comment
fj issue comment 42 "Comment body"
fj issue comment 42 --body-file notes.md

# Templates
fj issue templates -r owner/repo             # List available issue templates

# Browse
fj issue browse 42                           # Open in browser
```

### Pull Requests

```bash
# Create (from current branch)
fj pr create "Title" --body "Description"
fj pr create -A                             # Autofill from commits
fj pr create --base main --head feature -r owner/repo

# Search/list
fj --json pr search -r owner/repo -s open
fj --json pr search -r owner/repo -l "needs-review"

# View
fj --json pr view 10
fj pr view 10 diff                          # View diff
fj pr view 10 files                         # Changed files
fj pr view 10 commits                       # Commit list
fj pr view 10 comments                      # All comments

# CI Status
fj --json pr status 10
fj pr status 10 --wait                      # Block until CI finishes

# Edit
fj pr edit 10 title "New title"
fj pr edit 10 labels -a approved -r draft

# Merge
fj --yes pr merge 10 -M merge --delete      # Merge + delete branch
fj --yes pr merge 10 -M squash
fj --yes pr merge 10 -M rebase

# State
fj --yes pr close 10
fj --yes pr close 10 -w "Superseded by #11"  # Close with comment
fj --yes pr reopen 10
fj --yes pr reopen 10 -w "Reopening per review"

# Comment
fj pr comment 10 "LGTM"
fj pr comment 10 --body-file review.md

# Checkout locally
fj pr checkout 10
fj pr checkout 10 --branch-name my-branch    # Custom local branch name

# Browse
fj pr browse 10                              # Open in browser
```

### Repositories

```bash
fj repo create myrepo -d "Description" -p   # Private
fj repo fork owner/repo --name my-fork
fj repo migrate https://github.com/user/repo myrepo  # Mirror from other forges
fj repo migrate https://github.com/user/repo myrepo -m  # As mirror (auto-sync)
fj --json repo view                          # Current repo info
fj repo readme                               # View README
fj repo browse                               # Open in browser
fj repo clone owner/repo
fj repo clone owner/repo -S                  # Clone via SSH
fj --yes repo star
fj --yes repo unstar
fj --yes repo delete owner/repo --force      # Destructive

# Labels
fj --json repo labels view
fj repo labels view --archived               # Include archived labels
fj repo labels create "bug" "#d73a4a" -d "Something isn't working"
fj repo labels create "scope/api" "#0e8a16" -e  # Exclusive (scoped) label
fj repo labels edit 5 -n "renamed" -c "#ff0000"
fj repo labels delete 5 --force
```

### Releases & Tags

```bash
# Releases
fj --json release list
fj --json release view v1.0.0
fj release create v1.0.0 -T v1.0.0 -b "Release notes" --attach dist/app.tar.gz
fj release edit v1.0.0 --body "Updated notes"
fj --yes release delete v1.0.0 --force
fj release browse v1.0.0                    # Open in browser

# Attachments
fj release asset create v1.0.0 ./binary
fj release asset download v1.0.0 binary -o ./downloaded
fj --yes release asset delete v1.0.0 binary --force

# Tags
fj --json tag list
fj --json tag view v1.0.0
fj tag create v1.0.0 -B main
fj --yes tag delete v1.0.0 --force
```

### Milestones

```bash
fj --json milestone list
fj --json milestone list -s all             # Include closed
fj --json milestone view "Sprint 1"
fj milestone create "Sprint 2" --body "Goals" -d 2025-06-01T00:00:00Z
fj milestone edit "Sprint 1" --state closed
fj --yes milestone delete "Sprint 1" --force
```

### Organizations

```bash
fj --json org list
fj --json org list -m                        # Only orgs you belong to
fj --json org view myorg
fj org create myorg -d "Description" -v public
fj org edit myorg -d "Updated description"
fj --json org members myorg
fj org activity myorg
fj org visibility myorg                      # View membership visibility
fj org visibility myorg -s public            # Set membership visibility

# Org repos
fj --json org repo list myorg
fj org repo create myorg newrepo -d "Description"

# Teams
fj --json org team list myorg
fj --json org team view myorg devs
fj org team view myorg devs -p               # Show permissions
fj org team create myorg devs -d "Developers" -w code,issue
fj org team edit myorg devs --new-name "developers" -d "Updated"
fj org team member list myorg devs
fj org team member add myorg devs username
fj --yes org team member rm myorg devs username --force
fj org team repo list myorg devs
fj org team repo add myorg devs reponame
fj --yes org team repo rm myorg devs reponame --force
fj --yes org team delete myorg devs --force

# Org labels
fj --json org label list myorg
fj org label add myorg "priority" "#ff0000" -d "High priority" -e
fj org label edit myorg "priority" --new-name "urgent" -c "#cc0000"
fj --yes org label rm myorg "priority" --force
```

### Users

```bash
fj --json user view                          # Self
fj --json user view someuser
fj --json user repos                         # Own repos
fj --json user repos --starred               # Starred repos
fj --json user repos someuser                # Another user's repos
fj --json user search "query"
fj --json user orgs                          # Own org memberships
fj --json user following                     # Who you follow
fj --json user followers                     # Who follows you
fj --yes user follow someuser
fj --yes user unfollow someuser
fj --yes user block someuser
fj --yes user unblock someuser
fj user activity someuser
fj user browse someuser                      # Open profile in browser

# Profile editing
fj user edit bio "I write code"
fj user edit name "Display Name"
fj user edit name -u                         # Unset display name
fj user edit pronouns "they/them"
fj user edit location "Earth"
fj user edit website "https://example.com"
fj user edit activity -v public              # Set activity visibility
fj user edit email -a new@example.com -r old@example.com

# SSH keys
fj --json user key list
fj user key list -v                          # Verbose (detailed info)
fj --json user key view 42
fj user key upload ~/.ssh/id_ed25519.pub -t "My key"
fj user key upload ~/.ssh/id_ed25519.pub -r  # Read-only deploy key
fj --yes user key delete 42 --force

# GPG keys
fj --json user gpg list
fj --json user gpg view 42
fj user gpg upload "$(cat key.asc)"
fj user gpg verify 42
fj --yes user gpg delete 42 --force
```

### CI/CD Actions

```bash
fj --json actions tasks                      # List workflow runs
fj actions dispatch workflow.yml main        # Trigger workflow
fj actions dispatch deploy.yml main -I env=prod -I version=1.0

# Variables
fj --json actions variables list
fj actions variables create MY_VAR "value"
fj --yes actions variables delete MY_VAR --force

# Secrets
fj --json actions secrets list
fj actions secrets create MY_SECRET "s3cret"
fj --yes actions secrets delete MY_SECRET --force
```

### Wiki

```bash
fj wiki contents                             # List pages
fj wiki view "Home"                          # View page (rendered markdown)
fj wiki clone                                # Clone wiki repo
fj wiki clone -S                             # Clone via SSH
fj wiki browse "Home"                        # Open page in browser
```

### Other

```bash
fj version                                   # Show version
fj version -v                                # Verbose build info
fj completion bash > ~/.bash_completion.d/fj
fj completion zsh > ~/.zfunc/_fj
fj completion fish > ~/.config/fish/completions/fj.fish
fj completion powershell > fj.ps1
```

## Name/ID Resolution

Many commands accept either numeric IDs or names. The CLI tries numeric ID first, then falls back to name search:

- **Milestones**: by ID or title
- **Labels**: by ID or name
- **Releases**: by ID or name (also `--by-tag` for tag-based lookup)
- **Repos**: `owner/repo` format, or `-r owner/repo`, or inferred from git remote
- **Teams/users**: by name

## Repo Context

When run inside a git repo, `fj` auto-detects the Forgejo instance and repo from git remotes. Use `-R, --remote` to pick a specific remote, or `-r, --repo owner/repo` for cross-repo operations, or `-H, --host` to target a specific instance.

## Output Parsing

With `--json`, list commands return JSON arrays and view commands return JSON objects. Pipe through `jq` for field extraction:

```bash
fj --json issue search -s open | jq '.[].number'
fj --json pr view 10 | jq '.mergeable'
```
