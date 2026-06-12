# Changelog Release Entry Rules

Status: Draft
Standard: `../../../../specs/oss-changelog.md`

## Critical Rule

Every release entry MUST use `## VERSION - YYYY-MM-DD` and contain only one optional notice followed by zero or more allowed change groups.

## Required Checks

- The heading is a second-level Markdown heading.
- The visible version has no leading `v`.
- The date is ISO-style `YYYY-MM-DD`.
- A notice is a single sentence when present.
- There is no more than one notice per release.
- Release-level prose does not replace change groups unless the notice explains why no groups exist.

## Allowed Groups

Use only these groups, in this order when present:

1. `Changed`
2. `Added`
3. `Removed`
4. `Fixed`

## Reject

- Empty groups.
- Arbitrary group names such as `Misc`, `Other`, `Maintenance`, or `Security`.
- Multiple notices in one release.
- Upgrade-guide details that belong in a linked document.
