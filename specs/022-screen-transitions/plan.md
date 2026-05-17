# Implementation Plan: Screen Transitions

**Branch**: `main` | **Date**: 2026-05-17 | **Spec**: `specs/022-screen-transitions/spec.md`
**Input**: Feature specification from `specs/022-screen-transitions/spec.md`

## Summary

Add a topmost fullscreen transition overlay that starts black and fades in on app load, and wraps all view changes with fade-out -> switch-at-black -> hold-at-black -> fade-in using a default black color, 1.0 seconds fade time, and a 0.2 second full-black hold.

## Technical Context

**Language/Version**: Rust (workspace edition in current Bevy project)
**Primary Dependencies**: Bevy ECS/UI, existing runtime plugins/resources/systems under `bevy/crates/game/src/runtime/`
**Storage**: N/A
**Testing**: `scripts/other/RunTests.ps1`
**Target Platform**: Windows desktop + browser WebGPU
**Project Type**: Single Bevy runtime workspace
**Performance Goals**: Keep transitions lightweight (single fullscreen UI node + simple alpha updates); no observable frame hitch during the 1.2s full cycle.
**Constraints**: Preserve AppScene + ActiveView architecture, keep overlay in safe-area HUD stack, no gameplay behavior changes
**Scale/Scope**: One new transition model + one transition view layer + integration into existing view-switch path

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Feature follows active spec, constitution, and repo-local guidance: PASS
- Source/assets/scripts/docs/tests stay in required locations: PASS
- Bevy structure references `bevy/crates/template-crate`: PASS
- Lowercase Rust/bevy folder conventions preserved: PASS
- Runtime file ownership and purposeful names enforced: PASS
- `HUMAN:`/`AI:` two-line comments required on changed primary runtime items: PASS (implementation task)
- System names use `[domain]_[schedule]_system`: PASS (implementation task)
- Scene/Model/View naming conventions respected: PASS
- Theme asset rules unaffected (no theme assets changed): PASS
- Loading/toast/browser-localStorage/native DB constraints unaffected: PASS
- Browser-visible verification path required: PASS (`RunAppWeb.ps1` + manual transition check)
- Aspect-ratio-relative placement required for visible overlay: PASS
- Framework/API constraints captured in this plan: PASS

## Project Structure

### Documentation (this feature)

```text
specs/022-screen-transitions/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── screen-transition-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
bevy/crates/game/src/runtime/
├── resources/
│   └── mod.rs                         # transition state model + config values
├── components/
│   └── mod.rs                         # transition overlay marker component(s)
├── bundles/
│   └── transition_ui_bundle.rs        # fullscreen transition UI bundle (new)
├── systems/
│   └── transition_update_system.rs    # alpha animation + switch handoff (new)
└── plugins/
    └── core_game_plugin.rs            # register resource/startup/system order
```

**Structure Decision**: Implement as a small runtime vertical slice (resource + bundle + system + plugin wiring) under existing `bevy/crates/game/src/runtime/` modules to align with current ECS organization.

## Complexity Tracking

No constitution violations identified.
