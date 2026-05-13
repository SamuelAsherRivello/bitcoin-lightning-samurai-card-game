# Tasks: Opponent Modes and Two-Player Resolution

**Input**: Design documents from `specs/014-opponent/`  
**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/gameview-opponent-ui.md](./contracts/gameview-opponent-ui.md), [quickstart.md](./quickstart.md)

**Tests**: Included because quickstart defines expected unit/system coverage for mode cycling, Status text, CPU Brain Level 1, readiness, slot ownership, winner resolution, CPU-vs-CPU autoplay, CPU-owned passive card rendering, and GameView controls.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

| Marker | Meaning |
| ------ | ------- |
| `[P]` | Can run in parallel with other `[P]` tasks in the same phase because it touches different files or only adds independent tests. |
| `[US#]` | Maps task to the numbered user story in [spec.md](./spec.md). |

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Confirm local references and prepare the planned runtime/test files before feature implementation.

- [X] T001 Inspect `bevy/crates/template-crate/src/runtime/resources/template_resource.rs`, `bevy/crates/template-crate/src/runtime/systems/template_system.rs`, and `bevy/crates/template-crate/src/tests/runtime/resources/template_resource_tests.rs` for naming, comments, and test module patterns before editing Bevy runtime files
- [X] T002 Inspect existing GameView control, round, deck, slot, and scoring code in `bevy/crates/game/src/runtime/components/game_control_component.rs`, `bevy/crates/game/src/runtime/resources/game_round_model.rs`, `bevy/crates/game/src/runtime/resources/card_slot_model.rs`, `bevy/crates/game/src/runtime/resources/point_model.rs`, and `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T003 [P] Inspect existing runtime tests in `bevy/crates/game/src/tests/runtime/resources/game_round_model_tests.rs`, `bevy/crates/game/src/tests/runtime/resources/card_slot_model_tests.rs`, `bevy/crates/game/src/tests/runtime/resources/point_model_tests.rs`, and `bevy/crates/game/src/tests/runtime/systems/systems_tests.rs`
- [X] T004 [P] Review `specs/014-opponent/contracts/gameview-opponent-ui.md` and `specs/014-opponent/quickstart.md` to keep task implementation aligned with visible UI and verification expectations

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Add shared two-player match primitives required by all user stories.

**CRITICAL**: No user story work can begin until this phase is complete.

- [X] T005 [P] Add failing model tests for `MatchModeModel`, `MatchPlayerModel`, `MatchTurnModel`, `PlayerController`, `CpuController`, and hidden CPU Brain labels in `bevy/crates/game/src/tests/runtime/resources/opponent_match_model_tests.rs`
- [ ] T006 [P] Add failing model tests for near/far slot side ownership and non-local placement helpers in `bevy/crates/game/src/tests/runtime/resources/card_slot_model_tests.rs`
- [ ] T007 [P] Add failing model tests for non-draw final match tiebreaking in `bevy/crates/game/src/tests/runtime/resources/point_model_tests.rs`
- [X] T008 Create `bevy/crates/game/src/runtime/resources/opponent_match_model.rs` with `MatchModeModel`, `PlayerController`, `CpuController`, `CpuBrainLevel`, `MatchPlayerModel`, and `MatchTurnModel` primary items using required `HUMAN:` and `AI:` comments
- [X] T009 Export `opponent_match_model` from `bevy/crates/game/src/runtime/resources/mod.rs`
- [X] T010 Generalize side placement helpers in `bevy/crates/game/src/runtime/resources/card_slot_model.rs` so both near/bottom and far/top sides can be populated by owner-aware gameplay code while direct human drag remains near-only
- [ ] T011 Update `bevy/crates/game/src/runtime/resources/point_model.rs` so final match resolution can deterministically return near/local or far/opponent winner and never expose a draw result for completed matches
- [X] T012 Update plugin resource registration in `bevy/crates/game/src/runtime/plugins/mod.rs` to initialize the new transient opponent/match model resources
- [X] T013 [P] Add failing placement visibility tests for current-turn hidden placements, owner-visible fronts, and end-of-turn reveal in `bevy/crates/game/src/tests/runtime/resources/opponent_match_model_tests.rs`
- [X] T014 Add `PlacementVisibilityModel` and controller knowledge helpers to `bevy/crates/game/src/runtime/resources/opponent_match_model.rs` so current-turn placements hide from opposing controllers and reveal permanently at turn end

**Checkpoint**: Shared two-player match state exists, tests cover model transitions and no-draw scoring, and user story implementation can begin.

---

## Phase 3: User Story 1 - Choose Match Mode (Priority: P1)

**Goal**: The player can see the lower-left Status/Mode/Restart control stack, use a two-line Mode button above Restart, and cycle only between `Human versus CPU` and `CPU versus CPU`.

**Independent Test**: Launch or inspect GameView and verify the Mode button is above Restart, displays `Mode:` plus the active label, defaults to `Human versus CPU` without a saved preference, loads the last saved mode when available, cycles between exactly two labels, saves changes, and starts a fresh game on mode change.

### Tests for User Story 1

- [ ] T015 [P] [US1] Add failing unit tests for match mode label text, `Human versus CPU` default mode, saved mode load, saved mode write, and two-mode cycling in `bevy/crates/game/src/tests/runtime/resources/opponent_match_model_tests.rs`
- [ ] T016 [P] [US1] Add failing GameView control tests for Status text reservation above Mode, Mode button label, ordering above Restart, and hidden CPU Brain terminology in `bevy/crates/game/src/tests/runtime/systems/systems_tests.rs`

### Implementation for User Story 1

- [X] T017 [US1] Add `Mode` to `GameControlAction`, `GameControlButton`, and `GameControlLabel` handling in `bevy/crates/game/src/runtime/components/game_control_component.rs`
- [X] T018 [US1] Add mode preference persistence model and disk store helpers in `bevy/crates/game/src/runtime/resources/opponent_match_model.rs`
- [X] T019 [US1] Update GameView lower-left control spawning in `bevy/crates/game/src/runtime/systems/mod.rs` so Status appears above Mode, Mode appears above Restart, and Mode uses the Restart visual style family
- [X] T020 [US1] Update `update_game_control_ui_system` in `bevy/crates/game/src/runtime/systems/mod.rs` so Mode renders `Mode:` plus `Human versus CPU` or `CPU versus CPU` and never renders CPU Brain details
- [X] T021 [US1] Update startup loading in `bevy/crates/game/src/runtime/systems/mod.rs` and `bevy/crates/game/src/runtime/plugins/mod.rs` so saved mode loads at startup and missing preference defaults to `Human versus CPU`
- [X] T022 [US1] Update game control interaction handling in `bevy/crates/game/src/runtime/systems/mod.rs` so pressing Mode cycles mode, saves the selected mode to disk, and triggers a clean fresh game reset at turn `1/6`
- [ ] T023 [US1] Run targeted tests for mode model, mode persistence, and GameView controls with `scripts/other/RunTests.ps1`

**Checkpoint**: User Story 1 is functional and testable independently.

---

## Phase 4: User Story 2 - Play Human Versus CPU Rounds (Priority: P1)

**Goal**: In `Human versus CPU`, the near human and far CPU each have independent decks/hands/readiness, use their own slot side, and advance only after both mark Next.

**Independent Test**: Start a `Human versus CPU` game, play near cards into bottom slots, allow far cards in top slots, press Next before/after far readiness, and verify round advancement waits for both players.

### Tests for User Story 2

- [ ] T024 [P] [US2] Add failing model tests proving near/far players receive independent 12-card copies of the same master deck plus separate hands, energy, and readiness in `bevy/crates/game/src/tests/runtime/resources/opponent_match_model_tests.rs`
- [ ] T025 [P] [US2] Add failing system tests for Next readiness gating and Restart clearing both players in `bevy/crates/game/src/tests/runtime/systems/systems_tests.rs`
- [ ] T026 [P] [US2] Add failing slot placement tests proving near cards use bottom slots and far cards use top slots in `bevy/crates/game/src/tests/runtime/resources/card_slot_model_tests.rs`

### Implementation for User Story 2

- [X] T027 [US2] Extend `bevy/crates/game/src/runtime/resources/opponent_match_model.rs` with per-player active deck, hand, energy, ready state, and reset helpers for near and far players
- [X] T028 [US2] Update game model initialization in `bevy/crates/game/src/runtime/systems/mod.rs` so fresh games create two independent player deck/hand states from the same 12-card master deck source
- [X] T029 [US2] Update End Turn or Next handling in `bevy/crates/game/src/runtime/systems/mod.rs` so the human near player marks readiness instead of immediately advancing the round
- [X] T030 [US2] Add round readiness resolution in `bevy/crates/game/src/runtime/systems/mod.rs` so turns 1 through 5 advance only after both near and far readiness flags are set
- [X] T031 [US2] Update round advancement in `bevy/crates/game/src/runtime/systems/mod.rs` so both players receive the scheduled card deal and turn energy when a new turn starts
- [X] T032 [US2] Update Restart handling in `bevy/crates/game/src/runtime/systems/mod.rs` so it clears both players' decks, hands, placements, readiness, pending CPU state, turn state, and winner state
- [X] T033 [US2] Update hand/card synchronization in `bevy/crates/game/src/runtime/systems/mod.rs` to preserve existing near-human bottom hand behavior while leaving far-player hand off screen
- [ ] T034 [US2] Run targeted tests for two-player state, slot ownership, restart, and readiness with `scripts/other/RunTests.ps1`

**Checkpoint**: User Story 2 is functional and testable independently.

---

## Phase 5: User Story 5 - Resolve Winner After Round Six (Priority: P1)

**Goal**: After both players are ready on turn `6/6`, score all three locations left to right and declare exactly one winner.

**Independent Test**: Complete or simulate a six-turn game and verify each location compares top and bottom power totals, awards location ownership, declares near or far winner with no tie result, and updates Status text above Mode with the winning player number and controller type.

### Tests for User Story 5

- [ ] T035 [P] [US5] Add failing winner-resolution tests for left-to-right three-location evaluation and two-or-more location wins in `bevy/crates/game/src/tests/runtime/resources/point_model_tests.rs`
- [ ] T036 [P] [US5] Add failing point model tests for deterministic tied-location ownership before final match aggregation in `bevy/crates/game/src/tests/runtime/resources/point_model_tests.rs`
- [ ] T037 [P] [US5] Add failing readiness-to-winner system tests for turn `6/6` and final Status text above Mode in `bevy/crates/game/src/tests/runtime/systems/systems_tests.rs`

### Implementation for User Story 5

- [X] T038 [US5] Add winner state and completed-match transition helpers to `bevy/crates/game/src/runtime/resources/opponent_match_model.rs`
- [ ] T039 [US5] Update `bevy/crates/game/src/runtime/resources/point_model.rs` to expose a completed-match outcome helper that maps location ties through deterministic tiebreaking and never returns draw
- [X] T040 [US5] Add final winner resolution after both players are ready on turn `6/6` in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T041 [US5] Add or update GameView Status winner text in `bevy/crates/game/src/runtime/systems/mod.rs` so it appears above Mode, identifies the winning player number and controller type, and does not expose CPU Brain wording
- [ ] T042 [US5] Run targeted winner and turn-six tests with `scripts/other/RunTests.ps1`

**Checkpoint**: User Story 5 is functional and testable independently.

---

## Phase 6: User Story 3 - Show Believable CPU Opponent Actions (Priority: P2)

**Goal**: In `Human versus CPU`, the far CPU Brain Level 1 makes seeded, win-oriented legal affordable card moves with 0.5 to 1 second move/decision pacing and marks readiness when no move remains.

**Independent Test**: Start multiple `Human versus CPU` rounds with a known Brain seed and verify the far CPU uses top slots, produces deterministic same-seed move choices, favors win-oriented legal moves, each move or non-move readiness decision is delayed 0.5 to 1 second, and readiness is marked after no energy or legal affordable moves remain.

### Tests for User Story 3

- [ ] T043 [P] [US3] Add failing CPU Brain Level 1 tests for legal affordable move selection, win-oriented move preference, seeded deterministic move sequences, seeded random choice among acceptable moves, no-energy stop, no-legal-move stop, no-undo behavior, and readiness in `bevy/crates/game/src/tests/runtime/resources/cpu_brain_model_tests.rs`
- [ ] T044 [P] [US3] Add failing CPU Brain knowledge tests proving Brain can inspect own hand, open locations, and revealed slots but cannot inspect unrevealed deck order or opposing current-turn hidden placements in `bevy/crates/game/src/tests/runtime/resources/cpu_brain_model_tests.rs`
- [ ] T045 [P] [US3] Add failing system tests for 0.5 to 1 second CPU Brain move/readiness pacing state and no additional moves after readiness in `bevy/crates/game/src/tests/runtime/systems/systems_tests.rs`

### Implementation for User Story 3

- [X] T046 [US3] Create `bevy/crates/game/src/runtime/resources/cpu_brain_model.rs` with hidden `CpuBrainModel`, `CpuBrainLevel::Level1`, injectable random seed, 0.5 to 1 second move/readiness pacing state, win-oriented move scoring, and seeded legal move planning helpers using required `HUMAN:` and `AI:` comments
- [X] T047 [US3] Export `cpu_brain_model` from `bevy/crates/game/src/runtime/resources/mod.rs`
- [X] T048 [US3] Initialize CPU Brain resources in `bevy/crates/game/src/runtime/plugins/mod.rs`
- [X] T049 [US3] Add `cpu_brain_update_system` to `bevy/crates/game/src/runtime/systems/mod.rs` to schedule Level 1 far-player moves, apply one seeded win-oriented card move at a time, and mark readiness when exhausted
- [X] T050 [US3] Register `cpu_brain_update_system` in `bevy/crates/game/src/runtime/plugins/mod.rs` after round/card state is initialized and before readiness resolution
- [X] T051 [US3] Update CPU move application in `bevy/crates/game/src/runtime/systems/mod.rs` so CPU moves respect deck ownership, hand ownership, energy, legal slots, top-slot placement, and never dispatch Undo
- [ ] T052 [US3] Run targeted CPU Brain and pacing tests with `scripts/other/RunTests.ps1`

---

## Phase 6A: User Story 6 - Reveal Current-Turn Placements After Next (Priority: P1)

**Goal**: Current-turn placed cards are private/face down to the opposing player or controller, then reveal immediately when both players mark Next and stay face up forever.

**Independent Test**: Drag a near card and observe it remains front-facing to the owner but hidden from CPU Brain, observe far CPU current-turn cards as face down to the human, verify CPU-owned cards do not respond to mouse hover or rotate toward the cursor, end the turn, and verify all current-turn placements reveal permanently.

### Tests for User Story 6

- [ ] T053 [P] [US6] Add failing system tests for near owner-visible current-turn card fronts, far current-turn face-down cards, CPU-owned cards ignoring mouse hover/cursor rotation, and end-of-turn flip in `bevy/crates/game/src/tests/runtime/systems/systems_tests.rs`
- [ ] T054 [P] [US6] Add failing model tests that revealed prior-turn placements remain visible to both player controllers and CPU Brain in `bevy/crates/game/src/tests/runtime/resources/opponent_match_model_tests.rs`

### Implementation for User Story 6

- [X] T055 [US6] Update card placement logic in `bevy/crates/game/src/runtime/systems/mod.rs` so newly placed cards start as current-turn hidden placements for the opposing controller
- [X] T056 [US6] Update card view synchronization in `bevy/crates/game/src/runtime/systems/mod.rs` so near human-owned current-turn cards render as fronts to the human while far CPU current-turn cards render face down and all CPU-owned cards are excluded from mouse hover, drag affordance, and cursor-facing rotation behavior
- [X] T057 [US6] Update readiness resolution in `bevy/crates/game/src/runtime/systems/mod.rs` so all current-turn hidden placements reveal immediately after both players mark Next
- [ ] T058 [US6] Update CPU Brain knowledge construction in `bevy/crates/game/src/runtime/resources/cpu_brain_model.rs` so opposing current-turn hidden placements are excluded and prior-turn revealed placements are included
- [ ] T059 [US6] Run targeted placement visibility and reveal tests with `scripts/other/RunTests.ps1`

**Checkpoint**: User Story 6 is functional and testable independently.

**Checkpoint**: User Story 3 is functional and testable independently.

---

## Phase 7: User Story 4 - Simulate CPU Versus CPU (Priority: P2)

**Goal**: In `CPU versus CPU`, both near and far players are CPU Brain Level 1 controlled and the game plays automatically from turn `1/6` through final winner Status without human gameplay input.

**Independent Test**: Switch to `CPU versus CPU`, restart, provide no gameplay input, and verify both players make legal affordable moves to their own slot sides with 0.5 to 1 second decision pacing, readiness gates still apply, and the game reaches winner evaluation and Status text after round 6.

### Tests for User Story 4

- [X] T060 [P] [US4] Add failing model tests for `CPU versus CPU` controller mapping and hidden CPU Brain details in `bevy/crates/game/src/tests/runtime/resources/opponent_match_model_tests.rs`
- [X] T061 [P] [US4] Add failing system tests for both CPU players moving, waiting for both readiness flags, pacing decisions by 0.5 to 1 second, and reaching turn-six winner Status without human gameplay input in `bevy/crates/game/src/tests/runtime/systems/systems_tests.rs`

### Implementation for User Story 4

- [X] T062 [US4] Update controller mapping in `bevy/crates/game/src/runtime/resources/opponent_match_model.rs` so `CPU versus CPU` assigns CPU Brain Level 1 to both near and far players
- [X] T063 [US4] Generalize `cpu_brain_update_system` in `bevy/crates/game/src/runtime/systems/mod.rs` to process both near and far CPU-controlled players with paced autoplay through final winner Status without exposing CPU Brain UI labels
- [X] T064 [US4] Update CPU near-player move application in `bevy/crates/game/src/runtime/systems/mod.rs` so near CPU uses bottom slots while far CPU uses top slots
- [X] T065 [US4] Update human-only input guards in `bevy/crates/game/src/runtime/systems/mod.rs` so human drag/Next controls do not interfere with automated `CPU versus CPU` progress
- [X] T066 [US4] Run targeted CPU-versus-CPU controller and system tests with `scripts/other/RunTests.ps1`

**Checkpoint**: User Story 4 is functional and testable independently.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Verify full behavior, clean integration details, and document any platform gaps.

- [ ] T067 [P] Verify all changed Bevy runtime files follow `bevy/crates/template-crate` conventions, one-primary-concept structure, `Scene`/`Model`/`View` naming, `[domain]_[schedule]_system` system names, and required `HUMAN:`/`AI:` comments
- [ ] T068 [P] Verify `specs/014-opponent/quickstart.md` remains accurate after implementation and update only if behavior intentionally differs
- [ ] T069 Run full test suite with `scripts/other/RunTests.ps1`
- [X] T070 Run Windows desktop verification with `scripts/main/RunAppDesktop.ps1` and validate Status, Mode, Human versus CPU, CPU versus CPU autoplay, readiness, 0.5 to 1 second CPU pacing, passive CPU-owned card rendering, Restart, and winner result flows from `specs/014-opponent/quickstart.md`
- [ ] T071 Verify browser WebGPU workflow when practical, or document the exact blocker in `specs/014-opponent/quickstart.md`
- [X] T072 Inspect `git status --short` and ensure only scoped `014-opponent` spec/planning/runtime/test files are changed

---

## Dependencies & Execution Order

### Phase Dependencies

| Phase | Depends On | Blocks |
| ----- | ---------- | ------ |
| Phase 1 Setup | None | Phase 2 |
| Phase 2 Foundational | Phase 1 | All user stories |
| Phase 3 US1 | Phase 2 | Later visible mode integrations |
| Phase 4 US2 | Phase 2 | US3, US4, US5 runtime integration |
| Phase 5 US5 | Phase 2 and US2 readiness model | Final match completion |
| Phase 6 US3 | Phase 2 and US2 player/slot state | US4 dual CPU behavior |
| Phase 6A US6 | Phase 2 and US2 readiness/slot state | Hidden-information correctness |
| Phase 7 US4 | Phase 2, US1 mode selection, US3 CPU Brain | Full CPU-versus-CPU simulation |
| Phase 8 Polish | Desired completed user stories | Final validation |

### User Story Dependencies

| Story | Dependency | Rationale |
| ----- | ---------- | --------- |
| US1 Choose Match Mode | Foundation only | Can be implemented independently as visible mode cycling and reset. |
| US2 Human Versus CPU Rounds | Foundation only | Requires shared two-player state but not CPU pacing sophistication. |
| US5 Resolve Winner | US2 readiness flow | Winner evaluation triggers from turn-six readiness. |
| US3 CPU Opponent Actions | US2 player/slot flow | CPU Brain needs real player hands, energy, slots, and readiness. |
| US6 Reveal Current-Turn Placements | US2 readiness flow | Reveal happens when both players mark Next. |
| US4 CPU Versus CPU | US1 and US3 | Requires mode switching and CPU Brain for both players. |

### Within Each User Story

| Order | Rule |
| ----- | ---- |
| 1 | Tests first, and they should fail before implementation. |
| 2 | Models before systems. |
| 3 | System logic before plugin registration changes. |
| 4 | UI/control updates after model behavior exists. |
| 5 | Run targeted verification at each checkpoint. |

## Parallel Opportunities

| Area | Parallel Tasks |
| ---- | -------------- |
| Setup | T003 and T004 can run in parallel after T001/T002 context is understood. |
| Foundation tests | T005, T006, and T007 can run in parallel because they target different test files. |
| US1 tests | T015 and T016 can run in parallel. |
| US2 tests | T024, T025, and T026 can run in parallel. |
| US5 tests | T035 and T037 can run in parallel. |
| US3 tests | T043 and T045 can run in parallel. |
| US6 tests | T053 and T054 can run in parallel. |
| US4 tests | T060 and T061 can run in parallel. |
| Polish | T067 and T068 can run in parallel before full runtime verification. |

## Parallel Example: User Story 2

```text
Task: "T024 [P] [US2] Add failing model tests proving near/far players receive independent 12-card copies of the same master deck plus separate hands, energy, and readiness in bevy/crates/game/src/tests/runtime/resources/opponent_match_model_tests.rs"
Task: "T025 [P] [US2] Add failing system tests for Next readiness gating and Restart clearing both players in bevy/crates/game/src/tests/runtime/systems/systems_tests.rs"
Task: "T026 [P] [US2] Add failing slot placement tests proving near cards use bottom slots and far cards use top slots in bevy/crates/game/src/tests/runtime/resources/card_slot_model_tests.rs"
```

## Implementation Strategy

### MVP First

| Step | Scope |
| ---- | ----- |
| 1 | Complete Phase 1 and Phase 2. |
| 2 | Complete US1 so mode selection exists. |
| 3 | Complete US2 so Human versus CPU has real two-player readiness and slot ownership. |
| 4 | Complete US5 so the six-turn game has a final result. |
| 5 | Validate the P1 slice independently before adding CPU pacing and CPU-versus-CPU automation. |

### Incremental Delivery

| Increment | Delivers |
| --------- | -------- |
| US1 | Visible mode button, default mode, saved mode load, saved mode write, and two-mode cycle. |
| US2 | Two-player Human versus CPU round flow. |
| US5 | Final winner resolution after turn six. |
| US6 | Current-turn hidden placements and end-of-turn reveal. |
| US3 | Believable seeded CPU Brain Level 1 opponent actions with win-oriented legal move choice. |
| US4 | Fully automated CPU versus CPU mode. |

### Parallel Team Strategy

| Developer | Suggested Work |
| --------- | -------------- |
| A | US1 mode UI and reset flow. |
| B | US2 two-player match/readiness models and systems. |
| C | US5 scoring/winner model after US2 readiness is available. |
| D | US3 CPU Brain tests and model after shared player state stabilizes. |

## Notes

| Note | Detail |
| ---- | ------ |
| Scope | Do not expose CPU Brain or CpuBrainLevel in user-facing labels. |
| Runtime AI | CPU Brain is authored Rust gameplay logic and must not call runtime generative AI. |
| Human timing | No human turn timer is introduced. |
| CPU undo | CPU players never use Undo; Undo is a human reconsideration action only. |
| Hidden information | Current-turn placed cards reveal at end of turn and stay face up permanently. |
| Assets | No new theme assets are required for this feature. |
| Git | Keep branch `014-opponent`; avoid unrelated refactors and destructive Git operations. |
