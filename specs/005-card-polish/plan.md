# Implementation Plan: Card Polish

**Branch**: `005-card-polish` | **Date**: 2026-05-09 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/005-card-polish/spec.md`

## Summary

Implement a flat inspectable Bevy card that reads as four apparent-depth layers through `CardStructure` behavior and generated `CardType` artwork. Replace manual dot/rectangle placeholder art with generated static textures loaded from `bevy/crates/game/assets/cards/card_types/card_type_skybolt/` and `bevy/crates/game/assets/cards/card_types/card_type_tar/`, preserve the existing pointer-driven tilt/parallax flow, add a tilt-reactive frame shine, expose a HUD `T` card type toggle that cycles available card types, and carry forward the `R` AppScene reload plus persisted `H` hot-reload auto-restart workflow from the related `bevy-zoo-game` specs.

## Technical Context

**Language/Version**: Rust 2024  
**Primary Dependencies**: Bevy 0.18.1, bevy-inspector-egui, bevy-persistent, serde  
**Storage**: Runtime PNG assets under `bevy/crates/game/assets/cards/card_types/card_type_skybolt/` and `bevy/crates/game/assets/cards/card_types/card_type_tar/`; card structure asset placeholder under `bevy/crates/game/assets/cards/card_structure/`; local DebugHUD input state under `data/local_storage/debug-hud-input.json`  
**Testing**: `scripts/other/RunTests.ps1`, `scripts/other/RunAppDesktop.ps1 -CheckOnly`, `scripts/other/RunAppWeb.ps1 -CheckOnly`  
**Target Platform**: Windows desktop and browser WebGPU  
**Project Type**: Bevy Rust workspace application  
**Performance Goals**: One inspectable card, a small fixed number of textured planes, no per-frame entity churn  
**Constraints**: Preserve the 004 thin-slab inspection feel; use flat 2D visual layers; keep generated assets in the Bevy asset tree; do not introduce gameplay or multi-card flows  
**Scale/Scope**: One centered card, two available CardTypes, two card type registry slots

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- The implementation follows the active spec, constitution, and repo-local AGENTS guidance.
- Source remains under `bevy/crates/game/src/runtime/`; runtime assets remain under `bevy/crates/game/assets/`.
- Rust source and asset directories use lowercase snake_case paths.
- No database, template data, browser SQLite, OPFS, or schema behavior is introduced.
- Browser-visible behavior has a practical WebGPU check path through `scripts/other/RunAppWeb.ps1 -CheckOnly`.
- Bevy-specific constraints are documented: `StandardMaterial` texture loading, alpha blending for transparent PNG layers, and ECS-owned components/resources/systems.
- Imported `R`/`H` behavior is limited to reloadable `AppScene` scene content and DebugHUD input persistence; model browser and other `bevy-zoo-game` systems remain out of scope.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit-plan command output)
├── research.md          # Phase 0 output (/speckit-plan command)
├── data-model.md        # Phase 1 output (/speckit-plan command)
├── quickstart.md        # Phase 1 output (/speckit-plan command)
└── tasks.md             # Phase 2 output (/speckit-tasks command - NOT created by /speckit-plan)
```

### Source Code (repository root)

```text
bevy/crates/game/
├── assets/cards/card_structure/
│   └── .gitkeep
├── assets/cards/card_types/card_type_skybolt/
├── assets/cards/card_types/card_type_tar/
│   ├── background_clouds.png
│   ├── frame_pinstripe.png
│   ├── foreground_character.png
│   └── title_skybolt.png
└── src/runtime/
    ├── components/mod.rs
    ├── plugins/mod.rs
    ├── resources/mod.rs
    └── systems/mod.rs
```

**Structure Decision**: Use the existing Bevy runtime ECS layout and add no new crates. Keep tests as module unit tests beside the runtime systems/resources they validate.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | N/A | N/A |
