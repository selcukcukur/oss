# OSS CLI Standard

Status: Draft
Version: 0.1.0-draft
Domain: oss.ws

## 1. Purpose

The OSS CLI Standard defines how the `oss-ws` command line tool automates OSS-WS repository checks. The CLI exists to make standards repeatable across humans, AI agents, monorepos, single-repository projects, and operating systems.

The CLI MUST support cross-platform execution and MUST avoid platform-specific shell behavior in core logic.

## 2. Required Technology

The CLI MUST be written in Rust and MUST use `clap` for argument parsing.

The CLI SHOULD rely on the installed `git` executable for repository operations instead of shell-specific scripts.

## 3. Command Areas

The CLI MUST provide commands for:

- Git inspection.
- Diff comparison.
- Commit subject validation.
- Commit subject construction.
- Changelog structure checks.
- Phase completion checks.

## 4. Git Inspection

The CLI MUST be able to inspect a repository from any working directory inside that repository.

The inspection output SHOULD include:

- Repository root.
- Git directory.
- Current HEAD.
- Current branch or detached state.
- Whether the worktree is clean.
- Staged, modified, and untracked file counts.

## 5. Diff Comparison

The CLI MUST support unstaged diff comparison and staged diff comparison.

The CLI SHOULD support:

- Full diff output.
- Diff stats.
- Changed file names.
- Comparison against a caller-provided base ref.

## 6. Commit Validation

The CLI MUST validate commit subjects using `specs/oss-commit.md` rules:

```text
type(scope): subject
```

It MUST reject missing scopes, unsupported types, uppercase subjects, trailing periods, and unscoped conventional commits.

## 7. Changelog Checks

The CLI MUST validate that:

- The canonical changelog file is root `changelog.md`.
- `CHANGELOG.md` is not used as the canonical OSS-WS file.
- The first non-empty line is `# Changelog`.
- Release headings use `## VERSION - YYYY-MM-DD`.

## 8. Phase Checks

The CLI SHOULD check that a dirty phase includes a `changelog.md` update.

The CLI SHOULD report whether the working tree is clean when there is no active phase work.

## 9. Agent Requirements

An AI agent using or updating the CLI SHOULD:

- Keep commands deterministic and script-friendly.
- Avoid shell-specific syntax in Rust command execution.
- Preserve cross-platform path handling.
- Add tests for pure validation logic.
- Update changelog and skill rules when CLI behavior changes.
