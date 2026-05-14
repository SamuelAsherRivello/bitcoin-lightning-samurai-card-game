# Tasks: DeckScreen Meta Game UI

**Input**: Design documents from `/specs/018-deck-screen/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/deck-screen-ui-contract.md, quickstart.md

**Tests**: Automated tests are required by FR-016 for top-nav selection, DeckScreen state transitions, membership derivation, modal action enablement, and input blocking where practical.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish the DeckScreen implementation surface and project references.

- [ ] T001 Inspect `bevy/crates/template-crate` as the Bevy folder/file/style reference before creating DeckScreen runtime files
- [ ] T002 [P] Add DeckScreen and reusable top-navigation notes to `README.md`
- [ ] T003 [P] Confirm `AGENTS.md` active plan points to `specs/018-deck-screen/plan.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core model and wiring that all DeckScreen stories depend on.

- [ ] T004 Create `DeckScreenModel`, `DeckScreenMode`, `DeckEditorTabModel`, `DeckEditableZoneModel`, and modal action-state types in `bevy/crates/game/src/runtime/resources/deck_screen_model.rs`
- [ ] T005 Create `TopNavigationModel` and `TopNavigationDestination` in `bevy/crates/game/src/runtime/resources/top_navigation_model.rs`
- [ ] T006 Export DeckScreen and top-navigation resources from `bevy/crates/game/src/runtime/resources/mod.rs`
- [ ] T007 Create DeckScreen component markers for deck tiles, card tiles, tabs, modal actions, and lower input capture in `bevy/crates/game/src/runtime/components/deck_screen_component.rs`
- [ ] T008 Create top-navigation component markers for root, destination button, selected state, and disabled/unimplemented destination state in `bevy/crates/game/src/runtime/components/top_navigation_component.rs`
- [ ] T009 Export DeckScreen and top-navigation components from `bevy/crates/game/src/runtime/components/mod.rs`
- [ ] T010 Add DeckScreen and top-navigation resource initialization in `bevy/crates/game/src/runtime/plugins/mod.rs`
- [ ] T011 [P] Add model tests for DeckScreen mode transitions and modal action enablement in `bevy/crates/game/src/tests/runtime/resources/deck_screen_model_tests.rs`
- [ ] T012 [P] Add model tests for top-navigation destination ordering and selected state in `bevy/crates/game/src/tests/runtime/resources/top_navigation_model_tests.rs`

**Checkpoint**: DeckScreen and top-navigation state exists and can be tested without UI.

---

## Phase 3: User Story 1 - Navigate With Shared Top Navigation (Priority: P1) MVP

**Goal**: Show reusable top navigation on DeckScreen with `My Decks` selected.

**Independent Test**: Open DeckScreen and verify the top nav shows `Play Game`, `My Decks`, `Settings`, and `Debug`, with `My Decks` selected.

### Tests for User Story 1

- [ ] T013 [P] [US1] Add system tests for top-navigation rendering and selected `My Decks` state in `bevy/crates/game/src/tests/runtime/systems/top_navigation_update_system_tests.rs`
- [ ] T014 [P] [US1] Add system tests that top navigation is mounted on DeckScreen only for this feature in `bevy/crates/game/src/tests/runtime/systems/top_navigation_update_system_tests.rs`

### Implementation for User Story 1

- [ ] T015 [US1] Create reusable top-navigation view bundle in `bevy/crates/game/src/runtime/bundles/top_navigation_view_bundle.rs`
- [ ] T016 [US1] Export top-navigation view bundle from `bevy/crates/game/src/runtime/bundles/mod.rs`
- [ ] T017 [US1] Implement top-navigation spawn/update behavior in `bevy/crates/game/src/runtime/systems/top_navigation_update_system.rs`
- [ ] T018 [US1] Mount top navigation in DeckScene setup only in `bevy/crates/game/src/runtime/scenes/deck_scene.rs`
- [ ] T019 [US1] Register top-navigation systems in `bevy/crates/game/src/runtime/plugins/mod.rs`
- [ ] T020 [US1] Ensure modal-open state blocks top-navigation activation in `bevy/crates/game/src/runtime/systems/top_navigation_update_system.rs`

**Checkpoint**: DeckScreen has the reusable top nav, scoped only to DeckScreen.

---

## Phase 4: User Story 2 - Browse Decks (Priority: P1)

**Goal**: Show DeckScreen deck-selection state with `New Deck` and the single `Deck 01` tile.

**Independent Test**: Open DeckScreen and verify deck-selection tiles are readable inside the safe area.

### Tests for User Story 2

- [ ] T021 [P] [US2] Add system tests for DeckScreen deck-selection mode in `bevy/crates/game/src/tests/runtime/systems/deck_screen_update_system_tests.rs`
- [ ] T022 [P] [US2] Add system tests for `Deck 01` tile derivation from `PlayerDeckCollectionModel` in `bevy/crates/game/src/tests/runtime/systems/deck_screen_update_system_tests.rs`

### Implementation for User Story 2

- [ ] T023 [US2] Add DeckScreen deck-selection layout setup in `bevy/crates/game/src/runtime/scenes/deck_scene.rs`
- [ ] T024 [US2] Implement deck tile spawning for `New Deck` and `Deck 01` in `bevy/crates/game/src/runtime/systems/deck_screen_update_system.rs`
- [ ] T025 [US2] Implement deck tile input handling to enter selected deck editor mode in `bevy/crates/game/src/runtime/systems/deck_screen_update_system.rs`
- [ ] T026 [US2] Register DeckScreen update/input systems in `bevy/crates/game/src/runtime/plugins/mod.rs`
- [ ] T027 [US2] Ensure DebugHUD reports `DeckScreen` while DeckScene is active in `bevy/crates/game/src/runtime/systems/mod.rs`

**Checkpoint**: DeckScreen deck selection is functional and independently testable.

---

## Phase 5: User Story 3 - Edit Selected Deck From Library (Priority: P1)

**Goal**: Show selected deck editor with `Deck 01`, empty slots, and `Not In Deck` Library cards.

**Independent Test**: Select `Deck 01` and verify the split editor shows deck cards on the left and non-deck library cards on the right.

### Tests for User Story 3

- [ ] T028 [P] [US3] Add model tests for deck/library membership partitioning in `bevy/crates/game/src/tests/runtime/resources/deck_screen_model_tests.rs`
- [ ] T029 [P] [US3] Add system tests for selected deck editor layout entities in `bevy/crates/game/src/tests/runtime/systems/deck_screen_update_system_tests.rs`

### Implementation for User Story 3

- [ ] T030 [US3] Add selected deck editor layout containers and safe-area dimensions in `bevy/crates/game/src/runtime/scenes/deck_scene.rs`
- [ ] T031 [US3] Spawn `Deck 01` card tiles and empty-slot views in `bevy/crates/game/src/runtime/systems/deck_screen_update_system.rs`
- [ ] T032 [US3] Spawn `Not In Deck` Library card tiles from owned non-deck entries in `bevy/crates/game/src/runtime/systems/deck_screen_update_system.rs`
- [ ] T033 [US3] Implement Library tab selected visual state in `bevy/crates/game/src/runtime/systems/deck_screen_update_system.rs`
- [ ] T034 [US3] Implement real card tile selection to open `DeckScreenCardModalModel` in `bevy/crates/game/src/runtime/systems/deck_screen_update_system.rs`

**Checkpoint**: Selected deck editor works independently before shop and modal actions are complete.

---

## Phase 6: User Story 4 - View Shop Offers (Priority: P2)

**Goal**: Add the Shop tab as an empty state; shop contents come later.

**Independent Test**: Switch to Shop and verify the shop is empty while deck membership remains unchanged.

### Tests for User Story 4

- [ ] T035 [P] [US4] Add model tests for Library/Shop tab switching without deck mutation in `bevy/crates/game/src/tests/runtime/resources/deck_screen_model_tests.rs`
- [ ] T036 [P] [US4] Add system tests for shop offer tile rendering and selected tab state in `bevy/crates/game/src/tests/runtime/systems/deck_screen_update_system_tests.rs`

### Implementation for User Story 4

- [ ] T037 [US4] Add empty shop data derivation in `bevy/crates/game/src/runtime/resources/deck_screen_model.rs`
- [ ] T038 [US4] Implement Library and Shop tab input handling in `bevy/crates/game/src/runtime/systems/deck_screen_update_system.rs`
- [ ] T039 [US4] Spawn an empty shop state with no card tiles in `bevy/crates/game/src/runtime/systems/deck_screen_update_system.rs`
- [ ] T040 [US4] Keep shop interactions non-mutating until purchase behavior exists in `bevy/crates/game/src/runtime/systems/deck_screen_update_system.rs`

**Checkpoint**: Shop tab is visually represented and safe from unintended persistence changes.

---

## Phase 7: User Story 5 - Inspect And Move A Card In Modal (Priority: P2)

**Goal**: Add DeckScreen fullscreen card modal with dimming, action rail, lower input blocking, Back, and enabled move actions.

**Independent Test**: Select a card, verify the overlay blocks lower UI and top nav, then use Back or enabled move actions and verify visible/persisted results.

### Tests for User Story 5

- [ ] T041 [P] [US5] Add model tests for modal action enablement and illegal empty-slot selection in `bevy/crates/game/src/tests/runtime/resources/deck_screen_model_tests.rs`
- [ ] T042 [P] [US5] Add system tests for modal dimming, lower input blocking, top-nav blocking, and Back close behavior in `bevy/crates/game/src/tests/runtime/systems/deck_screen_update_system_tests.rs`
- [ ] T043 [P] [US5] Add persistence-oriented system tests for move-to-deck and move-to-library membership updates in `bevy/crates/game/src/tests/runtime/systems/deck_screen_update_system_tests.rs`

### Implementation for User Story 5

- [ ] T044 [US5] Spawn DeckScreen modal dim layer, large card preview, and action rail in `bevy/crates/game/src/runtime/systems/deck_screen_update_system.rs`
- [ ] T045 [US5] Implement modal input capture so lower DeckScreen and top navigation do not react while modal is open in `bevy/crates/game/src/runtime/systems/deck_screen_update_system.rs`
- [ ] T046 [US5] Implement `Back` action to close the modal without changing deck data in `bevy/crates/game/src/runtime/systems/deck_screen_update_system.rs`
- [ ] T047 [US5] Implement `Move To Deck 01` action with duplicate and capacity validation in `bevy/crates/game/src/runtime/systems/deck_screen_update_system.rs`
- [ ] T048 [US5] Implement `Move To Library` action with ownership and selected-deck validation in `bevy/crates/game/src/runtime/systems/deck_screen_update_system.rs`
- [ ] T049 [US5] Persist accepted move actions through the existing player deck collection store in `bevy/crates/game/src/runtime/systems/deck_screen_update_system.rs`
- [ ] T050 [US5] Keep `Transfer Out` visible and disabled until transfer behavior has a separate feature spec in `bevy/crates/game/src/runtime/systems/deck_screen_update_system.rs`

**Checkpoint**: Modal inspection and deck/library movement are functional and independently testable.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Final documentation, verification, and visual polish across stories.

- [ ] T051 [P] Update `specs/018-deck-screen/quickstart.md` with final implementation notes and any blocked browser verification
- [ ] T052 [P] Update `README.md` with DeckScreen controls, states, top-navigation reuse, and screen terminology
- [ ] T053 Run `scripts/other/RunTests.ps1` and record results or blockers in `specs/018-deck-screen/quickstart.md`
- [ ] T054 Inspect DeckScreen visually through desktop or AI runtime screenshot workflow and record safe-area/modal observations in `specs/018-deck-screen/quickstart.md`
- [ ] T055 Verify changed Bevy runtime files follow `bevy/crates/template-crate`, one-primary-concept organization, Scene/Model/View naming, `[domain]_[schedule]_system` naming, and HUMAN/AI comments

---

## Dependencies & Execution Order

| Phase | Depends On | Blocks |
| ----- | ---------- | ------ |
| Phase 1 Setup | None | Phase 2 |
| Phase 2 Foundational | Phase 1 | All user stories |
| Phase 3 US1 Top Navigation | Phase 2 | DeckScreen MVP shell |
| Phase 4 US2 Browse Decks | Phase 2, US1 | Editor entry |
| Phase 5 US3 Editor Library | Phase 2, US2 | Shop and modal workflows |
| Phase 6 US4 Shop | Phase 2, US3 | None |
| Phase 7 US5 Modal | Phase 2, US3 | Final modal/move workflow |
| Phase 8 Polish | Desired user stories | Release readiness |

## Parallel Opportunities

| Area | Parallel Work |
| ---- | ------------- |
| Setup | T002 and T003 can run in parallel. |
| Foundation | T011 and T012 can run in parallel. |
| US1 | T013 and T014 can run in parallel. |
| US3 | T028 and T029 can run in parallel. |
| US5 | T041, T042, and T043 can run in parallel. |

## Implementation Strategy

| Step | Scope |
| ---- | ----- |
| MVP | Complete setup, foundation, and US1 top navigation on DeckScreen. |
| Increment 2 | Add deck selection. |
| Increment 3 | Add editor Library view. |
| Increment 4 | Add Shop affordances. |
| Increment 5 | Add modal and move actions. |

## Deck View Constraint

| Requirement | Detail |
| ----------- | ------ |
| DeckViewBundle | Implementation MUST create a DeckViewBundle that renders a deck tile using the existing card back asset and the deck name only. |

