# Tasks: Card View State Model

**Input**: Design documents from `specs/015-card-states/`  
**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/card-state-contract.md](./contracts/card-state-contract.md)

**Tests**: Include focused model tests because the feature requires explicit validation rules, legal/illegal combinations, and behavior-preserving migration coverage.

**Organization**: Tasks are grouped by user story so each story can be implemented and verified independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel because it touches different files or has no dependency on incomplete tasks.
- **[Story]**: Maps task to the user story from [spec.md](./spec.md).
- Every task includes an exact repository path.

## Phase 1: Setup

**Purpose**: Confirm runtime conventions and preserve the current state inventory before implementation.

- [X] T001 Inspect `bevy/crates/template-crate` and record applicable Rust/Bevy file, item, test, and purpose-comment conventions in `specs/015-card-states/quickstart.md`
- [X] T002 [P] Audit current visual-root and face-layer ownership in `bevy/crates/game/src/runtime/bundles/card_view_bundle.rs` and `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T003 [P] Audit current local card state, slot occupancy, and gesture ownership in `bevy/crates/game/src/runtime/resources/card_slot_model.rs` and `bevy/crates/game/src/runtime/resources/card_gesture_model.rs`
- [X] T004 [P] Audit current CPU/opponent reveal and passive-card ownership in `bevy/crates/game/src/runtime/resources/opponent_match_model.rs` and `bevy/crates/game/src/runtime/components/mod.rs`

---

## Phase 2: Foundational

**Purpose**: Add the shared model/test files that all user stories depend on.

**Critical**: No user story implementation should start until this phase is complete.

- [X] T005 Create `bevy/crates/game/src/runtime/resources/card_instance_state_model.rs` with primary item stubs and required `HUMAN:`/`AI:` comments
- [X] T006 Export `card_instance_state_model` from `bevy/crates/game/src/runtime/resources/mod.rs`
- [X] T007 [P] Create `bevy/crates/game/src/tests/runtime/resources/card_instance_state_model_tests.rs` with compile-only test module structure
- [X] T008 Add the test module path from `card_instance_state_model.rs` to `bevy/crates/game/src/runtime/resources/card_instance_state_model.rs`

**Checkpoint**: Runtime can compile with empty model scaffolding and test module wiring.

---

## Phase 3: User Story 1 - Understand Current Card View State (Priority: P1) MVP

**Goal**: Make the current state axes explicit and traceable from source artifacts.

**Independent Test**: A developer can map every current card state axis to the current source artifact that owns it.

### Tests for User Story 1

- [X] T009 [P] [US1] Add current-state inventory tests for `CardFace`, `CardState`, `CardGestureState`, and `PlacementVisibility` in `bevy/crates/game/src/tests/runtime/resources/card_instance_state_model_tests.rs`

### Implementation for User Story 1

- [X] T010 [US1] Add `CardStateAxisModel` inventory helpers covering face, zone, interaction, slot, reveal, and CPU presentation axes in `bevy/crates/game/src/runtime/resources/card_instance_state_model.rs`
- [X] T011 [US1] Document current source ownership mappings in Rust doc comments in `bevy/crates/game/src/runtime/resources/card_instance_state_model.rs`
- [X] T012 [US1] Reconcile `specs/015-card-states/data-model.md` with the implemented current-state ownership inventory

**Checkpoint**: US1 is complete when tests show all current axes are represented and the data model remains accurate.

---

## Phase 4: User Story 2 - Separate Durable Gameplay State From Transient Presentation (Priority: P1)

**Goal**: Introduce a model that separates durable card identity/location/reveal state from transient interaction and render-facing view state.

**Independent Test**: Each proposed enum has one reason to change, and illegal combinations are impossible or explicitly validated.

### Tests for User Story 2

- [X] T013 [P] [US2] Add `CardInstanceId`, `CardZoneModel`, `LocationLockState`, and single-zone validation tests in `bevy/crates/game/src/tests/runtime/resources/card_instance_state_model_tests.rs`
- [X] T014 [US2] Add `CardRevealPolicy` and `CardViewStateModel.visible_face` derivation tests in `bevy/crates/game/src/tests/runtime/resources/card_instance_state_model_tests.rs`
- [X] T015 [US2] Add `CardInteractionModel` legal interaction and single-active-focus tests in `bevy/crates/game/src/tests/runtime/resources/card_instance_state_model_tests.rs`
- [X] T016 [US2] Add instance lookup and index strategy tests for owner, zone, and slot access in `bevy/crates/game/src/tests/runtime/resources/card_instance_state_model_tests.rs`

### Implementation for User Story 2

- [X] T017 [US2] Implement `CardInstanceId`, `CardOwnerModel`, `CardInstanceStateModel`, `CardZoneModel`, and `LocationLockState` in `bevy/crates/game/src/runtime/resources/card_instance_state_model.rs`
- [X] T018 [US2] Implement `CardRevealPolicy`, viewer-aware visible-face derivation, and hidden/revealed validation in `bevy/crates/game/src/runtime/resources/card_instance_state_model.rs`
- [X] T019 [US2] Implement `CardInteractionModel`, `CardInteractionState`, and draggable/selectable validation helpers in `bevy/crates/game/src/runtime/resources/card_instance_state_model.rs`
- [X] T020 [US2] Implement `CardViewStateModel`, pose source enum, z-band enum, and input-affordance derivation in `bevy/crates/game/src/runtime/resources/card_instance_state_model.rs`
- [X] T021 [US2] Implement lookup/index helpers for owner, zone, and slot access in `bevy/crates/game/src/runtime/resources/card_instance_state_model.rs`
- [X] T022 [US2] Add model-level validation methods for illegal deck-selected, locked-dragging, duplicate-zone, slot-mismatch, and missing-instance combinations in `bevy/crates/game/src/runtime/resources/card_instance_state_model.rs`
- [X] T023 [US2] Reconcile `specs/015-card-states/contracts/card-state-contract.md` with implemented model and enum names

**Checkpoint**: US2 is complete when model tests pass and durable state, reveal state, interaction state, and view state are independently represented.

---

## Phase 5: User Story 3 - Preserve Existing Behavior While Enabling Cleanup (Priority: P2)

**Goal**: Provide an adapter and migration path that preserves current hand, location, gesture, CPU reveal, and card face behavior.

**Independent Test**: Existing gesture and opponent-mode behavior can be mapped into the new model without changing user-visible behavior.

### Tests for User Story 3

- [X] T024 [P] [US3] Add adapter tests that map `GameHandModel`, `CardStateModel`, and `CardSlotBoardModel` into `CardInstanceStateModel` in `bevy/crates/game/src/tests/runtime/resources/card_instance_state_model_tests.rs`
- [X] T025 [US3] Add adapter tests that map `PlacementVisibilityModel`, `CpuHandCardView`, and `CpuPlacedCardView` semantics into reveal/view-state expectations in `bevy/crates/game/src/tests/runtime/resources/card_instance_state_model_tests.rs`

### Implementation for User Story 3

- [X] T026 [US3] Implement non-authoritative adapter functions from `GameHandModel`, `CardStateModel`, and `CardSlotBoardModel` in `bevy/crates/game/src/runtime/resources/card_instance_state_model.rs`
- [X] T027 [US3] Implement non-authoritative adapter functions from `PlacementVisibilityModel`, `CpuHandCardView`, and `CpuPlacedCardView` semantics in `bevy/crates/game/src/runtime/resources/card_instance_state_model.rs`
- [X] T028 [US3] Add migration notes for replacing `hand_index` slot occupancy with `CardInstanceId` in `specs/015-card-states/data-model.md`
- [X] T029 [US3] Add future integration notes for `card_gesture_update_system` and `card_gesture_animation_system` in `specs/015-card-states/quickstart.md`

**Checkpoint**: US3 is complete when adapters demonstrate behavior parity without changing existing gesture or opponent systems.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Verify consistency, formatting, and future implementation readiness.

- [X] T030 [P] Run `scripts/other/RunTests.ps1` and record the result in `specs/015-card-states/quickstart.md`
- [X] T031 [P] Verify changed Bevy runtime items in `bevy/crates/game/src/runtime/resources/card_instance_state_model.rs` use one primary concept, Scene/Model/View naming, and `HUMAN:`/`AI:` comments
- [X] T032 [P] Check generated documentation links and state terminology in `specs/015-card-states/spec.md`, `specs/015-card-states/plan.md`, `specs/015-card-states/data-model.md`, and `specs/015-card-states/contracts/card-state-contract.md`
- [X] T033 Run `git diff --check` from the repository root and fix whitespace issues in files changed for `015-card-states`

---

## Dependencies & Execution Order

### Phase Dependencies

| Phase | Depends On | Blocks |
| ----- | ---------- | ------ |
| Setup | None | Foundational |
| Foundational | Setup | All user stories |
| US1 | Foundational | None |
| US2 | Foundational | US3 adapter work benefits from US2 model names |
| US3 | Foundational, preferably US2 | Polish |
| Polish | Desired user stories complete | Release/implementation handoff |

### User Story Dependencies

| Story | Dependency | Notes |
| ----- | ---------- | ----- |
| US1 | Foundational | Can complete independently as an inventory/model-documentation increment. |
| US2 | Foundational | Can proceed in parallel with US1 after scaffolding, but should reconcile any inventory updates from US1. |
| US3 | US2 recommended | Adapter names depend on US2 model names; behavior mapping remains independently testable. |

### Parallel Opportunities

| Opportunity | Tasks |
| ----------- | ----- |
| Current-state audits | T002, T003, T004 |
| Test scaffolding | T007, T008 |
| US2 model tests | T013, T014, T015, T016 |
| US3 adapter tests | T024, T025 |
| Polish checks | T028, T029, T030 |

---

## Parallel Example: User Story 2

```text
Task: "Add CardInstanceId, CardZoneModel, LocationLockState, and single-zone validation tests in bevy/crates/game/src/tests/runtime/resources/card_instance_state_model_tests.rs"
Task: "Add CardRevealPolicy and CardViewStateModel.visible_face derivation tests in bevy/crates/game/src/tests/runtime/resources/card_instance_state_model_tests.rs"
Task: "Add CardInteractionModel legal interaction and single-active-focus tests in bevy/crates/game/src/tests/runtime/resources/card_instance_state_model_tests.rs"
Task: "Add instance lookup and index strategy tests for owner, zone, and slot access in bevy/crates/game/src/tests/runtime/resources/card_instance_state_model_tests.rs"
```

---

## Implementation Strategy

### MVP First

1. Complete Phase 1 setup.
2. Complete Phase 2 scaffolding.
3. Complete Phase 3 / US1 to make the current state hierarchy explicit.
4. Stop and validate that every current card state axis has a source owner and model mapping.

### Incremental Delivery

1. Deliver US1 as documentation-backed inventory.
2. Deliver US2 as pure model and validation logic.
3. Deliver US3 as adapters and migration notes before touching gesture systems.
4. Defer replacing existing gesture/slot/opponent systems until a follow-up implementation spec.

### Parallel Team Strategy

1. One developer audits current state owners while another creates model/test scaffolding.
2. After scaffolding, split US2 by zone/reveal/interaction derivation tests.
3. Keep US3 adapter work after model names stabilize to avoid churn in shared files.
