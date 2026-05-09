# Bevy Card Game Constitution

## Core Principles

### I. Project Intent Is The Contract
All implementation work MUST follow the active specification, the project constitution, and the repo-local agent guidance. Bevy-specific rules belong in project specs, README guidance, and repo-local Codex memory.

### II. Clear Project Boundaries
Project behavior belongs in the Bevy source tree under `Bevy/Crates`. Game runtime assets belong under `Bevy/Crates/Game/Assets`; non-game seed assets and repeatable scripts belong under `project/assets` and `project/scripts`.

### III. Visible User Feedback
User-facing workflows SHOULD provide visible loading, progress, success, and failure feedback when work is asynchronous or can fail.

### IV. Local State Stays Explicit
Local files, caches, generated outputs, and user preferences MUST be documented and kept out of version control unless a spec explicitly makes them source artifacts.

### V. Data Changes Are Explicit
Schema changes, migrations, seed data, and destructive data operations MUST be explicit in specs and plans. Destructive database operations are not performed by agents.

### VI. Verify Real Behavior
User-visible changes SHOULD be verified through the real target workflow when practical. Prefer project-local scripts in `project/scripts` after a generated project defines them.

### VII. Implementation Standards
All implementation code MUST follow Rust and Bevy conventions, including explicit ECS ownership through components, resources, systems, and plugins.

Specs and plans SHOULD record any framework-specific constraints before implementation begins.

Generated projects SHOULD keep implementation, tests, documentation, scripts, and assets in clearly named locations.

## Project Constraints

- Keep project-specific source layout documented in `README.md` and active specs.
- Keep reusable project assets under `project/assets` until the generated project defines a more specific asset location.
- Preserve the Codex and Specify workflow files unless a generated project intentionally replaces them.
- Keep `documentation/` updated when README images or supporting docs change.
- Keep `documentation/images/Overview01.png` and `documentation/images/Workflow01.png` as replaceable README image slots.
- Do not introduce unrelated refactors while implementing a feature spec.

## Development Workflow

1. Inspect current files before editing and keep changes scoped to the requested behavior.
2. Prefer project scripts over ad hoc commands for setup, web serving, desktop serving, and tests.
3. For framework or dependency guidance, use official documentation and the project-local Codex rules before changing code.
4. If a port or build artifact is stale or locked, diagnose the actual process or path instead of assuming a clean environment.
5. Treat build output, dependency caches, runtime data, and test output as generated artifacts unless a spec explicitly says otherwise.

## Governance

This constitution applies to all future Spec Kit specifications, plans, and task lists for this repository. Specs may add narrower acceptance criteria, but they must not contradict these principles without explicitly updating this constitution and documenting the reason.

**Version**: 1.1.0 | **Ratified**: 2026-04-30 | **Last Amended**: 2026-05-02
