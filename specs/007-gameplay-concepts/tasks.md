# Tasks: Gameplay Concepts

**Input**: Design documents from `specs/007-gameplay-concepts/`  
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/gameview-round-flow-contract.md`, `quickstart.md`

**Tests**: Include focused model and regression tests because deck selection, round energy, undo, restart, and GameView control behavior are high-risk user-visible gameplay behavior.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel when it touches a different file or has no dependency on incomplete work
- **[Story]**: Which user story this task belongs to (US1, US2, US3, US4, US5)
- Include exact file paths in every task description

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare module boundaries and confirm the Bevy runtime reference before adding gameplay state.

- [ ] T001 Review `bevy/crates/template-crate/src/runtime/` before adding or reorganizing runtime files for this feature
- [ ] T002 [P] Add game control component module exports in `bevy/crates/game/src/runtime/components/mod.rs`
- [ ] T003 [P] Add game deck, hand, location, and round model module exports in `bevy/crates/game/src/runtime/resources/mod.rs`
- [ ] T004 [P] Add game deck deal, location effect, round update, undo, and restart system module exports in `bevy/crates/game/src/runtime/systems/mod.rs`
- [ ] T005 Register new runtime resources and systems in the existing app/game setup path in `bevy/crates/game/src/runtime/systems/mod.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Add shared runtime state and deterministic helpers required by all GameView round-flow work.

**CRITICAL**: No user story implementation can begin until this phase is complete.

- [ ] T006 [P] Create `CardDefinitionModel`, `CardInstanceModel`, `CardZone`, and the four-card master list in `bevy/crates/game/src/runtime/resources/game_deck_model.rs`
- [ ] T007 [P] Add tests for card definition values and 12-card deck composition in `bevy/crates/game/src/runtime/resources/game_deck_model.rs`
- [ ] T008 [P] Create `GameDeckModel` with randomized deck construction and remaining-card tracking in `bevy/crates/game/src/runtime/resources/game_deck_model.rs`
- [ ] T009 [P] Add tests for energy-sorted deal selection, exact energy matching, partial matching, and no-match skipped deals in `bevy/crates/game/src/runtime/resources/game_deck_model.rs`
- [ ] T010 [P] Create `GameHandModel` with hand card ordering and centered safe-view layout data in `bevy/crates/game/src/runtime/resources/game_hand_model.rs`
- [ ] T011 [P] Add tests for hand append, return, removal, and centered layout overflow behavior in `bevy/crates/game/src/runtime/resources/game_hand_model.rs`
- [ ] T012 [P] Create `GameRoundModel`, round schedule helpers, and current-round move history records in `bevy/crates/game/src/runtime/resources/game_round_model.rs`
- [ ] T013 [P] Add tests for six-round energy, requested deal counts, required deal energy, round-6 End Turn, and move-history clearing in `bevy/crates/game/src/runtime/resources/game_round_model.rs`
- [ ] T014 [P] Create `GameControlAction`, `GameControlButton`, and `GameControlView` components with required `HUMAN:`/`AI:` comments in `bevy/crates/game/src/runtime/components/game_control_component.rs`
- [ ] T015 Add resource reset helpers that reset deck, hand, locations, slots, card gesture state, card states, round state, and control state in `bevy/crates/game/src/runtime/systems/game_restart_update_system.rs`

**Checkpoint**: Runtime models exist, are exported, and can be tested without rendering.

---

## Phase 3: User Story 5 - Play Local Round Progression (Priority: P1) MVP

**Goal**: The near human player can play through six local rounds with eligible energy-matched deals, energy spending, undo, End Turn, and Restart.

**Independent Test**: Launch or inspect GameView and verify lower-right End Turn remains present, lower-left Restart and Undo are present, eligible cards deal by energy match, energy progresses `1/1` through `6/6`, Undo affects only current-round placements, and Restart returns to a fresh round `1/6`.

### Tests for User Story 5

- [ ] T016 [P] [US5] Add contract-style tests for the GameView round schedule and energy-matched eligible deal counts in `bevy/crates/game/src/runtime/resources/game_round_model.rs`
- [ ] T017 [P] [US5] Add deal-selection tests for round 1, round 2, round 3, and skipped rounds 4 through 6 with the initial deck in `bevy/crates/game/src/runtime/resources/game_deck_model.rs`
- [ ] T018 [P] [US5] Add hand recentering tests after deal, placement, undo, and restart in `bevy/crates/game/src/runtime/resources/game_hand_model.rs`
- [ ] T019 [P] [US5] Add integration tests for affordable placement, over-cost rejection, energy deduction, and energy restoration on undo in `bevy/crates/game/src/runtime/systems/card_gesture_update_system.rs`
- [ ] T020 [P] [US5] Add restart reset tests for deck, hand, round, energy, slots, card states, gesture state, and move history in `bevy/crates/game/src/runtime/systems/game_restart_update_system.rs`

### Implementation for User Story 5

- [ ] T021 [US5] Implement start-of-game and start-of-round eligible deal orchestration in `bevy/crates/game/src/runtime/systems/game_deck_deal_system.rs`
- [ ] T022 [US5] Spawn or update visible hand card entities from `GameHandModel` in `bevy/crates/game/src/runtime/scenes/game_view_scene.rs`
- [ ] T023 [US5] Animate dealt cards from below safe-view screen center into centered hand positions in `bevy/crates/game/src/runtime/systems/game_deck_deal_system.rs`
- [ ] T024 [US5] Add lower-left Restart and Undo controls under the aspect-ratio-safe HUD root in `bevy/crates/game/src/runtime/scenes/game_view_scene.rs`
- [ ] T025 [US5] Preserve and update the lower-right End Turn control and round fraction in `bevy/crates/game/src/runtime/scenes/game_view_scene.rs`
- [ ] T026 [US5] Implement End Turn advancement, current-round history clearing, round energy reset, round-6 allowed End Turn, and next-round deal triggering in `bevy/crates/game/src/runtime/systems/game_round_update_system.rs`
- [ ] T027 [US5] Integrate card placement energy validation before slot placement in `bevy/crates/game/src/runtime/systems/card_gesture_update_system.rs`
- [ ] T028 [US5] Record current-round move history, placement energy cost, and applied location energy delta after successful hand-to-location placement in `bevy/crates/game/src/runtime/systems/card_gesture_update_system.rs`
- [ ] T029 [US5] Implement Undo to return only current-round placed cards to hand, remove active location energy deltas, restore their energy spend, clear current-round history, free slots, and recenter hand in `bevy/crates/game/src/runtime/systems/game_undo_update_system.rs`
- [ ] T030 [US5] Disable or grey out Undo when current-round move history is empty and update the `Energy current/max` label in `bevy/crates/game/src/runtime/systems/game_undo_update_system.rs`
- [ ] T031 [US5] Implement Restart control handling that clears active GameView play state and starts a fresh randomized round `1/6` game in `bevy/crates/game/src/runtime/systems/game_restart_update_system.rs`

**Checkpoint**: User Story 5 is fully functional and testable independently as the MVP runtime loop.

---

## Phase 4: User Story 1 - Preserve Gameplay Vocabulary (Priority: P1)

**Goal**: Developers and designers can rely on consistent gameplay terms while the runtime models use the same vocabulary.

**Independent Test**: Review the spec, plan, data model, contracts, and runtime model names; verify `Game`, `Player`, `Deck`, `CardSeries`, `CardDefinition`, `CardInstance`, `Hand`, `Placed Cards`, `Shared Location`, and `Table Top` are consistently used and do not require `006-card-bundle` changes.

### Implementation for User Story 1

- [ ] T032 [US1] Align runtime model and enum names with `CardDefinitionModel`, `CardInstanceModel`, `GameDeckModel`, `GameHandModel`, and `GameRoundModel` terminology in `bevy/crates/game/src/runtime/resources/game_deck_model.rs`
- [ ] T033 [US1] Add terse concept comments for deck, hand, round, and card instance runtime items in `bevy/crates/game/src/runtime/resources/game_deck_model.rs`
- [ ] T034 [US1] Verify `006-card-bundle` behavior remains untouched by reviewing feature-scoped changes and documenting any exception in `specs/007-gameplay-concepts/quickstart.md`

**Checkpoint**: Gameplay vocabulary is represented consistently in docs and runtime model names.

---

## Phase 5: User Story 2 - Describe Future Hidden Card Flow (Priority: P2)

**Goal**: Future hidden card placement and reveal terminology remains clear without adding reveal gameplay in this feature.

**Independent Test**: Review the docs and runtime boundaries; verify CardFront/CardBack ownership and future reveal concepts remain documented but no reveal, face-down, scoring, or CPU behavior is implemented.

### Implementation for User Story 2

- [ ] T035 [US2] Keep CardFront/CardBack ownership language reflected in code comments for card definition and card instance runtime concepts in `bevy/crates/game/src/runtime/resources/game_deck_model.rs`
- [ ] T036 [US2] Confirm no reveal-state behavior is added to card placement systems while documenting the scope exclusion in `specs/007-gameplay-concepts/quickstart.md`
- [ ] T037 [US2] Preserve existing card back/front rendering boundaries while integrating dealt cards in `bevy/crates/game/src/runtime/bundles/card_view_bundle.rs`

**Checkpoint**: Future hidden-card vocabulary is preserved without expanding runtime scope.

---

## Phase 6: User Story 3 - Clarify Prototype Scene Layering (Priority: P2)

**Goal**: AppScene remains persistent, GameView remains the startup child view, and DeckBuilderScene remains available through existing scene switching behavior.

**Independent Test**: Launch or inspect the runtime and verify AppScene owns exactly one active child, GameView starts by default, DeckBuilderScene can still be reached through the scene shortcut, and debug UI is not recreated on child scene changes.

### Tests for User Story 3

- [ ] T038 [P] [US3] Add or update active child scene lifecycle tests for AppScene, GameView, DeckBuilderScene, and DebugSettingsScene in `bevy/crates/game/src/runtime/systems/mod.rs`
- [ ] T039 [P] [US3] Add regression coverage that Restart resets only GameView play state and does not recreate AppScene debug UI in `bevy/crates/game/src/runtime/systems/game_restart_update_system.rs`

### Implementation for User Story 3

- [ ] T040 [US3] Preserve AppScene parenting and active child scene ownership while wiring GameView resources in `bevy/crates/game/src/runtime/scenes/game_view_scene.rs`
- [ ] T041 [US3] Preserve existing DeckBuilderScene setup and reload behavior while adding GameView round resources in `bevy/crates/game/src/runtime/scenes/deck_builder_scene.rs`
- [ ] T042 [US3] Verify `S` scene switching still cycles GameView, DeckBuilderScene, and DebugSettingsScene after GameView round setup in `bevy/crates/game/src/runtime/systems/mod.rs`

**Checkpoint**: Scene layering remains intact with the new GameView runtime loop.

---

## Phase 7: User Story 4 - Introduce Game Scene Table Top (Priority: P2)

**Goal**: GameView continues to show DesertWorld, three locations, hand area, TurnUI, and hybrid 2D/3D card presentation while supporting the new round loop.

**Independent Test**: Launch or inspect GameView and verify the DesertWorld background, exactly three locations, location open/closed outlines, centered title/body text, local hand area, End Turn, Restart, Undo, and 3D hand cards all appear within the safe area.

### Tests for User Story 4

- [ ] T043 [P] [US4] Add or update tests for exactly three GameView locations, location open/closed outline state, and safe-area placement in `bevy/crates/game/src/runtime/scenes/game_view_scene.rs`
- [ ] T044 [P] [US4] Add or update tests that hand card transforms derive from the aspect-ratio-safe GameView rather than raw window pixels in `bevy/crates/game/src/runtime/resources/game_hand_model.rs`

### Implementation for User Story 4

- [ ] T045 [US4] Preserve DesertWorld background and three location UI instances while adding round controls in `bevy/crates/game/src/runtime/scenes/game_view_scene.rs`
- [ ] T046 [US4] Keep local hand area and 3D card presentation aligned with the aspect-ratio-safe GameView in `bevy/crates/game/src/runtime/scenes/game_view_scene.rs`
- [ ] T047 [US4] Keep closed red and open green location outline behavior intact while adding placement and undo state changes in `bevy/crates/game/src/runtime/bundles/location_view_bundle.rs`

**Checkpoint**: Table top presentation remains visually stable while the local round loop runs.

---

## Phase 8: Location Display And Ability Extension

**Purpose**: Add open/closed location text and the initial effective-energy location ability behavior.

- [ ] T048 [P] [US4] Create `LocationDefinitionModel`, `LocationModel`, `LocationState`, and `LocationAbility` with Fortress Gate, Bamboo Crossing, and Normal definitions in `bevy/crates/game/src/runtime/resources/game_location_model.rs`
- [ ] T049 [P] [US4] Add tests for left, middle, and right location open rounds, closed titles, open titles, ability bodies, and `(No Ability)` fallback in `bevy/crates/game/src/runtime/resources/game_location_model.rs`
- [ ] T050 [US4] Render each location title as larger centered text about 30% from the location top with up to two lines in `bevy/crates/game/src/runtime/bundles/location_view_bundle.rs`
- [ ] T051 [US4] Render each location body as centered text below the title with up to three lines and blank body while closed in `bevy/crates/game/src/runtime/bundles/location_view_bundle.rs`
- [ ] T052 [US4] Update GameView location setup to use `Fortress Gate`, `Bamboo Crossing`, and `Normal` definitions instead of three generic locations in `bevy/crates/game/src/runtime/scenes/game_view_scene.rs`
- [ ] T053 [US5] Apply open location effective-energy deltas immediately after successful card placement in `bevy/crates/game/src/runtime/systems/game_location_effect_system.rs`
- [ ] T054 [P] [US5] Add tests for Fortress Gate `+2`, Bamboo Crossing `-2`, Normal no-op, and closed-location no-op effects in `bevy/crates/game/src/runtime/systems/game_location_effect_system.rs`
- [ ] T055 [US5] Remove active location effective-energy deltas when Undo returns current-round cards to hand in `bevy/crates/game/src/runtime/systems/game_undo_update_system.rs`
- [ ] T056 [US5] Recompute open/closed location state on End Turn round advancement and apply newly opened location effects to cards already there in `bevy/crates/game/src/runtime/systems/game_round_update_system.rs`
- [ ] T057 [US5] Reset all location state, title/body display, placed-card lists, and card effective-energy deltas on Restart in `bevy/crates/game/src/runtime/systems/game_restart_update_system.rs`

**Checkpoint**: Locations display the requested state text and apply only the defined open-location effective-energy effects.

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Verification, cleanup, and target parity across the full feature.

- [ ] T058 Run focused game crate tests with `cargo test -p bevy-card-game`
- [ ] T059 Run workspace tests with `scripts/other/RunTests.ps1`
- [ ] T060 Run desktop verification from `specs/007-gameplay-concepts/quickstart.md`
- [ ] T061 Run browser WebGPU verification from `specs/007-gameplay-concepts/quickstart.md` or document the exact blocker in `specs/007-gameplay-concepts/quickstart.md`
- [ ] T062 Capture an AI runtime screenshot through the Bevy Remote Protocol workflow and inspect GameView controls, hand centering, location text/effects, and current-round undo behavior with output under `target/ai-runtime-screenshots/`
- [ ] T063 Verify changed Bevy runtime files use `bevy/crates/template-crate` as the proper reference and follow one-primary-concept, Scene/Model/View naming, `[domain]_[schedule]_system` naming, and `HUMAN:`/`AI:` purpose comment standards in `bevy/crates/game/src/runtime/`
- [ ] T064 Update `specs/007-gameplay-concepts/quickstart.md` with final desktop/browser verification notes and any blocked target details

---

## Dependencies & Execution Order

### Phase Dependencies

| Phase | Depends On | Blocks |
| ----- | ---------- | ------ |
| Phase 1 Setup | None | Phase 2 |
| Phase 2 Foundational | Phase 1 | All user stories |
| Phase 3 US5 | Phase 2 | MVP and gameplay loop validation |
| Phase 4 US1 | Phase 2; should preserve US5 names | Final consistency |
| Phase 5 US2 | Phase 2; should preserve US5 placement boundaries | Final consistency |
| Phase 6 US3 | Phase 2 and US5 setup path | Final verification |
| Phase 7 US4 | Phase 2 and US5 presentation integration | Final verification |
| Phase 8 Location Display And Ability Extension | Phase 2, US4 setup, and US5 placement integration | Location text/effect validation |
| Phase 9 Polish | Desired user stories complete | Release/readiness |

### User Story Dependencies

| User Story | Dependency | Notes |
| ---------- | ---------- | ----- |
| US5 Play Local Round Progression | Foundational only | MVP runtime behavior requested by the latest spec update |
| US1 Preserve Gameplay Vocabulary | Foundational only; may run alongside US5 once model names are chosen | Primarily naming/docs consistency |
| US2 Describe Future Hidden Card Flow | Foundational only; should avoid adding reveal scope | Ensures future concepts remain boundaries, not implementation creep |
| US3 Clarify Prototype Scene Layering | US5 setup path | Protects AppScene/child-scene behavior while GameView grows |
| US4 Introduce Game Scene Table Top | US5 presentation integration | Protects visual layout while controls and dealt cards are added |

### Parallel Opportunities

| Area | Parallel Tasks |
| ---- | -------------- |
| Setup exports | T002, T003, T004 |
| Foundational models | T006, T010, T012, T014 |
| Foundational tests | T007, T009, T011, T013 after respective models |
| US5 tests | T016, T017, T018, T019, T020 |
| US3 tests | T038, T039 |
| US4 tests | T043, T044 |
| Location extension tests | T049, T054 |
| Polish verification | T060, T061, T062 after tests and runtime launch paths are ready |

---

## Parallel Example: User Story 5

```text
Task: "T016 [P] [US5] Add contract-style tests for the GameView round schedule and energy-matched eligible deal counts in bevy/crates/game/src/runtime/resources/game_round_model.rs"
Task: "T017 [P] [US5] Add deal-selection tests for round 1, round 2, round 3, and skipped rounds 4 through 6 with the initial deck in bevy/crates/game/src/runtime/resources/game_deck_model.rs"
Task: "T018 [P] [US5] Add hand recentering tests after deal, placement, undo, and restart in bevy/crates/game/src/runtime/resources/game_hand_model.rs"
Task: "T020 [P] [US5] Add restart reset tests for deck, hand, round, energy, slots, card states, gesture state, and move history in bevy/crates/game/src/runtime/systems/game_restart_update_system.rs"
```

## Parallel Example: User Story 4

```text
Task: "T043 [P] [US4] Add or update tests for exactly three GameView locations, location open/closed outline state, and safe-area placement in bevy/crates/game/src/runtime/scenes/game_view_scene.rs"
Task: "T044 [P] [US4] Add or update tests that hand card transforms derive from the aspect-ratio-safe GameView rather than raw window pixels in bevy/crates/game/src/runtime/resources/game_hand_model.rs"
```

---

## Implementation Strategy

### MVP First (User Story 5 Only)

1. Complete Phase 1 setup.
2. Complete Phase 2 foundational models, exports, and reset helpers.
3. Complete Phase 3 US5.
4. Stop and validate the GameView local round loop independently with focused tests and desktop inspection.

### Incremental Delivery

1. Add US5 to make GameView playable through local round progression.
2. Add US1 consistency checks so implementation vocabulary stays aligned with the concept spec.
3. Add US2 boundary checks so hidden-card concepts do not accidentally become runtime scope.
4. Add US3 scene-layering regression coverage.
5. Add US4 presentation regression coverage.
6. Add location text, open-state, and effective-energy ability behavior.
7. Finish with desktop and browser WebGPU verification.

### Parallel Team Strategy

| Contributor | Work |
| ----------- | ---- |
| A | Game deck and deal-selection models plus deal system |
| B | Game hand and round/energy models plus UI controls |
| C | Undo/restart integration plus scene/presentation regression checks |

## Notes

- `[P]` tasks use different files or independent test scopes.
- `[US#]` labels map tasks to user stories for traceability.
- Keep all visible layout derived from the aspect-ratio-safe `GameView`.
- Keep runtime source organized around one primary concept per file.
- Do not add CPU turns, scoring, card abilities, full location control, persistence, production mobile layout, reveal-resolution behavior, or additional location ability types in this feature.
- Avoid reverting unrelated modified runtime files already present in the working tree.
