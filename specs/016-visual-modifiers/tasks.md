# Tasks: Visual Modifier System

**Input**: Design documents from `specs/016-visual-modifiers/`  
**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/visual-modifier-contract.md](./contracts/visual-modifier-contract.md)

**Tests**: Include focused model and system tests because the feature defines independent visual rule Conditions, Targets, Treatments, and user-visible presentation outcomes.

**Organization**: Tasks are grouped by user story so each story can be implemented and verified independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel because it touches different files or has no dependency on incomplete tasks.
- **[Story]**: Maps task to the user story from [spec.md](./spec.md).
- Every task includes an exact repository path.

## Phase 1: Setup

**Purpose**: Confirm runtime conventions, point view structure, and 015 card-state integration points before implementation.

- [X] T001 Inspect `bevy/crates/template-crate` and record applicable Rust/Bevy file, item, test, system, and purpose-comment conventions in `specs/016-visual-modifiers/quickstart.md`
- [X] T002 [P] Audit current point view roots, circle/background children, card point spawning, and location point spawning in `bevy/crates/game/src/runtime/bundles/point_view_bundle.rs` and `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T003 [P] Audit completed 015 card-state helpers in `bevy/crates/game/src/runtime/resources/card_instance_state_model.rs` for `CardInstanceId`, `CardInstanceStateModel`, `CardZoneModel::Location`, and adapter functions
- [X] T004 [P] Audit current ability and score inputs in `bevy/crates/game/src/runtime/resources/game_location_model.rs`, `bevy/crates/game/src/runtime/resources/card_slot_model.rs`, and `bevy/crates/game/src/runtime/systems/mod.rs`

---

## Phase 2: Foundational

**Purpose**: Add the shared Condition/Target/Treatment model and point view target markers that all visual modification stories depend on.

**Critical**: No user story implementation should start until this phase is complete.

- [X] T005 Create `bevy/crates/game/src/runtime/components/point_view_visual_modifier_component.rs` with `VisualModificationRule`, `VisualModificationCondition`, `VisualModificationTarget`, `VisualModificationTreatment`, `VisualModifier`, `PointViewVisualModifiers`, `PointViewCircle`, and `PointViewCardInstanceLink` primary items plus required `HUMAN:`/`AI:` comments
- [X] T006 Export `point_view_visual_modifier_component` from `bevy/crates/game/src/runtime/components/mod.rs`
- [X] T007 [P] Create `bevy/crates/game/src/tests/runtime/components/point_view_visual_modifier_component_tests.rs` with compile-only module wiring and initial constructor/default tests
- [X] T008 Add the test module path from `point_view_visual_modifier_component.rs` to `bevy/crates/game/src/runtime/components/point_view_visual_modifier_component.rs`
- [X] T009 Add `PointViewCircle` markers to location and card point view circle/background child spawns in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T010 Add `PointViewCardInstanceLink` or documented adapter-link creation for card-owned point view roots in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T011 Create `bevy/crates/game/src/runtime/systems/visual_modifier_update_system.rs` with empty system scaffolding and required `HUMAN:`/`AI:` comments
- [X] T012 Create and wire `bevy/crates/game/src/tests/runtime/systems/visual_modifier_update_system_tests.rs` from `bevy/crates/game/src/runtime/systems/visual_modifier_update_system.rs`
- [X] T013 Export `visual_modifier_update_system` from `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T014 Register `visual_modifier_update_system` after point value update systems in `bevy/crates/game/src/runtime/plugins/mod.rs`

**Checkpoint**: Runtime can compile with VMS rule model, target markers, point-view links, and system registration in place.

---

## Phase 3: User Story 1 - Highlight Ability-Modified Card Points (Priority: P1) MVP

**Goal**: Card power point views affected by an active non-zero ability show `abilityoutline`: Condition is modified card power, Target is the card power point circle/background, Treatment is a gold outline.

**Independent Test**: Place a card at an open location with a non-zero active ability delta and verify the card's power point circle receives `abilityoutline`; place or view the same card without an active ability and verify the outline is absent.

### Tests for User Story 1

- [X] T015 [P] [US1] Add unit tests for `VisualModificationCondition::CardPowerModifiedByAbility` true/false cases in `bevy/crates/game/src/tests/runtime/components/point_view_visual_modifier_component_tests.rs`
- [X] T016 [P] [US1] Add system tests for card power point `abilityoutline` activation and clearing in `bevy/crates/game/src/tests/runtime/systems/visual_modifier_update_system_tests.rs`
- [X] T017 [US1] Add hidden/revealed card point visibility regression coverage for `abilityoutline` in `bevy/crates/game/src/tests/runtime/systems/visual_modifier_update_system_tests.rs`

### Implementation for User Story 1

- [X] T018 [US1] Implement the `abilityoutline` rule definition with Condition `CardPowerModifiedByAbility`, Target `CardPowerPointCircle`, and Treatment `Outline { color: gold }` in `bevy/crates/game/src/runtime/components/point_view_visual_modifier_component.rs`
- [X] T019 [US1] Implement card ability Condition evaluation using `PointViewCardInstanceLink`, `CardInstanceStateModel`, `CardZoneModel::Location`, and `GameLocationModel::ability_delta_for_location()` in `bevy/crates/game/src/runtime/systems/visual_modifier_update_system.rs`
- [X] T020 [US1] Implement adapter fallback from current hand-index card point views through 015 helpers in `bevy/crates/game/src/runtime/systems/visual_modifier_update_system.rs`
- [X] T021 [US1] Implement Treatment synchronization for gold outlines on world-space card point circle/background targets in `bevy/crates/game/src/runtime/systems/visual_modifier_update_system.rs`
- [X] T022 [US1] Preserve card point text, value, render layers, and 015 hidden/revealed visibility behavior while applying `abilityoutline` in `bevy/crates/game/src/runtime/systems/visual_modifier_update_system.rs`

**Checkpoint**: User Story 1 is complete when `abilityoutline` appears only on active ability-modified card power point circles and clears when the Condition is false.

---

## Phase 4: User Story 2 - Highlight Leading Location Totals (Priority: P2)

**Goal**: The higher location total point view shows `leadingscoreoutline`: Condition is strictly leading score, Target is the leading location total point circle/background, Treatment is a white outline.

**Independent Test**: Set one location's local and opponent totals to unequal values and verify only the higher total point circle receives `leadingscoreoutline`; set equal totals and verify neither total has it.

### Tests for User Story 2

- [X] T023 [P] [US2] Add unit tests for `VisualModificationCondition::LocationTotalIsLeading` local-leads, opponent-leads, tie, and empty-location cases in `bevy/crates/game/src/tests/runtime/components/point_view_visual_modifier_component_tests.rs`
- [X] T024 [P] [US2] Add system tests for location total `leadingscoreoutline` activation, clearing, and side switching in `bevy/crates/game/src/tests/runtime/systems/visual_modifier_update_system_tests.rs`

### Implementation for User Story 2

- [X] T025 [US2] Implement the `leadingscoreoutline` rule definition with Condition `LocationTotalIsLeading`, Target `LocationTotalPointCircle`, and Treatment `Outline { color: white }` in `bevy/crates/game/src/runtime/components/point_view_visual_modifier_component.rs`
- [X] T026 [US2] Implement paired `PointLocationView` comparison by `location_index` and `CardSlotSide` in `bevy/crates/game/src/runtime/systems/visual_modifier_update_system.rs`
- [X] T027 [US2] Implement tie and empty-location clearing for `leadingscoreoutline` in `bevy/crates/game/src/runtime/systems/visual_modifier_update_system.rs`
- [X] T028 [US2] Implement Treatment synchronization for white outlines on Bevy UI location total point circle/background targets in `bevy/crates/game/src/runtime/systems/visual_modifier_update_system.rs`

**Checkpoint**: User Story 2 is complete when exactly one location total point circle has `leadingscoreoutline` for unequal totals and neither side has it for ties.

---

## Phase 5: User Story 3 - Define Reusable Visual Modification Rules (Priority: P3)

**Goal**: Every visual modification is represented as a reusable rule with explicit Condition, Target, and Treatment so future card, location, score, and UI effects can be added predictably.

**Independent Test**: A developer can inspect the VMS rule list and identify for each visual modification why it activates, what UI element it changes, and what treatment is applied.

### Tests for User Story 3

- [X] T029 [P] [US3] Add rule completeness tests requiring every `VisualModificationRule` to define one Condition, one Target, and one Treatment in `bevy/crates/game/src/tests/runtime/components/point_view_visual_modifier_component_tests.rs`
- [X] T030 [P] [US3] Add deterministic multiple-rule and missing-target tests in `bevy/crates/game/src/tests/runtime/systems/visual_modifier_update_system_tests.rs`

### Implementation for User Story 3

- [X] T031 [US3] Implement a centralized VMS rule list for `abilityoutline` and `leadingscoreoutline` in `bevy/crates/game/src/runtime/components/point_view_visual_modifier_component.rs`
- [X] T032 [US3] Refactor `visual_modifier_update_system` to evaluate Conditions, resolve Targets, and apply Treatments as separate steps in `bevy/crates/game/src/runtime/systems/visual_modifier_update_system.rs`
- [X] T033 [US3] Ensure stale Treatments are cleared when Conditions are false, Targets are missing, or point views despawn in `bevy/crates/game/src/runtime/systems/visual_modifier_update_system.rs`
- [X] T034 [US3] Reconcile `specs/016-visual-modifiers/data-model.md` and `specs/016-visual-modifiers/contracts/visual-modifier-contract.md` with implemented VMS item names

**Checkpoint**: User Story 3 is complete when both initial rules use the same Condition/Target/Treatment pipeline and missing targets or inactive Conditions clear safely.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Verify runtime behavior, documentation, and project standards after desired user stories are complete.

- [X] T035 [P] Run `scripts/other/RunTests.ps1` and record the result in `specs/016-visual-modifiers/quickstart.md`
- [X] T036 [P] Verify changed runtime files use one primary concept per file, purposeful names, `[domain]_[schedule]_system` naming, and required `HUMAN:`/`AI:` comments in `bevy/crates/game/src/runtime/components/point_view_visual_modifier_component.rs` and `bevy/crates/game/src/runtime/systems/visual_modifier_update_system.rs`
- [ ] T037 [P] Verify desktop visual behavior with `scripts/main/RunAppDesktop.ps1` or the AI runtime screenshot workflow and record observations in `specs/016-visual-modifiers/quickstart.md`
- [ ] T038 [P] Verify browser WebGPU behavior when practical and record any blocker in `specs/016-visual-modifiers/quickstart.md`
- [X] T039 Check generated documentation links and terminology in `specs/016-visual-modifiers/spec.md`, `specs/016-visual-modifiers/plan.md`, `specs/016-visual-modifiers/data-model.md`, and `specs/016-visual-modifiers/contracts/visual-modifier-contract.md`
- [X] T040 Run `git diff --check` from the repository root and fix whitespace issues in files changed for `016-visual-modifiers`

---

## Dependencies & Execution Order

### Phase Dependencies

| Phase | Depends On | Blocks |
| ----- | ---------- | ------ |
| Setup | None | Foundational |
| Foundational | Setup | All user stories |
| US1 | Foundational | MVP and concrete card ability visual feedback |
| US2 | Foundational | Location leading score visual feedback |
| US3 | Foundational, US1, US2 recommended | Rule pipeline cleanup and future extensibility |
| Polish | Desired user stories complete | Implementation handoff |

### User Story Dependencies

| Story | Dependency | Notes |
| ----- | ---------- | ----- |
| US1 | Foundational | Can complete independently as the MVP `abilityoutline` rule. |
| US2 | Foundational | Can complete independently after point view target markers and VMS scaffolding exist. |
| US3 | US1 and US2 recommended | The generalized rule pipeline is clearest once both initial rules exist, though foundational rule types are shared from the start. |

### Parallel Opportunities

| Opportunity | Tasks |
| ----------- | ----- |
| Runtime audits | T002, T003, T004 |
| Test scaffolding | T007, T008, T012 |
| US1 tests | T015, T016 |
| US2 tests | T023, T024 |
| US3 tests | T029, T030 |
| Polish checks | T035, T036, T037, T038 |

---

## Parallel Example: User Story 1

```text
Task: "Add unit tests for VisualModificationCondition::CardPowerModifiedByAbility true/false cases in bevy/crates/game/src/tests/runtime/components/point_view_visual_modifier_component_tests.rs"
Task: "Add system tests for card power point abilityoutline activation and clearing in bevy/crates/game/src/tests/runtime/systems/visual_modifier_update_system_tests.rs"
```

---

## Implementation Strategy

### MVP First

1. Complete Phase 1 setup.
2. Complete Phase 2 foundational VMS rule and target scaffolding.
3. Complete Phase 3 / US1 to show gold `abilityoutline` on active ability-modified card power point circles.
4. Stop and validate User Story 1 independently in tests and, when practical, in the GameView.

### Incremental Delivery

1. Deliver US1 as the MVP card ability visual feedback.
2. Deliver US2 as location leading score visual feedback.
3. Deliver US3 to consolidate both visible effects into the reusable Condition/Target/Treatment pipeline.
4. Complete polish checks and update quickstart verification notes.

### Parallel Team Strategy

1. One developer audits point view/render target structure while another audits 015 card-state integration and ability/score inputs.
2. After foundational scaffolding, split US1 and US2 rule tests because they target different user-facing outcomes.
3. Keep shared pipeline refactoring in US3 after initial rule behavior is stable to reduce churn in shared system files.
