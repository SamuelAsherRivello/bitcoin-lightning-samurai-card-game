# Feature Specification: AI Integration

**Feature Branch**: `011-ai-integration`  
**Created**: 2026-05-11  
**Status**: Draft  
**Input**: User description: "Add a new spec to encapsulate all the previous, current, and future goals for AI integration. You are Codex and we use Codex and Spec Kit already in the project. Look into MCPs that Codex can use to read more about the runtime experience and take screenshots. Compare alternatives and choose one MCP, including https://github.com/ladvien/bevy_debugger_mcp."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Unify AI Integration Goals (Priority: P1)

A maintainer can read one spec that explains how Codex, Spec Kit, existing debugging work, runtime inspection, screenshots, and future AI-assisted workflows fit together.

**Why this priority**: Earlier specs define debugging, card presentation, gameplay concepts, theme organization, and point systems separately. AI integration needs one durable contract that ties them together without rewriting those feature specs.

**Independent Test**: Review this spec and verify that it references prior goals, current Codex/Spec Kit workflow, and future AI runtime tooling as one coherent roadmap.

**Acceptance Scenarios**:

1. **Given** a maintainer reads this spec, **When** they look for previous AI-adjacent goals, **Then** they can identify how `003-debugging`, `006-card-bundle`, `007-gameplay-concepts`, `009-theme-reorganization`, and `010-point-system` inform AI integration.
2. **Given** a future task needs AI runtime inspection, **When** the implementer plans it, **Then** this spec identifies the approved MCP direction and the safety limits.
3. **Given** a future task needs browser or desktop visual evidence, **When** the implementer plans QA, **Then** this spec distinguishes runtime MCP screenshots from browser QA screenshots.

---

### User Story 2 - Inspect Running Game State Through Codex (Priority: P1)

Codex can use an approved local MCP bridge to inspect the running Bevy game state, including reflected entities, components, resources, and diagnostic state exposed by the game.

**Why this priority**: Runtime state inspection is the main missing feedback channel between source edits, terminal logs, visual screenshots, and actual ECS behavior.

**Independent Test**: Start the game with the approved dev-only remote tooling enabled, query a known reflected component or resource through the MCP bridge, and verify the response matches the visible scene or expected game state.

**Acceptance Scenarios**:

1. **Given** the game is running locally with AI runtime tooling enabled, **When** Codex queries reflected ECS state through the MCP bridge, **Then** Codex receives structured runtime data without reading secrets or unrelated machine state.
2. **Given** the game is not running or the remote endpoint is disabled, **When** Codex attempts runtime inspection, **Then** the tool reports a clear connection failure rather than silently inventing state.
3. **Given** the browser WebGPU target is under review, **When** the selected MCP cannot run in that target, **Then** the limitation is documented and browser QA uses the existing served-web verification path.

---

### User Story 3 - Capture Screenshots For AI QA (Priority: P1)

Codex can request screenshots of the running game for visual debugging and compare those screenshots with spec expectations.

**Why this priority**: The project is visual and stateful. AI-assisted work needs image evidence, not only logs and tests.

**Independent Test**: Run the desktop game with AI runtime tooling enabled, request a screenshot through the approved MCP screenshot flow, and verify the saved image shows the current game window.

**Acceptance Scenarios**:

1. **Given** the desktop game is running with screenshot support enabled, **When** Codex requests a screenshot through the MCP bridge, **Then** an image artifact is written under a project-approved generated-output path.
2. **Given** a screenshot is captured, **When** it is reviewed, **Then** it can be associated with the current spec, target, command, and timestamp without relying on global machine paths.
3. **Given** the web build is running in a browser, **When** browser-specific screenshot QA is needed, **Then** Playwright or the in-app browser may capture the browser viewport as a complement to the selected runtime MCP.
4. **Given** the user asks Codex to "peek" at the app, running app, game, or desktop runtime, **When** the AI runtime endpoint is available, **Then** Codex captures a runtime screenshot, queries available BRP state, inspects the image, and reports concise runtime plus visual observations.

---

### User Story 4 - Keep AI Tooling Local And Safe (Priority: P1)

AI runtime inspection is local-only, development-only, and explicitly separated from production gameplay behavior.

**Why this priority**: Remote game inspection and mutation tools can expose or alter runtime state. The project needs a clear boundary before adopting them.

**Independent Test**: Inspect configuration and startup scripts to verify that AI runtime tooling is opt-in, localhost-scoped, excluded from browser production builds when unsupported, and documented as development tooling.

**Acceptance Scenarios**:

1. **Given** a normal app run command is used, **When** no AI tooling flag is enabled, **Then** no MCP or Bevy Remote Protocol endpoint is exposed.
2. **Given** an AI-enabled dev run command is used, **When** remote tooling starts, **Then** it binds only to localhost unless a future spec explicitly approves broader access.
3. **Given** runtime mutation tools are available, **When** Codex uses them, **Then** tasks must prefer observation and screenshots first and must document any deliberate state mutation.

---

### User Story 5 - Preserve Future AI Extensibility (Priority: P2)

Future AI-assisted workflows can add richer diagnostics, guided playtesting, bug reproduction, balancing analysis, and generated QA reports without changing the core integration contract.

**Why this priority**: The requested spec should cover previous, current, and future goals, not only the first MCP setup.

**Independent Test**: Review the entities and requirements and verify that adding a future tool such as replay capture, scripted scenario setup, or card-balance analysis fits under the same AI integration model.

**Acceptance Scenarios**:

1. **Given** a future feature needs AI-guided playtesting, **When** it is planned, **Then** it can reuse runtime observation, screenshot capture, and Spec Kit artifacts from this feature.
2. **Given** a future feature needs game-state mutation for experiments, **When** it is planned, **Then** it must define rollback or reset behavior before implementation.
3. **Given** a future feature introduces new card, location, world, or point entities, **When** runtime inspection is used, **Then** those entities expose enough reflected state to be queryable when development tooling is enabled.

### Edge Cases

| Edge Case | Expected Behavior |
| --------- | ----------------- |
| The selected MCP is installed but Codex has not been configured to use it | The repo documents the required local MCP configuration, but agents do not edit global Codex config unless explicitly asked. |
| The MCP can inspect desktop but not browser WebGPU | Desktop runtime MCP remains the selected primary bridge; browser screenshots use Playwright or the in-app browser as complementary QA. |
| Bevy Remote Protocol can mutate ECS state | Observation and screenshots are preferred; mutation requires a task-specific reason and documented reset or rollback path. |
| A component or resource is not reflected | The tool reports that the type is unavailable; implementation tasks may add reflection only for safe, relevant diagnostics. |
| The game is not running | Codex reports the connection failure and uses source inspection, tests, logs, or screenshots from other workflows instead. |
| Screenshot paths point outside the repository | The capture is invalid for project workflow; screenshots must be written under a documented project-local generated-output location. |
| AI tooling impacts frame rate | The tool remains dev-only and any overhead must be documented before enabling it in regular workflows. |

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The AI integration spec MUST describe how Codex, Spec Kit, local scripts, specs, runtime diagnostics, and screenshots work together.
- **FR-002**: The feature MUST treat `003-debugging` as the existing owner of developer-facing diagnostics, DebugHUD behavior, inspector access, terminal self-logging, and repeatable tests.
- **FR-003**: The feature MUST treat `006-card-bundle` as the early card presentation QA surface that benefits from AI screenshot review.
- **FR-004**: The feature MUST treat `007-gameplay-concepts`, `009-theme-reorganization`, and `010-point-system` as domain context that future AI runtime inspection should be able to reason about.
- **FR-005**: The feature MUST select one primary MCP for Bevy runtime inspection.
- **FR-006**: The selected primary MCP MUST be `bevy_debugger_mcp` unless a later research update finds it unusable for this repository.
- **FR-007**: The selected MCP MUST be integrated through an opt-in development workflow, not the default production or regular app startup path.
- **FR-008**: Runtime inspection MUST be local-only by default.
- **FR-009**: The game MUST expose Bevy Remote Protocol support only when an AI/debug feature flag or dev script explicitly enables it.
- **FR-010**: The integration MUST support querying reflected ECS entities and components needed to understand card, location, world, camera, DebugHUD, and point-system state.
- **FR-011**: The integration MUST support querying reflected resources needed to understand current scene, theme, card registry, location registry, diagnostics, and game-state summaries when those resources are safe to expose.
- **FR-012**: The integration MUST support screenshot capture for the desktop runtime through the selected MCP path.
- **FR-013**: Screenshot capture MUST save artifacts under a project-local generated-output path documented by the implementation plan or task.
- **FR-014**: Browser WebGPU screenshot QA MAY use Playwright or the in-app browser as complementary tooling, but those tools are not the selected Bevy runtime MCP.
- **FR-014A**: A user request to "peek" at the app, running app, game, or desktop runtime MUST mean Codex should use the AI runtime workflow: BRP discovery/query plus `bevy_debugger/screenshot` capture when available.
- **FR-015**: The integration MUST NOT require secrets, API keys, cookies, cloud credentials, or global machine credentials.
- **FR-016**: The integration MUST NOT expose local-only services to the public internet.
- **FR-017**: The integration MUST document whether each runtime operation is observational, screenshot-only, or mutating.
- **FR-018**: Mutating runtime operations MUST require a task-specific reason and a reset, rollback, or restart plan.
- **FR-019**: The integration MUST document desktop and browser target coverage separately.
- **FR-020**: The integration MUST preserve Windows desktop and browser WebGPU parity by documenting browser gaps rather than pretending desktop-only tools cover web behavior.
- **FR-021**: Future specs that add AI-observable gameplay state SHOULD define the reflected components or resources that Codex may query.
- **FR-022**: Future specs that add AI-generated reports SHOULD store generated outputs in project-approved documentation or scratch locations and exclude transient captures from commits unless explicitly promoted.
- **FR-023**: Codex setup instructions MUST avoid editing global Codex configuration unless the user explicitly asks for that class of change.
- **FR-024**: The feature MUST include a comparison of `bevy_debugger_mcp` against direct Bevy Remote Protocol scripts, Playwright/browser screenshot tooling, `bevy-inspector-egui`, and no-MCP terminal/test workflows.
- **FR-025**: The implementation plan MUST include quickstart guidance for verifying that the selected MCP can list tools, connect to the running game, query state, and capture a screenshot.
- **FR-026**: AI-assisted runtime QA MUST treat all on-screen positions and scaling as responsive: every visible 2D element and 3D presentation element must be positioned and scaled relative to the aspect-ratio-safe game view, and those values must update when the window, screen, or viewport size changes.

### Key Entities

| Entity | Description |
| ------ | ----------- |
| **AI Integration Profile** | The repo-level contract for what Codex may inspect, capture, mutate, and report while working on the Bevy game. |
| **MCP Runtime Bridge** | The local Model Context Protocol server that connects Codex to the running Bevy runtime. |
| **Bevy Remote Endpoint** | The local Bevy Remote Protocol endpoint exposed by the game when development AI tooling is enabled. |
| **Runtime Observation** | A structured read of reflected ECS entities, components, resources, schemas, or diagnostics. |
| **Screenshot Capture** | A project-local image artifact produced from the running desktop game or complementary browser QA tooling. |
| **Responsive Positioning And Scaling** | The project layout rule that on-screen positions and scales derive from the aspect-ratio-safe game view rather than raw window pixels or ad hoc world coordinates. |
| **Tool Operation Class** | The safety classification for a tool call: observational, screenshot-only, or mutating. |
| **AI QA Artifact** | A generated local artifact such as a screenshot, log excerpt, runtime-state snapshot, or QA report used to verify behavior. |
| **AI Safety Boundary** | The local-only, development-only rules that prevent exposing services, secrets, or production behavior through AI tooling. |

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A maintainer can identify the selected primary MCP and the reason it was chosen in under 2 minutes.
- **SC-002**: A future implementer can follow the quickstart to verify MCP tool listing, runtime connection, state query, and screenshot capture.
- **SC-003**: Runtime inspection requirements can be mapped to at least five existing or planned game concepts: DebugHUD, card, location, world, point, or camera state.
- **SC-004**: The plan clearly distinguishes desktop MCP coverage from browser WebGPU QA coverage.
- **SC-005**: The spec contains no requirement to expose a remote endpoint beyond localhost.
- **SC-006**: A reviewer can classify each planned AI operation as observational, screenshot-only, or mutating.
- **SC-007**: A reviewer can use AI screenshots or runtime state to verify that visible 2D and 3D elements preserve responsive positioning and scaling at fullscreen and smaller window sizes.

## Assumptions

| Assumption | Rationale |
| ---------- | --------- |
| Codex can use MCP servers when they are installed and configured in its runtime environment. | Codex exposes MCP/app tools in this session, but repo work should document setup rather than modify global config by default. |
| `bevy_debugger_mcp` is evaluated as an external development tool, not vendored into the repository by this spec. | The immediate request is planning and selection, not implementation or installation. |
| Desktop runtime inspection is the first target for MCP integration. | Bevy Remote Protocol HTTP support is native-only, while browser WebGPU requires separate browser automation. |
| Browser screenshots remain important even if the selected MCP is desktop-first. | The constitution requires desktop and browser WebGPU parity or documented gaps. |
| Responsive positioning includes scaling. | Current game-view and card-browser layout requirements define responsiveness as aspect-ratio-safe position and scale recalculation, not only anchoring. |
| Runtime mutation is useful but risky. | Bevy Remote Protocol and `bevy_debugger_mcp` can support state changes, so the spec requires reset or rollback guidance before mutation. |
| Generated screenshots and runtime snapshots are transient unless a future task promotes them. | The project keeps generated outputs explicit and avoids committing noisy artifacts by default. |
| `AppScene` and active views are distinct. | `AppScene` is always-present while `GameView`, `DeckBuilderScene`, `DebugSettingsScene`, or future views represent the active user-facing scene/view state. |
