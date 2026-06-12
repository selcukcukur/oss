# Changelog Change Item Rules

Status: Draft
Standard: `../../../../specs/oss-changelog.md`

## Critical Rule

Every changelog item MUST describe one meaningful user-facing change in imperative, present-tense language that remains understandable without its group heading.

## Required Checks

- Start with a verb such as `Add`, `Change`, `Remove`, `Fix`, `Document`, `Deprecate`, `Refactor`, or `Bump`.
- Keep the item brief and scannable.
- Merge related commits into one reader-facing change.
- Put breaking changes first inside their group.
- Prefix breaking changes with `**Breaking:**`.
- Use subsystem prefixes only when they clarify a genuinely distinct area.

## Prefer

```md
- Add release checklist for changelog reviews (#10)
```

## Avoid

```md
- Release checklist (#10)
- feat: add release checklist (#10)
- Added release checklist (#10)
```

## Reject

- Raw commit subjects copied without curation.
- Pull request titles copied without curation.
- Items that only make sense to maintainers.
- Multi-paragraph explanations inside a changelog bullet.
