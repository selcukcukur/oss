# Changelog Reference Rules

Status: Draft
Standard: `../../../../specs/oss-changelog.md`

## Critical Rule

Each changelog item SHOULD include the most useful available reference on the same line after the change text, wrapped in parentheses.

## Required Checks

- Put references after the change text.
- Keep references on the same line as the change.
- Wrap references in parentheses.
- Prefer the pull request when it contains the issue, code, review, and linked context.
- Prefer the issue when it explains the user problem better than the implementation.
- Use a commit reference when no better reference exists.
- Include only the best starting point when too many references would add noise.

## Examples

```md
- Fix broken release badge link (#18)
- Add changelog validation guidance (`53bd922`)
- Document external issue handling (owner/project#194)
```

## Authors

Authors MAY appear after references. When present, wrap them in parentheses:

```md
- Add release checklist (#12) (Ada Lovelace)
```

For bot-authored changes, prefer the human reviewer or merger as the author.
