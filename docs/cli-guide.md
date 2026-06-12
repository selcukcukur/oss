# CLI Guide

Status: Draft
Standard: `specs/oss-cli.md`

## Install From Source

Build the CLI from the repository root:

```text
cargo build
```

Run it during development:

```text
cargo run -- --help
```

## Git Inspection

Inspect the current repository:

```text
oss-ws git inspect
```

The command prints the repository root, git directory, current HEAD, branch, clean state, and file counts.

## Diff Commands

Print the unstaged diff:

```text
oss-ws git diff
```

Print staged diff stats:

```text
oss-ws git diff --staged --stat
```

Print changed file names against a base ref:

```text
oss-ws git files --base main
```

## Commit Commands

Validate a commit subject:

```text
oss-ws commit validate "feat(phase): add draft commit standard"
```

Build a commit subject:

```text
oss-ws commit build --kind feat --scope phase --subject "add draft cli standard"
```

Build a breaking commit subject:

```text
oss-ws commit build --kind feat --scope template --subject "require schema version" --breaking
```

## Changelog Checks

Validate root `changelog.md`:

```text
oss-ws changelog check
```

## Phase Checks

Check whether the current phase is ready:

```text
oss-ws phase check
```

Run the combined checks:

```text
oss-ws check --commit-subject "feat(phase): add draft cli standard"
```

## Agent Notes

Use the CLI before committing a phase. It should be safe for humans and agents to run repeatedly because it reads repository state and does not modify files.
