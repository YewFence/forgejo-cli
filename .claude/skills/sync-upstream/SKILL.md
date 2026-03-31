---
description: Sync forgejo-cli-plus with upstream forgejo-cli. Cherry-pick only -- never merge wholesale.
---

# Sync with Upstream

You are syncing `forgejo-cli-plus` (our Codeberg fork) with upstream `forgejo-cli`.

## Setup

Upstream remote should already exist:
```
upstream  https://codeberg.org/forgejo-contrib/forgejo-cli.git
```

If missing: `git remote add upstream https://codeberg.org/forgejo-contrib/forgejo-cli.git`

## Process

### 1. Create a sync branch

```sh
git checkout main
git checkout -b upstream-sync/<YYYY-MM-DD>
```

### 2. Fetch and identify new commits

```sh
git fetch upstream
```

Find the last sync tag:
```sh
git tag -l 'upstream-reviewed/*' --sort=-creatordate | head -1
```

List commits since last sync:
```sh
git log --oneline --reverse upstream/main --not <last-sync-tag>
```

### 3. Triage every commit

Categorize each commit into one of:

**Cherry-pick** -- the default. Use `git cherry-pick <sha>` and resolve conflicts if they arise:
- Bugfixes (auth, PKCE, logic errors)
- New features (editor support, new commands)
- Dependency version bumps (apply to Cargo.toml; Cargo.lock is regenerated later)

**Skip** -- changes that don't apply to our fork:
- Anti-AI/refusal string comments (obvious skip)
- Issue/PR templates (we maintain our own)
- Lock file maintenance (cargo will regenerate)
- CI configs for their infrastructure (Woodpecker, Renovate)
- Changes that remove our fork's features

### 4. Present the triage to the user

Before touching any code, show:
- Full list of new upstream commits with SHAs
- Your categorization (cherry-pick / skip) with reasoning
- For each cherry-pick candidate, note if conflicts are expected
- Links to upstream PRs/commits where available

Wait for user approval before proceeding.

### 5. Cherry-pick commits

Always use `git cherry-pick <sha>`. This preserves the original author, commit message, and history. Never manually recreate changes -- the original commits should show in our history.

When conflicts occur:
1. Read the conflict markers
2. Resolve by keeping our fork's additions (--yes, --json, --verbose, output module, milestone module, tests) while incorporating the upstream change
3. For Cargo.lock conflicts, just `git checkout --ours Cargo.lock` since we regenerate it later
4. `git add <resolved files> && git cherry-pick --continue --no-edit`

### 6. Update tests for new upstream behavior

Cherry-picked changes introduce new logic that our tests need to cover. This is NOT about making old tests pass -- it's about verifying the new behavior works correctly in our fork.

For each cherry-picked change, ask: "What new behavior did this introduce, and do our tests exercise it?"

Examples:
- Editor rework adds git-config lookup and $EDITOR fallback -- tests should verify both paths
- A new API endpoint needs wiremock mocks and assertions
- Changed error messages need updated assertion strings

Concrete steps:
- Update existing tests whose assertions no longer match the new code paths
- Add new test cases for new behavior introduced by cherry-picks
- Update snapshot tests if help text or output changed: `cargo insta test --accept`
- Run `cargo insta review` to interactively verify any snapshot changes

### 7. Regenerate Cargo.lock, build, and test

Regenerate the lock file to pick up dependency changes:
```sh
cargo generate-lockfile
```

If the lock file changed, commit it:
```sh
git add Cargo.lock
git commit -m "chore: regenerate Cargo.lock after upstream sync"
```

Build and run the full test suite:
```sh
cargo build
cargo test --all-targets
```

All tests must pass before proceeding.

### 8. Update the sync tag

```sh
git tag upstream-reviewed/<YYYY-MM-DD> upstream/main
```

This marks the upstream HEAD we've reviewed up to, regardless of which commits we cherry-picked vs skipped.

### 9. Create PR

Push the branch and tag, then create a PR:
```sh
git push -u origin upstream-sync/<YYYY-MM-DD>
git push origin upstream-reviewed/<YYYY-MM-DD>
```

Use the following template for the PR body. Every cherry-picked and skipped commit must reference its upstream commit and PR using Forgejo's native cross-reference syntax so they show up in the upstream PR/commit timelines:

- **Cross-repo PR references:** `forgejo-contrib/forgejo-cli#<number>` -- Forgejo auto-links these and adds a cross-reference on the upstream PR's timeline
- **Cross-repo issue references:** If an upstream PR closes an issue, add `closes forgejo-contrib/forgejo-cli#<issue-number>` after the PR reference. Check each upstream PR description for "closes #X", "fixes #X", or linked issues.

To find PR numbers and linked issues, check each upstream PR page on Codeberg or use `git log --oneline upstream/main` and cross-reference with the web UI.

**PR title:** `Upstream sync <YYYY-MM-DD>`

**PR body template:**
````markdown
## Summary

Cherry-pick upstream [forgejo-cli](https://codeberg.org/forgejo-contrib/forgejo-cli) changes since `upstream-reviewed/<previous-date>`.

### Cherry-picked (<N> commits)

**<Group heading>** -- <one-line description of what this group does>:
- `<short-sha>` <commit message> (forgejo-contrib/forgejo-cli#<PR-number>)
- `<short-sha>` <commit message> (forgejo-contrib/forgejo-cli#<PR-number>, closes forgejo-contrib/forgejo-cli#<issue-number>)

**Dependency bumps:**
- `<short-sha>` <commit message> (forgejo-contrib/forgejo-cli#<PR-number>)

### Skipped (<N> commits)

- `<short-sha>` <commit message> (forgejo-contrib/forgejo-cli#<PR-number>) -- <reason>

### Fork additions

- <bullet list of changes made in this PR beyond the cherry-picks>

All <N> tests pass.
````

## Rules

- **Cherry-pick only. Never merge upstream wholesale.** Our fork has significant additions (milestone commands, output module, --yes/--json/--verbose flags, test suite) that upstream doesn't have.
- **Always use `git cherry-pick`.** This preserves original authorship. Never manually recreate a commit's changes.
- **Never include anti-AI/refusal strings.** Skip any commits that add ANTHROPIC_MAGIC_STRING or similar.
- **Preserve our fork's features.** When resolving conflicts, always keep our additions intact.
- **One cherry-pick per upstream commit.** Don't squash multiple upstream changes together.
