# Tasks: Card Polish

**Input**: Design documents from `/specs/005-card-polish/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md

## Phase 1: Setup

- [X] T001 Create Speckit plan, research, data model, quickstart, and task artifacts in specs/005-card-polish/
- [X] T002 Update active Speckit plan reference in AGENTS.md
- [X] T003 Generate and place SKYBOLT texture assets in bevy/crates/game/assets/cards/card_types/card_type_skybolt/
- [X] T003a Generate and place TAR texture assets in bevy/crates/game/assets/cards/card_types/card_type_tar/

## Phase 2: Foundational

- [X] T004 Add CardStructure and CardType ECS/resource types in bevy/crates/game/src/runtime/components/mod.rs and bevy/crates/game/src/runtime/resources/mod.rs
- [X] T005 Register CardType resources and systems in bevy/crates/game/src/runtime/plugins/mod.rs

## Phase 3: User Story 1 - Perceive Layered Card Depth

**Goal**: Show background, frame, foreground, and title as four flat apparent-depth layers.

**Independent Test**: Launch the prototype and verify the four layers remain visually contained in the card front face while tilting.

- [X] T006 [US1] Replace manual dot and rectangle construction with SKYBOLT textured planes in bevy/crates/game/src/runtime/systems/mod.rs
- [X] T007 [US1] Add four-layer depth and aperture unit tests in bevy/crates/game/src/runtime/systems/mod.rs

## Phase 4: User Story 2 - Read the Frame as Holographic

**Goal**: Frame shine changes with smoothed card tilt and remains bound to the frame.

**Independent Test**: Move pointer left/right and top/bottom and verify only frame planes receive shine changes.

- [X] T008 [US2] Add frame layer marker and tilt-reactive frame shine system in bevy/crates/game/src/runtime/components/mod.rs and bevy/crates/game/src/runtime/systems/mod.rs
- [X] T009 [US2] Schedule frame shine update after card rotation in bevy/crates/game/src/runtime/plugins/mod.rs

## Phase 5: User Story 3 - Preserve Card Inspection POC Feel

**Goal**: Preserve one centered inspectable card and existing pointer-driven rotation.

**Independent Test**: Verify the prototype still launches directly into one inspectable card with no gameplay or menus.

- [X] T010 [US3] Keep existing card inspection input and parallax flow while extending layer coverage in bevy/crates/game/src/runtime/systems/mod.rs

## Phase 6: User Story 4 - Distinguish Layer Boundaries Clearly

**Goal**: Keep layer boundaries readable during motion and shine.

**Independent Test**: Inspect during movement and identify the four layers without implementation details.

- [X] T011 [US4] Tune layer sizes, z offsets, and alpha material settings in bevy/crates/game/src/runtime/systems/mod.rs

## Phase 7: User Story 5 - Toggle Card Card Type Slot

**Goal**: Pressing HUD `T` toggles card type state without invalid artwork while only SKYBOLT exists.

**Independent Test**: Press `T` repeatedly and confirm the HUD and card remain stable on SKYBOLT.

- [X] T012 [US5] Add `T` key HUD span and active card type display in bevy/crates/game/src/runtime/systems/mod.rs
- [X] T013 [US5] Add card type toggle system and registry tests in bevy/crates/game/src/runtime/resources/mod.rs and bevy/crates/game/src/runtime/systems/mod.rs

## Phase 8: Polish & Validation

## Phase 8: Imported DebugHUD Reload Workflow

**Goal**: Bring in only the `R` and `H` DebugHUD behaviors from the related `bevy-zoo-game` 001/003 specs.

**Independent Test**: Press `R` and confirm reloadable card scene content is rebuilt; press `H` and confirm hot-reload auto-restart toggles independently and persists through the approved local state path.

- [X] T014 [US6] Update 005 specs to include `R` AppScene reload and `H` hot-reload auto-restart scope from `bevy-zoo-game` specs 001/003
- [X] T015 [US6] Add reloadable AppScene markers and `R` reload system in bevy/crates/game/src/runtime/components/mod.rs and bevy/crates/game/src/runtime/systems/mod.rs
- [X] T016 [US6] Add persisted DebugHUD input state and `H` hot-reload auto-restart toggle in bevy/crates/game/src/runtime/resources/mod.rs and bevy/crates/game/src/runtime/systems/mod.rs
- [X] T017 [US6] Wire desktop hot-reload patch recording and auto-restart scheduling in bevy/crates/game/src/main.rs and bevy/crates/game/src/runtime/plugins/mod.rs
- [X] T018 [US6] Add focused tests for `R` non-toggle reload, `H` toggle behavior, and key classification in bevy/crates/game/src/runtime/systems/mod.rs

## Phase 9: Polish & Validation

- [X] T019 Run scripts/other/RunTests.ps1
- [X] T020 Run scripts/other/RunAppDesktop.ps1 -CheckOnly
- [X] T021 Run scripts/other/RunAppWeb.ps1 -CheckOnly
- [X] T022 Run cargo check -p bevy-card-game --features desktop-hot-reload

## Dependencies & Execution Order

Complete setup before foundational tasks. Complete foundational tasks before user story tasks. User stories should be implemented in priority order because they share the same runtime systems file.
