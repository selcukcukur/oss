# Changelog Items

See [`../../../specs/oss-changelog.md`](../../../specs/oss-changelog.md) and [`../../../docs/changelog-guide.md`](../../../docs/changelog-guide.md).

## Contents

- Imperative language
- Self-contained changes
- Breaking changes
- Sorting
- Curation

---

## Imperative Language

Start each item with a present-tense imperative verb.

**Incorrect:**

```md
- Added release checklist
- feat: add release checklist
- Release checklist
```

**Correct:**

```md
- Add release checklist
```

---

## Self-Contained Changes

Each item must make sense without the group heading.

**Incorrect:**

```md
### Added

- Template metadata
```

**Correct:**

```md
### Added

- Add template metadata
```

---

## Breaking Changes

Put breaking changes first inside their group and prefix them with `**Breaking:**`.

```md
- **Breaking:** require issue templates to declare an audience (#42)
```

---

## Sorting

Sort items by:

1. Breaking changes first.
2. User impact and operational importance.
3. Newest relevant work first.

---

## Curation

Do not copy raw commit subjects, issue titles, or pull request titles into the changelog without rewriting them for readers.
