# Commit Message Writing

## Contents

- Type selection
- Scope selection
- Subject wording
- Body content
- Agent review

---

## Type Selection

Choose exactly one allowed type:

- `feat` for new standards, capabilities, docs, templates, rules, or evals.
- `perf` for performance improvements.
- `docs` for documentation-only edits.
- `fix` for corrections.
- `refactor` for restructuring without intended behavior change.
- `chore` for maintenance.

---

## Scope Selection

Always include a lowercase scope.

Use `phase` for complete standard phases. Use a narrower scope when the commit only touches one area, such as `commit`, `changelog`, `skill`, `rules`, `evals`, `docs`, `specs`, or `repo`.

---

## Subject Wording

Write the subject after `: ` in lowercase.

**Incorrect:**

```text
docs(commit): Add commit guide
docs(commit): add commit guide.
commit: add guide
```

**Correct:**

```text
docs(commit): add commit guide
```

Literal identifiers may keep required spelling when lowercasing would change their meaning.

---

## Body Content

Use a body when the subject does not explain the reason, impact, migration, or references.

The body may use normal prose. Keep it useful; do not repeat the diff.

---

## Agent Review

Before proposing or creating a commit, verify:

- Subject matches `type(scope): subject`.
- Type is one of `feat`, `perf`, `docs`, `fix`, `refactor`, `chore`.
- Scope is present and lowercase.
- Subject is lowercase.
- Staged files match the commit subject.
- Unrelated user changes are not staged.
