# OSS Changelog Standard

Status: Draft
Version: 0.1.0-draft
Domain: oss.ws

## 1. Purpose

The OSS Changelog Standard defines how projects should record release changes in a way that is useful to humans and predictable for AI agents. A changelog is not a raw git log, a marketing post, or a full migration guide. It is a curated release record that helps readers understand what changed, why it matters, and where to find more context.

This standard is designed for:

- Human maintainers who need a consistent release-writing workflow.
- Contributors who want to describe changes in a reusable form.
- AI agents that prepare drafts, verify structure, or summarize releases.
- Downstream users who need to evaluate upgrade impact quickly.

## 2. Conformance Language

The keywords "MUST", "MUST NOT", "SHOULD", "SHOULD NOT", and "MAY" are to be interpreted as normative requirements for this standard.

## 3. File Requirements

A conforming project MUST keep its release history in a file named `changelog.md` at the repository root.

The file MUST be Markdown and MUST begin with:

```md
# Changelog
```

Release entries MUST be ordered newest first. Stable releases MUST be present in the changelog. Draft, prerelease, and internal entries MAY be included when they are useful to consumers or when the project publishes them externally.

## 4. Release Entry

Each release entry MUST begin with a second-level heading:

```md
## VERSION - DATE
```

`VERSION` MUST identify the release version without a leading `v`. `DATE` MUST use `YYYY-MM-DD`.

When a release maps to a public tag, package, or release page, the version SHOULD link to the best canonical release reference. Reference-style Markdown links are recommended when they keep the source file easier to read.

After the release heading, the entry MUST contain one of:

- One release notice.
- One or more change groups.
- One release notice followed by one or more change groups.

No other release-level content is allowed inside the changelog entry.

## 5. Release Notice

A release notice is a single sentence that explains release-level context. It SHOULD be italicized.

Use a notice for cases such as:

- The first release.
- A yanked, unpublished, or replaced release.
- A release that requires a separate upgrade guide.
- A stable release that is based directly on a prerelease.
- A draft release that intentionally has limited scope.

Each release entry MUST NOT contain more than one notice.

## 6. Change Groups

Change groups MUST use third-level headings. A heading MUST contain only the group name.

Allowed groups are, in order:

- `Changed` for changes to existing behavior.
- `Added` for new behavior, APIs, docs, templates, or supported workflows.
- `Removed` for removed behavior, APIs, docs, templates, or supported workflows.
- `Fixed` for bug fixes and corrections.

Projects SHOULD omit empty groups. Groups SHOULD be ordered as listed above so readers can scan releases consistently.

Each group MUST contain an unordered Markdown list. Each list item MUST describe one meaningful change.

## 7. Change Items

Each change item MUST start with a present-tense imperative verb, such as `Add`, `Change`, `Remove`, `Fix`, `Document`, `Deprecate`, `Refactor`, or `Bump`.

Each change item MUST be understandable without relying on the group heading. For example:

```md
### Added

- Add changelog validation guidance.
```

Change items SHOULD be one line. If more detail is required, the item SHOULD link to a pull request, issue, commit, design note, migration guide, or specification.

Change items SHOULD be sorted by:

1. Breaking changes first.
2. User impact and operational importance.
3. Newest relevant work first.

## 8. References

Each change item SHOULD include the best available reference for deeper context. References MAY point to pull requests, issues, commits, external tickets, design notes, or related specifications.

References MUST appear after the change text on the same line. References MUST be wrapped in parentheses.

When several references of the same kind apply, they SHOULD be grouped together:

```md
- Fix template validation for empty issue bodies (#24, #27)
```

When multiple reference types are available, include the reference that is most useful as a starting point. A pull request is usually preferred over the issue it closes because it includes discussion, code, review, and linked context.

## 9. Authors

Projects MAY include authors after references. When authors are included, they MUST appear after references and be wrapped in parentheses:

```md
- Add release checklist (#12) (Ada Lovelace)
```

For bot-authored changes, the listed author SHOULD be the human who reviewed or merged the change.

Projects with one maintainer MAY omit authors.

## 10. Prefixes

Breaking changes MUST be prefixed with:

```md
**Breaking:**
```

Subsystem prefixes MAY be used when a repository contains independently meaningful areas. The subsystem name MUST be bold:

```md
- **Docs:** add examples for release notices
- **CLI (breaking):** remove legacy changelog generator
```

Subsystem prefixes SHOULD be used sparingly because they can weaken the release-level signal of a changelog.

## 11. Drafting Workflow

Maintainers and agents SHOULD draft a release entry from commits, pull requests, issues, and design notes, then curate it manually.

The drafting process SHOULD:

1. Collect candidate changes since the previous release.
2. Remove changes that do not matter to consumers.
3. Merge related changes into one item.
4. Rewrite items into imperative, reader-focused language.
5. Add the best references.
6. Check whether the version number matches the actual impact.
7. Commit `changelog.md` before creating the release tag.

## 12. Noise Rules

The changelog SHOULD exclude routine maintenance that does not affect consumers. Examples include formatting-only edits, local tooling cleanup, minor dotfile updates, and development dependency churn.

The changelog SHOULD include maintenance-looking changes when they affect compatibility, runtime support, generated output, agent behavior, user workflows, public documentation, or release trust.

## 13. Antipatterns

A conforming changelog MUST NOT be generated by copying raw commit subjects, pull request titles, or issue titles without curation.

A changelog SHOULD NOT encode meaning primarily through machine-oriented prefixes such as `feat:`, `fix:`, or `chore:`. Those prefixes MAY appear in source history, but the changelog entry itself must use readable natural language.

A changelog MUST NOT use ambiguous regional date formats.

## 14. AI Agent Requirements

An AI agent preparing or reviewing a changelog SHOULD:

- Preserve the project standard instead of inventing a new layout.
- Prefer human-readable summaries over commit-title copying.
- Flag missing references, ambiguous dates, empty groups, and unscoped breaking changes.
- Avoid adding changes that cannot be traced to project history or maintainer intent.
- Keep release notes brief and move long explanations to linked documents.
- Update `changelog.md` in the same phase as the completed work.

## 15. Minimal Example

```md
# Changelog

## 1.1.0 - 2026-06-12

_See the upgrade guide before adopting the new template schema._

### Changed

- **Breaking:** require issue templates to declare an audience (#42)
- Clarify pull request checklist wording (#44)

### Added

- Add agent-readable metadata to changelog entries (#43)

### Fixed

- Fix invalid Markdown in the bug report template (#45)
```
