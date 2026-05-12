# Tasks: Point System

**Input**: Design documents from `specs/010-point-system/`
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/point-system-contract.md`, `quickstart.md`
**Tests**: Included because the feature specification defines independent tests and sample scoring scenarios.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare focused runtime modules for point data and point views.

- [X] T001 Add point model module export in `bevy/crates/game/src/runtime/resources/mod.rs`
- [X] T002 Add point view component module export in `bevy/crates/game/src/runtime/components/mod.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core point types shared by every user story.

- [X] T003 [P] Create `CostPointModel` and `PowerPointModel` with display-range helpers and tests in `bevy/crates/game/src/runtime/resources/point_model.rs`
- [X] T004 [P] Create `CostPointView` and `PowerPointView` component markers with required purpose comments in `bevy/crates/game/src/runtime/components/point_view_component.rs`
- [X] T005 Add card cost and base power fields to `CardModel` definitions in `bevy/crates/game/src/runtime/resources/mod.rs`

**Checkpoint**: Foundation ready - user story implementation can now begin.

---

## Phase 3: User Story 1 - Define Card Point Values (Priority: P1) MVP

**Goal**: Every playable card exposes distinct cost and power point values, and cost never scores.

**Independent Test**: Review card model tests and verify cost and power are distinct values with separate meanings.

### Tests for User Story 1

- [X] T006 [P] [US1] Add tests for card cost/power exposure and cost exclusion from scoring in `bevy/crates/game/src/runtime/resources/point_model.rs`

### Implementation for User Story 1

- [X] T007 [US1] Implement card instance effective power separate from `CardModel` base power in `bevy/crates/game/src/runtime/resources/point_model.rs`
- [X] T008 [US1] Wire sample card definitions to expose cost and base power in `bevy/crates/game/src/runtime/resources/mod.rs`

**Checkpoint**: User Story 1 is independently testable.

---

## Phase 4: User Story 2 - Show Location Totals (Priority: P1)

**Goal**: Each shared location can show opponent and local totals derived from revealed cards.

**Independent Test**: Inspect a location state and confirm it exposes local total, opponent total, and revealed-card sum behavior.

### Tests for User Story 2

- [X] T009 [P] [US2] Add tests for revealed-card totals, unrevealed-card exclusion, negative totals, movement, and location modifiers in `bevy/crates/game/src/runtime/resources/point_model.rs`

### Implementation for User Story 2

- [X] T010 [US2] Implement `LocationScoreModel` with default four-card capacity and local/opponent totals in `bevy/crates/game/src/runtime/resources/point_model.rs`
- [X] T011 [US2] Render top opponent and bottom local `PowerPointView` totals on each `GameLocation` in `bevy/crates/game/src/runtime/systems/mod.rs`

**Checkpoint**: User Story 2 is independently testable.

---

## Phase 5: User Story 3 - Determine Location Control (Priority: P2)

**Goal**: A location reports local control, opponent control, or no controller based on totals.

**Independent Test**: Simulate local lead, opponent lead, tied totals, and empty equal-zero totals.

### Tests for User Story 3

- [X] T012 [P] [US3] Add tests for all location-control outcomes in `bevy/crates/game/src/runtime/resources/point_model.rs`

### Implementation for User Story 3

- [X] T013 [US3] Implement `LocationControlModel` and control evaluation in `bevy/crates/game/src/runtime/resources/point_model.rs`

**Checkpoint**: User Story 3 is independently testable.

---

## Phase 6: User Story 4 - Determine Match Outcome (Priority: P2)

**Goal**: Final scoring returns local win, opponent win, or draw using controlled locations then total power.

**Independent Test**: Simulate final states with location-count wins, total-power tiebreaks, and full draws.

### Tests for User Story 4

- [X] T014 [P] [US4] Add tests for controlled-location wins, total-power tiebreaks, and draws in `bevy/crates/game/src/runtime/resources/point_model.rs`

### Implementation for User Story 4

- [X] T015 [US4] Implement `MatchScoreModel` and `MatchOutcomeModel` final scoring in `bevy/crates/game/src/runtime/resources/point_model.rs`

**Checkpoint**: User Story 4 is independently testable.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Validate behavior and repo conventions.

- [X] T016 Update point-system contract notes if implementation names diverge in `specs/010-point-system/contracts/point-system-contract.md`
- [X] T017 Run `scripts/other/RunTests.ps1`
- [X] T018 Verify changed Bevy runtime files use `bevy/crates/template-crate` as the proper reference and follow one-primary-concept, Scene/Model/View naming, system naming, and HUMAN/AI comment standards
- [X] T019 Verify `speckit-implement 010` completion by checking all tasks in `specs/010-point-system/tasks.md`

---

## Dependencies & Execution Order

| Phase | Depends On | Notes |
| ----- | ---------- | ----- |
| Setup | None | Module exports only |
| Foundational | Setup | Blocks all stories |
| US1 | Foundational | MVP for card point values |
| US2 | Foundational and US1 | Uses effective power from card instances |
| US3 | US2 | Control depends on location totals |
| US4 | US3 | Outcome depends on control and totals |
| Polish | Desired stories complete | Validation and cleanup |

## Parallel Opportunities

| Area | Parallel Tasks |
| ---- | -------------- |
| Foundational | T003 and T004 touch different files |
| US tests | T006, T009, T012, and T014 are independent test groups before implementation |
| Review | T016 and T018 can be reviewed after implementation while tests run |

## Implementation Strategy

| Step | Scope |
| ---- | ----- |
| MVP | Complete T001 through T008 so card cost and power are distinct and testable |
| Increment 2 | Complete T009 through T011 for location totals and visible `PowerPointView` totals |
| Increment 3 | Complete T012 through T015 for control and match outcome |
| Finish | Complete T016 through T019 and verify the repository scripts |
