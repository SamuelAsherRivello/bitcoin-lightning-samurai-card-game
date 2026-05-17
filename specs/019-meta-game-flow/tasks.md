# Tasks: Meta Game Flow

**Input**: Design documents from `/specs/019-meta-game-flow/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

## Phase 1: Setup

- [X] T001 Review `bevy/crates/template-crate` and existing runtime scene/resource/component structure before adding files.
- [X] T002 Update `AGENTS.md` active implementation plan to `specs/019-meta-game-flow/plan.md`.

## Phase 2: Foundational

- [X] T003 Add ActiveView variants and top navigation destination mapping in `bevy/crates/game/src/runtime/resources/`.
- [X] T004 Add matchmaking and meta settings resources with unit tests.
- [X] T005 Add reusable meta-screen UI components and bundles for menu/settings controls.
- [X] T006 Add persistent settings store creation/loading/saving through existing `bevy_persistent` patterns.

## Phase 3: User Story 1 - Start From Main Page

- [X] T007 Add MainMenuScreen scene setup with top navigation, Login with Lightning, and Start Game.
- [X] T008 Add LightningScreen scene setup with QR placeholder, Back, and Learn About Lightning.
- [X] T009 Add MatchmakingScreen scene setup and update timer transition to GameScreen.
- [X] T010 Add tests for initial MainMenuScreen active view and matchmaking state transitions.

## Phase 4: User Story 2 - Navigate And Refresh Screens

- [X] T011 Extend top navigation input handling to route/reload Play Game, My Decks, Settings, and Debug.
- [X] T012 Preserve clickability of selected top navigation buttons.
- [X] T013 Add tests for route/reload decisions, including GameScreen Play Game returning to MainMenuScreen.

## Phase 5: User Story 3 - Configure Match Before Game

- [X] T014 Add SettingsScreen UI for CPU Brain Level 1, match mode, SFX, and music.
- [X] T015 Remove match mode button from GameScreen controls.
- [X] T016 Wire SettingsScreen mode selection into the next game start.
- [X] T017 Add tests for settings toggles and persistence model conversion.

## Phase 6: User Story 4 - Preserve Deck And Debug Workflows

- [X] T018 Validate DeckScreen modal opens from both deck and library card zones.
- [X] T019 Keep Shop tab clickable but non-functional.
- [X] T020 Ensure DebugScreen top navigation route shows existing debug card and Card UI.
- [X] T021 Add or update tests for DeckScreen card overlay and Shop no-op behavior.

## Phase 7: Polish

- [X] T022 Run `scripts/other/RunTests.ps1`.
- [ ] T023 Run desktop AI runtime verification and use BRP screenshot/click workflow where available.
- [X] T024 Run browser verification or document blocker.

## Phase 8: Meta Game Feedback Fixes

- [X] T025 [US2] Rename MainMenuScreen/MainMenuScene code, UI names, and feature labels to MainMenuScreen/MainMenuScene in `bevy/crates/game/src/runtime/`, `bevy/crates/game/src/tests/`, and `specs/019-meta-game-flow/`.
- [X] T026 [US2] Rename LightningScreen/LightningScene code, UI names, and feature labels to LightningScreen/LightningScene in `bevy/crates/game/src/runtime/`, `bevy/crates/game/src/tests/`, and `specs/019-meta-game-flow/`.
- [X] T027 [US2] Remove top navigation from GameScreen and add a functional Quit Game button above Restart that transitions to MainMenuScreen in `bevy/crates/game/src/runtime/systems/mod.rs` and `bevy/crates/game/src/runtime/components/game_control_component.rs`.
- [X] T028 [US4] Add top navigation to DebugScreen and update DebugHUD screen text to DebugScreen while preserving AppScene plus DebugScene ownership in `bevy/crates/game/src/runtime/systems/mod.rs`.
- [X] T029 [US1] Replace LightningScreen mock QR blocks with a runtime-loaded generic QR image and bake a lightning bolt icon asset for Lightning buttons in `bevy/crates/game/assets/ui/` and `bevy/crates/game/src/runtime/systems/mod.rs`.
- [X] T030 [US1] Add LightningScreen instruction text to the right of the QR code using the mockup copy and a same-sized text area in `bevy/crates/game/src/runtime/systems/mod.rs`.
- [X] T031 [US1] Add the baked lightning bolt icon to the left of every Lightning-related button label in `bevy/crates/game/src/runtime/systems/mod.rs`.
- [X] T032 [US4] Create a reusable CardGrid design element with title, grey/light-blue rectangle, fixed dimensions, and left/right DeckScreen placement in `bevy/crates/game/src/runtime/components/deck_screen_component.rs` and `bevy/crates/game/src/runtime/systems/mod.rs`.
- [X] T033 [US4] Set DeckScreen editor grids to Deck 01 on the left and Not In Deck on the right, with Library/Shop buttons horizontally aligned above the right grid and equal shared width in `bevy/crates/game/src/runtime/systems/mod.rs`.
- [X] T034 [US4] Make all DeckScreen card tiles clickable, showing the selected-card action menu to the right of the selected card in `bevy/crates/game/src/runtime/resources/deck_screen_model.rs` and `bevy/crates/game/src/runtime/systems/mod.rs`.
- [X] T035 [US4] Ensure Move To Deck 01, Move To Library, and Back actions are functional from the DeckScreen selected-card menu in `bevy/crates/game/src/runtime/systems/mod.rs`.
- [X] T036 [US4] Make Library selected by default with exactly three cards visible, make Shop render zero cards in the same CardGrid size, and keep both tabs clickable in `bevy/crates/game/src/runtime/resources/deck_screen_model.rs` and `bevy/crates/game/src/runtime/systems/mod.rs`.
- [X] T037 [US4] Restore white point-view text on cards rendered in DeckScreen grids in `bevy/crates/game/src/runtime/systems/mod.rs`.
- [X] T038 Run `scripts/other/RunTests.ps1` after feedback fixes.
