# Tasks: Game Theme POC

**Input**: Design documents from `/specs/008-game-theme-poc/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/
**Tests**: Test and verification tasks are included because this feature permanently replaces visible runtime cards/world art and changes scene navigation behavior.
**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Confirm implementation context and prepare the permanent 008 asset/source layout.

- [x] T001 Inspect current card, world, scene, and persistence implementation in `bevy/crates/game/src/runtime/resources/mod.rs` and `bevy/crates/game/src/runtime/systems/mod.rs`
- [x] T002 [P] Create lowercase asset directories for 008 card structure, card types, worlds, and locations under `bevy/crates/game/assets/`
- [x] T003 [P] Review README runtime controls and structure entries in `README.md` for 008 documentation updates needed after implementation

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core data/assets that MUST be complete before any user story can be implemented.

**CRITICAL**: No user story work can begin until this phase is complete.

- [x] T004 Create new Japan Realism card back asset `bevy/crates/game/assets/cards/card_structure/card_back_japan_realism.png`
- [x] T005 [P] Create Kage Ren card assets in `bevy/crates/game/assets/cards/card_types/card_type_kage_ren/`
- [x] T006 [P] Create Lord Daichi card assets in `bevy/crates/game/assets/cards/card_types/card_type_lord_daichi/`
- [x] T007 [P] Create Sister Hotaru card assets in `bevy/crates/game/assets/cards/card_types/card_type_sister_hotaru/`
- [x] T008 [P] Create Yokai placeholder card assets in `bevy/crates/game/assets/cards/card_types/card_type_yokai_placeholder/`
- [x] T009 [P] Create Bamboo Forest world background asset in `bevy/crates/game/assets/worlds/bamboo_forest/world_background.png`
- [x] T010 [P] Create Coastal Harbor world background asset in `bevy/crates/game/assets/worlds/coastal_harbor/world_background.png`
- [x] T011 [P] Create six reusable location assets under `bevy/crates/game/assets/locations/`
- [x] T012 Replace legacy card constants and registry definitions with four 008 card identities in `bevy/crates/game/src/runtime/resources/mod.rs`
- [x] T013 Add world theme and tactical location data resources in `bevy/crates/game/src/runtime/resources/mod.rs`
- [x] T014 Add selected-card and active-world runtime resources in `bevy/crates/game/src/runtime/resources/mod.rs`
- [x] T015 Register new resources in `bevy/crates/game/src/runtime/plugins/mod.rs`
- [x] T016 Update existing resource tests for four card identities, new card back, and new asset paths in `bevy/crates/game/src/runtime/resources/mod.rs`

**Checkpoint**: Foundation ready; user story implementation can now begin.

---

## Phase 3: User Story 1 - Demonstrate Match Atmosphere (Priority: P1) MVP

**Goal**: GameScene shows a cohesive Japan Realism match view with one active world, three centered locations, and four bottom cards.

**Independent Test**: Open the game scene and confirm it communicates the selected world, centered tactical locations, and four-card lineup without gameplay actions.

### Implementation for User Story 1

- [x] T017 [US1] Replace the current single-card hand preview with a four-card bottom lineup in `bevy/crates/game/src/runtime/systems/mod.rs`
- [x] T018 [US1] Render the active world background from the active world resource in `bevy/crates/game/src/runtime/systems/mod.rs`
- [x] T019 [US1] Render three active tactical locations across the center of the GameScene in `bevy/crates/game/src/runtime/systems/mod.rs`
- [x] T020 [US1] Remove old desert/SkyBolt/Tar runtime presentation references from normal GameScene rendering in `bevy/crates/game/src/runtime/systems/mod.rs`
- [x] T021 [US1] Add or update GameScene ECS tests for four bottom cards, one active world background, and three locations in `bevy/crates/game/src/runtime/systems/mod.rs`

**Checkpoint**: User Story 1 is fully functional and testable independently.

---

## Phase 4: User Story 2 - Cycle World Themes (Priority: P1)

**Goal**: Pressing `T` in GameScene cycles Bamboo Forest and Coastal Harbor, updates the background/lighting/location rendering, and leaves cards unchanged.

**Independent Test**: Press `T` in GameScene and verify the world changes while the same four cards remain visible.

### Implementation for User Story 2

- [x] T022 [US2] Add GameScene-only world cycling behavior for `T` in `bevy/crates/game/src/runtime/systems/mod.rs`
- [x] T023 [US2] Add location reselection behavior on world change in `bevy/crates/game/src/runtime/systems/mod.rs`
- [x] T024 [US2] Ensure `T` in GameScene does not modify CardUI settings in `bevy/crates/game/src/runtime/systems/mod.rs`
- [x] T025 [US2] Add tests for world cycling, three-location reselection, and stable card identities in `bevy/crates/game/src/runtime/systems/mod.rs`

**Checkpoint**: User Stories 1 and 2 both work independently.

---

## Phase 5: User Story 3 - Interact With Cards Elegantly (Priority: P2)

**Goal**: Bottom cards subtly lean toward cursor or touch position while remaining readable and selectable.

**Independent Test**: Move cursor or touch position across the card row and verify restrained, readable card motion.

### Implementation for User Story 3

- [x] T026 [US3] Extend pointer target tracking to support the four-card GameScene lineup in `bevy/crates/game/src/runtime/systems/mod.rs`
- [x] T027 [US3] Apply restrained card tilt to each GameScene bottom card without changing card layout bounds in `bevy/crates/game/src/runtime/systems/mod.rs`
- [x] T028 [US3] Add tests for pointer/touch card tilt bounds and readability-safe transforms in `bevy/crates/game/src/runtime/systems/mod.rs`

**Checkpoint**: User Story 3 works independently with the existing GameScene card row.

---

## Phase 6: User Story 4 - Browse a Selected Card (Priority: P2)

**Goal**: Clicking any bottom card opens Card Browser focused on that card; CardUI settings are global; flip state remains temporary.

**Independent Test**: Select each bottom card, flip it, change CardUI settings with `T`, return to GameScene, and confirm world/CardUI separation plus non-persistent flip state.

### Implementation for User Story 4

- [x] T029 [US4] Add selected-card navigation state and click observers for each bottom card in `bevy/crates/game/src/runtime/systems/mod.rs`
- [x] T030 [US4] Update Card Browser spawning to use the selected card identity in `bevy/crates/game/src/runtime/systems/mod.rs`
- [x] T031 [US4] Ensure `T` in Card Browser changes global CardUI settings rather than active world in `bevy/crates/game/src/runtime/systems/mod.rs`
- [x] T032 [US4] Ensure Card Browser flip state resets or remains session-only when changing selected cards in `bevy/crates/game/src/runtime/systems/mod.rs`
- [x] T033 [US4] Update Card Browser and navigation tests for selected-card focus, global CardUI settings, and non-persistent flip state in `bevy/crates/game/src/runtime/systems/mod.rs`

**Checkpoint**: All user stories are independently functional.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Verification, documentation, and repo consistency after all user stories are implemented.

- [x] T034 [P] Update runtime controls and structure documentation for 008 cards/worlds in `README.md`
- [x] T035 [P] Update active implementation notes or memory if new durable workflow lessons are learned in `.codex/`
- [x] T036 Run `scripts/other/RunTests.ps1` and fix any failures in touched files
- [x] T037 Run `scripts/other/RunAppDesktop.ps1 -CheckOnly` and fix any desktop build failures in touched files
- [x] T038 Run `scripts/other/RunAppWeb.ps1 -CheckOnly` and document any browser WebGPU blocker in `specs/008-game-theme-poc/quickstart.md`
- [x] T039 Manually validate the quickstart smoke scenario in `specs/008-game-theme-poc/quickstart.md`
- [x] T040 Confirm `specs/008-game-theme-poc/contracts/ui-behavior-contract.md` matches implemented behavior and update if implementation reveals a spec-approved adjustment

---

## Dependencies & Execution Order

### Phase Dependencies

| Phase | Depends On | Blocks |
| ----- | ---------- | ------ |
| Phase 1: Setup | None | Phase 2 |
| Phase 2: Foundational | Phase 1 | All user stories |
| Phase 3: US1 | Phase 2 | MVP atmosphere validation |
| Phase 4: US2 | Phase 2, US1 preferred | World cycling validation |
| Phase 5: US3 | Phase 2, US1 preferred | Card tilt validation |
| Phase 6: US4 | Phase 2, US1 preferred | Card Browser selected-card validation |
| Phase 7: Polish | Desired user stories complete | Final readiness |

### User Story Dependencies

| User Story | Dependency | Notes |
| ---------- | ---------- | ----- |
| US1 Demonstrate Match Atmosphere | Phase 2 | MVP and best first implementation slice. |
| US2 Cycle World Themes | Phase 2; benefits from US1 rendering | Can be validated after active world rendering exists. |
| US3 Interact With Cards Elegantly | Phase 2; benefits from US1 card row | Can be implemented once all four cards exist. |
| US4 Browse a Selected Card | Phase 2; benefits from US1 card row | Depends on selected-card identity being represented in the row. |

### Within Each User Story

| Rule | Requirement |
| ---- | ----------- |
| Assets before rendering | Assets and registry entries must exist before scene rendering tasks use them. |
| Resources before systems | Runtime resources must exist before systems consume them. |
| Behavior before verification | Implement behavior before running story-specific test updates. |
| Story checkpoint | Validate each story independently before moving to polish. |

## Parallel Opportunities

| Area | Parallel Tasks |
| ---- | -------------- |
| Setup | T002 and T003 can run in parallel after T001. |
| Asset creation | T005, T006, T007, T008, T009, T010, and T011 can run in parallel after T004 conventions are clear. |
| Polish | T034 and T035 can run in parallel; verification tasks T036-T039 should run sequentially after implementation. |

## Parallel Example: Foundational Asset Creation

```text
Task: "Create Kage Ren card assets in bevy/crates/game/assets/cards/card_types/card_type_kage_ren/"
Task: "Create Lord Daichi card assets in bevy/crates/game/assets/cards/card_types/card_type_lord_daichi/"
Task: "Create Sister Hotaru card assets in bevy/crates/game/assets/cards/card_types/card_type_sister_hotaru/"
Task: "Create Yokai placeholder card assets in bevy/crates/game/assets/cards/card_types/card_type_yokai_placeholder/"
Task: "Create Bamboo Forest world background asset in bevy/crates/game/assets/worlds/bamboo_forest/world_background.png"
Task: "Create Coastal Harbor world background asset in bevy/crates/game/assets/worlds/coastal_harbor/world_background.png"
Task: "Create six reusable location assets under bevy/crates/game/assets/locations/"
```

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1 setup.
2. Complete Phase 2 foundations: assets, card registry, world/location resources, plugin registration, resource tests.
3. Complete Phase 3 US1: permanent Japan Realism game scene with four bottom cards, active world background, and three locations.
4. Stop and validate US1 independently before world cycling or Card Browser changes.

### Incremental Delivery

1. US1 delivers the permanent visible theme replacement.
2. US2 adds world cycling and location reselection.
3. US3 adds subtle card tilt for the full bottom row.
4. US4 wires selected-card Card Browser focus and global CardUI behavior.
5. Polish verifies desktop/web parity and documentation.

### Implementation Notes

| Note | Guidance |
| ---- | -------- |
| Art generation | New bitmap art must be grounded Japan Realism with no magic glow; mist, smoke, rain, embers, torch fire, and lantern light are allowed. |
| Asset paths | Keep all new paths lowercase `snake_case` under `bevy/crates/game/assets/`. |
| Permanent replacement | Do not keep old SkyBolt/Tar/desert visuals in normal runtime presentation. |
| CardUI settings | Use global persisted settings for cards; do not add per-card persistence. |
| Flip state | Keep flip animation state temporary and non-persistent. |
| Git safety | Do not use destructive git operations while implementing tasks. |
