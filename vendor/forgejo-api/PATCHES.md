# Local patches to forgejo-api 0.11.0

Source: [forgejo-api 0.11.0](https://codeberg.org/Cyborus/forgejo-api) as published
on crates.io. Applied to this workspace via `[patch.crates-io]` in the root
`Cargo.toml`. Licensed Apache-2.0 OR MIT (see LICENSE-APACHE / LICENSE-MIT).

## Why

forgejo-api 0.11.0 generates `Option` response fields whose serde attributes
(`with = "time::serde::rfc3339::option"`, `deserialize_with = "crate::none_if_blank_url"`,
etc.) require the JSON key to be *present* (null is fine, absent is an error).
Forgejo servers older than v16 omit some of these keys, which makes
deserialization of whole responses fail:

- `Organization.created` is omitted by Forgejo v15, both in `GET /orgs/{org}`
  and in the org object embedded in `Team`. Teams appear inside pull request
  responses (`requested_reviewers_teams`), so on v15 servers every
  `GET /repos/{owner}/{repo}/pulls*` call that touches a PR with a team review
  request fails with `missing field 'created'`. Observed live against
  codeberg.org (15.0.0) on forgejo/forgejo PRs.
- `PublicKey.updated_at` is omitted by Forgejo v15 in `GET /user/keys`.

## What

`src/generated/structs.rs`: added `default,` to every presence-required serde
attribute on an `Option` field (183 occurrences):

- `#[serde(with = "time::serde::rfc3339::option")]` (94)
- `#[serde(deserialize_with = "crate::none_if_blank_url")]` (87)
- `#[serde(deserialize_with = "crate::deserialize_optional_ssh_url")]` (1)
- `#[serde(deserialize_with = "crate::requested_reviewers_ignore_null")]` (1)

An absent key now deserializes to `None` instead of erroring. Present keys
(including null) behave exactly as before, and serialization is unchanged.
The one non-`Option` date field (`EditDeadlineOption.due_date`, a request
body) is untouched.

`Cargo.toml`: dropped the `[[test]]` targets and dev-dependencies from the
published manifest because the crate's test suite is not vendored. Added a
`rustls-tls-native-roots` feature forwarding to reqwest so callers can augment
the bundled WebPKI roots with platform roots and certificates selected through
`SSL_CERT_FILE`/`SSL_CERT_DIR`.

## When to remove

Remove `vendor/forgejo-api` and the `[patch.crates-io]` entry once a
forgejo-api release tolerates absent optional fields (or once supporting
pre-v16 Forgejo servers is no longer a goal).
