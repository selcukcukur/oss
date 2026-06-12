# Commit Structure

See [`../../../specs/oss-commit.md`](../../../specs/oss-commit.md) and [`../../../docs/commit-guide.md`](../../../docs/commit-guide.md).

## Contents

- Coherent scope
- Subject line
- Body
- Breaking changes
- References

---

## Coherent Scope

One commit should contain one coherent change or phase.

**Incorrect:**

```text
Add commit standard and redesign unrelated docs
```

**Correct:**

```text
Add draft commit standard
```

Include specs, docs, rules, evals, and `changelog.md` together when they all support the same standard phase.

---

## Subject Line

Use imperative, present-tense language.

**Incorrect:**

```text
Added commit guide
Commit guide
chore: docs
```

**Correct:**

```text
Add commit guide
```

---

## Body

Add a body when the subject does not explain intent, impact, risk, or context.

Do not repeat the diff. Explain why the change exists and what readers should understand.

---

## Breaking Changes

Make breaking changes visible in the subject or first body paragraph.

```text
Breaking: require template files to declare schema version
```

---

## References

Use references when they help readers move from history to context.

```text
Spec: specs/oss-commit.md
Guide: docs/commit-guide.md
Refs #42
```
