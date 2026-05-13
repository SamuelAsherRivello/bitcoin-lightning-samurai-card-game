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
| Performance Goals | DebugHUD update is lightweight and frame-local; FPS text is sampled on a 0.5 second interval; debug drawing should add only simple marker geometry or overlay primitives |
| Constraints | Keep reusable DebugHUD, inspector, and diagnostic input behavior under `bevy/crates/shared`; keep Card UI and scene-specific debug drawings under `bevy/crates/game`; keep overlays and debug drawings positioned from the aspect-ratio-safe game view; keep debug drawings temporary; keep scripts under `scripts/`; no gameplay HUD systems |
| Scale/Scope | One HUD panel, one FPS toggle, one inspector toggle, Card UI separation guidance, scoped terminal logs, temporary runtime debug drawings, automated tests for creation and input behavior |

## Constitution Check

| Gate | Status | Notes |
| ---- | ------ | ----- |
| Active spec and repo guidance followed | ✅ | Implementation follows `003-debugging` and repo-local AGENTS guidance |
| Source, scripts, and tests stay in approved locations | ✅ | Reusable DebugHUD runtime code is under `bevy/crates/shared`; game app code under `bevy/crates/game` only composes it |
| Visible feedback requirements respected | ✅ | DebugHUD is visible by default; Card UI remains separate; no unrelated loading/toast systems added |
| Browser/local storage constraints | ✅ | No storage, SQLite, OPFS, or browser-only state added |
| Real behavior verification path | ✅ | Desktop build and tests use repository scripts; browser WebGPU verification is documented in quickstart |
| Rust and Bevy ECS standards | ✅ | State is represented with components, resources, systems, and plugins; changed runtime files stay focused on one primary runtime concept |
| Aspect-ratio-safe layout | ✅ | DebugHUD, Card UI, and debug drawings derive placement from the aspect-ratio-safe game view rather than raw window pixels |
| Target parity risk documented | ✅ | Desktop is the fast iterative target; browser WebGPU must be verified before final acceptance or documented with an exact blocker |

## Project Structure

```text
bevy/crates/shared/src/runtime/
├── components/
│   ├── debug_hud_component.rs      # DebugHUD panel and key-label markers
│   └── inspector_component.rs      # Shared inspector marker/state components
├── plugins/
│   └── debug_tooling_plugin.rs     # Shared diagnostics plugin wiring
├── resources/
│   ├── debug_hud_model.rs          # DebugHUD state and FPS sample data
│   └── debug_input_model.rs        # Approved debug input classification/state
└── systems/
    ├── debug_hud_setup_system.rs   # Spawn DebugHUD under the aspect-ratio HUD root
    ├── debug_hud_update_system.rs  # Update status, FPS, and key-label presentation
    ├── debug_input_update_system.rs # Capture approved diagnostic key state
    └── inspector_update_system.rs  # Toggle/render inspector diagnostic window

bevy/crates/game/src/runtime/
├── components/
│   ├── card_ui_component.rs        # Temporary card-specific developer UI markers
│   └── debug_drawing_component.rs  # Game-scene debug drawing markers
├── resources/
│   └── debug_drawing_model.rs      # Requested scene-area mark metadata
└── systems/
    ├── card_ui_update_system.rs    # Temporary Card UI presentation behavior
    └── debug_drawing_update_system.rs # Aspect-ratio-safe scene-area marks

scripts/
├── main/
│   └── RunAppDesktop.ps1    # Windows desktop run entry point
└── other/
    ├── RunTests.ps1         # Automated test suite entry point
    └── StopApp.ps1          # Stops running project app/build processes
```

**Structure Decision**: Keep DebugHUD, inspector, and diagnostic input capture in `bevy/crates/shared` because they are reusable system-level diagnostics. Keep Card UI and debug drawing implementation in `bevy/crates/game` because they are card/game-scene-specific prototype surfaces, while shared diagnostics may expose only reusable state or conventions if needed later. Use `bevy/crates/template-crate` as the proper reference for Bevy crate folders, representative files, asset folders, and Rust coding standards. DebugHUD and Card UI should spawn under the aspect-ratio HUD root, and debug drawings should derive target placement from the aspect-ratio-safe `GameScene`, `DeckScene`, or `DebugScene` rather than raw window pixels or ad hoc world coordinates. Changed runtime files should each focus on one primary concept and new system functions should use `[domain]_[schedule]_system` names with the required `HUMAN:` and `AI:` purpose comments. Terminal logging should stay scoped to diagnostics being inspected. Do not add a separate UI framework, gameplay subsystem, asset pipeline, or persistence layer for this feature.

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
| No target-specific code introduced | ✅ | Bevy UI, input APIs, and aspect-ratio-safe placement are shared across desktop and browser targets |
| Constitution implementation standards reflected | ✅ | Plan uses `bevy/crates/template-crate` as the proper reference plus one-primary-concept runtime files, `[domain]_[schedule]_system` names, and required purpose comments for future runtime changes |
| Verification documented | ✅ | Tests, desktop build/run, safe-area layout checks, debug drawing checks, and browser WebGPU verification commands are documented |
