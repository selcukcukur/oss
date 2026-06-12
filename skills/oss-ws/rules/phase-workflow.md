# Phase Workflow

## Contents

- Update changelog
- Stage intentionally
- Commit coherently
- Keep artifact layers aligned

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

The commit subject must use the lowercase scoped format:

```text
type(scope): subject
```

Use `feat(phase): ...` when the phase updates multiple aligned layers.

---

## Keep Artifact Layers Aligned

When a standard changes, update every affected layer in the same phase:

- Normative standard text.
- Practical guide text.
- Skill rules.
- Eval expectations.
- Repository changelog.

Do not leave the skill rules as pointers to other files. The rule files must contain the actual behavior an agent should follow.
