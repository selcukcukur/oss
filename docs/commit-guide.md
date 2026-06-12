# Commit Guide

Status: Draft
Standard: `specs/oss-commit.md`

## Why This Exists

An OSS-WS commit is a small durable explanation. It should tell a future reader what changed, why the change belongs together, and whether it affects users, maintainers, releases, or agents.

Good commits make changelogs, pull requests, issue closure, release notes, and audits easier to write.

## Required Shape

Use a short imperative subject:

```text
Add draft commit standard
```

Add a body when the subject does not explain the reason, impact, or risk:

```text
Add draft commit standard

Define how OSS-WS commits should be scoped, written, referenced, and
reviewed by humans and agents.
```

## Subject Rules

Start with an imperative verb:

- `Add`
- `Change`
- `Remove`
- `Fix`
- `Document`
- `Deprecate`
- `Refactor`
- `Bump`

Prefer:

```text
Add changelog validation rules
```

Avoid:

```text
feat: add changelog validation rules
Added changelog validation rules
Changelog validation rules
```

If a repository requires conventional commits, keep the human-readable part clear:

```text
feat: add changelog validation rules
```

OSS-WS still treats the readable phrase as the important part.

## Body Rules

Use a body when the change needs context:

- Why the change exists.
- What behavior or workflow changes.
- What users or maintainers must do differently.
- What risk, migration, or compatibility impact exists.
- Which spec, doc, issue, or pull request gives more context.

Avoid bodies that only repeat the diff.

## Breaking Changes

Make breaking commits obvious:

```text
Breaking: require template files to declare schema version
```

In the body, explain the required action:

```text
Existing templates must add schema_version before they can be validated.
```

## Splitting Commits

Split commits when changes can be reviewed, reverted, or released independently.

Keep changes together when separating them would make the history harder to understand. For example, a spec addition, matching docs, matching skill rules, matching evals, and `changelog.md` update can be one coherent phase.

## References

Add references when they help:

```text
Refs #42
Closes #43
Spec: specs/oss-commit.md
Guide: docs/commit-guide.md
```

Use references to connect commits to issues, pull requests, external tickets, specs, docs, or design notes.

## Agent Checklist

Before committing, an agent should verify:

- The staged files match the subject.
- The commit is one coherent phase.
- The subject is imperative and present tense.
- The body explains intent when the diff is not enough.
- Breaking changes are visible.
- `changelog.md` was updated for completed phases.
- No unrelated user changes are staged.

## First Draft Convention

For this repository, early standards use draft versions until the project is ready to publish stable specifications. The initial commit standard version is:

```text
0.1.0-draft
```
