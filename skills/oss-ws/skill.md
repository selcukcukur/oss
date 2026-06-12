---
name: oss-ws
description: Guides OSS-WS standards work for changelogs, commits, pull requests, issues, releases, docs, and agent-facing project artifacts. Applies when drafting, reviewing, or updating oss.ws specifications, docs, rules, evals, skills, commit messages, or phase changelogs.
---

# OSS-WS

A standards skill for writing, reviewing, and maintaining open source project communication artifacts under `oss.ws`.

OSS-WS exists to make artifacts usable by both humans and AI agents. It treats changelogs, commits, pull requests, issues, docs, and future standards as connected parts of the same communication system.

## Current Project Context

- Normative changelog spec: [`../../specs/oss-changelog.md`](../../specs/oss-changelog.md)
- Practical changelog guide: [`../../docs/changelog-guide.md`](../../docs/changelog-guide.md)
- Normative commit spec: [`../../specs/oss-commit.md`](../../specs/oss-commit.md)
- Practical commit guide: [`../../docs/commit-guide.md`](../../docs/commit-guide.md)
- Repository changelog: [`../../changelog.md`](../../changelog.md)
- Skill evals: [`./evals/evals.json`](./evals/evals.json)

## Principles

1. **Write for humans first.** Automation may validate structure, but every artifact must be readable by maintainers, contributors, and users.
2. **Keep artifacts connected.** Specs define rules, docs teach usage, skill rules guide agents, evals test behavior, and `changelog.md` records completed phases.
3. **Prefer traceable language.** Changes should be concise, impact-focused, and easy to connect to a spec, doc, issue, pull request, or commit.
4. **Draft honestly.** Early standards stay marked as draft until the project explicitly stabilizes them.
5. **Finish phases cleanly.** Every completed phase updates `changelog.md` and lands as one coherent commit.

## Critical Rules

These rules are always enforced. Each links to a detailed file with concrete requirements.

### Changelog Structure -> [changelog.md](./rules/changelog.md)

- **Use root `changelog.md`.** OSS-WS does not use `CHANGELOG.md` as the canonical filename.
- **Start with `# Changelog`.** The root changelog must be Markdown and begin with that heading.
- **Use `## VERSION - YYYY-MM-DD`.** Release headings must be second-level headings with ISO-style dates.
- **Keep newest first.** Release entries are ordered newest first.

### Changelog Items -> [changelog-items.md](./rules/changelog-items.md)

- **Write imperative bullets.** Start each change with present-tense verbs such as `Add`, `Change`, `Remove`, `Fix`, or `Document`.
- **Make bullets self-contained.** A change must make sense without relying on its group heading.
- **Mark breaking changes.** Use `**Breaking:**` and list breaking changes first in their group.
- **Curate, never dump.** Do not copy raw commit subjects, issue titles, or pull request titles without rewriting for readers.

### References & Authorship -> [references.md](./rules/references.md)

- **Add the best reference.** Prefer the most useful starting point, usually a pull request when available.
- **Keep references inline.** Put references after the change text on the same line, wrapped in parentheses.
- **Use authors intentionally.** Authors may be included after references; for bot-authored changes, prefer the human reviewer or merger.

### Commit Structure -> [commit.md](./rules/commit.md)

- **Keep one coherent change.** A commit should contain work that belongs together and can be reviewed or reverted together.
- **Write imperative subjects.** Use subjects like `Add draft commit standard`, not `Added commit standard`.
- **Add context when needed.** Use the body for why, impact, risk, migration, and references.
- **Mark breaking commits.** Make breaking changes visible in the subject or first body paragraph.

### Commit Messages -> [commit-message.md](./rules/commit-message.md)

- **Prefer natural language.** Do not make readers decode `feat:`, `fix:`, or `chore:` unless a repository explicitly requires that convention.
- **Use trailers deliberately.** Put co-authors, refs, and provenance after the body.
- **Match staged files.** The commit message must describe the staged change, not the whole working tree.

### Noise & Scope -> [noise.md](./rules/noise.md)

- **Include consumer impact.** Include changes that affect users, compatibility, docs trust, generated output, agent behavior, or release decisions.
- **Exclude internal noise.** Leave out formatting-only edits, local editor configuration, and maintenance churn without consumer impact.
- **Explain maintenance impact.** If maintenance affects humans or agents, write the user-facing impact.

### Phase Workflow -> [phase-workflow.md](./rules/phase-workflow.md)

- **Update `changelog.md` before finishing.** Every completed phase must update the repository changelog.
- **Keep one coherent commit per phase.** Stage only intended phase files and commit after the changelog update.
- **Read before editing.** Consult the matching spec, guide, and rule files before changing governed artifacts.

## Key Patterns

```md
# Correct release entry
## 0.2.0-draft - 2026-06-12

_Draft release for agent-facing changelog rules._

### Added

- Add OSS-WS changelog validation rules.
```

```text
# Correct commit subject
Add draft commit standard
```

```text
# Correct breaking commit
Breaking: require template files to declare schema version
```

```md
# Wrong: raw commit style in changelog
- feat: add changelog validation rules
```

## Workflow

1. Identify the artifact type: changelog, commit, pull request, issue, release, docs, or standard.
2. Read the relevant files in `specs`, `docs`, and `skills/oss-ws/rules`.
3. Draft or edit the artifact using the critical rules.
4. Check the output against [`evals/evals.json`](./evals/evals.json) when coverage exists.
5. Update root `changelog.md`.
6. Commit the completed phase.

## Detailed References

- [rules/changelog.md](./rules/changelog.md) - file format, release headings, notices, and groups
- [rules/changelog-items.md](./rules/changelog-items.md) - item language, ordering, breaking changes, and curation
- [rules/references.md](./rules/references.md) - references, authors, and traceability
- [rules/commit.md](./rules/commit.md) - commit scope, subject, body, breaking changes, and references
- [rules/commit-message.md](./rules/commit-message.md) - commit message wording, trailers, and agent review
- [rules/noise.md](./rules/noise.md) - what belongs in a changelog
- [rules/phase-workflow.md](./rules/phase-workflow.md) - phase completion requirements
