# Tasks: Card Gestures

**Input**: Design documents from `specs/012-card-gestures/`  
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/card-gestures-contract.md`, `quickstart.md`

**Tests**: Include focused model/regression tests where practical because gesture threshold, slot legality, and view navigation changes are high-risk user-visible behavior.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3, US4)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add the tweening dependency and expose focused runtime modules for gesture work.

- [X] T001 Add `bevy_tweening = "0.15"` to `bevy/crates/game/Cargo.toml`
**Reference check**: Use `bevy/crates/template-crate` as the proper reference for Bevy crate folders, representative files, asset folders, and Rust coding standards before adding gesture runtime files.

- [X] T002 Register Bevy Tweening plugin setup in `bevy/crates/game/src/lib.rs`
- [X] T003 [P] Add gesture component module exports in `bevy/crates/game/src/runtime/components/mod.rs`
- [X] T004 [P] Add gesture resource module exports in `bevy/crates/game/src/runtime/resources/mod.rs`
- [X] T005 [P] Add gesture system module exports in `bevy/crates/game/src/runtime/systems/mod.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core gesture and slot state that every user story depends on.

**CRITICAL**: No user story work can begin until this phase is complete.

- [X] T006 [P] Create `CardSlotSide`, `CardSlotState`, `CardSlotModel`, and `CardSlotBoardModel` in `bevy/crates/game/src/runtime/resources/card_slot_model.rs`
- [X] T007 [P] Add model tests for three locations, twenty-four slots, twelve local valid slots, and empty/populated state in `bevy/crates/game/src/runtime/resources/card_slot_model.rs`
- [X] T008 [P] Create `PointerGestureModel`, `CardGestureState`, and `CardGestureModel` in `bevy/crates/game/src/runtime/resources/card_gesture_model.rs`
- [X] T009 [P] Add model tests for one active gesture focus and source-position tracking in `bevy/crates/game/src/runtime/resources/card_gesture_model.rs`
- [X] T010 [P] Create `HandCardGestureTarget`, `CardSlotGestureTarget`, and `CardGestureView` components in `bevy/crates/game/src/runtime/components/card_gesture_component.rs`
- [X] T011 Create initial `card_gesture_update_system` skeleton with required `HUMAN:`/`AI:` comments in `bevy/crates/game/src/runtime/systems/card_gesture_update_system.rs`
- [X] T012 Create initial `card_gesture_animation_system` skeleton with tween target helpers in `bevy/crates/game/src/runtime/systems/card_gesture_animation_system.rs`
- [X] T013 Wire gesture resources and systems into existing app/game setup in `bevy/crates/game/src/runtime/systems/mod.rs`

**Checkpoint**: Gesture and slot models are available, exported, and testable before story-specific behavior begins.

---

## Phase 3: User Story 1 - Inspect a Hand Card In Place (Priority: P1) MVP

**Goal**: Clicking or tapping a local hand card selects it for in-game inspection instead of opening `DeckBuilderScene`, while the existing Deck Builder view implementation remains unchanged.

**Independent Test**: Click or tap a hand card in `GameView`; verify `GameView` remains active, the card animates to center inspection at approximately 90% safe visible height, clicking the selected card returns it to hand position, and no user-facing GameView gesture reaches the Deck Builder view.

### Tests for User Story 1

- [X] T014 [P] [US1] Replace the `clicking_game_card_selects_in_game_without_opening_deck_builder` regression test with GameView in-place inspection and no DeckBuilderScene reachability expectations in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T015 [P] [US1] Add selected-card source/return transition tests in `bevy/crates/game/src/runtime/resources/card_gesture_model.rs`
- [X] T016 [P] [US1] Add selected-inspection transform target tests for safe-height sizing in `bevy/crates/game/src/runtime/systems/card_gesture_animation_system.rs`

### Implementation for User Story 1

- [X] T017 [US1] Change GameView hand-card click handling so it selects `CardGestureModel` instead of setting `ActiveView::DeckBuilderScene` in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T018 [US1] Remove GameView hand-to-DeckBuilderScene navigation without modifying DeckBuilderScene scene setup or behavior in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T019 [US1] Implement selected-inspection tween target calculation matching DeckBuilderScene center pose in `bevy/crates/game/src/runtime/systems/card_gesture_animation_system.rs`
- [X] T020 [US1] Apply selected-card and return tweens to `CardGestureView` entities in `bevy/crates/game/src/runtime/systems/card_gesture_animation_system.rs`
- [X] T021 [US1] Mark spawned local hand card previews with `HandCardGestureTarget` and `CardGestureView` in `bevy/crates/game/src/runtime/systems/mod.rs`

**Checkpoint**: User Story 1 is fully functional and testable independently.

---

## Phase 4: User Story 2 - Distinguish Click From Drag (Priority: P1)

**Goal**: Pointer movement beyond the threshold converts a pressed hand card into a drag and suppresses click inspection for that gesture.

**Independent Test**: Press and release within the threshold to inspect; press and move beyond the threshold to drag; verify the drag path never also triggers inspection.

### Tests for User Story 2

- [X] T022 [P] [US2] Add threshold boundary tests for click, drag, and exact-threshold behavior in `bevy/crates/game/src/runtime/resources/card_gesture_model.rs`
- [X] T023 [P] [US2] Add pointer press/move/release regression tests for drag suppressing inspection in `bevy/crates/game/src/runtime/systems/card_gesture_update_system.rs`

### Implementation for User Story 2

- [X] T024 [US2] Implement pointer press, move, release, and cancel handling in `bevy/crates/game/src/runtime/systems/card_gesture_update_system.rs`
- [X] T025 [US2] Calculate drag threshold from aspect-ratio-safe GameView units in `bevy/crates/game/src/runtime/resources/card_gesture_model.rs`
- [X] T026 [US2] Update drag preview transform while preserving card aspect ratio in `bevy/crates/game/src/runtime/systems/card_gesture_animation_system.rs`
- [X] T027 [US2] Ensure pointer/touch-compatible gesture inputs use the same model path in `bevy/crates/game/src/runtime/systems/card_gesture_update_system.rs`

**Checkpoint**: User Stories 1 and 2 both work independently without gesture conflicts.

---

## Phase 5: User Story 3 - Drag a Hand Card to a Valid Local Slot (Priority: P1)

**Goal**: Dragged hand cards snap into empty local-player slots below the three location areas.

**Independent Test**: Drag a hand card into each empty local-player slot and verify it snaps into place, fits the slot, preserves aspect ratio, and marks the slot populated.

### Tests for User Story 3

- [X] T028 [P] [US3] Add valid local-slot placement tests for all twelve local-player slots in `bevy/crates/game/src/runtime/resources/card_slot_model.rs`
- [X] T029 [P] [US3] Add drag-release placement integration tests in `bevy/crates/game/src/runtime/systems/card_gesture_update_system.rs`
- [X] T030 [P] [US3] Add slot-fit aspect ratio tests in `bevy/crates/game/src/runtime/systems/card_gesture_animation_system.rs`

### Implementation for User Story 3

- [X] T031 [US3] Spawn or mark four local-player slot targets below each GameView location in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T032 [US3] Implement empty local-slot hit-target resolution in `bevy/crates/game/src/runtime/systems/card_gesture_update_system.rs`
- [X] T033 [US3] Implement card snap-to-slot tween target and slot-fit sizing in `bevy/crates/game/src/runtime/systems/card_gesture_animation_system.rs`
- [X] T034 [US3] Mark a local-player slot populated after successful placement in `bevy/crates/game/src/runtime/resources/card_slot_model.rs`
- [X] T035 [US3] Remove or visually suppress the placed card from its prior hand slot after successful placement in `bevy/crates/game/src/runtime/systems/mod.rs`

**Checkpoint**: User Story 3 supports direct local placement into all twelve empty local-player slots.

---

## Phase 6: User Story 4 - Reject Invalid Drag Targets (Priority: P2)

**Goal**: Opponent slots, populated slots, and off-board drops reject placement and return the card to its source.

**Independent Test**: Drag a hand card over opponent-side slots, populated local slots, and empty board space; verify all invalid drops return the card without changing slot state.

### Tests for User Story 4

- [X] T036 [P] [US4] Add opponent-slot rejection tests in `bevy/crates/game/src/runtime/resources/card_slot_model.rs`
- [X] T037 [P] [US4] Add populated-slot and off-board rejection tests in `bevy/crates/game/src/runtime/resources/card_slot_model.rs`
- [X] T038 [P] [US4] Add invalid-drop return integration tests in `bevy/crates/game/src/runtime/systems/card_gesture_update_system.rs`

### Implementation for User Story 4

- [X] T039 [US4] Spawn or mark opponent-side slot targets above each GameView location as non-local targets in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T040 [US4] Reject opponent-side, populated-slot, and no-slot drop targets in `bevy/crates/game/src/runtime/systems/card_gesture_update_system.rs`
- [X] T041 [US4] Animate invalid-drop return to source position and source size in `bevy/crates/game/src/runtime/systems/card_gesture_animation_system.rs`
- [X] T042 [US4] Preserve slot occupancy and hand source state after invalid drops in `bevy/crates/game/src/runtime/resources/card_slot_model.rs`

**Checkpoint**: Invalid drops are rejected without losing cards or corrupting slot state.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Verification, cleanup, and target parity across all gesture stories.

- [X] T043 Run workspace tests with `scripts/other/RunTests.ps1`
- [X] T044 Run desktop gesture verification from `specs/012-card-gestures/quickstart.md`
- [X] T045 Run browser WebGPU gesture verification from `specs/012-card-gestures/quickstart.md` or document the exact blocker in `specs/012-card-gestures/quickstart.md`
- [X] T046 Verify changed Bevy runtime files use `bevy/crates/template-crate` as the proper reference and follow one-primary-concept, Scene/Model/View naming, `[domain]_[schedule]_system` naming, and `HUMAN:`/`AI:` purpose comment standards in `bevy/crates/game/src/runtime/`
- [X] T047 Update `specs/012-card-gestures/quickstart.md` with final verification notes, blocked target details, and confirmation that DeckBuilderScene remains unchanged

---

## Dependencies & Execution Order

### Phase Dependencies

| Phase | Depends On | Blocks |
| ----- | ---------- | ------ |
| Phase 1 Setup | None | Phase 2 |
| Phase 2 Foundational | Phase 1 | All user stories |
| Phase 3 US1 | Phase 2 | MVP and later gesture integration |
| Phase 4 US2 | Phase 2; should preserve US1 | Drag and placement stories |
| Phase 5 US3 | Phase 2 and US2 drag path | Final polish |
| Phase 6 US4 | Phase 2 and US2 drag path; may run alongside US3 after shared slot model exists | Final polish |
| Phase 7 Polish | Desired user stories complete | Release/readiness |

### User Story Dependencies

| User Story | Dependency | Notes |
| ---------- | ---------- | ----- |
| US1 Inspect in place | Foundational only | MVP; removes current DeckBuilderScene navigation from GameView hand cards without editing the Deck Builder view |
| US2 Click versus drag | Foundational only; must not regress US1 | Can be implemented after or alongside US1 once gesture model exists |
| US3 Valid local placement | US2 drag path | Needs drag state and slot model |
| US4 Invalid targets | US2 drag path | Can run alongside US3 after shared slot legality exists |

### Parallel Opportunities

| Area | Parallel Tasks |
| ---- | -------------- |
| Setup exports | T003, T004, T005 |
| Foundational models/components | T006, T008, T010 |
| Foundational tests | T007 and T009 after their respective models |
| US1 tests | T014, T015, T016 |
| US2 tests | T022, T023 |
| US3 tests | T028, T029, T030 |
| US4 tests | T036, T037, T038 |

---

## Parallel Example: User Story 1

```text
Task: "T014 [P] [US1] Replace the clicking_game_card_selects_in_game_without_opening_deck_builder regression test with GameView in-place inspection expectations in bevy/crates/game/src/runtime/systems/mod.rs"
Task: "T015 [P] [US1] Add selected-card source/return transition tests in bevy/crates/game/src/runtime/resources/card_gesture_model.rs"
Task: "T016 [P] [US1] Add selected-inspection transform target tests for safe-height sizing in bevy/crates/game/src/runtime/systems/card_gesture_animation_system.rs"
```

## Parallel Example: User Story 3

```text
Task: "T028 [P] [US3] Add valid local-slot placement tests for all twelve local-player slots in bevy/crates/game/src/runtime/resources/card_slot_model.rs"
Task: "T029 [P] [US3] Add drag-release placement integration tests in bevy/crates/game/src/runtime/systems/card_gesture_update_system.rs"
Task: "T030 [P] [US3] Add slot-fit aspect ratio tests in bevy/crates/game/src/runtime/systems/card_gesture_animation_system.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1 setup.
2. Complete Phase 2 foundational gesture and slot state.
3. Complete Phase 3 US1.
4. Stop and validate in-place card inspection independently.

### Incremental Delivery

1. Add US1 to remove disruptive GameView-to-DeckBuilderScene hand-card clicks while leaving the Deck Builder view implementation unchanged.
2. Add US2 to reliably separate click/tap from drag.
3. Add US3 to place cards into empty local slots.
4. Add US4 to reject invalid targets.
5. Finish with desktop and browser WebGPU verification.

### Parallel Team Strategy

| Contributor | Work |
| ----------- | ---- |
| A | Gesture model and US1 inspection behavior |
| B | Slot model and US3 local placement rules |
| C | Invalid target rejection and cross-target verification |

## Notes

- `[P]` tasks use different files or independent test scopes.
- `[US#]` labels map tasks to user stories for traceability.
- Keep all visible layout derived from the aspect-ratio-safe `GameView`.
- Do not modify the existing DeckBuilderScene implementation; after this feature, users simply have no GameView gesture path to reach it.
- Avoid adding turn legality, energy, CPU placement, reveal rules, scoring resolution, or production mobile packaging in this feature.

