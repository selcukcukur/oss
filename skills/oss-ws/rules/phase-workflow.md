# Phase Workflow

## Contents

- Read before editing
- Update changelog
- Stage intentionally
- Commit coherently

---

## Read Before Editing

Before changing a governed artifact, read the relevant files:

1. `specs/*` for normative requirements.
2. `docs/*` for practical guidance.
3. `skills/oss-ws/rules/*` for agent-critical rules.
4. `skills/oss-ws/evals/evals.json` for behavioral checks.

---

## Update Changelog

Every completed phase must update root `changelog.md`.

The changelog entry should describe the phase in reader-facing language, not raw file operations.

---

## Stage Intentionally

Before committing, verify the working tree contains only intended phase changes.

Do not mix unrelated cleanup with a standard, docs, rule, or eval phase.

---

## Commit Coherently

Commit the phase after `changelog.md` is updated.

The commit subject should describe what the phase adds or changes in imperative language.
