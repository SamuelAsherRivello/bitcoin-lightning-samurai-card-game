# Tasks: Debugging

**Input**: Design documents from `specs/003-debugging/`
**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/debughud-ui.md](./contracts/debughud-ui.md), [quickstart.md](./quickstart.md)

**Tests**: Required by FR-009, FR-010, FR-016, SC-006, and story independent tests. Write test tasks before implementation tasks in each story phase.

**Organization**: Tasks are grouped by user story so each story can be implemented and tested independently after the shared foundation is in place.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel because it touches different files and does not depend on incomplete tasks.
- **[Story]**: User-story label for story phases only.
- Every task includes an exact file path.

## Phase 1: Setup

**Purpose**: Prepare module structure, scripts, and build ownership for the debugging runtime.

- [X] T001 Create shared runtime module directories `bevy/crates/shared/src/runtime/components`, `bevy/crates/shared/src/runtime/plugins`, `bevy/crates/shared/src/runtime/resources`, and `bevy/crates/shared/src/runtime/systems`
- [X] T002 Create game runtime debug module paths `bevy/crates/game/src/runtime/components/card_ui_component.rs`, `bevy/crates/game/src/runtime/components/debug_drawing_component.rs`, `bevy/crates/game/src/runtime/resources/debug_drawing_model.rs`, and `bevy/crates/game/src/runtime/systems/debug_drawing_update_system.rs`
- [X] T003 Wire shared runtime module exports in `bevy/crates/shared/src/lib.rs` and `bevy/crates/shared/src/runtime/mod.rs`
- [X] T004 Wire game runtime component/resource/system module exports in `bevy/crates/game/src/runtime/components/mod.rs`, `bevy/crates/game/src/runtime/resources/mod.rs`, and `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T005 Add or verify the `bevy/crates/game` dependency on the shared crate in `bevy/crates/game/Cargo.toml`
- [X] T006 Keep the single repository test runner at `scripts/other/RunTests.ps1`

---

## Phase 2: Foundational

**Purpose**: Create reusable diagnostic primitives that block all user stories.

**Critical**: No user-story implementation should begin until the shared plugin, input model, layout access, and test harness are available.

- [X] T007 Create `DebugToolingPlugin` skeleton with `HUMAN:` and `AI:` purpose comments in `bevy/crates/shared/src/runtime/plugins/debug_tooling_plugin.rs`
- [X] T008 [P] Create `DebugHudState`, FPS sample fields, and approved debug key state model in `bevy/crates/shared/src/runtime/resources/debug_hud_model.rs`
- [X] T009 [P] Create `DebugInputModel` and toggle/hold key classification in `bevy/crates/shared/src/runtime/resources/debug_input_model.rs`
- [X] T010 [P] Create aspect-ratio-safe debug layout helper types in `bevy/crates/shared/src/runtime/resources/debug_layout_model.rs`
- [X] T011 Create module exports for shared components, plugins, resources, and systems in `bevy/crates/shared/src/runtime/components/mod.rs`, `bevy/crates/shared/src/runtime/plugins/mod.rs`, `bevy/crates/shared/src/runtime/resources/mod.rs`, and `bevy/crates/shared/src/runtime/systems/mod.rs`
- [X] T012 Add shared runtime plugin composition to the game startup path in `bevy/crates/game/src/main.rs`
- [X] T013 Add foundational shared runtime tests for plugin startup and key classification in `bevy/crates/shared/src/runtime/plugins/debug_tooling_plugin.rs`

**Checkpoint**: Shared debugging runtime can compile and be composed by the game crate.

---

## Phase 3: User Story 1 - Show DebugHUD Panel (Priority: P1) MVP

**Goal**: Show one translucent top-left DebugHUD panel by default with title/status text and labels for `W`, `A`, `S`, `D`, `F`, and `I`.

**Independent Test**: Launch the prototype and verify that one DebugHUD panel appears inside the aspect-ratio-safe HUD area with the expected text and key labels.

### Tests for User Story 1

- [ ] T014 [P] [US1] Add a startup test for exactly one DebugHUD panel in `bevy/crates/shared/src/runtime/systems/debug_hud_setup_system.rs`
- [ ] T015 [P] [US1] Add a content test for title/status text and `W`, `A`, `S`, `D`, `F`, `I` labels in `bevy/crates/shared/src/runtime/systems/debug_hud_update_system.rs`
- [ ] T016 [P] [US1] Add an aspect-ratio-safe placement test for DebugHUD under the HUD root in `bevy/crates/shared/src/runtime/systems/debug_hud_setup_system.rs`

### Implementation for User Story 1

- [ ] T017 [US1] Implement DebugHUD marker components and key-label components in `bevy/crates/shared/src/runtime/components/debug_hud_component.rs`
- [ ] T018 [US1] Implement DebugHUD setup under the aspect-ratio-safe HUD root in `bevy/crates/shared/src/runtime/systems/debug_hud_setup_system.rs`
- [ ] T019 [US1] Implement DebugHUD title/status text update from game tick state in `bevy/crates/shared/src/runtime/systems/debug_hud_update_system.rs`
- [ ] T020 [US1] Register DebugHUD setup and update systems in `bevy/crates/shared/src/runtime/plugins/debug_tooling_plugin.rs`
- [ ] T021 [US1] Compose `DebugToolingPlugin` into the game app in `bevy/crates/game/src/main.rs`
- [ ] T022 [US1] Document the manual DebugHUD launch check in `specs/003-debugging/quickstart.md`

**Checkpoint**: US1 is independently functional and verifies SC-001, SC-002, FR-001, FR-002, FR-003, FR-007, FR-008, FR-021.

---

## Phase 4: User Story 2 - Toggle FPS With F (Priority: P1)

**Goal**: Pressing `F` toggles FPS text visibility in the DebugHUD without changing inspector or card behavior.

**Independent Test**: Press `F` repeatedly while the prototype is running and verify that only FPS visibility changes.

### Tests for User Story 2

- [ ] T023 [P] [US2] Add a failing test for `F` toggling FPS visibility in `bevy/crates/shared/src/runtime/systems/debug_input_update_system.rs`
- [ ] T024 [P] [US2] Add a regression test proving `F` does not toggle inspector visibility in `bevy/crates/shared/src/runtime/systems/debug_input_update_system.rs`
- [ ] T025 [P] [US2] Add a DebugHUD FPS text rendering test in `bevy/crates/shared/src/runtime/systems/debug_hud_update_system.rs`

### Implementation for User Story 2

- [ ] T026 [US2] Implement approved `F` toggle handling in `bevy/crates/shared/src/runtime/systems/debug_input_update_system.rs`
- [ ] T027 [US2] Implement FPS sample accumulation and display value update in `bevy/crates/shared/src/runtime/resources/debug_hud_model.rs`
- [ ] T028 [US2] Render visible or empty FPS text based on `DebugHudState.is_fps_visible` in `bevy/crates/shared/src/runtime/systems/debug_hud_update_system.rs`
- [ ] T029 [US2] Register debug input and FPS update systems in `bevy/crates/shared/src/runtime/plugins/debug_tooling_plugin.rs`

**Checkpoint**: US2 is independently functional and verifies SC-003, FR-004, FR-006A.

---

## Phase 5: User Story 3 - Toggle Inspector With I (Priority: P1)

**Goal**: Pressing `I` toggles inspector visibility without affecting FPS or card behavior.

**Independent Test**: Press `I` repeatedly while the prototype is running and verify that only inspector visibility changes.

### Tests for User Story 3

- [ ] T030 [P] [US3] Add a failing test for `I` toggling inspector visibility in `bevy/crates/shared/src/runtime/systems/inspector_update_system.rs`
- [ ] T031 [P] [US3] Add a regression test proving `I` does not toggle FPS visibility in `bevy/crates/shared/src/runtime/systems/inspector_update_system.rs`

### Implementation for User Story 3

- [ ] T032 [US3] Implement inspector marker and state components in `bevy/crates/shared/src/runtime/components/inspector_component.rs`
- [ ] T033 [US3] Implement `I` toggle and hidden-inspector behavior in `bevy/crates/shared/src/runtime/systems/inspector_update_system.rs`
- [ ] T034 [US3] Register inspector update behavior in `bevy/crates/shared/src/runtime/plugins/debug_tooling_plugin.rs`

**Checkpoint**: US3 is independently functional and verifies SC-004, FR-005.

---

## Phase 6: User Story 4 - Keep WASD Non-Functional (Priority: P2)

**Goal**: `W`, `A`, `S`, and `D` appear in the DebugHUD key legend and may show hold feedback without changing gameplay, camera, card, FPS, or inspector state.

**Independent Test**: Press `W`, `A`, `S`, and `D` while the prototype is running and verify that only DebugHUD hold feedback may change.

### Tests for User Story 4

- [ ] T035 [P] [US4] Add hold-indicator tests for `W`, `A`, `S`, and `D` in `bevy/crates/shared/src/runtime/systems/debug_input_update_system.rs`
- [ ] T036 [P] [US4] Add regression tests proving `WASD` does not toggle FPS or inspector state in `bevy/crates/shared/src/runtime/systems/debug_input_update_system.rs`
- [ ] T037 [P] [US4] Add game regression tests proving `WASD` does not move camera or card state in `bevy/crates/game/src/runtime/systems/debug_input_integration_tests.rs`

### Implementation for User Story 4

- [ ] T038 [US4] Implement non-toggle hold indicator updates for `W`, `A`, `S`, and `D` in `bevy/crates/shared/src/runtime/systems/debug_input_update_system.rs`
- [ ] T039 [US4] Implement key-label visual feedback binding in `bevy/crates/shared/src/runtime/systems/debug_hud_update_system.rs`
- [ ] T040 [US4] Verify no game systems consume `WASD` by keeping card and camera input unchanged in `bevy/crates/game/src/runtime/systems/mod.rs`

**Checkpoint**: US4 is independently functional and verifies SC-005, SC-005A, FR-006, FR-006A, FR-011.

---

## Phase 7: User Story 5 - Use Developer QA Tooling (Priority: P2)

**Goal**: Implementers can use scoped terminal logs, repeatable tests, and documented checks to QA their own work without secrets or noisy unrelated output.

**Independent Test**: Run `scripts/other/RunTests.ps1` and verify terminal/build output and manual notes cover DebugHUD, inspector, Card UI separation, and diagnostic input behavior.

### Tests for User Story 5

- [ ] T041 [P] [US5] Add a test that diagnostics can emit scoped safe log messages without secrets in `bevy/crates/shared/src/runtime/systems/debug_log_update_system.rs`
- [ ] T042 [P] [US5] Add a script smoke test for `scripts/other/RunTests.ps1`

### Implementation for User Story 5

- [ ] T043 [US5] Implement scoped diagnostic logging hooks in `bevy/crates/shared/src/runtime/systems/debug_log_update_system.rs`
- [ ] T044 [US5] Register diagnostic logging hooks without noisy per-frame output in `bevy/crates/shared/src/runtime/plugins/debug_tooling_plugin.rs`
- [ ] T045 [US5] Update self-QA instructions for tests, logs, blocked checks, and safe output in `specs/003-debugging/quickstart.md`

**Checkpoint**: US5 is independently functional and verifies SC-006, SC-009, SC-010, FR-010, FR-015, FR-016.

---

## Phase 8: User Story 6 - Mark Scene Areas With Debug Drawing (Priority: P2)

**Goal**: Add temporary runtime visual annotations around requested game scene areas, such as the hand area, that remain until removal or replacement is requested.

**Independent Test**: Request a debug drawing around a known scene area, launch the app, and verify the mark is visible, aspect-ratio-safe, temporary, and removable.

### Tests for User Story 6

- [X] T046 [P] [US6] Add debug drawing request/state tests in `bevy/crates/game/src/runtime/resources/debug_drawing_model.rs`
- [X] T047 [P] [US6] Add aspect-ratio-safe debug drawing placement tests in `bevy/crates/game/src/runtime/systems/debug_drawing_update_system.rs`
- [X] T048 [P] [US6] Add debug drawing removal/replacement tests in `bevy/crates/game/src/runtime/systems/debug_drawing_update_system.rs`

### Implementation for User Story 6

- [X] T049 [US6] Implement debug drawing marker components in `bevy/crates/game/src/runtime/components/debug_drawing_component.rs`
- [X] T050 [US6] Implement debug drawing target metadata and stale/replacement state in `bevy/crates/game/src/runtime/resources/debug_drawing_model.rs`
- [X] T051 [US6] Implement aspect-ratio-safe debug drawing placement and visibility in `bevy/crates/game/src/runtime/systems/debug_drawing_update_system.rs`
- [X] T052 [US6] Register debug drawing systems in `bevy/crates/game/src/runtime/plugins/mod.rs`
- [X] T053 [US6] Document debug drawing request, persistence, removal, and replacement workflow in `specs/003-debugging/quickstart.md`

**Checkpoint**: US6 is independently functional and verifies SC-011, SC-012, SC-013, FR-017, FR-018, FR-019, FR-020, FR-021, FR-022.

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Final verification, browser parity, documentation consistency, and implementation standards.

- [ ] T054 [P] Add Card UI separation documentation to `specs/003-debugging/contracts/debughud-ui.md`
- [ ] T055 [P] Add browser WebGPU verification notes and blockers, if any, to `specs/003-debugging/quickstart.md`
- [ ] T056 Run `scripts/other/RunTests.ps1` and record result in `specs/003-debugging/quickstart.md`
- [ ] T057 Run `scripts/main/RunAppDesktop.ps1 -CheckOnly` and record result in `specs/003-debugging/quickstart.md`
- [ ] T058 Run `scripts/other/RunAppWeb.ps1 -CheckOnly` and record result or exact blocker in `specs/003-debugging/quickstart.md`
- [ ] T059 Verify changed Bevy runtime files use `bevy/crates/template-crate` as the proper reference and follow one-primary-concept layout, `[domain]_[schedule]_system` naming, and `HUMAN:`/`AI:` comment rules in `bevy/crates/shared/src/runtime/` and `bevy/crates/game/src/runtime/`
- [ ] T060 Run a final requirements trace from FR-001 through FR-022 and SC-001 through SC-013 in `specs/003-debugging/tasks.md`

---

## Dependencies & Execution Order

### Phase Dependencies

| Phase | Depends On | Blocks |
| ----- | ---------- | ------ |
| Phase 1 Setup | None | Phase 2 |
| Phase 2 Foundational | Phase 1 | All user stories |
| Phase 3 US1 | Phase 2 | US2, US4 visual label integration |
| Phase 4 US2 | Phase 2, US1 display surface | Polish |
| Phase 5 US3 | Phase 2 | Polish |
| Phase 6 US4 | Phase 2, US1 labels | Polish |
| Phase 7 US5 | Phase 2 | Polish |
| Phase 8 US6 | Phase 2 | Polish |
| Phase 9 Polish | Desired user stories complete | Final acceptance |

### User Story Dependencies

| User Story | Dependency | Reason |
| ---------- | ---------- | ------ |
| US1 | Foundational only | MVP DebugHUD panel establishes visible diagnostic surface |
| US2 | US1 preferred | FPS text appears inside DebugHUD panel |
| US3 | Foundational only | Inspector state can be tested independently of FPS |
| US4 | US1 preferred | Hold feedback appears through DebugHUD key labels |
| US5 | Foundational only | Logging and tests can be added independently |
| US6 | Foundational only | Debug drawing is game-scene-specific and separate from DebugHUD |

### Parallel Opportunities

| Scope | Parallel Tasks |
| ----- | -------------- |
| Setup | T002 can run in parallel with T003 after T001 |
| Foundational | T008, T009, T010 can run in parallel |
| US1 tests | T014, T015, T016 can run in parallel |
| US2 tests | T023, T024, T025 can run in parallel |
| US3 tests | T030, T031 can run in parallel |
| US4 tests | T035, T036, T037 can run in parallel |
| US5 tests | T041, T042 can run in parallel |
| US6 tests | T046, T047, T048 can run in parallel |
| Polish | T054 and T055 can run in parallel |

---

## Parallel Examples

### User Story 1

```text
Task: "T014 [P] [US1] Add a startup test for exactly one DebugHUD panel in bevy/crates/shared/src/runtime/systems/debug_hud_setup_system.rs"
Task: "T015 [P] [US1] Add a content test for title/status text and W/A/S/D/F/I labels in bevy/crates/shared/src/runtime/systems/debug_hud_update_system.rs"
Task: "T016 [P] [US1] Add an aspect-ratio-safe placement test for DebugHUD under the HUD root in bevy/crates/shared/src/runtime/systems/debug_hud_setup_system.rs"
```

### User Story 6

```text
Task: "T046 [P] [US6] Add debug drawing request/state tests in bevy/crates/game/src/runtime/resources/debug_drawing_model.rs"
Task: "T047 [P] [US6] Add aspect-ratio-safe debug drawing placement tests in bevy/crates/game/src/runtime/systems/debug_drawing_update_system.rs"
Task: "T048 [P] [US6] Add debug drawing removal/replacement tests in bevy/crates/game/src/runtime/systems/debug_drawing_update_system.rs"
```

---

## Implementation Strategy

### MVP First

| Step | Work |
| ---- | ---- |
| 1 | Complete Phase 1 setup |
| 2 | Complete Phase 2 foundational diagnostics |
| 3 | Complete Phase 3 US1 DebugHUD panel |
| 4 | Run `scripts/other/RunTests.ps1` and launch desktop app to validate the panel |

### Incremental Delivery

| Increment | Adds | Validation |
| --------- | ---- | ---------- |
| MVP | US1 DebugHUD panel | One visible safe-area DebugHUD with expected labels |
| Increment 2 | US2 FPS toggle | `F` toggles FPS only |
| Increment 3 | US3 inspector toggle | `I` toggles inspector only |
| Increment 4 | US4 WASD hold indicators | `WASD` affects DebugHUD feedback only |
| Increment 5 | US5 self-QA tooling | Repeatable tests and safe logs |
| Increment 6 | US6 debug drawing | Temporary scene marks around requested areas |

### Final Acceptance

| Check | Command or Review |
| ----- | ----------------- |
| Automated tests | `scripts/other/RunTests.ps1` |
| Desktop compile | `scripts/main/RunAppDesktop.ps1 -CheckOnly` |
| Browser compile | `scripts/other/RunAppWeb.ps1 -CheckOnly` |
| Manual runtime | `scripts/main/RunAppDesktop.ps1` |
| Standards review | One-primary-concept files, `[domain]_[schedule]_system`, `HUMAN:`/`AI:` comments, aspect-ratio-safe layout |
