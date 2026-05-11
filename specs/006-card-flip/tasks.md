# Tasks: Card Flip

**Input**: Design documents from `specs/006-card-flip/`
**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/card-flip-ui.md](./contracts/card-flip-ui.md), [quickstart.md](./quickstart.md)

**Tests**: Include focused Rust tests because this feature changes state transitions, transform composition, face visibility, asset ownership, and existing `T` behavior.

**Organization**: Tasks are grouped by user story so each story can be implemented and verified independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel with other marked tasks in the same phase when files do not overlap.
- **[Story]**: User story label from [spec.md](./spec.md).
- Every task includes an exact repository path.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Confirm the active feature context, preserve the DeckBuilder prototype entry point, and prepare the shared asset slot.

- [X] T001 Verify `.specify/feature.json` points to `specs/006-card-flip` and `AGENTS.md` active plan points to `specs/006-card-flip/plan.md`
- [X] T002 [P] Create the shared card back source asset `bevy/crates/game/assets/cards/card_structure/card_back_superhero_pattern.png`
- [X] T003 [P] Audit current DeckBuilder runtime entry points in `bevy/crates/game/src/runtime/systems/mod.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Add shared ECS state and markers required before any user story can be implemented.

**CRITICAL**: No user story work can begin until this phase is complete.

- [X] T004 Add `CardFace`, `CardFlipState`, and card back asset path constants in `bevy/crates/game/src/runtime/resources/mod.rs`
- [X] T005 Add front/back face marker components or roles for CardFront and CardBack visibility in `bevy/crates/game/src/runtime/components/mod.rs`
- [X] T006 Initialize `CardFlipState` in `bevy/crates/game/src/runtime/plugins/mod.rs`
- [X] T007 Add unit tests for `CardFlipState` defaults, 180-degree targets, midpoint side switching, and mid-animation reversal in `bevy/crates/game/src/runtime/resources/mod.rs`
- [X] T008 Add a card back asset existence test for `cards/card_structure/card_back_superhero_pattern.png` in `bevy/crates/game/src/runtime/resources/mod.rs`

**Checkpoint**: Flip state, face identity, and shared back asset path exist and are covered by focused tests.

---

## Phase 3: User Story 1 - Flip Card From Front To Back (Priority: P1) MVP

**Goal**: The temporary Card UI has a `Flip` button that animates the current front-facing card to the shared back-facing state.

**Independent Test**: Launch the DeckBuilder prototype, click `Flip` in the Card UI while CardFront is visible, and verify the card animates to CardBack without adding gameplay or extra cards.

### Tests for User Story 1

- [X] T009 [P] [US1] Add a Card UI contract test for `Flip` changing `CardFlipState` toward CardBack in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T010 [P] [US1] Add a scene composition test that exactly one `CardPlaceholder` exists after adding CardBack visuals in `bevy/crates/game/src/runtime/systems/mod.rs`

### Implementation for User Story 1

- [X] T011 [US1] Spawn the CardBack visual as part of the existing card structure in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T012 [US1] Add the `Flip` button to the temporary Card UI, not DebugHUD, in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T013 [US1] Implement Card UI activation so `Flip` starts a front-to-back animation in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T014 [US1] Schedule the flip input and animation update systems in `bevy/crates/game/src/runtime/plugins/mod.rs`

**Checkpoint**: User Story 1 is complete when clicking `Flip` from CardFront visibly animates to CardBack and one centered card remains.

---

## Phase 4: User Story 2 - Preserve Pointer-Driven Inspection During Flip (Priority: P1)

**Goal**: Pointer-driven inspection continues during the flip, with flip rotation layered onto existing card orientation.

**Independent Test**: Move the pointer before and during a flip, click `Flip`, and verify the card starts from its current apparent angle and continues responding to pointer movement.

### Tests for User Story 2

- [X] T015 [P] [US2] Add transform composition tests for pointer rotation plus flip y-rotation in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T016 [P] [US2] Add a no-snap regression test for starting a flip from a non-neutral pointer target in `bevy/crates/game/src/runtime/systems/mod.rs`

### Implementation for User Story 2

- [X] T017 [US2] Refactor card rotation update to compose `CardInspectionState` rotation with `CardFlipState` y rotation in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T018 [US2] Ensure card translation remains centered while composed rotation changes in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T019 [US2] Update parallax and frame shine reads to use the inspection-facing tilt appropriately while flip animation is active in `bevy/crates/game/src/runtime/systems/mod.rs`

**Checkpoint**: User Story 2 is complete when flip animation and pointer inspection work together without snapping to neutral.

---

## Phase 5: User Story 3 - Swap Face Graphics At Edge-On Point (Priority: P1)

**Goal**: CardFront remains visible before the edge-on midpoint and CardBack becomes visible after it, with the inverse behavior when flipping back.

**Independent Test**: Start a flip from neutral, observe the midpoint, and verify the visible face changes only when the card is edge-on.

### Tests for User Story 3

- [X] T020 [P] [US3] Add face visibility tests for front-to-back midpoint behavior in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T021 [P] [US3] Add face visibility tests for back-to-front midpoint behavior in `bevy/crates/game/src/runtime/systems/mod.rs`

### Implementation for User Story 3

- [X] T022 [US3] Implement front/back visibility switching from normalized flip progress in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T023 [US3] Implement back-to-front flip activation and midpoint side switching in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T024 [US3] Implement mid-animation `Flip` reversal from current progress in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T025 [US3] Schedule face visibility updates after flip animation and before parallax/shine reads in `bevy/crates/game/src/runtime/plugins/mod.rs`

**Checkpoint**: User Story 3 is complete when both flip directions swap graphics at the edge-on midpoint and mid-animation clicks reverse cleanly.

---

## Phase 6: User Story 4 - Use Shared Card-Series Back Design (Priority: P2)

**Goal**: CardBack is one shared card-series design independent of the active CardFront and compatible with the existing superhero card fronts.

**Independent Test**: Press `T`, flip to CardBack, and verify the same abstract superhero-pattern back appears regardless of active front.

### Tests for User Story 4

- [X] T026 [P] [US4] Add a test that CardBack asset path is not stored on individual front entries in `bevy/crates/game/src/runtime/resources/mod.rs`
- [X] T027 [P] [US4] Add a test that active front changes keep CardBack visible and stable in `bevy/crates/game/src/runtime/systems/mod.rs`

### Implementation for User Story 4

- [X] T028 [US4] Load `card_back_superhero_pattern.png` from `bevy/crates/game/assets/cards/card_structure/` in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T029 [US4] Ensure CardBack material and mesh are shared across active front changes in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T030 [US4] Verify the CardBack art contains no words, readable letters, characters, logos, or clear icon-like symbols in `bevy/crates/game/assets/cards/card_structure/card_back_superhero_pattern.png`

**Checkpoint**: User Story 4 is complete when CardBack is visually stable across front changes and the asset passes the art-direction audit.

---

## Phase 7: User Story 5 - Change Hidden Front While Face Down (Priority: P2)

**Goal**: Pressing `T` changes the active CardFront while CardBack remains visible until the card is flipped face up.

**Independent Test**: Flip face down, press `T`, confirm the back remains visible, then flip face up and confirm the newly selected CardFront appears.

### Tests for User Story 5

- [X] T031 [P] [US5] Add a hidden-front test for pressing `T` while CardBack is visible in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T032 [P] [US5] Add a face-up front-switch test for pressing `T` while CardFront is visible in `bevy/crates/game/src/runtime/systems/mod.rs`

### Implementation for User Story 5

- [X] T033 [US5] Update active front switching so `T` changes CardFront data without forcing CardFront visible in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T034 [US5] Preserve CardBack visibility when card scene reloads due to active front changes while face down in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T035 [US5] Ensure flipping face up after hidden `T` selection shows the newly active CardFront in `bevy/crates/game/src/runtime/systems/mod.rs`

**Checkpoint**: User Story 5 is complete when `T` updates hidden front state face down and reveals the changed front only after flipping face up.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, verification, and final cleanup across all stories.

- [X] T036 [P] Update implementation notes for card front/back terminology in `specs/006-card-flip/quickstart.md`
- [X] T037 Run `scripts/other/RunTests.ps1` and record any blocker in `specs/006-card-flip/quickstart.md`
- [X] T038 Run `scripts/other/RunAppDesktop.ps1 -CheckOnly` and record any blocker in `specs/006-card-flip/quickstart.md`
- [X] T039 Run `scripts/other/RunAppWeb.ps1 -CheckOnly` and record any blocker in `specs/006-card-flip/quickstart.md`
- [ ] T040 Manually verify desktop Card UI flip, midpoint swap, pointer inspection, `T` while face up, and `T` while face down using `scripts/other/RunAppDesktop.ps1`
- [ ] T041 Manually verify browser Card UI flip behavior using `scripts/other/RunAppWeb.ps1 -NoOpen` or document exact browser/WebGPU blocker in `specs/006-card-flip/quickstart.md`
- [X] T042 Verify docs state DeckBuilder is the current prototype entry point but not final user-facing game UI in `specs/006-card-flip/spec.md`
- [X] T043 Verify docs state Card UI is temporary and separate from DebugHUD in `specs/006-card-flip/spec.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 Setup**: No dependencies.
- **Phase 2 Foundational**: Depends on Phase 1 and blocks all user story work.
- **Phase 3 US1**: Depends on Phase 2.
- **Phase 4 US2**: Depends on Phase 3 because rotation composition extends the initial flip path.
- **Phase 5 US3**: Depends on Phase 3 and Phase 4 because midpoint visibility uses flip progress and composed rotation.
- **Phase 6 US4**: Depends on Phase 3 and can proceed in parallel with Phase 5 after CardBack exists.
- **Phase 7 US5**: Depends on Phase 5 and Phase 6 because hidden front switching requires correct face visibility and stable CardBack.
- **Phase 8 Polish**: Depends on selected user stories being complete.

### User Story Dependencies

- **US1 (P1)**: MVP; no dependencies beyond foundational tasks.
- **US2 (P1)**: Requires US1 flip activation and animation.
- **US3 (P1)**: Requires US1 flip state and US2 composed rotation.
- **US4 (P2)**: Requires US1 CardBack visual; independent of US2/US3 for asset audit, but final verification uses completed face switching.
- **US5 (P2)**: Requires US3 face visibility and US4 shared CardBack behavior.

### Parallel Opportunities

- T002 and T003 can run in parallel after T001.
- T009 and T010 can run in parallel before US1 implementation.
- T015 and T016 can run in parallel before US2 implementation.
- T020 and T021 can run in parallel before US3 implementation.
- T026 and T027 can run in parallel before US4 implementation.
- T031 and T032 can run in parallel before US5 implementation.
- T036 can run in parallel with final manual verification once behavior is complete.

---

## Parallel Examples

### User Story 1

```text
Task: "Add a Card UI contract test for `Flip` changing `CardFlipState` toward CardBack in `bevy/crates/game/src/runtime/systems/mod.rs`"
Task: "Add a scene composition test that exactly one `CardPlaceholder` exists after adding CardBack visuals in `bevy/crates/game/src/runtime/systems/mod.rs`"
```

### User Story 4

```text
Task: "Add a test that CardBack asset path is not stored on individual front entries in `bevy/crates/game/src/runtime/resources/mod.rs`"
Task: "Add a test that active front changes do not change CardBack material or texture handle in `bevy/crates/game/src/runtime/systems/mod.rs`"
```

---

## Implementation Strategy

### MVP First

1. Complete Phase 1 setup.
2. Complete Phase 2 foundational state and asset-path tests.
3. Complete Phase 3 User Story 1.
4. Stop and validate that `Flip` animates one CardFront to CardBack from the Card UI.

### Incremental Delivery

1. Add US1 for basic flip.
2. Add US2 for pointer-plus-flip composition.
3. Add US3 for edge-on face swap and reversal behavior.
4. Add US4 for shared card-series back design.
5. Add US5 for hidden front switching while face down.
6. Run final desktop and browser verification.

### Notes

- Keep broad Game, Player, Deck, hand, placed card, shared location, and Table Top concepts in `specs/007-gameplay-concepts/spec.md`.
- Keep implementation scoped to `bevy/crates/game` runtime ECS files and `bevy/crates/game/assets/cards/card_structure/`.
- Keep `DeckBuilder` as the current project entry point for this prototype, but do not treat it as final user-facing game UI.
- Keep Card UI temporary and separate from DebugHUD; do not add Flip as a DebugHUD toggle.
- Do not introduce gameplay, tabletop placement, hand UI, location UI, deck browser UI, turns, scoring, dragging, or menu flow in this feature.
