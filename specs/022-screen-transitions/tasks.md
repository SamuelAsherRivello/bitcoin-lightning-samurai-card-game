# Tasks: Screen Transitions

**Input**: Design documents from `/specs/022-screen-transitions/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare module surface for transition runtime additions.

- [X] T001 Confirm runtime file ownership and naming conventions against `bevy/crates/template-crate` before edits (reference check only)
- [X] T002 Add transition bundle module export in `bevy/crates/game/src/runtime/bundles/mod.rs`
- [X] T003 Add transition system module export in `bevy/crates/game/src/runtime/systems/mod.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Add shared transition primitives required by all transition behaviors.

- [X] T004 Add `ScreenTransitionPhase` and `ScreenTransitionModel` resource types to `bevy/crates/game/src/runtime/resources/mod.rs`
- [X] T005 [P] Add `ScreenTransitionOverlay` marker component to `bevy/crates/game/src/runtime/components/mod.rs`
- [X] T006 [P] Add transition contract notes for queue/replace policy and phase timing to `specs/022-screen-transitions/contracts/screen-transition-contract.md`
- [X] T007 Register transition resource initialization in `bevy/crates/game/src/runtime/plugins/core_game_plugin.rs`

**Checkpoint**: Transition resource and marker primitives are available for all story work.

---

## Phase 3: User Story 1 - Startup Fade-In From Black (Priority: P1) 🎯 MVP

**Goal**: App starts with a fullscreen black overlay that fades to transparent.

**Independent Test**: Launch app and verify first visible frame is black, then it fades in to reveal active screen content.

### Implementation for User Story 1

- [X] T008 [US1] Create transition overlay UI bundle with fullscreen node defaults in `bevy/crates/game/src/runtime/bundles/transition_ui_bundle.rs`
- [X] T009 [US1] Spawn transition overlay under AppScene/HUD setup path in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T010 [US1] Implement startup fade phase alpha progression system (`transition_update_system`) in `bevy/crates/game/src/runtime/systems/transition_update_system.rs`
- [X] T011 [US1] Wire startup transition update system scheduling in `bevy/crates/game/src/runtime/plugins/core_game_plugin.rs`
- [X] T012 [US1] Ensure transition overlay renders above all screens/HUD layers in `bevy/crates/game/src/runtime/systems/transition_update_system.rs`

**Checkpoint**: Startup fade-in behavior is functional and visually correct.

---

## Phase 4: User Story 2 - Fade Out/Switch/Fade In On Screen Changes (Priority: P2)

**Goal**: Any view switch is visually masked by black fade out/in with switch committed at full black.

**Independent Test**: Switch among Game, Deck, and Debug views and verify sequence is always fade out -> switch -> fade in with no popping.

### Implementation for User Story 2

- [X] T013 [US2] Route screen/view change requests through transition pending-view flow in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T014 [US2] Extend `transition_update_system` for runtime phase machine (`FadeOutPendingSwitch`, `SwitchAtBlack`, `FadeInAfterSwitch`) in `bevy/crates/game/src/runtime/systems/transition_update_system.rs`
- [X] T015 [US2] Apply `ActiveView` switch only at full-black gate in `bevy/crates/game/src/runtime/systems/transition_update_system.rs`
- [X] T016 [US2] Add queue/replace behavior for requests during active transition in `bevy/crates/game/src/runtime/resources/mod.rs`
- [X] T017 [US2] Update BRP/AI-runtime screen-switch entry points to use transition routing in `bevy/crates/game/src/runtime/plugins/ai_runtime_plugin.rs`

**Checkpoint**: Runtime screen transitions are deterministic and fully masked.

---

## Phase 5: User Story 3 - Default Config: Black + 0.5s Total (Priority: P3)

**Goal**: Transition defaults match configured product baseline and are centralized for tuning.

**Independent Test**: Inspect defaults and visually confirm cycle timing approximates 0.5s total (0.25 out + 0.25 in).

### Implementation for User Story 3

- [X] T018 [US3] Set default transition color to black and total duration to `0.5` seconds in `bevy/crates/game/src/runtime/resources/mod.rs`
- [X] T019 [US3] Clamp non-positive duration to safe minimum in `bevy/crates/game/src/runtime/systems/transition_update_system.rs`
- [X] T020 [US3] Expose transition constants in one runtime location for future tuning in `bevy/crates/game/src/runtime/resources/mod.rs`

**Checkpoint**: Transition baseline config is stable and maintainable.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Verify parity, guard regressions, and align docs.

- [X] T021 [P] Add/adjust runtime tests for transition phase progression and switch-at-black behavior in `bevy/crates/game/src/tests/runtime/`
- [X] T022 Run `scripts/other/RunTests.ps1` and record transition-related results in `specs/022-screen-transitions/quickstart.md`
- [ ] T023 Validate desktop + browser transition behavior using `scripts/main/RunAppDesktop.ps1` and `scripts/main/RunAppWeb.ps1`, then document verification notes in `specs/022-screen-transitions/quickstart.md`
- [X] T024 Verify all changed runtime items include `HUMAN:`/`AI:` comments and `[domain]_[schedule]_system` naming in `bevy/crates/game/src/runtime/`

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 1 -> Phase 2 -> Phase 3 -> Phase 4 -> Phase 5 -> Phase 6
- User stories start only after foundational phase completes.

### User Story Dependencies

- US1 (P1) depends on foundational tasks only.
- US2 (P2) depends on US1 overlay/system availability.
- US3 (P3) depends on US1/US2 transition model and update loop.

### Parallel Opportunities

- T005 and T006 can run in parallel in Phase 2.
- T021 can run in parallel with documentation updates in Phase 6.

---

## Parallel Example: User Story 2

```bash
Task: "Route screen/view change requests through transition pending-view flow in bevy/crates/game/src/runtime/systems/mod.rs"
Task: "Update BRP/AI-runtime screen-switch entry points to use transition routing in bevy/crates/game/src/runtime/plugins/ai_runtime_plugin.rs"
```

---

## Implementation Strategy

### MVP First (US1)

1. Complete Setup + Foundational.
2. Complete US1 startup fade-in path.
3. Validate startup behavior before moving on.

### Incremental Delivery

1. Ship US1 startup fade.
2. Add US2 runtime switch masking.
3. Add US3 centralized defaults and timing safeguards.
4. Finish with tests and parity validation.

