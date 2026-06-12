# Commit Message Writing

See [`../../../specs/oss-commit.md`](../../../specs/oss-commit.md) and [`../../../docs/commit-guide.md`](../../../docs/commit-guide.md).

## Contents

- Imperative verbs
- Natural language
- Body content
- Trailers
- Agent review

---

## Imperative Verbs

Start subjects with verbs such as:

- `Add`
- `Change`
- `Remove`
- `Fix`
- `Document`
- `Deprecate`
- `Refactor`
- `Bump`

The subject should describe what applying the commit does.

---

## Natural Language

OSS-WS commits should be readable without decoding machine prefixes.

**Incorrect:**

```text
feat(commit): add rules
```

**Correct:**

```text
Add commit rules
```

If a repository explicitly requires conventional commits, keep the readable phrase clear after the prefix.

---

## Body Content

Use the body to explain:

- Reason.
- Impact.
- Migration.
- Tradeoffs.
- Links to specs, docs, issues, or pull requests.

Skip the body when the subject and diff are enough.

---

## Trailers

Put trailers after a blank line at the end of the message.

```text
Co-authored-by: Ada Lovelace <ada@example.com>
Refs #42
```

Trailers must stay human-readable.

---

## Agent Review

Before proposing or creating a commit, verify that the staged files match the subject and that unrelated user changes are not included.
