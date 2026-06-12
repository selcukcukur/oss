# Changelog Noise Rules

Status: Draft
Standard: `../../../../specs/oss-changelog.md`

## Critical Rule

The changelog SHOULD include changes that affect consumers, compatibility, documentation trust, generated output, agent behavior, or release decisions, and SHOULD exclude routine internal noise.

## Usually Exclude

- Formatting-only edits.
- Local editor configuration.
- Minor dotfile updates.
- Development-only dependency bumps with no consumer impact.
- Internal cleanup that cannot affect released behavior.

## Usually Include

- Refactors with possible behavior or compatibility impact.
- Changes to supported runtimes, platforms, tools, or schemas.
- New documentation for previously undocumented behavior.
- Changes to generated templates, agent instructions, or release workflows.
- Fixes to examples that users or agents may copy.
- Dependency changes that alter runtime behavior, security posture, or compatibility.

## Agent Check

If a change looks like maintenance but affects how humans or agents use the project, include it and describe the user-facing impact.
