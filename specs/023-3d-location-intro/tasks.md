# Tasks: 3D Location Intro

## Phase 1: Planning

- [X] T001 Create implementation plan matching latest GameScene code in `specs/023-3d-location-intro/plan.md`.
- [X] T002 Create research, data model, contract, and quickstart artifacts for the 3D location intro.

## Phase 2: Tests

- [X] T003 Update GameScene location ownership tests for 3D `LocationBundle` surfaces, UI overlays, and depth ordering.
- [X] T004 Add intro timing tests for 0% to 100% opacity, 150% to 100% scale, 0.5-second duration, and 0.5-second post-completion waits.

## Phase 3: Implementation

- [X] T005 Add `LocationBundle` marker and intro components in `bevy/crates/game/src/runtime/components/mod.rs`.
- [X] T006 Spawn 3D location rectangle surfaces from `CardSlotBoardModel::location_area_rect` in `bevy/crates/game/src/runtime/systems/mod.rs`.
- [X] T007 Keep existing location title, body, border, and point views as a safe-area overlay attached to each location.
- [X] T008 Implement `location_intro_update_system` to synchronize surface material opacity, overlay scale, and overlay child opacity.
- [X] T009 Wire `location_intro_update_system` into `CoreGamePlugin`.

## Phase 4: Validation

- [X] T010 Run focused cargo tests for GameScene ownership and location intro timing.
- [X] T011 Run broader project test workflow or document the blocker.

## Validation Notes

| Check | Result |
| ----- | ------ |
| `cargo test -p samurai-card-game location_intro_reveals_locations_in_sequence` | Passed |
| `cargo test -p samurai-card-game game_scene_owns_camera_world_background_and_three_locations` | Passed |
| `scripts/other/RunTests.ps1` | Passed: 323 game lib tests, 3 game binary tests, and 13 shared lib tests. |
