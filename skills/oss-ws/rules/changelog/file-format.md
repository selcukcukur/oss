# Changelog File Format Rules

Status: Draft
Standard: `../../../../specs/oss-changelog.md`

## Critical Rule

An OSS-WS conforming project MUST keep release history in root `changelog.md`, written in Markdown, starting with `# Changelog`, and ordered newest first.

## Required Checks

- The filename is exactly `changelog.md`.
- The file is at the repository root.
- The first non-empty line is `# Changelog`.
- Release entries are sorted newest first.
- Dates use `YYYY-MM-DD`.
- Draft entries remain visibly draft through the version or notice.

## Reject

- `CHANGELOG.md` as the canonical OSS-WS filename.
- Regional date formats such as `12/06/2026` or `06/12/2026`.
- A changelog that is a copied git log.
- A changelog that mixes blog-style paragraphs into release entries.
