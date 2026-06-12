# Commit Guide

Status: Draft
Standard: `specs/oss-commit.md`

## Why This Exists

OSS-WS commits use one lowercase format everywhere:

```text
type(scope): subject
```

This keeps history consistent for humans, AI agents, monorepos, single-repository projects, changelogs, release notes, and audits.

## Required Shape

Use:

```text
feat(phase): add draft commit standard
```

The full subject has three required parts:

- `type`: one of `feat`, `perf`, `docs`, `fix`, `refactor`, `chore`.
- `scope`: lowercase area in parentheses.
- `subject`: lowercase summary after `: `.

## Allowed Types

Use only these types:

- `feat` for new behavior, standards, docs, templates, rules, evals, or capabilities.
- `perf` for performance improvements.
- `docs` for documentation-only changes.
- `fix` for corrections and bug fixes.
- `refactor` for restructuring without intended behavior change.
- `chore` for maintenance without user-facing behavior change.

## Scope Rules

The scope is always required.

Use the smallest meaningful area:

```text
docs(commit): document lowercase subject rules
fix(changelog): correct release date example
refactor(skill): flatten rule references
```

For full standards phases that touch specs, docs, rules, evals, and `changelog.md`, use:

```text
feat(phase): add draft commit standard
```

This applies to monorepos and single-repository projects. Monorepos may use package or workspace names when they are the clearest scope.

## Lowercase Rules

The subject line should be lowercase.

Prefer:

```text
feat(commit): add lowercase commit format
```

Avoid:

```text
Add lowercase commit format
feat: add lowercase commit format
feat(commit): Add lowercase commit format
feat(commit): add lowercase commit format.
```

Literal identifiers may keep their required spelling when lowercasing would change their meaning.

## Breaking Changes

Use `!` after the scope:

```text
feat(template)!: require schema version
```

Add body context when users must migrate:

```text
existing templates must add schema_version before validation.
```

## Body Rules

Add a body when useful. The body may use normal prose, filenames, identifiers, and proper nouns.

Use the body to explain:

- Reason.
- Impact.
- Migration.
- Tradeoffs.
- References to specs, docs, issues, or pull requests.

## References

Add references when they help:

```text
refs #42
closes #43
spec: specs/oss-commit.md
guide: docs/commit-guide.md
```

## Agent Checklist

Before committing, an agent should verify:

- The subject matches `type(scope): subject`.
- The type is allowed.
- The scope is present and lowercase.
- The subject is lowercase and has no trailing period.
- Breaking changes use `!` after the scope when possible.
- The staged files match the subject.
- `changelog.md` was updated for completed phases.
- No unrelated user changes are staged.

## First Draft Convention

For this repository, early standards use draft versions until the project is ready to publish stable specifications. The initial commit standard version is:

```text
0.1.0-draft
```
