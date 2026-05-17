# Tasks: Shared AppScene 3D Camera

**Input**: Design documents from `/specs/024-shared-app-camera/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Runtime tests are required by FR-013 and are listed before implementation tasks for each story where practical.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Confirm existing camera/UI ownership and prepare focused module surfaces for the shared AppScene camera migration.

- [X] T001 Confirm Bevy crate folder and runtime file conventions against `bevy/crates/template-crate`
- [X] T002 [P] Add `app_camera_bundle` module wiring in `bevy/crates/game/src/runtime/bundles/mod.rs`
- [X] T003 [P] Add `app_camera_model` module wiring in `bevy/crates/game/src/runtime/resources/mod.rs`
- [X] T004 Add `app_camera_update_system` module wiring in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T005 Add `shared_overlay_update_system` module wiring in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T006 Inventory all current `Camera2d`, `Camera3d`, `UiTargetCamera`, `IsDefaultUiCamera`, `PrimaryViewCamera`, `ScreenTransitionCamera`, and `CardPointTextCamera` references in `bevy/crates/game/src/runtime/`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Add the shared camera primitives and prove the Bevy UI/shared-3D-camera path before deleting existing cameras.

**CRITICAL**: No user story implementation should begin until this phase confirms the viable shared-camera rendering path.

- [X] T007 Add `AppSceneCamera` marker component and shared-camera overlay/layer marker types in `bevy/crates/game/src/runtime/components/mod.rs`
- [X] T008 Add `AppCameraModel` resource with locked transform, projection, and viewport defaults in `bevy/crates/game/src/runtime/resources/app_camera_model.rs`
- [X] T009 Add `AppCameraBundle` for the single AppScene-owned `Camera3d` in `bevy/crates/game/src/runtime/bundles/app_camera_bundle.rs`
- [X] T010 Add shared camera spawn helper and safe-area viewport helper in `bevy/crates/game/src/runtime/systems/app_camera_update_system.rs`
- [X] T011 Wire `AppCameraModel` initialization and shared camera startup/update systems in `bevy/crates/game/src/runtime/plugins/core_game_plugin.rs`
- [X] T012 Add a focused runtime test proving Bevy UI, text, button interaction, and one 3D mesh can render through one `Camera3d` in `bevy/crates/game/src/tests/runtime/systems/systems_mod_tests.rs`

**Checkpoint**: Shared AppScene 3D camera primitives exist and the UI feasibility test passes.

---

## Phase 3: User Story 1 - Single Shared Runtime Camera (Priority: P1) MVP

**Goal**: Runtime owns exactly one locked AppScene 3D camera and no normal view owns or spawns a camera.

**Independent Test**: Launch or test startup ECS state and confirm one active `Camera3d`, zero `Camera2d`, AppScene ownership, and stable camera entity/transform across all view switches.

### Tests for User Story 1

- [X] T013 [US1] Add runtime test for exactly one active AppScene-owned `Camera3d` and zero `Camera2d` entities after startup in `bevy/crates/game/src/tests/runtime/systems/systems_mod_tests.rs`
- [ ] T014 [US1] Add runtime test that switching Game, Deck, Debug, and meta views preserves the same AppScene camera entity and transform in `bevy/crates/game/src/tests/runtime/systems/systems_mod_tests.rs`
- [X] T015 [US1] Add runtime test that GameScene, DeckScene, DebugScene, and meta scene setup do not spawn view-owned cameras in `bevy/crates/game/src/tests/runtime/systems/systems_mod_tests.rs`

### Implementation for User Story 1

- [ ] T016 [US1] Spawn the shared `AppSceneCamera` from `setup_app_scene` and remove fallback app-camera spawning from view setup in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T017 [US1] Replace `spawn_primary_camera`, `spawn_debug_primary_camera`, `spawn_game_scene_card_camera`, and `spawn_game_scene_card_overlay_camera` call paths with shared AppScene camera usage in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T018 [US1] Remove `Camera3d` ownership expectations from GameScene, DeckScene, and DebugScene setup tests in `bevy/crates/game/src/tests/runtime/systems/systems_mod_tests.rs`
- [X] T019 [US1] Update card selection and picking camera queries from scene-owned `PrimaryViewCamera` to `AppSceneCamera` plus `ActiveView` context in `bevy/crates/game/src/runtime/systems/card_selection_update_system.rs`
- [X] T020 [US1] Update shared camera viewport constraint logic to replace `constrain_game_scene_3d_cameras_to_safe_area`, `constrain_deck_camera_to_safe_area`, and `constrain_debug_camera_to_safe_area` in `bevy/crates/game/src/runtime/systems/app_camera_update_system.rs`
- [X] T021 [US1] Update plugin system ordering to run only the shared AppScene camera constraint system in `bevy/crates/game/src/runtime/plugins/core_game_plugin.rs`

**Checkpoint**: User Story 1 is functional and testable independently with one AppScene-owned 3D camera.

---

## Phase 4: User Story 2 - Visual Parity Across Screens (Priority: P2)

**Goal**: Preserve player-visible layout, scale, safe-area framing, and stacking across Game, Deck, Debug, and meta screens.

**Independent Test**: Compare desktop and browser screenshots for each screen and verify no visible layout or layering regressions.

### Tests for User Story 2

- [X] T022 [US2] Add GameScene hierarchy/parity assertions for world background, location surfaces, hand cards, controls, DebugHUD, and no view-owned camera in `bevy/crates/game/src/tests/runtime/systems/systems_mod_tests.rs`
- [X] T023 [US2] Add DeckScene hierarchy/parity assertions for deck grids, top navigation, preview card, selected-card menu, validation prompt, and no view-owned camera in `bevy/crates/game/src/tests/runtime/systems/systems_mod_tests.rs`
- [X] T024 [US2] Add DebugScene and meta scene hierarchy/parity assertions for navigation, panels, text, controls, and no view-owned camera in `bevy/crates/game/src/tests/runtime/systems/systems_mod_tests.rs`

### Implementation for User Story 2

- [X] T025 [US2] Replace `spawn_game_scene_camera`, `spawn_deck_ui_camera`, `spawn_debug_ui_camera`, and `spawn_meta_ui_camera` with shared camera targeting in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T026 [US2] Update `spawn_game_scene_ui`, `spawn_deck_scene_contents`, `spawn_debug_scene_contents`, and meta scene spawn helpers to use the shared AppScene camera entity for UI roots in `bevy/crates/game/src/runtime/systems/mod.rs`
- [ ] T027 [US2] Update `ModalUiBundle` so modal roots target the shared camera or use a camera-independent root path in `bevy/crates/game/src/runtime/bundles/modal_ui_bundle.rs`
- [X] T028 [US2] Update DeckScene selected-card menu and validation prompt targeting to no longer require per-view `Camera2d` entities in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T029 [US2] Replace DebugHUD active-view UI camera retargeting with one shared-camera target path in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T030 [US2] Update GameScene, DeckScene, DebugScene, and meta screen layout helpers to keep visible positions derived from the aspect-ratio-safe game view in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T031 [US2] Document desktop and browser screenshot parity expectations for each current screen in `specs/024-shared-app-camera/quickstart.md`

**Checkpoint**: User Story 2 preserves visual parity for current screens through the shared camera.

---

## Phase 5: User Story 3 - Camera-Independent Screen Transitions and Overlays (Priority: P3)

**Goal**: Transitions, modals, point labels, debug drawings, and HUD content no longer depend on per-screen 2D cameras.

**Independent Test**: Trigger startup fade, screen transitions, modal prompts, card point labels, and debug drawing with only the shared 3D camera active.

### Tests for User Story 3

- [X] T032 [US3] Add transition overlay coverage test for startup fade and view-switch fade using only `AppSceneCamera` in `bevy/crates/game/src/tests/runtime/systems/systems_mod_tests.rs`
- [X] T033 [US3] Add card point label alignment test without `CardPointTextCamera` in `bevy/crates/game/src/tests/runtime/systems/systems_mod_tests.rs`
- [ ] T034 [US3] Add debug drawing placement test using the shared AppScene camera and active GameScene context in `bevy/crates/game/src/tests/runtime/systems/debug_drawing_update_system_tests.rs`

### Implementation for User Story 3

- [X] T035 [US3] Replace `ScreenTransitionCamera` and transition `UiTargetCamera` retargeting with shared-camera overlay logic in `bevy/crates/game/src/runtime/systems/transition_update_system.rs`
- [X] T036 [US3] Add shared overlay update helpers for a 3D fade plane or shared-camera UI overlay in `bevy/crates/game/src/runtime/systems/shared_overlay_update_system.rs`
- [X] T037 [US3] Update transition overlay bundle usage to remove any `Camera2d` dependency in `bevy/crates/game/src/runtime/bundles/transition_ui_bundle.rs`
- [X] T038 [US3] Remove `CardPointTextCamera` spawn paths and render card point labels through shared-camera-compatible ordering in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T039 [US3] Update card point text render-layer constants and placement helpers for shared camera visibility in `bevy/crates/game/src/runtime/systems/mod.rs`
- [ ] T040 [US3] Update debug drawing camera queries and placement conversion to use `AppSceneCamera` plus GameScene active-view context in `bevy/crates/game/src/runtime/systems/debug_drawing_update_system.rs`
- [ ] T041 [US3] Remove obsolete `ScreenTransitionCamera`, `CardPointTextCamera`, and per-view default UI camera markers from `bevy/crates/game/src/runtime/components/mod.rs`

**Checkpoint**: User Story 3 removes remaining 2D-camera-dependent overlay paths.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Verify parity, remove obsolete camera paths, and update documentation.

- [X] T042 Search for remaining `Camera2d`, `ScreenTransitionCamera`, `CardPointTextCamera`, per-view `PrimaryViewCamera`, and obsolete `UiTargetCamera` usage in `bevy/crates/game/src/runtime/`
- [X] T043 Run `scripts/other/RunTests.ps1` and record automated results in `specs/024-shared-app-camera/quickstart.md`
- [ ] T044 Validate desktop visual parity with `scripts/main/RunAppDesktop.ps1` and record results in `specs/024-shared-app-camera/quickstart.md`
- [ ] T045 Validate browser WebGPU visual parity with `scripts/main/RunAppWeb.ps1` and record results in `specs/024-shared-app-camera/quickstart.md`
- [X] T046 Update GameScreen entity hierarchy notes to report one AppScene camera and no view-owned cameras in `specs/024-shared-app-camera/quickstart.md`
- [ ] T047 Verify changed runtime items include `HUMAN:`/`AI:` comments, `[domain]_[schedule]_system` naming, and one-primary-concept ownership in `bevy/crates/game/src/runtime/`

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 1 -> Phase 2 -> Phase 3 -> Phase 4 -> Phase 5 -> Phase 6
- User stories start only after the shared camera primitives and UI feasibility test are complete.

### User Story Dependencies

- US1 depends on Phase 2 and provides the MVP single-camera architecture.
- US2 depends on US1 because visual parity must be verified against the shared camera.
- US3 depends on US1 and parts of US2 because overlays and labels must render on top of screen content through the shared camera.

### Parallel Opportunities

- T002, T003, T004, and T005 can run in parallel after T001.
- T013, T014, and T015 can be written in parallel before US1 implementation.
- T022, T023, and T024 can be written in parallel before US2 implementation.
- T032, T033, and T034 can be written in parallel before US3 implementation.
- T044 and T045 can run in parallel after automated tests pass if desktop and web workflows are available.

---

## Parallel Example: User Story 2

```text
Task: "T022 [US2] Add GameScene hierarchy/parity assertions for world background, location surfaces, hand cards, controls, DebugHUD, and no view-owned camera in bevy/crates/game/src/tests/runtime/systems/systems_mod_tests.rs"
Task: "T023 [US2] Add DeckScene hierarchy/parity assertions for deck grids, top navigation, preview card, selected-card menu, validation prompt, and no view-owned camera in bevy/crates/game/src/tests/runtime/systems/systems_mod_tests.rs"
Task: "T024 [US2] Add DebugScene and meta scene hierarchy/parity assertions for navigation, panels, text, controls, and no view-owned camera in bevy/crates/game/src/tests/runtime/systems/systems_mod_tests.rs"
```

---

## Implementation Strategy

### MVP First (US1)

1. Complete setup and foundational shared-camera primitives.
2. Add failing runtime tests for single-camera ownership.
3. Spawn one AppScene-owned 3D camera and remove per-view 3D camera usage.
4. Validate US1 independently before changing broad UI and overlay paths.

### Incremental Delivery

1. Deliver US1 single AppScene 3D camera.
2. Deliver US2 visual parity for all current screens.
3. Deliver US3 camera-independent transitions, overlays, labels, and debug drawing.
4. Finish with automated, desktop, and browser verification.

### Risk Control

Keep each step reversible by validating one rendering layer at a time: shared camera, 3D content, UI roots, overlays, labels, then interactions.
