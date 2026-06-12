# Commit Structure

## Contents

- Required shape
- Allowed types
- Required scope
- Lowercase subject
- Breaking changes
- Coherent phase commits

---

## Required Shape

Every OSS-WS commit subject must use:

```text
type(scope): subject
```

**Incorrect:**

```text
Add draft commit standard
feat: add draft commit standard
feat(commit): Add draft commit standard
```

**Correct:**

```text
feat(commit): add draft commit standard
```

---

## Allowed Types

Use only:

- `feat`
- `perf`
- `docs`
- `fix`
- `refactor`
- `chore`

No other type is valid.

---

## Required Scope

The scope is always required, even in a single-repository project.

```text
feat(phase): add draft commit standard
docs(commit): document lowercase format
fix(changelog): correct release entry
```

Use `phase` when one coherent phase updates specs, docs, rules, evals, and `changelog.md` together.

---

## Lowercase Subject

The subject line should be lowercase.

```text
refactor(skill): flatten commit rules
```

Do not use uppercase sentence-style subjects or trailing periods.

---

## Breaking Changes

Use `!` after the scope.

```text
feat(template)!: require schema version
```

Explain migration details in the body when needed.

---

## Coherent Phase Commits

Specs, docs, rules, evals, and `changelog.md` can be committed together when they define one standard phase.

Use:

```text
feat(phase): add draft commit standard
```
