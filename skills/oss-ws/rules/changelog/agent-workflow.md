# Changelog Agent Workflow Rules

Status: Draft
Standard: `../../../../specs/oss-changelog.md`

## Critical Rule

Before an agent finishes a phase, it MUST update `changelog.md`, verify the changelog against the OSS-WS changelog rules, and commit the phase as one coherent unit when commit access is available.

## Workflow

1. Read the relevant spec and guide.
2. Collect the changes made in the phase.
3. Decide which changes matter to consumers, maintainers, or agents.
4. Add or update the release entry in `changelog.md`.
5. Use imperative, present-tense changelog items.
6. Mark breaking changes explicitly.
7. Add references when available.
8. Verify the working tree only contains the intended phase changes.
9. Commit the phase.

## Completion Criteria

- `changelog.md` reflects the phase.
- Related `specs`, `docs`, `rules`, and `evals` are consistent.
- Draft work remains marked as draft.
- No raw commit-log dump appears in the changelog.
- The phase can be understood from the commit and changelog together.
