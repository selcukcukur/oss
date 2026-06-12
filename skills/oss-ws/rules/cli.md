# CLI Automation

## Contents

- Required implementation
- Git inspection
- Diff comparison
- Commit validation
- Changelog checks
- Phase checks
- Agent behavior

---

## Required Implementation

The OSS-WS CLI is a Rust command named `oss-ws`.

It uses `clap` for argument parsing and avoids platform-specific shell syntax in core logic. Git operations are executed through the installed `git` executable with explicit arguments.

---

## Git Inspection

The CLI must inspect the repository from any directory inside the worktree.

It reports:

- Repository root.
- Git directory.
- Current HEAD.
- Current branch or detached state.
- Clean state.
- Staged, modified, and untracked file counts.

---

## Diff Comparison

The CLI supports:

- Unstaged diffs.
- Staged diffs.
- Diff stats.
- Changed file names.
- Comparison against a base ref.

Diff commands print Git output and do not rewrite it.

---

## Commit Validation

The CLI validates the OSS-WS lowercase scoped commit format:

```text
type(scope): subject
```

Allowed types are `feat`, `perf`, `docs`, `fix`, `refactor`, and `chore`.

The scope is required. The subject is lowercase and has no trailing period. Breaking changes use `!` after the scope.

---

## Changelog Checks

The CLI checks that the repository uses root `changelog.md`, starts it with `# Changelog`, and formats release headings as `## VERSION - YYYY-MM-DD`.

---

## Phase Checks

When the working tree is dirty, the CLI checks whether `changelog.md` is part of the phase.

When the working tree is clean, the CLI reports a clean phase state.

---

## Agent Behavior

Before committing CLI work, run formatting, tests, and the relevant CLI checks when dependencies are available.

Do not use the CLI to modify repository files unless a future command explicitly documents that behavior.
