# OSS Commit Standard

Status: Draft
Version: 0.1.0-draft
Domain: oss.ws

## 1. Purpose

The OSS Commit Standard defines how projects should write commits that are useful to humans, reviewable by maintainers, and reusable by AI agents when preparing changelogs, pull requests, issues, release notes, and audits.

A commit is not only a storage unit for code. It is a durable explanation of one intentional project change. Clean commits make the rest of the project communication stack stronger.

This standard is designed for:

- Human contributors who need to describe changes clearly.
- Maintainers who need reviewable history.
- AI agents that draft changelogs, pull requests, release notes, and summaries from project history.
- Downstream readers who inspect history to understand why a change happened.

## 2. Conformance Language

The keywords "MUST", "MUST NOT", "SHOULD", "SHOULD NOT", and "MAY" are to be interpreted as normative requirements for this standard.

## 3. Commit Scope

Each commit SHOULD represent one coherent change. A coherent change is a set of edits that should be understood, reviewed, reverted, or released together.

A commit MUST NOT mix unrelated work. Examples of unrelated work include:

- A feature plus formatting cleanup in unrelated files.
- A docs rewrite plus a behavior change that can ship independently.
- A refactor plus a bug fix that needs separate review.

A commit MAY include tests, docs, rules, evals, or changelog updates when those files directly support the same change.

## 4. Subject Line

Each commit MUST have a subject line. The subject line SHOULD be no more than 72 characters when practical.

The subject line MUST use imperative, present-tense language. It SHOULD describe what applying the commit does.

Examples:

```text
Add draft commit standard
Fix changelog release heading examples
Document commit splitting rules
Remove obsolete issue template guidance
```

The subject line MUST NOT rely on machine-oriented prefixes as its primary meaning. Conventional-commit prefixes such as `feat:`, `fix:`, and `chore:` MAY appear in repositories that require them, but OSS-WS commit subjects SHOULD remain readable natural language.

## 5. Body

A commit body SHOULD be added when the subject alone does not explain the intent, context, impact, or tradeoff.

The body SHOULD explain:

- Why the change is needed.
- What important behavior or workflow changes.
- Any compatibility, migration, or release impact.
- Any alternatives or constraints that affected the decision.

The body SHOULD wrap paragraphs at a readable width when practical. The body MUST NOT duplicate obvious file-level facts that can be seen in the diff unless those facts explain intent.

## 6. Breaking Changes

Breaking commits MUST make the breaking nature visible in the subject or the first paragraph of the body.

The recommended subject prefix is:

```text
Breaking: ...
```

Example:

```text
Breaking: require template files to declare schema version
```

The commit body SHOULD explain what users must do differently or link to a migration guide when one exists.

## 7. References

Commits SHOULD reference related issues, pull requests, external tickets, specs, or docs when those references help readers understand the change.

References MAY appear in the subject when short and useful, but SHOULD usually appear in the body or footer to keep the subject readable.

Examples:

```text
Refs #42
Closes #43
Spec: specs/oss-changelog.md
Guide: docs/changelog-guide.md
```

## 8. Authorship And Co-Authorship

Commit authorship SHOULD reflect the person or agent responsible for preparing the change.

When multiple humans materially contribute to a commit, co-authorship trailers MAY be used:

```text
Co-authored-by: Ada Lovelace <ada@example.com>
```

When an AI agent prepares a commit, the commit message SHOULD remain accountable to the human maintainer and SHOULD NOT hide material AI-generated changes.

## 9. Trailers

Commit trailers MAY be used for structured metadata such as co-authors, issue references, review acknowledgements, or provenance.

Trailers SHOULD appear after the body and a blank line. Trailers MUST remain readable to humans.

## 10. Relationship To Changelog

Commit subjects SHOULD be close enough to changelog language that agents and maintainers can draft changelog entries with minimal rewriting.

Commits and changelogs have different jobs:

- A commit explains one project step.
- A changelog explains release impact.

Maintainers and agents SHOULD curate commit history before using it in `changelog.md`.

## 11. Agent Requirements

An AI agent preparing or reviewing commits SHOULD:

- Keep commits scoped to one coherent phase or change.
- Use imperative, present-tense subjects.
- Add a body when intent, risk, or impact is not obvious.
- Mark breaking changes visibly.
- Avoid raw conventional-commit style unless the repository explicitly requires it.
- Update `changelog.md` before committing a completed phase.
- Verify staged files match the commit subject.
- Refuse to commit unrelated user changes unless explicitly asked.

## 12. Minimal Example

```text
Add draft commit standard

Define subject, body, breaking-change, reference, and agent workflow
requirements for OSS-WS commits.

Spec: specs/oss-commit.md
Guide: docs/commit-guide.md
```
