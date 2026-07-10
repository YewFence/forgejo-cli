# Command-Line Help for `fj`

This document contains the help content for the `fj` command-line program.

**Command Overview:**

* [`fj`↴](#fj)
* [`fj repo`↴](#fj-repo)
* [`fj repo create`↴](#fj-repo-create)
* [`fj repo fork`↴](#fj-repo-fork)
* [`fj repo migrate`↴](#fj-repo-migrate)
* [`fj repo view`↴](#fj-repo-view)
* [`fj repo readme`↴](#fj-repo-readme)
* [`fj repo clone`↴](#fj-repo-clone)
* [`fj repo star`↴](#fj-repo-star)
* [`fj repo unstar`↴](#fj-repo-unstar)
* [`fj repo delete`↴](#fj-repo-delete)
* [`fj repo browse`↴](#fj-repo-browse)
* [`fj repo labels`↴](#fj-repo-labels)
* [`fj repo labels list`↴](#fj-repo-labels-list)
* [`fj repo labels create`↴](#fj-repo-labels-create)
* [`fj repo labels delete`↴](#fj-repo-labels-delete)
* [`fj repo labels edit`↴](#fj-repo-labels-edit)
* [`fj repo edit`↴](#fj-repo-edit)
* [`fj repo units`↴](#fj-repo-units)
* [`fj repo units issues`↴](#fj-repo-units-issues)
* [`fj repo units prs`↴](#fj-repo-units-prs)
* [`fj repo units actions`↴](#fj-repo-units-actions)
* [`fj repo units wiki`↴](#fj-repo-units-wiki)
* [`fj repo units packages`↴](#fj-repo-units-packages)
* [`fj repo units projects`↴](#fj-repo-units-projects)
* [`fj repo units releases`↴](#fj-repo-units-releases)
* [`fj issue`↴](#fj-issue)
* [`fj issue create`↴](#fj-issue-create)
* [`fj issue edit`↴](#fj-issue-edit)
* [`fj issue edit title`↴](#fj-issue-edit-title)
* [`fj issue edit body`↴](#fj-issue-edit-body)
* [`fj issue edit comment`↴](#fj-issue-edit-comment)
* [`fj issue edit labels`↴](#fj-issue-edit-labels)
* [`fj issue edit assignees`↴](#fj-issue-edit-assignees)
* [`fj issue comment`↴](#fj-issue-comment)
* [`fj issue close`↴](#fj-issue-close)
* [`fj issue reopen`↴](#fj-issue-reopen)
* [`fj issue assign`↴](#fj-issue-assign)
* [`fj issue unassign`↴](#fj-issue-unassign)
* [`fj issue search`↴](#fj-issue-search)
* [`fj issue view`↴](#fj-issue-view)
* [`fj issue view body`↴](#fj-issue-view-body)
* [`fj issue view comment`↴](#fj-issue-view-comment)
* [`fj issue view comments`↴](#fj-issue-view-comments)
* [`fj issue templates`↴](#fj-issue-templates)
* [`fj issue browse`↴](#fj-issue-browse)
* [`fj pr`↴](#fj-pr)
* [`fj pr search`↴](#fj-pr-search)
* [`fj pr create`↴](#fj-pr-create)
* [`fj pr view`↴](#fj-pr-view)
* [`fj pr view body`↴](#fj-pr-view-body)
* [`fj pr view comment`↴](#fj-pr-view-comment)
* [`fj pr view comments`↴](#fj-pr-view-comments)
* [`fj pr view labels`↴](#fj-pr-view-labels)
* [`fj pr view diff`↴](#fj-pr-view-diff)
* [`fj pr view files`↴](#fj-pr-view-files)
* [`fj pr view commits`↴](#fj-pr-view-commits)
* [`fj pr status`↴](#fj-pr-status)
* [`fj pr checkout`↴](#fj-pr-checkout)
* [`fj pr comment`↴](#fj-pr-comment)
* [`fj pr edit`↴](#fj-pr-edit)
* [`fj pr edit title`↴](#fj-pr-edit-title)
* [`fj pr edit body`↴](#fj-pr-edit-body)
* [`fj pr edit comment`↴](#fj-pr-edit-comment)
* [`fj pr edit labels`↴](#fj-pr-edit-labels)
* [`fj pr edit assignees`↴](#fj-pr-edit-assignees)
* [`fj pr close`↴](#fj-pr-close)
* [`fj pr reopen`↴](#fj-pr-reopen)
* [`fj pr merge`↴](#fj-pr-merge)
* [`fj pr browse`↴](#fj-pr-browse)
* [`fj pr review`↴](#fj-pr-review)
* [`fj pr review list`↴](#fj-pr-review-list)
* [`fj pr assign`↴](#fj-pr-assign)
* [`fj pr unassign`↴](#fj-pr-unassign)
* [`fj wiki`↴](#fj-wiki)
* [`fj wiki contents`↴](#fj-wiki-contents)
* [`fj wiki view`↴](#fj-wiki-view)
* [`fj wiki clone`↴](#fj-wiki-clone)
* [`fj wiki browse`↴](#fj-wiki-browse)
* [`fj actions`↴](#fj-actions)
* [`fj actions tasks`↴](#fj-actions-tasks)
* [`fj actions run`↴](#fj-actions-run)
* [`fj actions run list`↴](#fj-actions-run-list)
* [`fj actions run view`↴](#fj-actions-run-view)
* [`fj actions run jobs`↴](#fj-actions-run-jobs)
* [`fj actions run logs`↴](#fj-actions-run-logs)
* [`fj actions run cancel`↴](#fj-actions-run-cancel)
* [`fj actions run delete`↴](#fj-actions-run-delete)
* [`fj actions artifact`↴](#fj-actions-artifact)
* [`fj actions artifact list`↴](#fj-actions-artifact-list)
* [`fj actions artifact download`↴](#fj-actions-artifact-download)
* [`fj actions artifact delete`↴](#fj-actions-artifact-delete)
* [`fj actions variables`↴](#fj-actions-variables)
* [`fj actions variables list`↴](#fj-actions-variables-list)
* [`fj actions variables create`↴](#fj-actions-variables-create)
* [`fj actions variables delete`↴](#fj-actions-variables-delete)
* [`fj actions secrets`↴](#fj-actions-secrets)
* [`fj actions secrets list`↴](#fj-actions-secrets-list)
* [`fj actions secrets create`↴](#fj-actions-secrets-create)
* [`fj actions secrets delete`↴](#fj-actions-secrets-delete)
* [`fj actions dispatch`↴](#fj-actions-dispatch)
* [`fj whoami`↴](#fj-whoami)
* [`fj auth`↴](#fj-auth)
* [`fj auth login`↴](#fj-auth-login)
* [`fj auth logout`↴](#fj-auth-logout)
* [`fj auth add-key`↴](#fj-auth-add-key)
* [`fj auth use-ssh`↴](#fj-auth-use-ssh)
* [`fj auth list`↴](#fj-auth-list)
* [`fj release`↴](#fj-release)
* [`fj release create`↴](#fj-release-create)
* [`fj release edit`↴](#fj-release-edit)
* [`fj release delete`↴](#fj-release-delete)
* [`fj release list`↴](#fj-release-list)
* [`fj release view`↴](#fj-release-view)
* [`fj release browse`↴](#fj-release-browse)
* [`fj release asset`↴](#fj-release-asset)
* [`fj release asset create`↴](#fj-release-asset-create)
* [`fj release asset delete`↴](#fj-release-asset-delete)
* [`fj release asset download`↴](#fj-release-asset-download)
* [`fj milestone`↴](#fj-milestone)
* [`fj milestone list`↴](#fj-milestone-list)
* [`fj milestone view`↴](#fj-milestone-view)
* [`fj milestone create`↴](#fj-milestone-create)
* [`fj milestone edit`↴](#fj-milestone-edit)
* [`fj milestone delete`↴](#fj-milestone-delete)
* [`fj tag`↴](#fj-tag)
* [`fj tag create`↴](#fj-tag-create)
* [`fj tag delete`↴](#fj-tag-delete)
* [`fj tag list`↴](#fj-tag-list)
* [`fj tag view`↴](#fj-tag-view)
* [`fj user`↴](#fj-user)
* [`fj user search`↴](#fj-user-search)
* [`fj user view`↴](#fj-user-view)
* [`fj user browse`↴](#fj-user-browse)
* [`fj user follow`↴](#fj-user-follow)
* [`fj user unfollow`↴](#fj-user-unfollow)
* [`fj user following`↴](#fj-user-following)
* [`fj user followers`↴](#fj-user-followers)
* [`fj user block`↴](#fj-user-block)
* [`fj user unblock`↴](#fj-user-unblock)
* [`fj user repos`↴](#fj-user-repos)
* [`fj user orgs`↴](#fj-user-orgs)
* [`fj user activity`↴](#fj-user-activity)
* [`fj user edit`↴](#fj-user-edit)
* [`fj user edit bio`↴](#fj-user-edit-bio)
* [`fj user edit name`↴](#fj-user-edit-name)
* [`fj user edit pronouns`↴](#fj-user-edit-pronouns)
* [`fj user edit location`↴](#fj-user-edit-location)
* [`fj user edit activity`↴](#fj-user-edit-activity)
* [`fj user edit email`↴](#fj-user-edit-email)
* [`fj user edit website`↴](#fj-user-edit-website)
* [`fj user key`↴](#fj-user-key)
* [`fj user key list`↴](#fj-user-key-list)
* [`fj user key view`↴](#fj-user-key-view)
* [`fj user key delete`↴](#fj-user-key-delete)
* [`fj user key upload`↴](#fj-user-key-upload)
* [`fj user gpg`↴](#fj-user-gpg)
* [`fj user gpg list`↴](#fj-user-gpg-list)
* [`fj user gpg view`↴](#fj-user-gpg-view)
* [`fj user gpg delete`↴](#fj-user-gpg-delete)
* [`fj user gpg upload`↴](#fj-user-gpg-upload)
* [`fj user gpg verify`↴](#fj-user-gpg-verify)
* [`fj org`↴](#fj-org)
* [`fj org list`↴](#fj-org-list)
* [`fj org view`↴](#fj-org-view)
* [`fj org create`↴](#fj-org-create)
* [`fj org edit`↴](#fj-org-edit)
* [`fj org activity`↴](#fj-org-activity)
* [`fj org members`↴](#fj-org-members)
* [`fj org visibility`↴](#fj-org-visibility)
* [`fj org team`↴](#fj-org-team)
* [`fj org team list`↴](#fj-org-team-list)
* [`fj org team view`↴](#fj-org-team-view)
* [`fj org team create`↴](#fj-org-team-create)
* [`fj org team edit`↴](#fj-org-team-edit)
* [`fj org team delete`↴](#fj-org-team-delete)
* [`fj org team repo`↴](#fj-org-team-repo)
* [`fj org team repo list`↴](#fj-org-team-repo-list)
* [`fj org team repo add`↴](#fj-org-team-repo-add)
* [`fj org team repo rm`↴](#fj-org-team-repo-rm)
* [`fj org team member`↴](#fj-org-team-member)
* [`fj org team member list`↴](#fj-org-team-member-list)
* [`fj org team member add`↴](#fj-org-team-member-add)
* [`fj org team member rm`↴](#fj-org-team-member-rm)
* [`fj org label`↴](#fj-org-label)
* [`fj org label list`↴](#fj-org-label-list)
* [`fj org label add`↴](#fj-org-label-add)
* [`fj org label edit`↴](#fj-org-label-edit)
* [`fj org label rm`↴](#fj-org-label-rm)
* [`fj org repo`↴](#fj-org-repo)
* [`fj org repo list`↴](#fj-org-repo-list)
* [`fj org repo create`↴](#fj-org-repo-create)
* [`fj version`↴](#fj-version)
* [`fj completion`↴](#fj-completion)

## `fj`

**Usage:** `fj [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `repo`
* `issue`
* `pr`
* `wiki`
* `actions`
* `whoami`
* `auth`
* `release`
* `milestone`
* `tag`
* `user`
* `org`
* `version`
* `completion`

###### **Options:**

* `--config <CONFIG>` — Directory containing fj configuration and credentials
* `-H`, `--host <HOST>` — Forgejo instance host or URL to use instead of auto-detection
* `--token <TOKEN>` — Forgejo API token to use instead of stored credentials
* `--style <STYLE>`

  Possible values:
  - `fancy`:
    Use special characters, and colors
  - `minimal`:
    No special characters and no colors. Always used in non-terminal contexts (i.e. pipes)

* `--json` — Output results as JSON (for scripting and agents)
* `-y`, `--yes` — Skip all confirmation prompts (auto-confirm destructive actions)
* `-v`, `--verbose` — Show verbose output (API calls, resolution steps)



## `fj repo`

**Usage:** `fj repo <COMMAND>`

###### **Subcommands:**

* `create` — Creates a new repository
* `fork` — Fork a repository onto your account
* `migrate` — Migrate or mirror an existing repository
* `view` — View a repo's info
* `readme` — View a repo's README
* `clone` — Clone a repo's code locally
* `star` — Add a star to a repo
* `unstar` — Take away a star from a repo
* `delete` — Delete a repository
* `browse` — Open a repository's page in your browser
* `labels` — Manage a repo's issue labels
* `edit` — Edit a repository's properties
* `units` — Manage a repo's units



## `fj repo create`

Creates a new repository

**Usage:** `fj repo create [OPTIONS] <REPO>`

###### **Arguments:**

* `<REPO>` — Repository name, or org/name to create under an organization

###### **Options:**

* `-d`, `--description <DESCRIPTION>`
* `-P`, `--private`
* `-r`, `--remote <REMOTE>` — Creates a new remote with the given name for the new repo
* `-p`, `--push` — Pushes the current branch to the default branch on the new repo. Implies `--remote=origin` (setting remote manually overrides this)
* `-S`, `--ssh <SSH>` — Use SSH for the new remote instead of HTTP(S)

  Possible values: `true`, `false`




## `fj repo fork`

Fork a repository onto your account

**Usage:** `fj repo fork [OPTIONS] <REPO>`

###### **Arguments:**

* `<REPO>`

###### **Options:**

* `--name <NAME>`
* `-R`, `--remote <REMOTE>`



## `fj repo migrate`

Migrate or mirror an existing repository

**Usage:** `fj repo migrate [OPTIONS] <REPO> <[OWNER]/NAME>`

###### **Arguments:**

* `<REPO>` — URL of the repo to migrate
* `<[OWNER]/NAME>` — Name of the new mirror, and optionally which org/user should own it

###### **Options:**

* `-m`, `--mirror` — Whether to mirror the repo instead of migrating it
* `-p`, `--private` — Whether the new migration should be private
* `-i`, `--include <INCLUDE>` — Comma-separated list of items to include. Defaults to nothing but git data.

   These are `lfs`, `wiki`, `issues`, `prs`, `milestones`, `labels`, and `releases`. You can use `all` to include everything.
* `-L`, `--lfs-endpoint <LFS_ENDPOINT>` — The URL to fetch LFS files from
* `-s`, `--service <SERVICE>` — The type of Git service the original repo is on. Defaults to `git`

  Possible values: `git`, `github`, `gitlab`, `forgejo`, `gitea`, `gogs`, `onedev`, `gitbucket`, `codebase`

* `-t`, `--source-token` — If enabled, will read an access token in from stdin to use for fetching the source repo.

   Mutually exclusive with `--login`
* `-l`, `--login` — If enabled, will read a username and password from stdin to use for fetching.

   Mutually exclusive with `--source-token`.

   This is not recommended, `--source-token` should be used instead whenever possible.



## `fj repo view`

View a repo's info

**Usage:** `fj repo view [OPTIONS] [NAME]`

###### **Arguments:**

* `<NAME>`

###### **Options:**

* `-R`, `--remote <REMOTE>`



## `fj repo readme`

View a repo's README

**Usage:** `fj repo readme [OPTIONS] [NAME]`

###### **Arguments:**

* `<NAME>`

###### **Options:**

* `-R`, `--remote <REMOTE>`



## `fj repo clone`

Clone a repo's code locally

**Usage:** `fj repo clone [OPTIONS] <REPO> [PATH]`

###### **Arguments:**

* `<REPO>`
* `<PATH>`

###### **Options:**

* `-S`, `--ssh <SSH>` — Clone the repo over SSH instead of HTTP(S)

  Possible values: `true`, `false`

* `-I`, `--identity-file <IDENTITY_FILE>` — An SSH key file to use when cloning over SSH



## `fj repo star`

Add a star to a repo

**Usage:** `fj repo star [OPTIONS] [REPO]`

###### **Arguments:**

* `<REPO>`

###### **Options:**

* `-R`, `--remote <REMOTE>`



## `fj repo unstar`

Take away a star from a repo

**Usage:** `fj repo unstar [OPTIONS] [REPO]`

###### **Arguments:**

* `<REPO>`

###### **Options:**

* `-R`, `--remote <REMOTE>`



## `fj repo delete`

Delete a repository

This cannot be undone!

**Usage:** `fj repo delete [OPTIONS] <REPO>`

###### **Arguments:**

* `<REPO>`

###### **Options:**

* `-f`, `--force` — Skip confirmation prompt
* `--dry-run` — Preview without executing



## `fj repo browse`

Open a repository's page in your browser

**Usage:** `fj repo browse [OPTIONS] [NAME]`

###### **Arguments:**

* `<NAME>`

###### **Options:**

* `-R`, `--remote <REMOTE>`



## `fj repo labels`

Manage a repo's issue labels

**Usage:** `fj repo labels [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `list` — List a repo's labels
* `create` — Create a new label
* `delete` — Delete a label
* `edit` — Edit a label

###### **Options:**

* `-r`, `--repo <REPO>` — The repo whose labels to manage



## `fj repo labels list`

List a repo's labels

**Usage:** `fj repo labels list [OPTIONS]`

###### **Options:**

* `-a`, `--archived` — Show archived labels



## `fj repo labels create`

Create a new label

**Usage:** `fj repo labels create [OPTIONS] <NAME> <COLOR>`

###### **Arguments:**

* `<NAME>` — Name of the new label. You may include a '/' here to namespace the label
* `<COLOR>` — Color of the new label in hexadecimal format

###### **Options:**

* `-d`, `--description <DESCRIPTION>` — A description for the new label. If no argument is given, open the editor
* `-e`, `--exclusive` — Make this label exclusive with other labels in the same namespace
* `-a`, `--archived` — Create an archived label



## `fj repo labels delete`

Delete a label

**Usage:** `fj repo labels delete [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — The ID or name of the label to delete

###### **Options:**

* `-f`, `--force` — Skip confirmation prompt
* `--dry-run` — Preview without executing



## `fj repo labels edit`

Edit a label

**Usage:** `fj repo labels edit [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — The ID or name of the label to edit

###### **Options:**

* `-n`, `--name <NAME>` — New name for the label
* `-c`, `--color <COLOR>` — New color for the label
* `-d`, `--description <DESCRIPTION>` — New description for the label. If no argument is given, open the editor
* `-e`, `--exclusive <EXCLUSIVE>` — New exclusive status

  Possible values: `true`, `false`

* `-a`, `--archived <ARCHIVED>` — New archived status

  Possible values: `true`, `false`




## `fj repo edit`

Edit a repository's properties

**Usage:** `fj repo edit [OPTIONS]`

###### **Options:**

* `-r`, `--repo <REPO>` — The repo to edit
* `-a`, `--archived <ARCHIVED>` — Archive or unarchive

  Possible values: `true`, `false`

* `--default-branch <DEFAULT_BRANCH>` — Set the default branch
* `-d`, `--description <DESCRIPTION>` — Set the description
* `--enable-prune <ENABLE_PRUNE>` — Remove obsolete remote-tracking references when mirroring

  Possible values: `true`, `false`

* `--mirror-interval <MIRROR_INTERVAL>` — Set the interval for push mirrors. Use a string like 8h30m0s
* `--name <NAME>` — Set the repo's name
* `-p`, `--private <PRIVATE>` — Set this repository's private status

  Possible values: `true`, `false`

* `-t`, `--template <TEMPLATE>` — Set if this repository should be a template repository

  Possible values: `true`, `false`

* `-w`, `--website <WEBSITE>` — Set a URL for this repository's website



## `fj repo units`

Manage a repo's units

**Usage:** `fj repo units [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `issues` — Manage the issues unit
* `prs` — Manage the pull requests unit
* `actions` — Manage the actions unit
* `wiki` — Manage the wiki unit
* `packages` — Manage the packages unit
* `projects` — Manage the projects unit
* `releases` — Manage the releases unit

###### **Options:**

* `-r`, `--repo <REPO>` — The repo whose units to manage



## `fj repo units issues`

Manage the issues unit

**Usage:** `fj repo units issues [OPTIONS]`

###### **Options:**

* `-e`, `--enable <ENABLE>` — Enable or disable issues

  Possible values: `true`, `false`




## `fj repo units prs`

Manage the pull requests unit

**Usage:** `fj repo units prs [OPTIONS]`

###### **Options:**

* `-e`, `--enable <ENABLE>` — Enable or disable pull requests

  Possible values: `true`, `false`

* `--allow-fast-forward-only-merge <ALLOW_FAST_FORWARD_ONLY_MERGE>` — Allow fast-forward only merging

  Possible values: `true`, `false`

* `--allow-manual-merge <ALLOW_MANUAL_MERGE>` — Allow manual merging

  Possible values: `true`, `false`

* `--allow-merge-commits <ALLOW_MERGE_COMMITS>` — Allow merge commits

  Possible values: `true`, `false`

* `--allow-rebase <ALLOW_REBASE>` — Allow rebase merging

  Possible values: `true`, `false`

* `--allow-rebase-explicit <ALLOW_REBASE_EXPLICIT>` — Allow rebase merging with explicit merge commits

  Possible values: `true`, `false`

* `--allow-rebase-update <ALLOW_REBASE_UPDATE>` — Allow updating PR branches by rebase

  Possible values: `true`, `false`

* `--allow-squash-merge <ALLOW_SQUASH_MERGE>` — Allow squash merging

  Possible values: `true`, `false`

* `--autodetect-manual-merge <AUTODETECT_MANUAL_MERGE>` — Automatically detect manual merges

  Possible values: `true`, `false`

* `--default-allow-maintainer-edit <DEFAULT_ALLOW_MAINTAINER_EDIT>` — Allow maintainer edits by default

  Possible values: `true`, `false`

* `--default-delete-branch-after-merge <DEFAULT_DELETE_BRANCH_AFTER_MERGE>` — Delete branch after merge by default

  Possible values: `true`, `false`

* `--default-merge-style <DEFAULT_MERGE_STYLE>` — Default merge style

  Possible values: `merge`, `rebase`, `rebase-merge`, `squash`, `fast-forward-only`

* `--default-update-style <DEFAULT_UPDATE_STYLE>` — Default update style

  Possible values: `rebase`, `merge`

* `--ignore-whitespace-conflicts <IGNORE_WHITESPACE_CONFLICTS>` — Ignore whitespace merge conflicts

  Possible values: `true`, `false`




## `fj repo units actions`

Manage the actions unit

**Usage:** `fj repo units actions [OPTIONS]`

###### **Options:**

* `-e`, `--enable <ENABLE>` — Enable or disable actions

  Possible values: `true`, `false`




## `fj repo units wiki`

Manage the wiki unit

**Usage:** `fj repo units wiki [OPTIONS]`

###### **Options:**

* `-e`, `--enable <ENABLE>` — Enable or disable the wiki

  Possible values: `true`, `false`

* `--branch <BRANCH>` — Set the branch used for the wiki
* `--external-url <EXTERNAL_URL>` — Set the URL for an external wiki
* `--globally-editable <GLOBALLY_EDITABLE>` — Set the globally editable state of the wiki

  Possible values: `true`, `false`




## `fj repo units packages`

Manage the packages unit

**Usage:** `fj repo units packages [OPTIONS]`

###### **Options:**

* `-e`, `--enable <ENABLE>` — Enable or disable the package registry

  Possible values: `true`, `false`




## `fj repo units projects`

Manage the projects unit

**Usage:** `fj repo units projects [OPTIONS]`

###### **Options:**

* `-e`, `--enable <ENABLE>` — Enable or disable the project board

  Possible values: `true`, `false`




## `fj repo units releases`

Manage the releases unit

**Usage:** `fj repo units releases [OPTIONS]`

###### **Options:**

* `-e`, `--enable <ENABLE>` — Enable or disable the releases unit

  Possible values: `true`, `false`




## `fj issue`

**Usage:** `fj issue [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `create` — Create a new issue on a repo
* `edit` — Edit an issue
* `comment` — Add a comment on an issue
* `close` — Close an issue
* `reopen` — Reopen a closed issue
* `assign` — Assign users to an issue
* `unassign` — Unassign users from an issue
* `search` — Search for an issue in a repo
* `view` — View an issue's info
* `templates` — List the issue templates in a repo
* `browse` — Open an issue in your browser

###### **Options:**

* `-R`, `--remote <REMOTE>` — The local git remote that points to the repo to operate on



## `fj issue create`

Create a new issue on a repo

**Usage:** `fj issue create [OPTIONS] [TITLE]`

###### **Arguments:**

* `<TITLE>` — Title of the issue

###### **Options:**

* `--body <BODY>` — The text body of the issue

   Leaving this out will open your editor, unless --body-file is specified.
* `--body-file <BODY_FILE>` — The text body of the issue, to read from a file
* `--template <TEMPLATE>` — The template to use when creating an issue

   If the repo has disabled blank issues, this flag is required.
* `--no-template` — Don't use a template for this issue.

   If the repo has disabled blank issues, this will fail.
* `-M`, `--milestone <MILESTONE>` — Milestone to assign (name or numeric ID)
* `--assignee <ASSIGNEES>` — Assign users (repeatable, e.g. --assignee alice --assignee bob)
* `-r`, `--repo <REPO>` — The repo to create this issue on
* `--web` — Open the issue creation page in your web browser



## `fj issue edit`

Edit an issue

**Usage:** `fj issue edit [OPTIONS] <ISSUE> <COMMAND>`

###### **Subcommands:**

* `title` — Edit an issue's title
* `body` — Edit an issue's text content
* `comment` — Edit a comment on an issue
* `labels` — Edit an issue's labels
* `assignees` — Edit an issue's assignees

###### **Arguments:**

* `<ISSUE>`

###### **Options:**

* `-r`, `--repo <REPO>` — The repo to operate on (alternative to owner/repo#id syntax)



## `fj issue edit title`

Edit an issue's title

**Usage:** `fj issue edit title [NEW_TITLE]`

###### **Arguments:**

* `<NEW_TITLE>`



## `fj issue edit body`

Edit an issue's text content

**Usage:** `fj issue edit body [NEW_BODY]`

###### **Arguments:**

* `<NEW_BODY>`



## `fj issue edit comment`

Edit a comment on an issue

**Usage:** `fj issue edit comment <IDX> [NEW_BODY]`

###### **Arguments:**

* `<IDX>`
* `<NEW_BODY>`



## `fj issue edit labels`

Edit an issue's labels

**Usage:** `fj issue edit labels [OPTIONS]`

###### **Options:**

* `-a`, `--add <ADD>` — The labels to add
* `-r`, `--rm <RM>` — The labels to remove



## `fj issue edit assignees`

Edit an issue's assignees

**Usage:** `fj issue edit assignees [OPTIONS]`

###### **Options:**

* `-a`, `--add <ADD>` — Usernames to add as assignees
* `-r`, `--rm <RM>` — Usernames to remove from assignees



## `fj issue comment`

Add a comment on an issue

**Usage:** `fj issue comment [OPTIONS] <ISSUE> [BODY]`

###### **Arguments:**

* `<ISSUE>`
* `<BODY>` — The text content of the comment.

   Leaving this out will open your editor, unless --body-file is specified.

###### **Options:**

* `--body-file <BODY_FILE>` — The text content of the comment, to read from a file
* `-r`, `--repo <REPO>` — The repo to operate on (alternative to owner/repo#id syntax)



## `fj issue close`

Close an issue

**Usage:** `fj issue close [OPTIONS] <ISSUE>`

###### **Arguments:**

* `<ISSUE>`

###### **Options:**

* `-w`, `--with-msg <WITH_MSG>` — A comment to leave on the issue before closing it
* `-r`, `--repo <REPO>` — The repo to operate on (alternative to owner/repo#id syntax)



## `fj issue reopen`

Reopen a closed issue

**Usage:** `fj issue reopen [OPTIONS] <ISSUE>`

###### **Arguments:**

* `<ISSUE>`

###### **Options:**

* `-w`, `--with-msg <WITH_MSG>` — A comment to leave on the issue before reopening it
* `-r`, `--repo <REPO>` — The repo to operate on (alternative to owner/repo#id syntax)



## `fj issue assign`

Assign users to an issue

**Usage:** `fj issue assign [OPTIONS] <ISSUE> <USERS>...`

###### **Arguments:**

* `<ISSUE>`
* `<USERS>` — Usernames to assign

###### **Options:**

* `-r`, `--repo <REPO>` — The repo to operate on (alternative to owner/repo#id syntax)



## `fj issue unassign`

Unassign users from an issue

**Usage:** `fj issue unassign [OPTIONS] <ISSUE> <USERS>...`

###### **Arguments:**

* `<ISSUE>`
* `<USERS>` — Usernames to unassign

###### **Options:**

* `-r`, `--repo <REPO>` — The repo to operate on (alternative to owner/repo#id syntax)



## `fj issue search`

Search for an issue in a repo

**Usage:** `fj issue search [OPTIONS] [QUERY]`

###### **Arguments:**

* `<QUERY>`

###### **Options:**

* `-r`, `--repo <REPO>` — The repo to search in
* `-l`, `--labels <LABELS>`
* `-c`, `--creator <CREATOR>`
* `-a`, `--assignee <ASSIGNEE>`
* `-s`, `--state <STATE>` — Filter issues by state. Default: open

  Possible values: `open`, `closed`, `all`

* `-M`, `--milestone <MILESTONE>` — Filter by milestone name



## `fj issue view`

View an issue's info

**Usage:** `fj issue view [OPTIONS] <ID> [COMMAND]`

###### **Subcommands:**

* `body` — View an issue's title and body. The default
* `comment` — View a specific
* `comments` — List every comment

###### **Arguments:**

* `<ID>`

###### **Options:**

* `-r`, `--repo <REPO>` — The repo to operate on (alternative to owner/repo#id syntax)



## `fj issue view body`

View an issue's title and body. The default

**Usage:** `fj issue view body`



## `fj issue view comment`

View a specific

**Usage:** `fj issue view comment <IDX>`

###### **Arguments:**

* `<IDX>`



## `fj issue view comments`

List every comment

**Usage:** `fj issue view comments`



## `fj issue templates`

List the issue templates in a repo

**Usage:** `fj issue templates [OPTIONS]`

###### **Options:**

* `-r`, `--repo <REPO>` — The repo to view the templates of



## `fj issue browse`

Open an issue in your browser

**Usage:** `fj issue browse [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>`

###### **Options:**

* `-r`, `--repo <REPO>` — The repo to operate on (alternative to owner/repo#id syntax)



## `fj pr`

**Usage:** `fj pr [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `search` — Search a repository's pull requests
* `create` — Create a new pull request
* `view` — View the contents of a pull request
* `status` — View the mergability and CI status of a pull request
* `checkout` — Checkout a pull request in a new branch
* `comment` — Add a comment on a pull request
* `edit` — Edit the contents of a pull request
* `close` — Close a pull request, without merging
* `reopen` — Reopen a closed pull request
* `merge` — Merge a pull request
* `browse` — Open a pull request in your browser
* `review` — View the review on a pull request
* `assign` — Assign users to a pull request
* `unassign` — Unassign users from a pull request

###### **Options:**

* `-R`, `--remote <REMOTE>` — The local git remote that points to the repo to operate on



## `fj pr search`

Search a repository's pull requests

**Usage:** `fj pr search [OPTIONS] [QUERY]`

###### **Arguments:**

* `<QUERY>`

###### **Options:**

* `-l`, `--labels <LABELS>`
* `-c`, `--creator <CREATOR>`
* `-a`, `--assignee <ASSIGNEE>`
* `-s`, `--state <STATE>` — Filter PRs by state. Default: open

  Possible values: `open`, `closed`, `all`

* `-M`, `--milestone <MILESTONE>` — Filter by milestone name
* `--base <BASE>` — Filter by base branch name (server-side, via the pulls endpoint)
* `--head <HEAD>` — Filter by head branch name (server-side, via the pulls endpoint)
* `-r`, `--repo <REPO>` — The repo to search in



## `fj pr create`

Create a new pull request

**Usage:** `fj pr create [OPTIONS] [TITLE]`

###### **Arguments:**

* `<TITLE>` — What to name the new pull request.

   Prefix with "WIP: " to mark this PR as a draft.

###### **Options:**

* `--base <BASE>` — The branch to merge onto
* `--head <HEAD>` — The branch to pull changes from
* `--body <BODY>` — The text body of the pull request.

   Leaving this out will open your editor, unless --body-file is specified.
* `--body-file <BODY_FILE>` — The text body of the issue, to read from a file
* `-A`, `--autofill` — Automatically populate the PR's title and body from its commits.

   If there's a single commit, the PR will match its title and contents. Otherwise the title will be the branch title, and the contents will include a list of every commit's message.
* `-M`, `--milestone <MILESTONE>` — Milestone to assign (name or numeric ID)
* `--assignee <ASSIGNEES>` — Assign users (repeatable, e.g. --assignee alice --assignee bob)
* `-r`, `--repo <REPO>` — The repo to create this pull request on
* `-w`, `--web` — Open the PR creation page in your web browser
* `-a`, `--agit` — Open the PR using AGit workflow



## `fj pr view`

View the contents of a pull request

**Usage:** `fj pr view [OPTIONS] [ID] [COMMAND]`

###### **Subcommands:**

* `body` — View the title and body of a pull request
* `comment` — View a comment on a pull request
* `comments` — View all comments on a pull request
* `labels` — View the labels applied to a pull request
* `diff` — View the diff between the base and head branches of a pull request
* `files` — View the files changed in a pull request
* `commits` — View the commits in a pull request

###### **Arguments:**

* `<ID>` — The pull request to view

###### **Options:**

* `-r`, `--repo <REPO>` — The repo to operate on (alternative to owner/repo#id syntax)



## `fj pr view body`

View the title and body of a pull request

**Usage:** `fj pr view body`



## `fj pr view comment`

View a comment on a pull request

**Usage:** `fj pr view comment <IDX>`

###### **Arguments:**

* `<IDX>` — The index of the comment to view, 0-indexed



## `fj pr view comments`

View all comments on a pull request

**Usage:** `fj pr view comments`



## `fj pr view labels`

View the labels applied to a pull request

**Usage:** `fj pr view labels`



## `fj pr view diff`

View the diff between the base and head branches of a pull request

**Usage:** `fj pr view diff [OPTIONS]`

###### **Options:**

* `-p`, `--patch` — Get the diff in patch format
* `-e`, `--editor` — View the diff in your text editor



## `fj pr view files`

View the files changed in a pull request

**Usage:** `fj pr view files`



## `fj pr view commits`

View the commits in a pull request

**Usage:** `fj pr view commits [OPTIONS]`

###### **Options:**

* `-o`, `--oneline` — View one commit per line



## `fj pr status`

View the mergability and CI status of a pull request

**Usage:** `fj pr status [OPTIONS] [ID]`

###### **Arguments:**

* `<ID>` — The pull request to view

###### **Options:**

* `--wait` — Wait for all checks to finish before exiting
* `-r`, `--repo <REPO>` — The repo to operate on (alternative to owner/repo#id syntax)



## `fj pr checkout`

Checkout a pull request in a new branch

**Usage:** `fj pr checkout [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — The pull request to check out.

   Prefix with ^ to get a pull request from the parent repo.

###### **Options:**

* `--branch-name <NAME>` — The name to give the newly created branch.

   Defaults to naming after the host url, repo owner, and PR number.
* `-S`, `--ssh <SSH>` — Pull the commits using SSH instead of HTTP(S)

  Possible values: `true`, `false`

* `-I`, `--identity-file <IDENTITY_FILE>` — An SSH key file to use when cloning over SSH



## `fj pr comment`

Add a comment on a pull request

**Usage:** `fj pr comment [OPTIONS] [PR] [BODY]`

###### **Arguments:**

* `<PR>` — The pull request to comment on
* `<BODY>` — The text content of the comment.

   Leaving this out will open your editor, unless --body-file is specified.

###### **Options:**

* `--body-file <BODY_FILE>` — The text content of the comment, to read from a file
* `-r`, `--repo <REPO>` — The repo to operate on (alternative to owner/repo#id syntax)



## `fj pr edit`

Edit the contents of a pull request

**Usage:** `fj pr edit [OPTIONS] [PR] <COMMAND>`

###### **Subcommands:**

* `title` — Edit the title
* `body` — Edit the text body
* `comment` — Edit a comment
* `labels` — Edit a PR's labels
* `assignees` — Edit a PR's assignees

###### **Arguments:**

* `<PR>` — The pull request to edit

###### **Options:**

* `-r`, `--repo <REPO>` — The repo to operate on (alternative to owner/repo#id syntax)



## `fj pr edit title`

Edit the title

**Usage:** `fj pr edit title [NEW_TITLE]`

###### **Arguments:**

* `<NEW_TITLE>` — New PR title.

   Leaving this out will open the current title in your editor.



## `fj pr edit body`

Edit the text body

**Usage:** `fj pr edit body [NEW_BODY]`

###### **Arguments:**

* `<NEW_BODY>` — New PR body.

   Leaving this out will open the current body in your editor.



## `fj pr edit comment`

Edit a comment

**Usage:** `fj pr edit comment <IDX> [NEW_BODY]`

###### **Arguments:**

* `<IDX>` — The index of the comment to edit, 0-indexed
* `<NEW_BODY>` — New comment body.

   Leaving this out will open the current body in your editor.



## `fj pr edit labels`

Edit a PR's labels

**Usage:** `fj pr edit labels [OPTIONS]`

###### **Options:**

* `-a`, `--add <ADD>` — The labels to add
* `-r`, `--rm <RM>` — The labels to remove



## `fj pr edit assignees`

Edit a PR's assignees

**Usage:** `fj pr edit assignees [OPTIONS]`

###### **Options:**

* `-a`, `--add <ADD>` — Usernames to add as assignees
* `-r`, `--rm <RM>` — Usernames to remove from assignees



## `fj pr close`

Close a pull request, without merging

**Usage:** `fj pr close [OPTIONS] [PR]`

###### **Arguments:**

* `<PR>` — The pull request to close

###### **Options:**

* `-w`, `--with-msg <WITH_MSG>` — A comment to add before closing.

   Adding without an argument will open your editor
* `-r`, `--repo <REPO>` — The repo to operate on (alternative to owner/repo#id syntax)



## `fj pr reopen`

Reopen a closed pull request

**Usage:** `fj pr reopen [OPTIONS] [PR]`

###### **Arguments:**

* `<PR>` — The pull request to reopen

###### **Options:**

* `-w`, `--with-msg <WITH_MSG>` — A comment to add before reopening.

   Adding without an argument will open your editor
* `-r`, `--repo <REPO>` — The repo to operate on (alternative to owner/repo#id syntax)



## `fj pr merge`

Merge a pull request

**Usage:** `fj pr merge [OPTIONS] [PR]`

###### **Arguments:**

* `<PR>` — The pull request to merge

###### **Options:**

* `-M`, `--method <METHOD>` — The merge style to use

  Possible values: `merge`, `rebase`, `rebase-merge`, `squash`, `manual`

* `-d`, `--delete` — Option to delete the corresponding branch afterwards
* `-t`, `--title <TITLE>` — The title of the merge or squash commit to be created
* `-m`, `--message <MESSAGE>` — The body of the merge or squash commit to be created
* `-r`, `--repo <REPO>` — The repo to operate on (alternative to owner/repo#id syntax)



## `fj pr browse`

Open a pull request in your browser

**Usage:** `fj pr browse [OPTIONS] [ID]`

###### **Arguments:**

* `<ID>` — The pull request to open in your browser

###### **Options:**

* `-r`, `--repo <REPO>` — The repo to operate on (alternative to owner/repo#id syntax)



## `fj pr review`

View the review on a pull request

**Usage:** `fj pr review [OPTIONS] [ID] [COMMAND]`

###### **Subcommands:**

* `list` — List reviews on a pull request

###### **Arguments:**

* `<ID>` — The pull request to view

###### **Options:**

* `-r`, `--repo <REPO>` — The repo to operate on (alternative to owner/repo#id syntax)



## `fj pr review list`

List reviews on a pull request

**Usage:** `fj pr review list [OPTIONS]`

###### **Options:**

* `-c`, `--comments` — List inline comments in reviews on a pull request
* `-a`, `--all` — Include all reviews, including stale and dismissed ones



## `fj pr assign`

Assign users to a pull request

**Usage:** `fj pr assign [OPTIONS] <USERS>...`

###### **Arguments:**

* `<USERS>` — Usernames to assign

###### **Options:**

* `-p`, `--pr <PR>` — The pull request to assign users to
* `-r`, `--repo <REPO>` — The repo to operate on (alternative to owner/repo#id syntax)



## `fj pr unassign`

Unassign users from a pull request

**Usage:** `fj pr unassign [OPTIONS] <USERS>...`

###### **Arguments:**

* `<USERS>` — Usernames to unassign

###### **Options:**

* `-p`, `--pr <PR>` — The pull request to unassign users from
* `-r`, `--repo <REPO>` — The repo to operate on (alternative to owner/repo#id syntax)



## `fj wiki`

**Usage:** `fj wiki [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `contents`
* `view`
* `clone`
* `browse`

###### **Options:**

* `-R`, `--remote <REMOTE>` — The local git remote that points to the repo to operate on
* `-r`, `--repo <REPO>` — The repo to operate on



## `fj wiki contents`

**Usage:** `fj wiki contents`



## `fj wiki view`

**Usage:** `fj wiki view <PAGE>`

###### **Arguments:**

* `<PAGE>`



## `fj wiki clone`

**Usage:** `fj wiki clone [OPTIONS]`

###### **Options:**

* `-p`, `--path <PATH>`
* `-S`, `--ssh <SSH>` — Clone the repo over SSH instead of HTTP(S)

  Possible values: `true`, `false`

* `-I`, `--identity-file <IDENTITY_FILE>` — An SSH key file to use when cloning over SSH



## `fj wiki browse`

**Usage:** `fj wiki browse <PAGE>`

###### **Arguments:**

* `<PAGE>`



## `fj actions`

**Usage:** `fj actions [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `tasks` — List the tasks on a repo
* `run` — List and manage workflow runs
* `artifact` — List and manage workflow run artifacts
* `variables` — List and manage variables
* `secrets`
* `dispatch` — Dispatch a workflow

###### **Options:**

* `-R`, `--remote <REMOTE>` — The local git remote that points to the repo to operate on
* `-r`, `--repo <REPO>` — The repo to operate on



## `fj actions tasks`

List the tasks on a repo

**Usage:** `fj actions tasks [OPTIONS]`

###### **Options:**

* `-p`, `--page <PAGE>` — The page to show. One page always includes up to 20 tasks

  Default value: `1`
* `--status <STATUS>` — Only show tasks with this status. Can be given multiple times

  Possible values: `unknown`, `waiting`, `running`, `success`, `failure`, `cancelled`, `skipped`, `blocked`




## `fj actions run`

List and manage workflow runs

**Usage:** `fj actions run <COMMAND>`

###### **Subcommands:**

* `list` — List workflow runs
* `view` — View a workflow run
* `jobs` — List the jobs of a workflow run
* `logs` — Print the logs of a workflow run
* `cancel` — Cancel a pending or running workflow run
* `delete` — Delete a completed workflow run



## `fj actions run list`

List workflow runs

**Usage:** `fj actions run list [OPTIONS]`

###### **Options:**

* `-p`, `--page <PAGE>` — The page to show. One page always includes up to 20 runs

  Default value: `1`
* `--ref <REF>` — Only show runs on this git reference, e.g. `refs/heads/main`
* `--workflow-id <WORKFLOW_ID>` — Only show runs of this workflow file, e.g. `ci.yml`
* `--status <STATUS>` — Only show runs with this status. Can be given multiple times

  Possible values: `unknown`, `waiting`, `running`, `success`, `failure`, `cancelled`, `skipped`, `blocked`




## `fj actions run view`

View a workflow run

**Usage:** `fj actions run view <ID>`

###### **Arguments:**

* `<ID>` — The id of the run to view



## `fj actions run jobs`

List the jobs of a workflow run

**Usage:** `fj actions run jobs <ID>`

###### **Arguments:**

* `<ID>` — The id of the run to list jobs for



## `fj actions run logs`

Print the logs of a workflow run

With `--job`, prints the plaintext logs of that job to stdout. Without it, writes a ZIP archive containing the logs of every job in the run to stdout.

**Usage:** `fj actions run logs [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — The id of the run to fetch logs for

###### **Options:**

* `-j`, `--job <JOB>` — Print the plaintext logs of this job (see `run jobs` for job ids)



## `fj actions run cancel`

Cancel a pending or running workflow run

**Usage:** `fj actions run cancel <ID>`

###### **Arguments:**

* `<ID>` — The id of the run to cancel



## `fj actions run delete`

Delete a completed workflow run

**Usage:** `fj actions run delete [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — The id of the run to delete

###### **Options:**

* `-f`, `--force` — Skip confirmation prompt
* `--dry-run` — Preview without executing



## `fj actions artifact`

List and manage workflow run artifacts

**Usage:** `fj actions artifact <COMMAND>`

###### **Subcommands:**

* `list` — List artifacts
* `download` — Download an artifact's ZIP archive
* `delete` — Delete an artifact



## `fj actions artifact list`

List artifacts

**Usage:** `fj actions artifact list [OPTIONS]`

###### **Options:**

* `--run <RUN>` — Only list artifacts of this workflow run



## `fj actions artifact download`

Download an artifact's ZIP archive

**Usage:** `fj actions artifact download [OPTIONS] <ARTIFACT>`

###### **Arguments:**

* `<ARTIFACT>` — The artifact to download, by id or name

###### **Options:**

* `-o`, `--output <OUTPUT>` — Where to save the artifact. Defaults to `<name>.zip`



## `fj actions artifact delete`

Delete an artifact

**Usage:** `fj actions artifact delete [OPTIONS] <ARTIFACT>`

###### **Arguments:**

* `<ARTIFACT>` — The artifact to delete, by id or name

###### **Options:**

* `-f`, `--force` — Skip confirmation prompt
* `--dry-run` — Preview without executing



## `fj actions variables`

List and manage variables

**Usage:** `fj actions variables <COMMAND>`

###### **Subcommands:**

* `list` — List variables
* `create` — Create a new variable
* `delete`



## `fj actions variables list`

List variables

**Usage:** `fj actions variables list [OPTIONS]`

###### **Options:**

* `-v`, `--verbose` — Also print owner_id and repo_id



## `fj actions variables create`

Create a new variable

**Usage:** `fj actions variables create [OPTIONS] <NAME> [DATA]`

###### **Arguments:**

* `<NAME>` — The name of the new variable
* `<DATA>` — The data to save into the variable. Omit to invoke editor

###### **Options:**

* `-f`, `--force` — Override existing variables



## `fj actions variables delete`

**Usage:** `fj actions variables delete [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>` — The variable to delete

###### **Options:**

* `-f`, `--force` — Skip confirmation prompt
* `--dry-run` — Preview without executing



## `fj actions secrets`

**Usage:** `fj actions secrets <COMMAND>`

###### **Subcommands:**

* `list` — List secrets
* `create` — Create a new secret
* `delete`



## `fj actions secrets list`

List secrets

**Usage:** `fj actions secrets list`



## `fj actions secrets create`

Create a new secret

**Usage:** `fj actions secrets create <NAME> <DATA>`

###### **Arguments:**

* `<NAME>` — The name of the new secret
* `<DATA>` — The data to save into the secret



## `fj actions secrets delete`

**Usage:** `fj actions secrets delete [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>` — The secret to delete

###### **Options:**

* `-f`, `--force` — Skip confirmation prompt
* `--dry-run` — Preview without executing



## `fj actions dispatch`

Dispatch a workflow

**Usage:** `fj actions dispatch [OPTIONS] <NAME> <REF>`

###### **Arguments:**

* `<NAME>` — Name of the workflow to dispatch
* `<REF>` — Git revision to dispatch the workflow on

###### **Options:**

* `-I`, `--inputs <INPUTS>`



## `fj whoami`

**Usage:** `fj whoami [OPTIONS]`

###### **Options:**

* `-r`, `--remote <REMOTE>`



## `fj auth`

**Usage:** `fj auth <COMMAND>`

###### **Subcommands:**

* `login` — Log in to an instance
* `logout` — Deletes login info for an instance
* `add-key` — Add an application token for an instance
* `use-ssh`
* `list` — List all instances you're currently logged into



## `fj auth login`

Log in to an instance.

Opens an auth page in your browser

**Usage:** `fj auth login`



## `fj auth logout`

Deletes login info for an instance

**Usage:** `fj auth logout <HOST>`

###### **Arguments:**

* `<HOST>`



## `fj auth add-key`

Add an application token for an instance

Use this if `fj auth login` doesn't work

**Usage:** `fj auth add-key [KEY]`

###### **Arguments:**

* `<KEY>` — The key to add. If not present, the key will be read in from stdin



## `fj auth use-ssh`

**Usage:** `fj auth use-ssh [USE_SSH]`

###### **Arguments:**

* `<USE_SSH>`

  Possible values: `true`, `false`




## `fj auth list`

List all instances you're currently logged into

**Usage:** `fj auth list`



## `fj release`

**Usage:** `fj release [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `create` — Create a new release
* `edit` — Edit a release's info
* `delete` — Delete a release
* `list` — List all the releases on a repo
* `view` — View a release's info
* `browse` — Open a release in your browser
* `asset` — Commands on a release's attached files

###### **Options:**

* `-R`, `--remote <REMOTE>` — The local git remote that points to the repo to operate on
* `-r`, `--repo <REPO>` — The name of the repository to operate on



## `fj release create`

Create a new release

**Usage:** `fj release create [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>`

###### **Options:**

* `-T`, `--create-tag <CREATE_TAG>` — Create a new corresponding tag for this release. Defaults to release's name
* `-t`, `--tag <TAG>` — Pre-existing tag to use

   If you need to create a new tag for this release, use `--create-tag`
* `-a`, `--attach <ATTACH>` — Include a file as an attachment

   `--attach=<FILE>` will set the attachment's name to the file name
   `--attach=<FILE>:<ASSET>` will use the provided name for the attachment
* `-b`, `--body <BODY>` — Text of the release body.

   Using this flag without an argument will open your editor.
* `-B`, `--branch <BRANCH>`
* `-d`, `--draft`
* `-p`, `--prerelease`



## `fj release edit`

Edit a release's info

**Usage:** `fj release edit [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>`

###### **Options:**

* `-n`, `--rename <RENAME>`
* `-t`, `--tag <TAG>` — Corresponding tag for this release
* `-b`, `--body <BODY>` — Text of the release body.

   Using this flag without an argument will open your editor.
* `-d`, `--draft <DRAFT>`

  Possible values: `true`, `false`

* `-p`, `--prerelease <PRERELEASE>`

  Possible values: `true`, `false`




## `fj release delete`

Delete a release

**Usage:** `fj release delete [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>`

###### **Options:**

* `-t`, `--by-tag`
* `-f`, `--force` — Skip confirmation prompt
* `--dry-run` — Preview without executing



## `fj release list`

List all the releases on a repo

**Usage:** `fj release list [OPTIONS]`

###### **Options:**

* `-p`, `--include-prerelease`
* `-d`, `--include-draft`



## `fj release view`

View a release's info

**Usage:** `fj release view [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>`

###### **Options:**

* `-t`, `--by-tag`



## `fj release browse`

Open a release in your browser

**Usage:** `fj release browse [NAME]`

###### **Arguments:**

* `<NAME>`



## `fj release asset`

Commands on a release's attached files

**Usage:** `fj release asset <COMMAND>`

###### **Subcommands:**

* `create` — Create a new attachment on a release
* `delete` — Remove an attachment from a release
* `download` — Download an attached file



## `fj release asset create`

Create a new attachment on a release

**Usage:** `fj release asset create <RELEASE> <PATH> [NAME]`

###### **Arguments:**

* `<RELEASE>`
* `<PATH>`
* `<NAME>`



## `fj release asset delete`

Remove an attachment from a release

**Usage:** `fj release asset delete [OPTIONS] <RELEASE> <ASSET>`

###### **Arguments:**

* `<RELEASE>`
* `<ASSET>`

###### **Options:**

* `-f`, `--force` — Skip confirmation prompt
* `--dry-run` — Preview without executing



## `fj release asset download`

Download an attached file

Use `source.zip` or `source.tar.gz` to download the repo archive

**Usage:** `fj release asset download [OPTIONS] <RELEASE> <ASSET>`

###### **Arguments:**

* `<RELEASE>`
* `<ASSET>`

###### **Options:**

* `-o`, `--output <OUTPUT>`



## `fj milestone`

**Usage:** `fj milestone [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `list` — List milestones on a repo
* `view` — View a milestone's details
* `create` — Create a new milestone
* `edit` — Edit an existing milestone
* `delete` — Delete a milestone

###### **Options:**

* `-R`, `--remote <REMOTE>` — The local git remote that points to the repo to operate on
* `-r`, `--repo <REPO>` — The name of the repository to operate on



## `fj milestone list`

List milestones on a repo

**Usage:** `fj milestone list [OPTIONS]`

###### **Options:**

* `-s`, `--state <STATE>` — Filter by state: open, closed, all. Default: open

  Default value: `open`



## `fj milestone view`

View a milestone's details

**Usage:** `fj milestone view <NAME>`

###### **Arguments:**

* `<NAME>` — Milestone title or numeric ID



## `fj milestone create`

Create a new milestone

**Usage:** `fj milestone create [OPTIONS] <TITLE>`

###### **Arguments:**

* `<TITLE>` — Title of the milestone

###### **Options:**

* `-b`, `--body <BODY>` — Description of the milestone
* `-d`, `--due <DUE>` — Due date (RFC 3339, e.g. 2025-06-01T00:00:00Z)



## `fj milestone edit`

Edit an existing milestone

**Usage:** `fj milestone edit [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>` — Milestone title or numeric ID

###### **Options:**

* `-t`, `--title <TITLE>` — New title
* `-b`, `--body <BODY>` — New description
* `-d`, `--due <DUE>` — New due date (RFC 3339, e.g. 2025-06-01T00:00:00Z)
* `-s`, `--state <STATE>` — New state: open or closed



## `fj milestone delete`

Delete a milestone

**Usage:** `fj milestone delete [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>` — Milestone title or numeric ID

###### **Options:**

* `-f`, `--force` — Skip confirmation prompt
* `--dry-run` — Preview without executing



## `fj tag`

**Usage:** `fj tag [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `create` — Create a new tag
* `delete` — Delete a tag
* `list` — List all the tags on a repo
* `view` — View a tag's info

###### **Options:**

* `-R`, `--remote <REMOTE>` — The local git remote that points to the repo to operate on
* `-r`, `--repo <REPO>` — The name of the repository to operate on



## `fj tag create`

Create a new tag

**Usage:** `fj tag create [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>`

###### **Options:**

* `-b`, `--body <BODY>` — Text of the tag's message.

   Using this flag without an argument will open your editor.
* `-B`, `--branch <BRANCH>`



## `fj tag delete`

Delete a tag

**Usage:** `fj tag delete [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>`

###### **Options:**

* `-f`, `--force` — Skip confirmation prompt
* `--dry-run` — Preview without executing



## `fj tag list`

List all the tags on a repo

**Usage:** `fj tag list [OPTIONS]`

###### **Options:**

* `-p`, `--page <PAGE>`

  Default value: `1`



## `fj tag view`

View a tag's info

**Usage:** `fj tag view <NAME>`

###### **Arguments:**

* `<NAME>`



## `fj user`

**Usage:** `fj user [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `search` — Search for a user by username
* `view` — View a user's profile page
* `browse` — Open a user's profile page in your browser
* `follow` — Follow a user
* `unfollow` — Unfollow a user
* `following` — List everyone a user's follows
* `followers` — List a user's followers
* `block` — Block a user
* `unblock` — Unblock a user
* `repos` — List a user's repositories
* `orgs` — List the organizations a user is a member of
* `activity` — List a user's recent activity
* `edit` — Edit your user settings
* `key` — Manage SSH keys
* `gpg` — Manage GPG keys

###### **Options:**

* `-R`, `--remote <REMOTE>` — The local git remote that points to the repo to operate on



## `fj user search`

Search for a user by username

**Usage:** `fj user search [OPTIONS] <QUERY>`

###### **Arguments:**

* `<QUERY>` — The name to search for

###### **Options:**

* `-p`, `--page <PAGE>`



## `fj user view`

View a user's profile page

**Usage:** `fj user view [USER]`

###### **Arguments:**

* `<USER>` — The name of the user to view

   Omit to view your own page



## `fj user browse`

Open a user's profile page in your browser

**Usage:** `fj user browse [USER]`

###### **Arguments:**

* `<USER>` — The name of the user to open in your browser

   Omit to view your own page



## `fj user follow`

Follow a user

**Usage:** `fj user follow <USER>`

###### **Arguments:**

* `<USER>` — The name of the user to follow



## `fj user unfollow`

Unfollow a user

**Usage:** `fj user unfollow <USER>`

###### **Arguments:**

* `<USER>` — The name of the user to follow



## `fj user following`

List everyone a user's follows

**Usage:** `fj user following [USER]`

###### **Arguments:**

* `<USER>` — The name of the user whose follows to list

   Omit to view your own follows



## `fj user followers`

List a user's followers

**Usage:** `fj user followers [USER]`

###### **Arguments:**

* `<USER>` — The name of the user whose followers to list

   Omit to view your own followers



## `fj user block`

Block a user

**Usage:** `fj user block <USER>`

###### **Arguments:**

* `<USER>` — The name of the user to block



## `fj user unblock`

Unblock a user

**Usage:** `fj user unblock <USER>`

###### **Arguments:**

* `<USER>` — The name of the user to unblock



## `fj user repos`

List a user's repositories

**Usage:** `fj user repos [OPTIONS] [USER]`

###### **Arguments:**

* `<USER>` — The name of the user whose repos to list

   Omit to view your own repos.

###### **Options:**

* `--starred` — List starred repos instead of owned repos
* `--sort <SORT>` — Method by which to sort the list

  Possible values: `name`, `modified`, `created`, `stars`, `forks`

* `--page <PAGE>` — Page of repos to get

  Default value: `1`



## `fj user orgs`

List the organizations a user is a member of

**Usage:** `fj user orgs [USER]`

###### **Arguments:**

* `<USER>` — The name of the user to view org membership of

   Omit to view your own orgs.



## `fj user activity`

List a user's recent activity

**Usage:** `fj user activity [USER]`

###### **Arguments:**

* `<USER>` — The name of the user to view the activity of

   Omit to view your own activity.



## `fj user edit`

Edit your user settings

**Usage:** `fj user edit <COMMAND>`

###### **Subcommands:**

* `bio` — Set your bio
* `name` — Set your full name
* `pronouns` — Set your pronouns
* `location` — Set your activity visibility
* `activity` — Set your activity visibility
* `email` — Manage the email addresses associated with your account
* `website` — Set your linked website



## `fj user edit bio`

Set your bio

**Usage:** `fj user edit bio [CONTENT]`

###### **Arguments:**

* `<CONTENT>` — The new description. Leave this out to open your editor



## `fj user edit name`

Set your full name

**Usage:** `fj user edit name [OPTIONS] [NAME]`

###### **Arguments:**

* `<NAME>` — The new name

###### **Options:**

* `-u`, `--unset` — Remove your name from your profile



## `fj user edit pronouns`

Set your pronouns

**Usage:** `fj user edit pronouns [OPTIONS] [PRONOUNS]`

###### **Arguments:**

* `<PRONOUNS>` — The new pronouns

###### **Options:**

* `-u`, `--unset` — Remove your pronouns from your profile



## `fj user edit location`

Set your activity visibility

**Usage:** `fj user edit location [OPTIONS] [LOCATION]`

###### **Arguments:**

* `<LOCATION>` — The new location

###### **Options:**

* `-u`, `--unset` — Remove your location from your profile



## `fj user edit activity`

Set your activity visibility

**Usage:** `fj user edit activity --visibility <VISIBILITY>`

###### **Options:**

* `--visibility <VISIBILITY>` — The visibility of your activity

  Possible values: `hidden`, `public`




## `fj user edit email`

Manage the email addresses associated with your account

**Usage:** `fj user edit email [OPTIONS]`

###### **Options:**

* `--visibility <VISIBILITY>` — Set the visibility of your email address

  Possible values: `hidden`, `public`

* `-a`, `--add <ADD>` — Add a new email address
* `-r`, `--rm <RM>` — Remove an email address



## `fj user edit website`

Set your linked website

**Usage:** `fj user edit website [OPTIONS] [URL]`

###### **Arguments:**

* `<URL>` — Your website URL

###### **Options:**

* `-u`, `--unset` — Remove your website from your profile



## `fj user key`

Manage SSH keys

**Usage:** `fj user key <COMMAND>`

###### **Subcommands:**

* `list` — List your SSH keys
* `view` — View an SSH key
* `delete` — Delete an SSH key
* `upload` — Upload an SSH key



## `fj user key list`

List your SSH keys

**Usage:** `fj user key list [OPTIONS]`

###### **Options:**

* `-v`, `--verbose` — Show detailed information about every key



## `fj user key view`

View an SSH key

**Usage:** `fj user key view <ID>`

###### **Arguments:**

* `<ID>`



## `fj user key delete`

Delete an SSH key

**Usage:** `fj user key delete [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>`

###### **Options:**

* `-f`, `--force` — Skip confirmation prompt
* `--dry-run` — Preview without executing



## `fj user key upload`

Upload an SSH key

**Usage:** `fj user key upload [OPTIONS] [KEYFILE]`

###### **Arguments:**

* `<KEYFILE>` — Path to the key file or '-' to read from stdin. If omitted, will try to guess

###### **Options:**

* `-t`, `--title <TITLE>` — The title of the key. If omitted, will try to guess from the file content
* `-f`, `--force` — If provided, will skip checks against accidentally uploading private keys
* `-r`, `--read-only` — If provided, the new key will only have read access



## `fj user gpg`

Manage GPG keys

**Usage:** `fj user gpg <COMMAND>`

###### **Subcommands:**

* `list` — List your GPG keys
* `view` — Show details about a GPG key
* `delete` — Deletes a GPG key. This will un-verify all commits signed with that key!
* `upload` — Upload a new GPG key from your local keyring. This command requires `gpg` to be installed
* `verify` — Verifies a GPG key. You need to have the to-be-verified key installed locally in order to sign some data with it. This command requires `gpg` to be installed



## `fj user gpg list`

List your GPG keys

**Usage:** `fj user gpg list [OPTIONS]`

###### **Options:**

* `-v`, `--verbose` — Show detailed information about every key



## `fj user gpg view`

Show details about a GPG key

**Usage:** `fj user gpg view <ID>`

###### **Arguments:**

* `<ID>` — ID of the GPG key to show as shown in `user gpg list`



## `fj user gpg delete`

Deletes a GPG key. This will un-verify all commits signed with that key!

**Usage:** `fj user gpg delete [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — ID of the GPG key to delete as shown in `user gpg list`

###### **Options:**

* `-f`, `--force` — Skip confirmation prompt
* `--dry-run` — Preview without executing



## `fj user gpg upload`

Upload a new GPG key from your local keyring. This command requires `gpg` to be installed

**Usage:** `fj user gpg upload [OPTIONS] <KEY>`

###### **Arguments:**

* `<KEY>` — The key to add. This can be anything the GPG CLI recognizes such as an email associated with the key or the key ID

###### **Options:**

* `-n`, `--no-verify` — Skip the verification step. With this disabled, you can only add keys with emails associated with your account



## `fj user gpg verify`

Verifies a GPG key. You need to have the to-be-verified key installed locally in order to sign some data with it. This command requires `gpg` to be installed

**Usage:** `fj user gpg verify <ID>`

###### **Arguments:**

* `<ID>` — ID of the GPG key to verify as shown in `user gpg list`



## `fj org`

**Usage:** `fj org [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `list` — List all organizations
* `view` — View info about an organization
* `create` — Create a new organization
* `edit` — Edit an organization's information
* `activity` — View the activity in an organization
* `members` — List the members of an organization
* `visibility` — View and change the visibility of your membership in an organization
* `team`
* `label`
* `repo`

###### **Options:**

* `-R`, `--remote <REMOTE>` — The local git remote that points to the repo to operate on



## `fj org list`

List all organizations

**Usage:** `fj org list [OPTIONS]`

###### **Options:**

* `-p`, `--page <PAGE>` — Which page of the results to view

  Default value: `1`
* `-o`, `--only-member-of` — Only list organizations you are a member of



## `fj org view`

View info about an organization

**Usage:** `fj org view <NAME>`

###### **Arguments:**

* `<NAME>` — The name of the organization to view



## `fj org create`

Create a new organization

**Usage:** `fj org create [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>` — The username for the organization.

   It can only have alphanumeric characters, dash, underscore, or period. It must start and end with an alphanumeric character, and can't have consecutive dashes, underscores, or periods.

   If you want a name that doesn't have these restrictions, see the `--full-name` option.

###### **Options:**

* `-f`, `--full-name <FULL_NAME>` — The display name for the organization.

   This doesn't have the restrictions the `name` argument does, and can contain any UTF-8 text.
* `-d`, `--description <DESCRIPTION>` — The organization's description
* `-e`, `--email <EMAIL>` — Contact email for the organization
* `-l`, `--location <LOCATION>` — The organizations's location
* `-w`, `--website <WEBSITE>` — The organization's website
* `-V`, `--visibility <VISIBILITY>` — The visibility of the organization.

   Public organizations can be viewed by anyone, limited orgs can only be viewed by logged-in users, and private orgs can only be viewed by members of that org.

  Possible values: `private`, `limited`, `public`

* `-a`, `--admin-can-change-team-access <ADMIN_CAN_CHANGE_TEAM_ACCESS>` — Whether the admin of a repo can change org teams' access to it

  Possible values: `true`, `false`




## `fj org edit`

Edit an organization's information

**Usage:** `fj org edit [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>` — The name of the organization to edit.

   Note that this is the username, *not* the display name.

###### **Options:**

* `-f`, `--full-name <FULL_NAME>` — The display name for the organization.

   This doesn't have the restrictions the `name` argument does, and can contain any UTF-8 text.
* `-d`, `--description <DESCRIPTION>` — The organization's description
* `-e`, `--email <EMAIL>` — Contact email for the organization
* `-l`, `--location <LOCATION>` — The organizations's location
* `-w`, `--website <WEBSITE>` — The organization's website
* `-V`, `--visibility <VISIBILITY>` — The visibility of the organization.

   Public organizations can be viewed by anyone, limited orgs can only be viewed by logged-in users, and private orgs can only be viewed by members of that org.

  Possible values: `private`, `limited`, `public`

* `-a`, `--admin-can-change-team-access <ADMIN_CAN_CHANGE_TEAM_ACCESS>` — Whether the admin of a repo can change org teams' access to it

  Possible values: `true`, `false`




## `fj org activity`

View the activity in an organization

**Usage:** `fj org activity <NAME>`

###### **Arguments:**

* `<NAME>` — The name of the organization to view activity for



## `fj org members`

List the members of an organization

**Usage:** `fj org members [OPTIONS] <ORG>`

###### **Arguments:**

* `<ORG>` — The name of the organization to view the members of

###### **Options:**

* `-p`, `--page <PAGE>` — Which page of the results to view

  Default value: `1`



## `fj org visibility`

View and change the visibility of your membership in an organization

**Usage:** `fj org visibility [OPTIONS] <ORG>`

###### **Arguments:**

* `<ORG>` — The name of the organization to view your visibility in

###### **Options:**

* `-s`, `--set <SET>` — Set a new visibility for yourself

  Possible values: `private`, `public`




## `fj org team`

**Usage:** `fj org team <COMMAND>`

###### **Subcommands:**

* `list` — View all the teams in an organization
* `view` — View info about a single team
* `create` — Create a new team
* `edit` — Edit a team's information and permissions
* `delete` — Delete a team from an organization
* `repo`
* `member`



## `fj org team list`

View all the teams in an organization

**Usage:** `fj org team list <ORG>`

###### **Arguments:**

* `<ORG>` — The name of the organization to list the teams in



## `fj org team view`

View info about a single team

**Usage:** `fj org team view [OPTIONS] <ORG> <NAME>`

###### **Arguments:**

* `<ORG>` — The name of the organization the team is part of
* `<NAME>` — The name of the new team

###### **Options:**

* `-p`, `--list-permissions`



## `fj org team create`

Create a new team

**Usage:** `fj org team create [OPTIONS] <ORG> <NAME>`

###### **Arguments:**

* `<ORG>` — The name of the organization to create the team in
* `<NAME>` — The name of the new team

   This must only contain alphanumeric characters.

###### **Options:**

* `-c`, `--can-create-repos` — Allow members of this team to create repos in the organization
* `-i`, `--include-all-repos` — Give this team access to every repo
* `-A`, `--admin` — Give this team administrator abilities in the organization
* `-d`, `--description <DESCRIPTION>` — A description of what the team does
* `-r`, `--read-permissions <READ_PERMISSIONS>` — A comma-separated list of read permissions to give this team

   List of permissions: - wiki - ext_wiki - issues - ext_issues - pulls - projects - actions - code - releases - packages

   Alternatively, you can use `all` to allow every read permission.
* `-w`, `--write-permissions <WRITE_PERMISSIONS>` — A comma-separated list of read+write permissions to give this team

   List of permissions: - wiki - ext_wiki - issues - ext_issues - pulls - projects - actions - code - releases - packages

   Alternatively, you can use `all` to allow every read+write permission



## `fj org team edit`

Edit a team's information and permissions

**Usage:** `fj org team edit [OPTIONS] <ORG> <NAME>`

###### **Arguments:**

* `<ORG>` — The name of the organization the team is in
* `<NAME>` — The name of the team to edit

###### **Options:**

* `-n`, `--new-name <NEW_NAME>` — Can members of this team to create repos in the organization?
* `-c`, `--can-create-repos <CAN_CREATE_REPOS>` — Allow members of this team to create repos in the organization

  Possible values: `true`, `false`

* `-i`, `--include-all-repos <INCLUDE_ALL_REPOS>` — Give this team access to every repo

  Possible values: `true`, `false`

* `-A`, `--admin <ADMIN>` — Give this team administrator abilities in the organization

  Possible values: `true`, `false`

* `-d`, `--description <DESCRIPTION>` — A description of what the team does
* `-r`, `--read-permissions <READ_PERMISSIONS>` — A comma-separated list of read permissions to give this team

   List of permissions: - wiki - ext_wiki - issues - ext_issues - pulls - projects - actions - code - releases - packages

   Alternatively, you can use `all` to allow every read permission.
* `-w`, `--write-permissions <WRITE_PERMISSIONS>` — A comma-separated list of read+write permissions to give this team

   List of permissions: - wiki - ext_wiki - issues - ext_issues - pulls - projects - actions - code - releases - packages

   Alternatively, you can use `all` to allow every read+write permission



## `fj org team delete`

Delete a team from an organization.

Note that this does NOT delete the repos the team has!

**Usage:** `fj org team delete [OPTIONS] <ORG> <NAME>`

###### **Arguments:**

* `<ORG>` — The name of the organization the team is in
* `<NAME>` — The name of the team to delete

###### **Options:**

* `-f`, `--force` — Skip confirmation prompt
* `--dry-run` — Preview without executing



## `fj org team repo`

**Usage:** `fj org team repo <COMMAND>`

###### **Subcommands:**

* `list` — List all the repos this team can access
* `add` — Add access to an existing repo to a team
* `rm` — Remove access to a repo from a team



## `fj org team repo list`

List all the repos this team can access

**Usage:** `fj org team repo list [OPTIONS] <ORG> <TEAM>`

###### **Arguments:**

* `<ORG>` — The name of the organization the team is in
* `<TEAM>` — The name of the team to view the repos of

###### **Options:**

* `-p`, `--page <PAGE>` — Which page of the results to view

  Default value: `1`



## `fj org team repo add`

Add access to an existing repo to a team

**Usage:** `fj org team repo add <ORG> <TEAM> <REPO>`

###### **Arguments:**

* `<ORG>` — The name of the organization the team is in
* `<TEAM>` — The name of the team to add a repo to
* `<REPO>` — The name of the repo to add to the team



## `fj org team repo rm`

Remove access to a repo from a team

Note that this does NOT delete the repository!

**Usage:** `fj org team repo rm [OPTIONS] <ORG> <TEAM> <REPO>`

###### **Arguments:**

* `<ORG>` — The name of the organization the team is in
* `<TEAM>` — The name of the team to remove the repo from
* `<REPO>` — The name of the repo to remove from the team

###### **Options:**

* `-f`, `--force` — Skip confirmation prompt
* `--dry-run` — Preview without executing



## `fj org team member`

**Usage:** `fj org team member <COMMAND>`

###### **Subcommands:**

* `list` — List all the members of a team
* `add` — Add someone to a team
* `rm` — Remove someone from a team



## `fj org team member list`

List all the members of a team

**Usage:** `fj org team member list [OPTIONS] <ORG> <TEAM>`

###### **Arguments:**

* `<ORG>` — The name of the organization the team is in
* `<TEAM>` — The name of the team to view the members of

###### **Options:**

* `-p`, `--page <PAGE>` — Which page of the results to view

  Default value: `1`



## `fj org team member add`

Add someone to a team

**Usage:** `fj org team member add <ORG> <TEAM> <USER>`

###### **Arguments:**

* `<ORG>` — The name of the organization the team is in
* `<TEAM>` — The name of the team to add a user to
* `<USER>` — The name of the user to add to the team



## `fj org team member rm`

Remove someone from a team

**Usage:** `fj org team member rm [OPTIONS] <ORG> <TEAM> <USER>`

###### **Arguments:**

* `<ORG>` — The name of the organization the team is in
* `<TEAM>` — The name of the team to remove the user from
* `<USER>` — The name of the user to remove from the team

###### **Options:**

* `-f`, `--force` — Skip confirmation prompt
* `--dry-run` — Preview without executing



## `fj org label`

**Usage:** `fj org label <COMMAND>`

###### **Subcommands:**

* `list` — List all the issue labels an organization uses
* `add` — Add a new issue label to an organization
* `edit` — Edit an issue label an organization uses
* `rm` — Remove an issue label from an organization



## `fj org label list`

List all the issue labels an organization uses

**Usage:** `fj org label list <ORG>`

###### **Arguments:**

* `<ORG>` — The name of the organization to list the labels of



## `fj org label add`

Add a new issue label to an organization

**Usage:** `fj org label add [OPTIONS] <ORG> <NAME> <COLOR>`

###### **Arguments:**

* `<ORG>` — The name of the organization the label should be added to
* `<NAME>` — The name of the label to add
* `<COLOR>` — The hexcode of the label to add

###### **Options:**

* `-d`, `--description <DESCRIPTION>` — A description of what the label is for
* `-e`, `--exclusive` — If this label is named `{scope}/{name}`, make it exclusive with other labels with the same scope



## `fj org label edit`

Edit an issue label an organization uses

**Usage:** `fj org label edit [OPTIONS] <ORG> <NAME>`

###### **Arguments:**

* `<ORG>` — The name of the organization the label is in
* `<NAME>` — The name of the label to edit

###### **Options:**

* `-n`, `--new-name <NEW_NAME>` — Set a new name for the label
* `-c`, `--color <COLOR>` — Set a new hexcode for the label
* `-d`, `--description <DESCRIPTION>` — Set a description of what the label is for
* `-e`, `--exclusive` — Set whether this label is exclusive with others of the same scope
* `-a`, `--archived <ARCHIVED>` — Set whether this label is archived

  Possible values: `true`, `false`




## `fj org label rm`

Remove an issue label from an organization

**Usage:** `fj org label rm [OPTIONS] <ORG> <LABEL>`

###### **Arguments:**

* `<ORG>` — The name of the organization the label is in
* `<LABEL>` — The name of the label to remove from the organization

###### **Options:**

* `-f`, `--force` — Skip confirmation prompt
* `--dry-run` — Preview without executing



## `fj org repo`

**Usage:** `fj org repo <COMMAND>`

###### **Subcommands:**

* `list` — List all the repos owned by this organization
* `create` — Create a new repository in this organization



## `fj org repo list`

List all the repos owned by this organization

**Usage:** `fj org repo list [OPTIONS] <ORG>`

###### **Arguments:**

* `<ORG>` — The name of the organization to list the repos of

###### **Options:**

* `-p`, `--page <PAGE>` — Which page of the results to view

  Default value: `1`



## `fj org repo create`

Create a new repository in this organization

**Usage:** `fj org repo create [OPTIONS] <ORG> <REPO>`

###### **Arguments:**

* `<ORG>` — The name of the organization to create the repo in
* `<REPO>` — Repository name, or org/name to create under an organization

###### **Options:**

* `-d`, `--description <DESCRIPTION>`
* `-P`, `--private`
* `-r`, `--remote <REMOTE>` — Creates a new remote with the given name for the new repo
* `-p`, `--push` — Pushes the current branch to the default branch on the new repo. Implies `--remote=origin` (setting remote manually overrides this)
* `-S`, `--ssh <SSH>` — Use SSH for the new remote instead of HTTP(S)

  Possible values: `true`, `false`




## `fj version`

**Usage:** `fj version`



## `fj completion`

**Usage:** `fj completion [OPTIONS] <SHELL>`

###### **Arguments:**

* `<SHELL>`

  Possible values: `bash`, `elvish`, `fish`, `power-shell`, `zsh`, `nushell`


###### **Options:**

* `--bin-name <BIN_NAME>`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
