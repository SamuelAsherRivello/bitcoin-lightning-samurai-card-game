# Tasks: Card Selected Modal Backdrop

**Input**: Design documents from `specs/017-card-selected/`  
**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/card-selected-modal-contract.md](./contracts/card-selected-modal-contract.md), [quickstart.md](./quickstart.md)

**Tests**: Include focused model/system tests because the feature changes input classification, modal blocking, and render-layer behavior.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish exact Bevy runtime references and current behavior before edits.

- [X] T001 Inspect `bevy/crates/template-crate` and record applicable Rust/Bevy folder, item, and test patterns for this feature in `specs/017-card-selected/quickstart.md`
- [X] T002 Inspect current selection, gesture, card view, CPU card, point text, and game control scheduling in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T003 Inspect local gesture selection and selected transform behavior in `bevy/crates/game/src/runtime/systems/card_gesture_update_system.rs` and `bevy/crates/game/src/runtime/systems/card_gesture_animation_system.rs`
- [X] T004 Inspect current card state and instance affordance models in `bevy/crates/game/src/runtime/resources/card_slot_model.rs` and `bevy/crates/game/src/runtime/resources/card_instance_state_model.rs`
- [X] T005 [P] Inspect point overlay bundle/components in `bevy/crates/game/src/runtime/bundles/point_view_bundle.rs` and `bevy/crates/game/src/runtime/components/point_view_visual_modifier_component.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Add shared selectable/modal data surfaces required by all stories.

**CRITICAL**: No user story work can begin until this phase is complete.

- [X] T006 Add selected modal state constants and `SelectedCardModalModel` in `bevy/crates/game/src/runtime/resources/selected_card_modal_model.rs`
- [X] T007 Export `SelectedCardModalModel` from `bevy/crates/game/src/runtime/resources/mod.rs`
- [X] T008 Add selectable card source and movement-state components in `bevy/crates/game/src/runtime/components/card_selection_component.rs`
- [X] T009 Export card selection components from `bevy/crates/game/src/runtime/components/mod.rs`
- [X] T010 Add model tests for modal fade progress, max opacity, selected-card click no-op flag, and backdrop-dismiss flag in `bevy/crates/game/src/tests/runtime/resources/selected_card_modal_model_tests.rs`
- [X] T011 Add component tests for selectable eligibility data and stationary-vs-moving state mapping in `bevy/crates/game/src/tests/runtime/components/card_selection_component_tests.rs`
- [X] T012 Register the selected modal resource and new systems in `bevy/crates/game/src/runtime/plugins/mod.rs`

**Checkpoint**: Shared selectable and selected-modal state exists and is registered.

---

## Phase 3: User Story 1 - Select Any Front-Facing Card (Priority: P1) MVP

**Goal**: Players can click any front-facing, stationary human, CPU, near-player, far-player, hand, or location card to inspect it.

**Independent Test**: Start the app, click stationary front-facing cards from local and CPU/near/far hands or locations without dragging, and confirm the clicked card scales to selected inspection.

### Tests for User Story 1

- [X] T013 [P] [US1] Add selectable eligibility tests for front-facing stationary cards and non-selectable moving/back-facing cards in `bevy/crates/game/src/tests/runtime/systems/card_selection_update_system_tests.rs`
- [X] T014 [P] [US1] Add click-versus-drag regression tests preserving `CARD_GESTURE_DRAG_THRESHOLD` behavior in `bevy/crates/game/src/tests/runtime/systems/card_gesture_update_system_tests.rs`
- [X] T015 [P] [US1] Add selected inspection tests for CPU/far/near card sources in `bevy/crates/game/src/tests/runtime/systems/card_selection_update_system_tests.rs`
- [X] T016 [P] [US1] Add selectable inspection tests for front-facing stationary non-GameScene card screens in `bevy/crates/game/src/tests/runtime/systems/card_selection_update_system_tests.rs`

### Implementation for User Story 1

- [X] T017 [US1] Implement `card_selection_update_system` for selectable front-facing stationary card roots in `bevy/crates/game/src/runtime/systems/card_selection_update_system.rs`
- [X] T018 [US1] Export `card_selection_update_system` from `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T019 [US1] Attach selectable source components to local hand and local placed card roots in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T020 [US1] Attach selectable source components to CPU, near-player, and far-player hand/location card roots in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T021 [US1] Attach selectable source components to front-facing stationary non-GameScene card roots such as CardBrowserView, DeckScene, and DebugScene cards in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T022 [US1] Route release-before-threshold selection into `SelectedCardModalModel` while preserving drag start behavior in `bevy/crates/game/src/runtime/systems/card_gesture_update_system.rs`
- [X] T023 [US1] Apply selected inspection target transforms for all selectable card sources in `bevy/crates/game/src/runtime/systems/card_gesture_animation_system.rs`
- [X] T024 [US1] Register `card_selection_update_system` ordering before gesture and hover consumers in `bevy/crates/game/src/runtime/plugins/mod.rs`

**Checkpoint**: User Story 1 is independently functional and testable.

---

## Phase 4: User Story 2 - Modal Backdrop During Selection (Priority: P1)

**Goal**: Selecting a card darkens all lower content with a fullscreen black layer that fades from 0% to 50% opacity over 0.5 seconds while the selected card remains on top.

**Independent Test**: Select a card and verify the backdrop fades in over 0.5 seconds, reaches 50% opacity, and does not dim the selected card.

### Tests for User Story 2

- [X] T025 [P] [US2] Add modal opacity progression tests in `bevy/crates/game/src/tests/runtime/systems/card_selected_modal_update_system_tests.rs`
- [X] T026 [P] [US2] Add selected-card render-order tests for backdrop behind card and above lower content in `bevy/crates/game/src/tests/runtime/systems/card_selected_modal_update_system_tests.rs`

### Implementation for User Story 2

- [X] T027 [US2] Implement modal backdrop spawn/despawn and opacity update in `bevy/crates/game/src/runtime/systems/card_selected_modal_update_system.rs`
- [X] T028 [US2] Export `card_selected_modal_update_system` from `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T029 [US2] Add modal backdrop component markers and purpose comments in `bevy/crates/game/src/runtime/components/card_selection_component.rs`
- [X] T030 [US2] Position and size the backdrop from aspect-ratio-safe GameScene/fullscreen layout helpers in `bevy/crates/game/src/runtime/systems/card_selected_modal_update_system.rs`
- [X] T031 [US2] Ensure selected card transform/render band is above the modal backdrop in `bevy/crates/game/src/runtime/systems/card_gesture_animation_system.rs`
- [X] T032 [US2] Register modal update ordering after selection state changes and before render-dependent point overlay updates in `bevy/crates/game/src/runtime/plugins/mod.rs`

**Checkpoint**: User Story 2 is independently functional and visually testable.

---

## Phase 5: User Story 3 - Modal Blocks Lower Interaction And Dismisses Selection (Priority: P1)

**Goal**: While a card is selected, lower controls and cards cannot hover, click, or drag; clicking the modal backdrop dismisses selection, and clicking the selected card does not.

**Independent Test**: Select a card, attempt lower button/card/location interactions and verify no effect, click the backdrop and verify selection returns, click the selected card and verify it remains selected.

### Tests for User Story 3

- [X] T033 [P] [US3] Add modal backdrop click dismissal tests in `bevy/crates/game/src/tests/runtime/systems/card_selected_modal_update_system_tests.rs`
- [X] T034 [P] [US3] Add selected-card click no-dismiss tests in `bevy/crates/game/src/tests/runtime/systems/card_selected_modal_update_system_tests.rs`
- [X] T035 [P] [US3] Add lower interaction blocking tests for game controls and card gestures in `bevy/crates/game/src/tests/runtime/systems/card_selected_modal_update_system_tests.rs`
- [X] T036 [P] [US3] Add modal blocking tests for egui windows and general Bevy UI controls in `bevy/crates/game/src/tests/runtime/systems/card_selected_modal_update_system_tests.rs`
- [X] T037 [P] [US3] Add point overlay suppression tests for non-selected card `CardPointTextView` entities in `bevy/crates/game/src/tests/runtime/systems/card_point_overlay_selection_update_system_tests.rs`

### Implementation for User Story 3

- [X] T038 [US3] Implement modal input capture and dismissal handling in `bevy/crates/game/src/runtime/systems/card_selected_modal_update_system.rs`
- [X] T039 [US3] Gate card gesture presses, hovers, drop hints, and lower card interactions while modal selection is active in `bevy/crates/game/src/runtime/systems/card_gesture_update_system.rs`
- [X] T040 [US3] Gate game control button interactions while modal selection is active in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T041 [US3] Gate egui windows and general Bevy UI interactions while modal selection is active in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T042 [US3] Prevent selected-card clicks from dismissing selection in `bevy/crates/game/src/runtime/systems/card_selected_modal_update_system.rs`
- [X] T043 [US3] Implement non-selected point text hiding or ordering in `bevy/crates/game/src/runtime/systems/card_point_overlay_selection_update_system.rs`
- [X] T044 [US3] Export `card_point_overlay_selection_update_system` from `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T045 [US3] Register point overlay suppression after `update_card_point_text2d_overlay_system` and selected modal state updates in `bevy/crates/game/src/runtime/plugins/mod.rs`

**Checkpoint**: User Story 3 is independently functional and covers the reported point-number issue.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Verify the feature end to end and tighten documentation.

- [X] T046 [P] Run formatting and fix only 017-related formatting issues in `bevy/crates/game/src/runtime/`
- [X] T047 Run `scripts/other/RunTests.ps1` and address failures related to 017 implementation
- [ ] T048 Run desktop verification with `scripts/main/RunAppDesktop.ps1` or AI runtime screenshot workflow and record results in `specs/017-card-selected/quickstart.md`
- [X] T049 Verify selected-card modal behavior against `specs/017-card-selected/contracts/card-selected-modal-contract.md`
- [ ] T050 Verify browser WebGPU workflow or document the exact blocker in `specs/017-card-selected/quickstart.md`
- [X] T051 Verify changed Bevy runtime files follow one-primary-concept naming, `[domain]_[schedule]_system`, and `HUMAN:`/`AI:` purpose comments in `bevy/crates/game/src/runtime/`

---

## Dependencies & Execution Order

### Phase Dependencies

| Phase | Depends On | Blocks |
| --- | --- | --- |
| Phase 1 Setup | None | Phase 2 |
| Phase 2 Foundational | Phase 1 | All user stories |
| Phase 3 US1 | Phase 2 | Enables broad card selection |
| Phase 4 US2 | Phase 2; can integrate with US1 for full demo | Modal visual behavior |
| Phase 5 US3 | Phase 2; best after US2 | Modal input blocking and point overlay fix |
| Phase 6 Polish | Desired user stories complete | Release readiness |

### User Story Dependencies

| Story | Dependency | Notes |
| --- | --- | --- |
| US1 Select any front-facing card | Foundational | MVP selection path. |
| US2 Modal backdrop | Foundational | Can be tested with a seeded selected modal state, but integrates naturally after US1. |
| US3 Modal blocks interaction | Foundational and modal state | Best implemented after US2 to reuse backdrop/input capture. |

### Parallel Opportunities

| Scope | Parallel Tasks |
| --- | --- |
| Setup inspection | T005 can run with T002-T004. |
| Foundational tests | T010 and T011 can run after T006-T009 file surfaces are known. |
| US1 tests | T013, T014, T015, T016 can run in parallel. |
| US2 tests | T025 and T026 can run in parallel. |
| US3 tests | T033, T034, T035, T036, T037 can run in parallel. |
| Polish | T046 can run independently before full verification; T048-T050 are sequential runtime checks. |

---

## Parallel Example: User Story 1

```text
Task: "Add selectable eligibility tests for front-facing stationary cards and non-selectable moving/back-facing cards in bevy/crates/game/src/tests/runtime/systems/card_selection_update_system_tests.rs"
Task: "Add click-versus-drag regression tests preserving CARD_GESTURE_DRAG_THRESHOLD behavior in bevy/crates/game/src/tests/runtime/systems/card_gesture_update_system_tests.rs"
Task: "Add selected inspection tests for CPU/far/near card sources in bevy/crates/game/src/tests/runtime/systems/card_selection_update_system_tests.rs"
Task: "Add selectable inspection tests for front-facing stationary non-GameScene card screens in bevy/crates/game/src/tests/runtime/systems/card_selection_update_system_tests.rs"
```

## Parallel Example: User Story 3

```text
Task: "Add modal backdrop click dismissal tests in bevy/crates/game/src/tests/runtime/systems/card_selected_modal_update_system_tests.rs"
Task: "Add selected-card click no-dismiss tests in bevy/crates/game/src/tests/runtime/systems/card_selected_modal_update_system_tests.rs"
Task: "Add lower interaction blocking tests for game controls and card gestures in bevy/crates/game/src/tests/runtime/systems/card_selected_modal_update_system_tests.rs"
Task: "Add modal blocking tests for egui windows and general Bevy UI controls in bevy/crates/game/src/tests/runtime/systems/card_selected_modal_update_system_tests.rs"
Task: "Add point overlay suppression tests for non-selected card CardPointTextView entities in bevy/crates/game/src/tests/runtime/systems/card_point_overlay_selection_update_system_tests.rs"
```

---

## Implementation Strategy

### MVP First

| Step | Scope |
| --- | --- |
| 1 | Complete Phase 1 setup. |
| 2 | Complete Phase 2 foundational state/components/tests. |
| 3 | Complete Phase 3 US1 selection coverage. |
| 4 | Stop and validate selecting front-facing stationary cards without changing drag behavior. |

### Incremental Delivery

| Increment | Delivered Behavior |
| --- | --- |
| US1 | Any eligible front-facing stationary card can enter selected inspection. |
| US2 | Selection displays the 50% black modal backdrop behind the selected card. |
| US3 | Modal blocks lower interaction, dismisses by backdrop click, and fixes point overlay rendering over selected cards. |

### Validation Gates

| Gate | Command or Action |
| --- | --- |
| Test gate | `scripts/other/RunTests.ps1` |
| Desktop gate | `scripts/main/RunAppDesktop.ps1` or AI runtime screenshot workflow |
| Contract gate | Compare observed behavior to `specs/017-card-selected/contracts/card-selected-modal-contract.md` |
| Browser gate | Verify browser WebGPU workflow or document blocker |
