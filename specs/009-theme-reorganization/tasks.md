# Tasks: Theme Reorganization

**Input**: Design documents from `/specs/009-theme-reorganization/`
**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/asset-organization-contract.md](./contracts/asset-organization-contract.md), [quickstart.md](./quickstart.md)

**Tests**: Include focused regression and static validation tasks because the spec defines independent tests and requires unchanged card, world, and location behavior.

**Organization**: Tasks are grouped by user story so each story can be implemented and validated as an independent increment.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel because it touches different files or has no dependency on incomplete tasks.
- **[Story]**: Maps task to a user story from [spec.md](./spec.md).
- Every task includes exact file paths.

## Path Conventions

| Area | Path |
| ---- | ---- |
| Bevy runtime source | `bevy/crates/game/src/runtime/` |
| Bevy runtime assets | `bevy/crates/game/assets/` |
| Theme root | `bevy/crates/game/assets/themes/theme_japan/` |
| Documentation | `README.md`, `AGENTS.md`, `specs/009-theme-reorganization/` |
| Verification scripts | `scripts/other/RunTests.ps1`, `scripts/other/RunAppDesktop.ps1`, `scripts/other/RunAppWeb.ps1` |

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Confirm current references and prepare the theme asset root before story work.

- [X] T001 Inventory current card, location, and world asset references in `README.md`, `bevy/crates/game/src/main.rs`, `bevy/crates/game/src/runtime/resources/mod.rs`, and `scripts/other/GenerateCardFrameAssets.py`
- [X] T002 Create the theme category directory structure under `bevy/crates/game/assets/themes/theme_japan/cards`, `bevy/crates/game/assets/themes/theme_japan/locations`, and `bevy/crates/game/assets/themes/theme_japan/worlds`
- [X] T003 [P] Confirm shared assets that remain outside the theme root in `bevy/crates/game/assets/shaders/card_background_mask.wgsl`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Add path constants and validation coverage that all stories rely on.

**CRITICAL**: No user story work should begin until this phase is complete.

- [X] T004 Add or update theme asset path constants in `bevy/crates/game/src/runtime/resources/mod.rs` for `themes/theme_japan/cards`, `themes/theme_japan/locations`, and `themes/theme_japan/worlds`
- [X] T005 Add static path validation tests in `bevy/crates/game/src/runtime/resources/mod.rs` covering theme root paths, shared shader separation, and lowercase `snake_case` path expectations
- [X] T006 Update the desktop asset-root smoke test in `bevy/crates/game/src/main.rs` to check a `themes/theme_japan/cards/card_kage_ren/background.png` asset

**Checkpoint**: Shared path constants and tests are ready for story implementation.

---

## Phase 3: User Story 1 - Organize Theme Assets for Growth (Priority: P1) MVP

**Goal**: A developer can find all current Japan theme cards, locations, and worlds under one theme-specific root.

**Independent Test**: Inspect `bevy/crates/game/assets/themes/theme_japan` and confirm `cards`, `locations`, and `worlds` contain the current Japan theme assets.

### Tests for User Story 1

- [X] T007 [P] [US1] Add an asset existence regression test in `bevy/crates/game/src/runtime/resources/mod.rs` for the four card folders, six location folders, and two world folders under `themes/theme_japan`
- [X] T008 [P] [US1] Add a documentation path regression check in `bevy/crates/game/src/runtime/resources/mod.rs` or an existing test module that asserts runtime card, location, and world paths start with `themes/theme_japan/`

### Implementation for User Story 1

- [X] T009 [US1] Relocate card assets from `bevy/crates/game/assets/cards/card_structure` and `bevy/crates/game/assets/cards/card_types` into `bevy/crates/game/assets/themes/theme_japan/cards`
- [X] T010 [US1] Relocate location assets from `bevy/crates/game/assets/locations` into `bevy/crates/game/assets/themes/theme_japan/locations`
- [X] T011 [US1] Relocate world assets from `bevy/crates/game/assets/worlds` into `bevy/crates/game/assets/themes/theme_japan/worlds`
- [X] T012 [US1] Update card, location, and world texture paths in `bevy/crates/game/src/runtime/resources/mod.rs` to resolve through `themes/theme_japan`
- [X] T013 [US1] Update the frame-generation asset root in `scripts/other/GenerateCardFrameAssets.py` to write frames under `bevy/crates/game/assets/themes/theme_japan/cards`
- [X] T014 [US1] Update README asset location rows in `README.md` to describe `bevy/crates/game/assets/themes/theme_japan/{cards,locations,worlds}`

**Checkpoint**: User Story 1 is independently verifiable by asset inspection and runtime path tests.

---

## Phase 4: User Story 2 - Use Theme-Local Naming (Priority: P1)

**Goal**: Theme-owned card, location, and world asset names identify their category without repeating `japan`.

**Independent Test**: Review all theme-owned asset folders and confirm names start with `card_`, `location_`, or `world_`, and no owned item repeats `japan` outside `theme_japan`.

### Tests for User Story 2

- [X] T015 [P] [US2] Add naming validation tests in `bevy/crates/game/src/runtime/resources/mod.rs` for `card_`, `location_`, and `world_` folder prefixes under `themes/theme_japan`
- [X] T016 [P] [US2] Add a validation test in `bevy/crates/game/src/runtime/resources/mod.rs` that owned card, location, and world path segments after `theme_japan` do not contain `japan`

### Implementation for User Story 2

- [X] T017 [US2] Rename card folders in `bevy/crates/game/assets/themes/theme_japan/cards` to `card_kage_ren`, `card_lord_daichi`, `card_sister_hotaru`, and `card_yokai_placeholder`
- [X] T018 [US2] Rename location folders in `bevy/crates/game/assets/themes/theme_japan/locations` to `location_fortress_gate`, `location_bamboo_crossing`, `location_shrine_ruins`, `location_battlefield`, `location_spirit_well`, and `location_market_square`
- [X] T019 [US2] Rename world folders in `bevy/crates/game/assets/themes/theme_japan/worlds` to `world_bamboo_forest` and `world_coastal_harbor`
- [X] T020 [US2] Rename the card back asset reference from `card_back_japan_realism.png` to `card_back.png` in `bevy/crates/game/assets/themes/theme_japan/cards` and `bevy/crates/game/src/runtime/resources/mod.rs`
- [X] T021 [US2] Update any remaining old card, location, and world path references in `README.md`, `bevy/crates/game/src/main.rs`, `bevy/crates/game/src/runtime/resources/mod.rs`, and `scripts/other/GenerateCardFrameAssets.py`

**Checkpoint**: User Story 2 is independently verifiable by naming tests and path review.

---

## Phase 5: User Story 3 - Use Purposeful Scene and Model/View Naming (Priority: P2)

**Goal**: A developer can distinguish app structure, card data, and rendering through `AppScene`, `GameView`, `DeckBuilderScene`, `DebugSettingsScene`, `DebugSettingsScene`, `CardModel`, `CardView`, and `CardViewBundle`.

**Independent Test**: Read the updated documentation and trace `AppScene` hosting one active sub-screen view, then trace one loaded card from `CardModel` data to a rendered `CardView` created by `CardViewBundle`.

### Tests for User Story 3

- [X] T022 [P] [US3] Add or update card model registry tests in `bevy/crates/game/src/runtime/resources/mod.rs` to assert each `CardModel` has background, frame, foreground, title, and shared back presentation paths
- [X] T023 [P] [US3] Add or update scene/view structure tests in `bevy/crates/game/src/runtime/systems/mod.rs` to keep `AppScene` always present while one of `GameView`, `DeckBuilderScene`, or `DebugSettingsScene` is active

### Implementation for User Story 3

- [X] T024 [US3] Rename or document card data types in `bevy/crates/game/src/runtime/resources/mod.rs` toward `CardModel`, `CardModelRegistry`, and `ActiveCardModel` after approval
- [X] T025 [US3] Rename or document scene and sub-screen presentation helpers in `bevy/crates/game/src/runtime/components/mod.rs`, `bevy/crates/game/src/runtime/resources/mod.rs`, and `bevy/crates/game/src/runtime/systems/mod.rs` toward `AppScene`, `ActiveView`, `GameView`, and `DeckBuilderScene` after approval
- [X] T026 [US3] Rename or document card rendering helpers in `bevy/crates/game/src/runtime/systems/mod.rs` toward `CardView` and `CardViewBundle` after approval
- [X] T027 [US3] Document the `AppScene`, `GameView`, `DeckBuilderScene`, `DebugSettingsScene`, `DebugSettingsScene`, `CardModel`, `CardView`, and `CardViewBundle` concepts and required contents in `README.md`
- [X] T028 [US3] Update `specs/009-theme-reorganization/contracts/asset-organization-contract.md` if approved implementation naming differs from the proposal
- [X] T029 [US3] Split changed runtime source in `bevy/crates/game/src/runtime` so each changed file centers on one primary Plugin, Component, Scene, View, Model, or System concept
- [X] T030 [US3] Rename changed runtime system functions in `bevy/crates/game/src/runtime/systems` to `[domain]_[schedule]_system`
- [X] T031 [US3] Add terse `HUMAN:` and `AI:` purpose comments above each changed or new primary runtime item in `bevy/crates/game/src/runtime`

**Checkpoint**: User Story 3 is independently verifiable by documentation review and card model/view behavior tests.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Validate the full feature, update planning notes if implementation details changed, and confirm desktop/browser parity.

- [X] T032 [P] Update `specs/009-theme-reorganization/quickstart.md` if the final verification commands or asset paths differ from the plan
- [X] T033 [P] Update `specs/009-theme-reorganization/data-model.md` if final entity names or path mappings differ from implementation
- [X] T034 Run `scripts/other/RunTests.ps1` and record any blockers in `specs/009-theme-reorganization/quickstart.md`
- [X] T035 Run `scripts/other/RunAppDesktop.ps1 -CheckOnly` and record any blockers in `specs/009-theme-reorganization/quickstart.md`
- [X] T036 Run `scripts/other/RunAppWeb.ps1 -CheckOnly` and record any blockers in `specs/009-theme-reorganization/quickstart.md`
- [X] T037 Perform the quickstart behavior smoke in `specs/009-theme-reorganization/quickstart.md` for `GameView` display, world toggle, card click, card flip, and CardUI toggle
- [X] T038 Search for stale pre-009 asset, scene, and view references in `README.md`, `AGENTS.md`, `bevy/crates/game/src`, `scripts`, and `specs/009-theme-reorganization`
- [X] T039 Verify changed Bevy runtime files follow one-primary-concept, Scene/Model/View naming, `[domain]_[schedule]_system`, and `HUMAN:` / `AI:` purpose comment standards

---

## Dependencies & Execution Order

### Phase Dependencies

| Phase | Depends On | Blocks |
| ----- | ---------- | ------ |
| Phase 1: Setup | None | Phase 2 |
| Phase 2: Foundational | Phase 1 | All user stories |
| Phase 3: US1 | Phase 2 | Full asset reorganization MVP |
| Phase 4: US2 | Phase 2; can proceed alongside US1 once target folders exist | Naming acceptance and final path references |
| Phase 5: US3 | Phase 2; benefits from US1 path migration | Card bundle documentation and behavior traceability |
| Phase 6: Polish | Desired user stories complete | Final validation |

### User Story Dependencies

| User Story | Dependency | Notes |
| ---------- | ---------- | ----- |
| US1: Organize Theme Assets for Growth | Phase 2 | MVP; creates the theme-root organization. |
| US2: Use Theme-Local Naming | Phase 2 and target theme folders from US1 | Can validate naming independently after folders exist. |
| US3: Use Purposeful Scene and Model/View Naming | Phase 2 and card paths from US1 | Documentation can start early; final traceability depends on approved `AppScene`, `GameView`, `DeckBuilderScene`, `DebugSettingsScene`, `DebugSettingsScene`, `CardModel`, and `CardViewBundle` names. |

### Within Each User Story

| Order | Rule |
| ----- | ---- |
| 1 | Add or update focused validation tests before implementation tasks in that story. |
| 2 | Update assets and path constants before changing docs that describe final locations. |
| 3 | Run the independent checkpoint before moving to polish. |

### Parallel Opportunities

| Scope | Parallel Tasks |
| ----- | -------------- |
| Setup | T003 can run in parallel with T001 after the repository is inspected. |
| US1 tests | T007 and T008 touch validation coverage and can be prepared together. |
| US2 tests | T015 and T016 can be prepared together. |
| US3 tests | T022 and T023 can be prepared together. |
| Polish docs | T032 and T033 can run in parallel. |

---

## Parallel Example: User Story 1

```text
Task: "T007 [P] [US1] Add an asset existence regression test in bevy/crates/game/src/runtime/resources/mod.rs for the four card folders, six location folders, and two world folders under themes/theme_japan"
Task: "T008 [P] [US1] Add a documentation path regression check in bevy/crates/game/src/runtime/resources/mod.rs or an existing test module that asserts runtime card, location, and world paths start with themes/theme_japan/"
```

---

## Parallel Example: User Story 2

```text
Task: "T015 [P] [US2] Add naming validation tests in bevy/crates/game/src/runtime/resources/mod.rs for card_, location_, and world_ folder prefixes under themes/theme_japan"
Task: "T016 [P] [US2] Add a validation test in bevy/crates/game/src/runtime/resources/mod.rs that owned card, location, and world path segments after theme_japan do not contain japan"
```

---

## Parallel Example: User Story 3

```text
Task: "T022 [P] [US3] Add or update card model registry tests in bevy/crates/game/src/runtime/resources/mod.rs to assert each CardModel has background, frame, foreground, title, and shared back presentation paths"
Task: "T023 [P] [US3] Add or update scene/view structure tests in bevy/crates/game/src/runtime/systems/mod.rs to keep AppScene always present while one of GameView, DeckBuilderScene, or DebugSettingsScene is active"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup.
2. Complete Phase 2: Foundational.
3. Complete Phase 3: User Story 1.
4. Stop and validate the theme root contains card, location, and world categories and runtime paths resolve below `themes/theme_japan`.

### Incremental Delivery

1. Deliver US1 to establish the theme root and migrated path references.
2. Deliver US2 to enforce category prefixes and remove repeated theme names from owned asset names.
3. Deliver US3 to complete `AppScene`, `GameView`, `DeckBuilderScene`, `DebugSettingsScene`, `DebugSettingsScene`, `CardModel`, `CardView`, and `CardViewBundle` terminology and traceability.
4. Complete Phase 6 to verify tests, desktop check, browser check, and quickstart smoke.

### Parallel Team Strategy

| Developer | Work |
| --------- | ---- |
| Developer A | US1 asset relocation and runtime path migration. |
| Developer B | US2 naming tests and folder rename validation after theme folders exist. |
| Developer C | US3 documentation and Scene/Model/View terminology updates. |

---

## Notes

- `[P]` tasks touch different files or can be prepared without depending on incomplete implementation.
- `[US1]`, `[US2]`, and `[US3]` labels map to the user stories in [spec.md](./spec.md).
- Use `AppScene` for the always-present app-level scene; use `GameView`, `DeckBuilderScene`, and `DebugSettingsScene` for active sub-screen presentations; use `CardModel` for card data, `CardView` for rendered presentation, and `CardViewBundle` for the visual bundle that creates card visuals after approval.
- Do not move shared shader assets into the theme root unless the spec is changed.
