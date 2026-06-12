# Changelog Structure

See [`../../../specs/oss-changelog.md`](../../../specs/oss-changelog.md) and [`../../../docs/changelog-guide.md`](../../../docs/changelog-guide.md).

## Contents

- Canonical filename
- First heading
- Release heading
- Release notice
- Change groups
- Dates and order

---

## Canonical Filename

OSS-WS uses root `changelog.md`.

**Incorrect:**

```text
CHANGELOG.md
```

**Correct:**

```text
changelog.md
```

---

## First Heading

The first non-empty line must be:

```md
# Changelog
```

Do not add project descriptions, badges, or prose before the heading.

---

## Release Heading

Each release entry uses:

```md
## VERSION - YYYY-MM-DD
```

**Incorrect:**

```md
## v1.2.0 - 12/06/2026
```

**Correct:**

```md
## 1.2.0 - 2026-06-12
```

---

## Release Notice

A release may contain one single-sentence notice before change groups.

```md
_Draft release for agent-facing changelog rules._
```

Use notices for first releases, yanked releases, upgrade-guide pointers, prerelease promotion, or draft status.

---

## Change Groups

Use only these groups, in this order when present:

1. `Changed`
2. `Added`
3. `Removed`
4. `Fixed`

Do not include empty groups. Do not use `Misc`, `Other`, `Maintenance`, or arbitrary categories.

---

## Dates And Order

Dates must use `YYYY-MM-DD`. Release entries are newest first.
