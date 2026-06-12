# OSS-WS Skill

Status: Draft
Domain: oss.ws

## Purpose

OSS-WS is a standards skill for writing, reviewing, and maintaining open source project communication artifacts. It helps AI agents and humans produce consistent changelogs, commits, pull requests, issues, release notes, docs, and future OSS standards under the `oss.ws` project model.

This skill is intentionally rule-driven. The top-level file explains how to operate; detailed critical rules live under `rules/*` and evaluation cases live under `evals/*`.

## Principles

- Write for humans first and automation second.
- Preserve project intent before optimizing format.
- Prefer concise, traceable, release-safe language.
- Keep every artifact consistent with its matching spec and guide.
- Treat every completed phase as a release-quality unit: update docs, update `changelog.md`, then commit.
- Draft standards clearly when they are not yet stable.

## Source Of Truth

Use repository standards in this order:

1. `specs/*` for normative requirements.
2. `docs/*` for practical guidance and examples.
3. `skills/oss-ws/rules/*` for agent-critical operating rules.
4. `skills/oss-ws/evals/*` for behavioral checks.
5. `changelog.md` for completed phase history.

When files disagree, prefer the most specific normative spec. If the conflict changes behavior, flag it instead of silently choosing.

## Critical Rules

### Changelog

- Always follow `rules/changelog/file-format.md` before creating or editing a changelog file.
- Always follow `rules/changelog/release-entry.md` before adding a release entry.
- Always follow `rules/changelog/change-items.md` before writing change bullets.
- Always follow `rules/changelog/references.md` before adding or omitting references.
- Always follow `rules/changelog/noise.md` before deciding whether a change belongs in the changelog.
- Always follow `rules/changelog/agent-workflow.md` before finishing a phase.

### Repository Phase Work

- Update `changelog.md` for every completed phase before committing the phase.
- Keep draft work marked as draft until a stable version is explicitly requested.
- Do not invent stable guarantees that are not present in `specs/*`.
- Keep new standards split into `specs`, `docs`, `skills/oss-ws/rules`, and `skills/oss-ws/evals` when the topic needs both human guidance and agent enforcement.

### Agent Behavior

- Read the relevant spec, guide, and rule files before editing an artifact governed by this skill.
- Prefer small, reviewable phases over broad undocumented changes.
- Do not copy raw commits, issue titles, or pull request titles into user-facing standards without curation.
- When references are unavailable, write the best human-readable draft and leave the structure ready for later references.

## Operating Flow

1. Identify the artifact type: changelog, commit, pull request, issue, release, docs, or another OSS standard.
2. Read the relevant files under `specs`, `docs`, and `skills/oss-ws/rules`.
3. Draft or edit the artifact using the critical rules.
4. Check the artifact against matching eval expectations where they exist.
5. Update `changelog.md`.
6. Commit the completed phase.

## Current Coverage

- Changelog standard: Draft.
- Commit standard: Planned.
- Pull request standard: Planned.
- Issue standard: Planned.
- Release standard: Planned.
- Documentation standard: Planned.
