# Changelog Noise

## Contents

- Usually exclude
- Usually include
- Maintenance impact

---

## Usually Exclude

Leave out changes that do not affect consumers, maintainers, or agents:

- Formatting-only edits.
- Local editor configuration.
- Minor dotfile updates.
- Development-only dependency bumps with no consumer impact.
- Internal cleanup that cannot affect released behavior.

---

## Usually Include

Include changes that affect:

- Compatibility.
- Supported runtimes, platforms, tools, or schemas.
- Generated templates or release workflows.
- Agent behavior.
- Public documentation users rely on.
- Examples that users or agents may copy.
- Dependency behavior, security posture, or compatibility.

---

## Maintenance Impact

If a maintenance-looking change affects how humans or agents use the project, include it and describe the user-facing impact.
