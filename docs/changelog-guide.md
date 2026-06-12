# Changelog Guide

Status: Draft
Standard: `specs/oss-changelog.md`

## Why This Exists

An OSS changelog is a release map for people first and automation second. It should let a maintainer, contributor, user, or AI agent answer three questions quickly:

- What changed?
- Why does it matter?
- Where can I learn more?

The changelog is intentionally shorter than a release post and more curated than git history.

## Required Shape

Every project should keep a root `changelog.md` file that starts with:

```md
# Changelog
```

Each release starts with:

```md
## VERSION - YYYY-MM-DD
```

Use the newest release first. Use `YYYY-MM-DD` dates. Do not use `v` in the visible version heading.

## Release Template

```md
## 1.2.0 - 2026-06-12

_Optional single-sentence notice._

### Changed

- **Breaking:** change public behavior that requires user action (#12)
- Improve an existing workflow (#13)

### Added

- Add a new feature, document, template, or supported workflow (#14)

### Removed

- **Breaking:** remove an old feature, document, template, or workflow (#15)

### Fixed

- Fix a bug, incorrect behavior, broken link, or invalid example (#16)
```

Only include groups that have entries.

## Writing Rules

Start each change with an imperative verb:

- `Add`
- `Change`
- `Remove`
- `Fix`
- `Document`
- `Deprecate`
- `Refactor`
- `Bump`

Write each item so it still makes sense if the group heading is removed.

Prefer:

```md
- Add release checklist for changelog reviews (#10)
```

Avoid:

```md
- Release checklist (#10)
```

## Choosing Groups

Use `Changed` when existing behavior, wording, compatibility, or workflow changes.

Use `Added` when the release introduces something new.

Use `Removed` when something is deleted or no longer supported.

Use `Fixed` when the release corrects a bug, invalid example, broken link, typo with user impact, or incorrect behavior.

## References

Add the best reference after the change text:

```md
- Fix broken release badge link (#18)
```

Prefer a pull request when it contains the full context. Use an issue when the issue explains the user problem better. Use a commit when no better reference exists.

## Breaking Changes

Put breaking changes first in their group and start them with:

```md
**Breaking:**
```

Example:

```md
- **Breaking:** require template files to declare a schema version (#21)
```

If the breaking change needs a long explanation, keep the changelog item short and link to an upgrade guide.

## What To Leave Out

Leave out changes that do not matter to users of the released project:

- Formatting-only edits.
- Local editor configuration.
- Minor internal cleanup.
- Development-only dependency bumps with no user impact.

Include maintenance-looking work when it changes compatibility, supported runtimes, generated output, documentation users rely on, or agent behavior.

## Agent Checklist

Before finishing a phase, an agent should verify:

- `changelog.md` was updated.
- The release entry uses `VERSION - YYYY-MM-DD`.
- The status is clear when the release is a draft.
- Empty groups were omitted.
- Change items start with imperative verbs.
- Breaking changes are visibly marked.
- References are present when available.
- The changelog entry is committed with the phase.

## First Draft Convention

For this repository, early standards use draft versions until the project is ready to publish stable specifications. The initial draft line is:

```md
## 0.1.0-draft - 2026-06-12
```
