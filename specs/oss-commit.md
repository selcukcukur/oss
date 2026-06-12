# OSS Commit Standard

Status: Draft
Version: 0.1.0-draft
Domain: oss.ws

## 1. Purpose

The OSS Commit Standard defines the required commit message format for OSS-WS projects. It is designed to keep commit history consistent across monorepos, single-repository projects, human contributors, and AI agents.

The standard intentionally uses a small, lowercase, typed format so commit history can be scanned, filtered, linted, summarized, and converted into changelogs or release notes without guessing.

## 2. Conformance Language

The keywords "MUST", "MUST NOT", "SHOULD", "SHOULD NOT", and "MAY" are to be interpreted as normative requirements for this standard.

## 3. Required Format

Every OSS-WS commit subject MUST use this format:

```text
type(scope): subject
```

The subject line MUST be entirely lowercase except for literal identifiers that cannot be lowercased without changing their meaning.

Examples:

```text
feat(phase): add draft commit standard
docs(commit): document commit message format
fix(changelog): correct release heading example
refactor(skill): flatten rule references
perf(evals): reduce validation fixture size
chore(repo): update generated metadata
```

## 4. Allowed Types

The `type` MUST be one of:

- `feat` for new behavior, standards, docs, templates, rules, evals, or capabilities.
- `perf` for performance improvements.
- `docs` for documentation-only changes.
- `fix` for corrections and bug fixes.
- `refactor` for restructuring without intended behavior change.
- `chore` for maintenance that does not change user-facing behavior.

No other type is allowed in OSS-WS commits.

## 5. Scope

The `scope` MUST be present and MUST be wrapped in parentheses.

The scope MUST be lowercase. It SHOULD name the smallest meaningful area affected by the commit.

Recommended scopes include:

- `phase` for a coherent standards phase touching specs, docs, rules, evals, and changelog together.
- `changelog` for changelog-only work.
- `commit` for commit-standard work.
- `skill` for skill behavior or navigation.
- `rules` for agent rule files.
- `evals` for evaluation fixtures.
- `docs` for general documentation.
- `specs` for general specification work.
- `repo` for repository maintenance.

Monorepos and single-repository projects MUST both use the same scoped format. A monorepo MAY use package or workspace names as scopes when those names are the clearest affected area.

## 6. Subject

The `subject` MUST be lowercase and MUST describe the change after the colon and one space.

The subject SHOULD be concise and SHOULD avoid a trailing period.

The subject MUST NOT start with an uppercase letter. It MUST NOT use imperative-title style such as `Add draft commit standard`.

Prefer:

```text
feat(phase): add draft commit standard
```

Avoid:

```text
Add draft commit standard
feat: add draft commit standard
feat(commit): Add draft commit standard
feat(commit): add draft commit standard.
```

## 7. Commit Scope

Each commit SHOULD represent one coherent change. A coherent change is a set of edits that should be understood, reviewed, reverted, or released together.

A commit MUST NOT mix unrelated work.

A commit MAY include specs, docs, rules, evals, and `changelog.md` when they directly support the same phase. In that case, `feat(phase): ...` is usually the clearest subject.

## 8. Body

A commit body MAY be added when the subject does not explain the reason, risk, migration, or review context.

The body SHOULD use readable sentences. It MAY contain uppercase letters where normal prose, identifiers, filenames, or proper nouns require them.

The body SHOULD explain:

- Why the change is needed.
- What behavior, standard, or workflow changes.
- Any compatibility, migration, or release impact.
- Any references that help readers understand the change.

## 9. Breaking Changes

Breaking changes MUST be visible in the subject or the first paragraph of the body.

The subject SHOULD use `!` after the scope:

```text
feat(template)!: require schema version
```

The body SHOULD explain the required user action.

## 10. References And Trailers

References MAY appear in the body or trailers.

Examples:

```text
refs #42
closes #43
spec: specs/oss-commit.md
guide: docs/commit-guide.md
```

Trailers SHOULD be lowercase when they are project-defined. Standard Git trailers such as `Co-authored-by` MAY keep their conventional spelling.

## 11. Relationship To Changelog

Commit subjects and changelog entries have different formats.

Commits MUST use lowercase typed subjects. Changelog entries SHOULD remain human-readable release-impact bullets according to `specs/oss-changelog.md`.

Maintainers and agents MUST curate commit subjects before copying meaning into `changelog.md`.

## 12. Agent Requirements

An AI agent preparing or reviewing commits MUST:

- Use `type(scope): subject`.
- Use only `feat`, `perf`, `docs`, `fix`, `refactor`, or `chore`.
- Require a lowercase scope.
- Require a lowercase subject.
- Prefer `feat(phase): ...` for coherent standards phases that touch specs, docs, rules, evals, and changelog together.
- Mark breaking changes with `!` after the scope when possible.
- Update `changelog.md` before committing a completed phase.
- Verify staged files match the commit subject.
- Refuse to stage unrelated user changes unless explicitly asked.

## 13. Minimal Example

```text
feat(phase): add draft commit standard

define the oss-ws commit format for specs, docs, skill rules, evals,
and phase changelog updates.

spec: specs/oss-commit.md
guide: docs/commit-guide.md
```
