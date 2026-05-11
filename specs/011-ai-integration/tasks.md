# Tasks: AI Integration

**Input**: Design documents from `/specs/011-ai-integration/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, quickstart.md, contracts/ai-runtime-mcp-contract.md

**Tests**: Include check/build tasks because this feature changes dev runtime startup and feature-gated Bevy wiring.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish the branch, active Spec Kit pointers, and external-tool documentation baseline.

- [X] T001 Update repo-local AGENTS Git rules to allow non-destructive branch creation and branch switching in AGENTS.md
- [X] T002 Create and switch to local branch `011-ai-integration`
- [X] T003 Point active Spec Kit feature metadata at `specs/011-ai-integration` in .specify/feature.json and AGENTS.md
- [X] T004 [P] Create AI integration planning artifacts in specs/011-ai-integration/

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Add dev-only runtime bridge plumbing that all runtime inspection and screenshot work depends on.

- [X] T005 Add `ai-runtime` feature wiring to bevy/crates/game/Cargo.toml using Bevy Remote Protocol behind `bevy/bevy_remote`
- [X] T006 [P] Add `ai_runtime_plugin.rs` dev plugin in bevy/crates/game/src/runtime/plugins/ for desktop-only Bevy Remote Protocol registration
- [X] T007 Export `AiRuntimePlugin` from bevy/crates/game/src/runtime/plugins/mod.rs and add targeted plugin tests
- [X] T008 Wire `AiRuntimePlugin` into bevy/crates/game/src/main.rs only for non-wasm `ai-runtime` builds
- [X] T009 Add an `-AiRuntime` option to scripts/other/RunAppDesktop.ps1 that includes `ai-runtime` in feature lists and prints the localhost BRP endpoint

**Checkpoint**: Desktop builds can opt into BRP without changing normal startup.

---

## Phase 3: User Story 1 - Unify AI Integration Goals (Priority: P1) MVP

**Goal**: Keep the repo-level AI integration contract discoverable and aligned with the selected implementation path.

**Independent Test**: Read the spec, plan, research, quickstart, and contract to confirm the selected MCP, alternatives, safety limits, and implementation workflow are clear.

- [X] T010 [P] [US1] Update specs/011-ai-integration/quickstart.md with the implemented `-AiRuntime` desktop command
- [X] T011 [P] [US1] Update specs/011-ai-integration/contracts/ai-runtime-mcp-contract.md with the concrete Cargo feature and script flag names
- [X] T012 [US1] Verify no placeholder text remains in specs/011-ai-integration/*.md or specs/011-ai-integration/contracts/*.md

**Checkpoint**: The spec set can guide a future implementer without additional context.

---

## Phase 4: User Story 2 - Inspect Running Game State Through Codex (Priority: P1)

**Goal**: Provide a local desktop runtime bridge that Codex can connect to through the selected MCP.

**Independent Test**: Run `scripts/other/RunAppDesktop.ps1 -CheckOnly -AiRuntime` and verify the `ai-runtime` feature compiles for desktop.

- [X] T013 [US2] Run `scripts/other/RunAppDesktop.ps1 -CheckOnly -AiRuntime` to verify desktop BRP feature wiring
- [X] T014 [P] [US2] Document the initial MCP connection check in specs/011-ai-integration/quickstart.md
- [X] T015 [US2] Confirm normal desktop check without `-AiRuntime` still compiles with `scripts/other/RunAppDesktop.ps1 -CheckOnly`

**Checkpoint**: Runtime inspection is opt-in and normal startup remains unchanged.

---

## Phase 5: User Story 3 - Capture Screenshots For AI QA (Priority: P1)

**Goal**: Preserve the screenshot workflow contract even before custom BRP screenshot methods are implemented.

**Independent Test**: Review quickstart and contract and verify desktop MCP screenshots and browser screenshot QA are clearly separated.

- [X] T016 [P] [US3] Add project-local screenshot output guidance to specs/011-ai-integration/quickstart.md
- [X] T017 [P] [US3] Document browser screenshot QA as complementary tooling in specs/011-ai-integration/contracts/ai-runtime-mcp-contract.md

**Checkpoint**: Screenshot expectations are clear before adding screenshot code.

---

## Phase 6: User Story 4 - Keep AI Tooling Local And Safe (Priority: P1)

**Goal**: Ensure AI runtime tooling is opt-in, local-only, and classified by operation type.

**Independent Test**: Review feature gating and scripts to verify no default startup exposes BRP.

- [X] T018 [US4] Confirm `ai-runtime` is not included in default features in bevy/crates/game/Cargo.toml
- [X] T019 [US4] Confirm scripts/other/RunAppDesktop.ps1 only enables AI runtime when `-AiRuntime` is passed
- [X] T020 [P] [US4] Add safety notes for mutating MCP operations to specs/011-ai-integration/quickstart.md

**Checkpoint**: Safety constraints are implemented in wiring and documented in workflow.

---

## Phase 7: User Story 5 - Preserve Future AI Extensibility (Priority: P2)

**Goal**: Leave clear extension points for screenshot methods, reflected diagnostics, and AI QA reports.

**Independent Test**: Review task notes and contracts to verify future screenshot and reflected-state tasks have a place to land.

- [X] T021 [P] [US5] Add future extension notes for reflected components/resources to specs/011-ai-integration/data-model.md
- [X] T022 [P] [US5] Add future custom BRP screenshot method note to specs/011-ai-integration/contracts/ai-runtime-mcp-contract.md

**Checkpoint**: Future implementation can extend the integration without changing the core contract.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Validate the completed implementation slice.

- [ ] T023 Run `scripts/other/RunTests.ps1` for repository test coverage
- [ ] T024 Run `scripts/other/RunAppWeb.ps1 -CheckOnly` to confirm browser WebGPU remains unaffected
- [ ] T025 Review changed Bevy runtime files for one-primary-concept, plugin naming, system naming, and HUMAN/AI comments
- [ ] T026 Review `git diff` for unrelated changes before handoff

---

## Dependencies & Execution Order

| Phase | Dependency |
| ----- | ---------- |
| Setup | None |
| Foundational | Setup |
| US1 | Foundational |
| US2 | Foundational |
| US3 | Foundational |
| US4 | Foundational |
| US5 | US1 through US4 documentation context |
| Polish | Desired user stories complete |

## Parallel Opportunities

| Area | Parallel Tasks |
| ---- | -------------- |
| Planning docs | T010, T011, T014, T016, T017, T020, T021, T022 |
| Runtime code | T006 can be drafted separately before T007/T008 integrate it |
| Verification | Desktop AI check, normal desktop check, and web check must use separate target directories or run sequentially |

## Implementation Strategy

| Step | Work |
| ---- | ---- |
| MVP | Complete T005 through T015 so Codex has an opt-in desktop BRP bridge path. |
| Safety | Complete T016 through T020 so screenshots and mutation limits are documented. |
| Extension | Complete T021 through T022 to preserve future screenshot/custom method work. |
| Validation | Complete T023 through T026 before handoff. |
