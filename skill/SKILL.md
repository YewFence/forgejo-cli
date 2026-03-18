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
fj auth list                     # Show logged-in instances
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

# Comment
fj pr comment 10 "LGTM"

# Checkout locally
fj pr checkout 10
```

### Repositories

```bash
fj repo create myrepo -d "Description" -p   # Private
fj repo fork owner/repo --name my-fork
fj --json repo view                          # Current repo info
fj repo readme                               # View README
fj repo browse                               # Open in browser
fj repo clone owner/repo
fj --yes repo star
fj --yes repo delete owner/repo --force      # Destructive

# Labels
fj --json repo labels view
fj repo labels create "bug" "#d73a4a" -d "Something isn't working"
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

# Attachments
fj release asset create v1.0.0 ./binary
fj release asset download v1.0.0 binary -o ./downloaded

# Tags
fj --json tag list
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
fj --json org view myorg
fj org create myorg -d "Description" -v public
fj --json org members myorg
fj org activity myorg

# Teams
fj --json org team list myorg
fj org team create myorg devs -d "Developers" -w code,issue
fj org team member add myorg devs username
fj org team repo add myorg devs reponame
fj --yes org team delete myorg devs --force

# Org labels
fj --json org label list myorg
fj org label add myorg "priority" "#ff0000"
```

### Users

```bash
fj --json user view                          # Self
fj --json user view someuser
fj --json user repos --starred
fj --json user search "query"
fj --yes user follow someuser
fj user activity someuser

# Profile editing
fj user edit bio "I write code"
fj user edit name "Display Name"

# SSH keys
fj --json user key list
fj user key upload ~/.ssh/id_ed25519.pub
fj --yes user key delete 42 --force

# GPG keys
fj --json user gpg list
fj user gpg upload "$(cat key.asc)"
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
fj wiki view "Home"                          # View page
fj wiki clone                                # Clone wiki repo
```

### Shell Completions

```bash
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
