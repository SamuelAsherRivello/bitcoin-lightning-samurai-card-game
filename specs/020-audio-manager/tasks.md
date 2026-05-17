# Tasks: Audio Manager

**Input**: Design documents from `specs/020-audio-manager/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, quickstart.md
**Tests**: Required by FR-023 and quickstart verification scenarios.
**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Confirm project structure, existing assets, and task prerequisites.

- [x] T001 Inspect `bevy/crates/template-crate` and `.codex/rules/bevy-runtime-structure.md` for runtime file, naming, and comment conventions.
- [x] T002 Confirm required SFX assets exist at `bevy/crates/game/assets/audio/sfx/Click01.wav`, `Slide01.wav`, `Tamborine01.wav`, and `Upgrade01.wav`.
- [x] T003 Verify existing settings source fields in `bevy/crates/game/src/runtime/resources/meta_game_settings_model.rs`.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core audio and button infrastructure that blocks all user stories.

**CRITICAL**: No user story work can begin until this phase is complete.

- [x] T004 [P] Add audio manager model, channel enum, audio enum, mapping helpers, request queue, and unit tests in `bevy/crates/game/src/runtime/resources/audio_manager_model.rs`.
- [x] T005 Export the audio manager model from `bevy/crates/game/src/runtime/resources/mod.rs`.
- [x] T006 [P] Add shared default button UI bundle and style tests in `bevy/crates/game/src/runtime/bundles/button_ui_bundle.rs`.
- [x] T007 Export the shared button UI bundle from `bevy/crates/game/src/runtime/bundles/mod.rs`.
- [x] T008 Add audio playback/update system registration and audio resource initialization in `bevy/crates/game/src/runtime/plugins/mod.rs`.
- [x] T009 Add audio update system module exports in `bevy/crates/game/src/runtime/systems/mod.rs`.

**Checkpoint**: Audio model and shared button bundle are available to all stories.

---

## Phase 3: User Story 1 - Respect Audio Settings (Priority: P1) MVP

**Goal**: SFX and Music channel behavior follows the existing settings values.

**Independent Test**: Toggle SFX and Music settings, request SFX/music, and verify observable audio requests follow channel settings without changing gameplay state.

### Tests for User Story 1

- [x] T010 [P] [US1] Add tests for SFX and Music channel muting behavior in `bevy/crates/game/src/tests/runtime/resources/resources_tests.rs`.
- [x] T011 [P] [US1] Add tests for enum-to-file mapping coverage in `bevy/crates/game/src/tests/runtime/resources/resources_tests.rs`.

### Implementation for User Story 1

- [x] T012 [US1] Implement channel gating and request draining in `bevy/crates/game/src/runtime/resources/audio_manager_model.rs`.
- [x] T013 [US1] Implement Bevy audio playback for mapped requests in `bevy/crates/game/src/runtime/systems/audio_update_system.rs`.
- [x] T014 [US1] Wire the audio update system to existing meta-game settings in `bevy/crates/game/src/runtime/plugins/mod.rs`.

**Checkpoint**: User Story 1 is fully functional and testable independently.

---

## Phase 4: User Story 2 - Hear Consistent Button Feedback (Priority: P1)

**Goal**: All current game buttons use the shared default button style and accepted clicks request `ButtonClick`.

**Independent Test**: Open reachable screens, click visible buttons, and verify they use the shared default style and request `ButtonClick` when SFX is enabled.

### Tests for User Story 2

- [x] T015 [P] [US2] Add shared button bundle tests in `bevy/crates/game/src/tests/runtime/systems/systems_tests.rs`.
- [x] T016 [P] [US2] Add accepted button click audio request tests in `bevy/crates/game/src/tests/runtime/systems/systems_tests.rs`.

### Implementation for User Story 2

- [x] T017 [US2] Migrate top navigation button spawning to `button_ui_bundle` in `bevy/crates/game/src/runtime/systems/mod.rs`.
- [x] T018 [US2] Migrate main, lightning, matchmaking, settings, deck, debug, and modal button spawning to `button_ui_bundle` in `bevy/crates/game/src/runtime/systems/mod.rs`.
- [x] T019 [US2] Emit `AudioEnum.ButtonClick` only for accepted shared button actions in `bevy/crates/game/src/runtime/systems/mod.rs`.

**Checkpoint**: User Story 2 is fully functional and testable independently.

---

## Phase 5: User Story 3 - Hear Card Movement Feedback (Priority: P2)

**Goal**: Accepted deck-to-hand and hand-to-location card movement from near or far side requests `CardSlide`.

**Independent Test**: Simulate near/far deck-to-hand and near/far hand-to-location movement, and verify exactly one slide request per accepted movement.

### Tests for User Story 3

- [x] T020 [P] [US3] Add near/far deck-to-hand slide audio tests in `bevy/crates/game/src/tests/runtime/systems/systems_tests.rs`.
- [x] T021 [P] [US3] Add near/far hand-to-location slide audio tests in `bevy/crates/game/src/tests/runtime/systems/systems_tests.rs`.

### Implementation for User Story 3

- [x] T022 [US3] Emit `AudioEnum.CardSlide` for accepted deck-to-hand movement in `bevy/crates/game/src/runtime/systems/card_gesture_update_system.rs`.
- [x] T023 [US3] Emit `AudioEnum.CardSlide` for accepted hand-to-location movement in `bevy/crates/game/src/runtime/systems/card_gesture_update_system.rs`.

**Checkpoint**: User Story 3 is fully functional and testable independently.

---

## Phase 6: User Story 4 - Hear Location State Feedback (Priority: P2)

**Goal**: Location open transitions and new non-tied winning-side changes request the correct SFX exactly once.

**Independent Test**: Change a location from closed to open and step scores through `0,0 -> 0,3 -> 0,5 -> 6,5`; verify one open request and exactly two lead-change requests.

### Tests for User Story 4

- [x] T024 [P] [US4] Add closed-to-open location audio tests in `bevy/crates/game/src/tests/runtime/systems/systems_tests.rs`.
- [x] T025 [P] [US4] Add winning-side transition sequence tests in `bevy/crates/game/src/tests/runtime/resources/resources_tests.rs`.

### Implementation for User Story 4

- [x] T026 [US4] Add previous/current location winning-side tracking helpers in `bevy/crates/game/src/runtime/resources/audio_manager_model.rs`.
- [x] T027 [US4] Emit `AudioEnum.LocationOpen` for closed-to-open transitions in `bevy/crates/game/src/runtime/systems/visual_modifier_update_system.rs`.
- [x] T028 [US4] Emit `AudioEnum.LocationLeadChange` for new non-tied winning sides in `bevy/crates/game/src/runtime/systems/visual_modifier_update_system.rs`.

**Checkpoint**: User Story 4 is fully functional and testable independently.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Validate the complete feature and keep documentation aligned.

- [x] T029 Run `scripts/other/RunTests.ps1` from the repository root.
- [x] T030 Validate quickstart scenarios in `specs/020-audio-manager/quickstart.md`.
- [x] T031 Verify changed runtime files use `bevy/crates/template-crate` as the proper reference and follow one-primary-concept, Scene/Model/View naming, system naming, and `HUMAN:`/`AI:` comment standards.
- [x] T032 Update `specs/020-audio-manager/quickstart.md` if implementation verification uncovers a target-specific browser or desktop blocker.

---

## Dependencies & Execution Order

### Phase Dependencies

| Phase | Depends On | Blocks |
| ----- | ---------- | ------ |
| Phase 1 Setup | None | Phase 2 |
| Phase 2 Foundational | Phase 1 | All user stories |
| Phase 3 US1 | Phase 2 | Complete MVP audio gating |
| Phase 4 US2 | Phase 2, US1 audio request path | Button feedback |
| Phase 5 US3 | Phase 2, US1 audio request path | Card movement feedback |
| Phase 6 US4 | Phase 2, US1 audio request path | Location feedback |
| Phase 7 Polish | Desired user story phases | Final validation |

### User Story Dependencies

| Story | Dependency |
| ----- | ---------- |
| US1 | Starts after foundational audio model/system exists. |
| US2 | Starts after foundational button bundle and audio request path exist. |
| US3 | Starts after foundational audio request path exists. |
| US4 | Starts after foundational audio request path exists. |

### Parallel Opportunities

| Scope | Parallel Tasks |
| ----- | -------------- |
| Foundational | T004 and T006 touch different files after T001-T003. |
| US1 tests | T010 and T011 can be authored together in the same test file only with merge coordination. |
| US2 tests | T015 and T016 can be authored together in the same test file only with merge coordination. |
| US3 tests | T020 and T021 can be authored together in the same test file only with merge coordination. |
| US4 tests | T024 and T025 touch different conceptual checks but require merge coordination if placed in shared test files. |

---

## Implementation Strategy

### MVP First

1. Complete Phase 1 setup.
2. Complete Phase 2 foundational audio model, bundle, exports, and plugin wiring.
3. Complete Phase 3 US1 so all later stories share settings-driven channel behavior.
4. Validate US1 tests before adding button, card, and location triggers.

### Incremental Delivery

1. Add audio manager and channel gating.
2. Migrate buttons and add click feedback.
3. Add card movement feedback.
4. Add location open and lead-change feedback.
5. Run full quickstart and repository tests.

### Notes

- Tests should be written before implementation where practical and should fail before the corresponding implementation task.
- Mark each task as complete only after the file changes and local verification for that task are done.
- Avoid unrelated refactors while migrating button construction.
