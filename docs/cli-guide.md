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
cargo run -p ossws -- --help
```

## Git Inspection

Inspect the current repository:

```text
ossws git inspect
```

The command prints the repository root, git directory, current HEAD, branch, clean state, and file counts.

## Diff Commands

Print the unstaged diff:

```text
ossws git diff
```

Print staged diff stats:

```text
ossws git diff --staged --stat
```

Print changed file names against a base ref:

```text
ossws git files --base main
```

## Analyze Changes

Group changed files by OSS-WS area:

```text
ossws analyze
```

Analyze staged changes:

```text
ossws analyze --staged
```

Analyze changes against a base ref:

```text
ossws analyze --base main
```

## Commit Commands

Validate a commit subject:

```text
ossws commit validate "feat(phase): add draft commit standard"
```

Build a commit subject:

```text
ossws commit build --kind feat --scope phase --subject "add draft cli standard"
```

Build a breaking commit subject:

```text
ossws commit build --kind feat --scope template --subject "require schema version" --breaking
```

## Changelog Checks

Validate root `changelog.md`:

```text
ossws changelog check
```

## Phase Checks

Check whether the current phase is ready:

```text
ossws phase check
```

Run the combined checks:

```text
ossws check --commit-subject "feat(phase): add draft cli standard"
```

## Agent Notes

Use the CLI before committing a phase. It should be safe for humans and agents to run repeatedly because it reads repository state and does not modify files.
