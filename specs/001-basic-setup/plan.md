# Implementation Plan: Basic Setup

**Branch**: `001-basic-setup` | **Date**: 2026-05-09 | **Spec**: `specs/001-basic-setup/spec.md`
**Input**: Feature specification from `specs/001-basic-setup/spec.md`

## Summary

Provide repeatable build, test, and desktop run entry points; support VS Code task execution in the integrated terminal; default the desktop app to an 800x600 window; and persist desktop window size, x/y position, and screen identity as ignored local runtime state that restores on the next desktop launch.

## Technical Context

| Item | Decision |
| ---- | -------- |
| Language/Version | Rust 2024 workspace |
| Primary Dependencies | Bevy 0.18.1; serde and serde_json for local placement state |
| Storage | Ignored local JSON under `generated/runtime/window-placement.json` |
| Testing | `scripts/main/RunTests.ps1` runs `cargo test --workspace` |
| Target Platform | Windows desktop primary for placement; browser WebGPU must continue to start without desktop placement dependency |
| Project Type | Bevy desktop/browser game prototype |
| Performance Goals | Placement load/save is tiny local file IO only at startup and normal close |
| Constraints | Keep scripts under `scripts`; keep local runtime output under ignored `generated/`; do not add card, DebugHUD, or gameplay behavior for this feature |
| Scale/Scope | One primary desktop window, two-screen restore support, repository scripts, VS Code tasks |

## Constitution Check

| Gate | Status | Notes |
| ---- | ------ | ----- |
| Active spec and repo guidance followed | ✅ | Implements `001-basic-setup` behavior only |
| Source, scripts, and state stay in approved locations | ✅ | Runtime code under `Bevy/Crates/Game`; scripts under `scripts`; local state under ignored `generated/` |
| Visible feedback requirements respected | ✅ | Script output remains visible in terminal workflows |
| Browser/local storage constraints | ✅ | Desktop placement state is file-based and does not introduce browser persistence |
| Real behavior verification path | ✅ | Test script and desktop run script are verified locally |
| Rust and Bevy ECS standards | ✅ | Placement tracking is implemented as resources and systems |
| Target parity risk documented | ✅ | Desktop placement applies only to desktop; browser startup must not depend on it |

## Project Structure

```text
Bevy/Crates/Game/src/
├── main.rs                         # Window default and startup placement
└── runtime/
    ├── plugins/mod.rs              # Core plugin system wiring
    ├── resources/mod.rs            # Window placement state and file IO
    └── systems/mod.rs              # Window restore, tracking, and close-save systems

scripts/
├── main/
│   ├── InstallDependencies.ps1
│   ├── RunAppDesktop.ps1
│   └── RunTests.ps1
└── other/
    ├── RunTests.ps1
    └── StopApp.ps1

.vscode/
└── tasks.json                      # Local VS Code task entries
```

**Structure Decision**: Keep placement behavior in the existing Bevy runtime. Do not introduce app-wide settings screens, card rendering, DebugHUD behavior, gameplay behavior, or a broader configuration framework.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
| --------- | ---------- | ------------------------------------ |
| N/A | N/A | N/A |

## Phase 0 Research

See `specs/001-basic-setup/research.md`.

## Phase 1 Design

See `specs/001-basic-setup/data-model.md`, `specs/001-basic-setup/contracts/window-placement.md`, and `specs/001-basic-setup/quickstart.md`.

## Post-Design Constitution Check

| Gate | Status | Notes |
| ---- | ------ | ----- |
| No unresolved clarifications | ✅ | Restore, fallback, and save timing are clarified |
| Local state remains explicit | ✅ | Placement state path and ignored status are documented |
| Verification documented | ✅ | Scripts, unit tests, and desktop build checks are listed |
