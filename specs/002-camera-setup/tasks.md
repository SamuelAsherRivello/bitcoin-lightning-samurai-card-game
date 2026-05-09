# Tasks: Camera Setup

**Input**: Design artifacts from `specs/002-camera-setup/`

## Phase 1: Setup

- [X] T001 Verify `.gitignore` continues to ignore generated runtime outputs in `.gitignore`

## Phase 2: Foundational

- [X] T002 Add `PrimarySceneCamera` marker component in `Bevy/Crates/Game/src/runtime/components/mod.rs`
- [X] T003 Add `PrimaryCameraDefaults` resource in `Bevy/Crates/Game/src/runtime/resources/mod.rs`

## Phase 3: User Story 1 - Show a 3D Scene View (P1)

**Independent Test**: Launch startup systems and verify exactly one active primary 3D camera exists.

- [X] T004 [US1] Add a startup test for exactly one primary camera in `Bevy/Crates/Game/src/runtime/plugins/mod.rs`
- [X] T005 [US1] Implement `setup_primary_camera` in `Bevy/Crates/Game/src/runtime/systems/mod.rs`
- [X] T006 [US1] Wire camera defaults and startup system in `Bevy/Crates/Game/src/runtime/plugins/mod.rs`

## Phase 4: User Story 2 - Keep Camera Parameters Stable (P1)

**Independent Test**: Inspect startup camera transform/projection and verify input does not mutate it.

- [X] T007 [US2] Add tests for camera transform, projection, clear color, and input stability in `Bevy/Crates/Game/src/runtime/plugins/mod.rs`
- [X] T008 [US2] Apply documented position, target, FOV, near/far, and clear color defaults in `Bevy/Crates/Game/src/runtime/resources/mod.rs` and `Bevy/Crates/Game/src/runtime/systems/mod.rs`

## Phase 5: User Story 3 - Preserve Overlay Compatibility (P2)

**Independent Test**: Run combined startup and verify DebugHUD and inspector entities still exist with the 3D camera.

- [X] T009 [US3] Extend combined startup coverage for camera, DebugHUD, and inspector coexistence in `Bevy/Crates/Game/src/runtime/plugins/mod.rs`

## Final Phase: Polish & Cross-Cutting

- [X] T010 Run `scripts/other/RunTests.ps1` and document any blocked target verification in `specs/002-camera-setup/quickstart.md`
- [X] T011 Ensure `AGENTS.md` points to `specs/002-camera-setup/plan.md`

## Dependencies

| Story | Depends On | Notes |
| ---- | ---- | ---- |
| US1 | T001-T003 | MVP camera creation |
| US2 | US1 | Validates and locks down camera defaults |
| US3 | US1 | Confirms overlay coexistence after camera creation |

## Parallel Execution Examples

| Scope | Parallel Tasks |
| ---- | ---- |
| Foundation | T002 and T003 can be prepared independently, then wired together |
| Documentation and verification | T010 documentation updates can follow test execution while code remains stable |

## Implementation Strategy

| Increment | Tasks | Result |
| ---- | ---- | ---- |
| MVP | T001-T006 | One primary 3D camera exists at startup |
| Defaults | T007-T008 | Camera parameters and fixed input behavior are verified |
| Compatibility | T009-T011 | Overlay coexistence, tests, and active plan reference are complete |
