# Implementation Plan: Debugging

**Branch**: `003-debugging` | **Date**: 2026-05-09 | **Spec**: `specs/003-debugging/spec.md`
**Input**: Feature specification from `specs/003-debugging/spec.md`

## Summary

Add developer-facing debugging tooling to the Bevy card inspection prototype. The implementation keeps the rendered DebugHUD and inspector diagnostics contained in the Bevy runtime ECS modules, shows one translucent top-left DebugHUD panel by default, exposes `F` and `I` diagnostic toggles, leaves `WASD` as visible non-functional legend labels, keeps temporary Card UI separate from DebugHUD, supports scoped terminal self-logging, supports temporary runtime debug drawing around requested scene areas, and excludes toast, minimap, reticle, movement, shooting, health, score, and other gameplay HUD behavior.

## Technical Context

| Item | Decision |
| ---- | -------- |
| Language/Version | Rust 2024 workspace |
| Primary Dependencies | Bevy 0.18.1, bevy-inspector-egui 0.36.0 |
| Storage | N/A |
| Testing | `scripts/other/RunTests.ps1` runs `cargo test --workspace` |
| Target Platform | Windows desktop now; browser WebGPU remains a required final verification target |
| Project Type | Bevy ECS desktop/browser game prototype |
| Performance Goals | HUD update is lightweight and frame-local; FPS text is sampled on a 0.5 second interval |
| Constraints | Keep reusable DebugHUD, inspector, and diagnostic input behavior under `bevy/crates/shared`; keep Card UI separate from DebugHUD; keep debug drawings temporary; keep scripts under `scripts/`; no gameplay HUD systems |
| Scale/Scope | One HUD panel, one FPS toggle, one inspector toggle, Card UI separation guidance, scoped terminal logs, temporary runtime debug drawings, automated tests for creation and input behavior |

## Constitution Check

| Gate | Status | Notes |
| ---- | ------ | ----- |
| Active spec and repo guidance followed | ✅ | Implementation follows `003-debugging` and repo-local AGENTS guidance |
| Source, scripts, and tests stay in approved locations | ✅ | Reusable DebugHUD runtime code is under `bevy/crates/shared`; game app code under `bevy/crates/game` only composes it |
| Visible feedback requirements respected | ✅ | DebugHUD is visible by default; Card UI remains separate; no unrelated loading/toast systems added |
| Browser/local storage constraints | ✅ | No storage, SQLite, OPFS, or browser-only state added |
| Real behavior verification path | ✅ | Desktop build and tests use repository scripts; browser WebGPU verification is documented in quickstart |
| Rust and Bevy ECS standards | ✅ | State is represented with components, resources, systems, and plugins |
| Target parity risk documented | ✅ | Desktop is verified in this iteration; browser WebGPU must be verified separately if toolchain is unavailable |

## Project Structure

```text
bevy/crates/shared/src/runtime/
├── components.rs        # DebugHUD, key legend, inspector, Card UI markers, debug drawing markers, and diagnostic marker components
├── plugins.rs           # Shared diagnostics plugin wiring and automated tests
├── resources.rs         # Game tick and DebugHUD state resources
└── systems.rs           # DebugHUD setup/update, inspector toggle/UI, input capture, debug drawing, terminal logging hooks, and scaling

bevy/crates/game/src/
└── runtime/             # Game-specific card systems that compose shared diagnostics

scripts/
├── main/
│   ├── RunTests.ps1         # Automated test suite entry point
│   └── RunAppDesktop.ps1    # Windows desktop run entry point
└── other/
    ├── RunTests.ps1         # Test implementation
    └── StopApp.ps1          # Stops running project app/build processes
```

**Structure Decision**: Keep DebugHUD, inspector, and diagnostic input capture in `bevy/crates/shared` because they are reusable system-level diagnostics. `bevy/crates/game` should only compose the shared diagnostics with card-specific systems and keep temporary Card UI clearly separate from DebugHUD. Debug drawings should be implemented as runtime-only visual annotations scoped to requested game scene areas, and should remain until removal is requested while still being treated as temporary scaffolding. Terminal logging should stay scoped to diagnostics being inspected. Do not add a separate UI framework, gameplay subsystem, asset pipeline, or persistence layer for this feature.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
| --------- | ---------- | ------------------------------------ |
| N/A | N/A | N/A |

## Phase 0 Research

See `specs/003-debugging/research.md`.

## Phase 1 Design

See `specs/003-debugging/data-model.md`, `specs/003-debugging/contracts/debughud-ui.md`, and `specs/003-debugging/quickstart.md`.

## Post-Design Constitution Check

| Gate | Status | Notes |
| ---- | ------ | ----- |
| No unresolved clarifications | ✅ | Spec clarifications define HUD content, `F`/`I`, and non-functional `WASD` |
| No target-specific code introduced | ✅ | Bevy UI and input APIs are shared across desktop and browser targets |
| Verification documented | ✅ | Tests, desktop build/run, and browser WebGPU verification commands are documented |
