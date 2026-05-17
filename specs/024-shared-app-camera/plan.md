# Implementation Plan: Shared AppScene 3D Camera

**Branch**: `[024-shared-app-camera]` | **Date**: 2026-05-18 | **Spec**: [spec.md](spec.md)  
**Input**: Feature specification from `specs/024-shared-app-camera/spec.md`

## Summary

Replace the current per-screen camera model with one locked 3D camera owned by `AppScene`. Game, Deck, Debug, and meta screens will render through that shared camera while preserving the current player-visible layout, safe-area framing, overlays, transitions, card interactions, and desktop/browser parity. The migration should first prove the Bevy UI/shared-3D-camera path, then remove per-scene 3D cameras, then remove all 2D cameras and camera-retarget systems.

## Technical Context

**Language/Version**: Rust 2024, Bevy 0.18.1  
**Primary Dependencies**: Bevy ECS, Bevy UI, Bevy 3D camera/render layers, existing aspect-ratio-safe layout helpers, existing transition/modal/HUD systems  
**Storage**: N/A  
**Testing**: Focused runtime tests plus `scripts/other/RunTests.ps1`; desktop check through `scripts/main/RunAppDesktop.ps1`; browser check through `scripts/main/RunAppWeb.ps1`  
**Target Platform**: Windows desktop and browser WebGPU  
**Project Type**: Bevy ECS game runtime in `bevy/crates/game`  
**Performance Goals**: No added frame hitch during screen transitions or view switches; one camera should reduce render scheduling overhead rather than increase it  
**Constraints**: One active AppScene-owned `Camera3d`; zero runtime `Camera2d` entities; user-visible output remains unchanged; all placement derives from the 1280x800 aspect-ratio-safe game view  
**Scale/Scope**: Current GameScene, DeckScene, DebugScene, Main Menu, Lightning, Matchmaking, Settings, screen transitions, DebugHUD, modals, point labels, debug drawing, and card interactions

## Constitution Check

| Gate | Status | Notes |
| ---- | ------ | ----- |
| Active spec and repo-local guidance | ✅ | Spec is `specs/024-shared-app-camera/spec.md`; this plan is scoped to rendering architecture and parity. |
| Source/assets/scripts/docs/tests locations | ✅ | Runtime source stays under `bevy/crates/game/src/runtime`; docs stay under `specs/024-shared-app-camera`. |
| Bevy template and Rust naming conventions | ✅ | New runtime files should use lowercase module names and project ECS ownership conventions. |
| One primary concept per changed runtime file | ✅ | Shared camera configuration, overlay conversion, and camera constraint logic should be split into focused files where practical. |
| HUMAN/AI purpose comments | ✅ | New or changed primary runtime items must include two-line `HUMAN:`/`AI:` comments. |
| System naming | ✅ | New or changed systems should use names such as `app_camera_update_system` and `shared_overlay_update_system`. |
| Scene/Model/View naming | ✅ | `AppScene` owns the camera; active views remain GameScene, DeckScene, DebugScene, and meta scenes. |
| Theme asset placement | ✅ | No theme asset changes are planned. |
| Browser-visible verification path | ✅ | Browser WebGPU verification is required because camera/UI behavior can differ by target. |
| Safe-area 2D/3D positioning | ✅ | Existing virtual game-view coordinates remain the source of truth for visual parity. |
| Framework API constraints documented | ✅ | Research records the Bevy UI, `UiTargetCamera`, camera order, and render layer risks. |

## Project Structure

### Documentation

```text
specs/024-shared-app-camera/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── shared-camera-rendering-contract.md
├── checklists/
│   └── requirements.md
└── tasks.md
```

### Source Code

```text
bevy/crates/game/src/runtime/
├── components/
│   └── mod.rs
├── resources/
│   ├── app_camera_model.rs
│   └── mod.rs
├── bundles/
│   ├── app_camera_bundle.rs
│   └── mod.rs
├── systems/
│   ├── app_camera_update_system.rs
│   ├── shared_overlay_update_system.rs
│   └── mod.rs
└── plugins/
    └── core_game_plugin.rs

bevy/crates/game/src/tests/runtime/
└── systems/
    └── systems_mod_tests.rs
```

**Structure Decision**: Add focused runtime files for the shared camera model, bundle, and update systems. Existing `systems/mod.rs` currently owns much of scene spawning, so implementation may initially touch it to remove camera spawns and route UI/overlay setup, but new long-lived camera behavior should live in dedicated files where practical.

## Migration Plan

| Phase | Goal | Key Changes | Exit Criteria |
| --- | --- | --- | --- |
| 0 | Prove shared-camera UI path | Verify Bevy UI roots can render and receive interaction when targeted to one `Camera3d` | Minimal test or spike shows UI, text, buttons, and 3D card render through one camera |
| 1 | Add AppScene camera | Spawn one locked `AppSceneCamera` during `setup_app_scene` with centralized defaults | App starts with the shared camera and existing screens still render |
| 2 | Move 3D content to shared camera | Remove per-view 3D camera creation and update camera queries to use AppScene camera | Game, Deck, Debug 3D content renders through shared camera |
| 3 | Move UI content to shared camera | Replace per-screen `Camera2d` spawns and `UiTargetCamera` retargeting | No `Camera2d` entities remain in normal runtime |
| 4 | Replace camera-order overlays | Convert transition, card point text, and modal ordering to shared-camera-compatible Z/layer policy | Transitions and overlays still cover or align correctly |
| 5 | Update interaction math | Use shared camera plus active-view context for picking, gestures, debug drawing, and viewport conversions | Card interaction and debug drawing tests pass |
| 6 | Verify parity | Run tests and visual checks on desktop/browser | Quickstart records automated, desktop, and browser results |

## Challenges And Solutions

| Challenge | Risk | Solution |
| --- | --- | --- |
| Bevy UI currently depends on `Camera2d` and `UiTargetCamera` | Removing 2D cameras can make UI disappear or stop receiving interaction | First prove whether UI can target the shared `Camera3d`; if not, convert blocking overlays to 3D-facing panel/text views while preserving behavior |
| Transition overlay uses a dedicated 2D transition camera | Fade may no longer cover all content | Prefer a 3D full-screen fade plane placed closest to the AppScene camera; drive alpha from `ScreenTransitionResource` |
| Camera order currently separates world, UI, cards, point text, and transition layers | Shared camera can change stacking | Replace order values with explicit Z-depth, render layers visible to the shared camera, and `GlobalZIndex` for retained UI |
| Card point labels use a separate `CardPointTextCamera` | Text can misalign or layer incorrectly | Render labels through the shared camera using the existing safe-area/world conversion helpers and a fixed overlay depth |
| DebugHUD retargets to active view UI camera | HUD can lose target after removing per-view cameras | Remove active-view camera selection and attach or target HUD to the shared AppScene camera once |
| Camera queries are scene-scoped | Existing systems may fail when no camera has `GameSceneEntity`, `DeckSceneEntity`, or `DebugSceneEntity` | Introduce an `AppSceneCamera` marker and pass active view separately where behavior differs by screen |
| Viewport constraints are duplicated by scene | Fullscreen/browser resizing can regress | Collapse to one shared-camera viewport constraint system and keep existing aspect-ratio-safe calculations |

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
| --------- | ---------- | ------------------------------------ |
| None | N/A | N/A |
