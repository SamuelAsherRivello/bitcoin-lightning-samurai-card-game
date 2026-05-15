# Feature Specification: Debugging

**Feature Branch**: `003-debugging`  
**Created**: 2026-05-09  
**Status**: Draft  
**Input**: User description: "Copy how the HUD works in https://github.com/SamuelAsherRivello/bevy-jam-1. Keep the main HUD panel, title/status text, and key legend. WASD should be visible but do nothing. Keep F and I only and bring in that functionality. Do not include toast, minimap, reticle, or other HUD-related systems. Add tests and a RunTests script. Refactor this spec to be called 003-debugging. It will focus on the DebugHUD and other developer-facing tooling that may or may not be rendered to the end user. Mention the Card UI too, and how you can log to yourself through the terminal and test so you QA your own work. Add a concept called debug drawing: runtime visual marks drawn around requested game scene areas, such as the hand area, to help us visually discuss and QA. These usually stay in the game until removal is requested, but are temporary debugging scaffolds to remove or replace with real art later."

## Clarifications

### Session 2026-05-09

- Q: Which bevy-jam-1 HUD content should be kept? -> A: Keep the top-left HUD panel, title/status text, key legend with non-functional `WASD`, and functional `F` and `I`; exclude toast, minimap, reticle, and other listed HUD systems.
- Q: How should `003-debugging` relate to the no-HUD rule in `006-card-bundle` when both are implemented? -> A: `003-debugging` replaces the old no-HUD rule; the final app should show the DebugHUD by default.
- Q: Should `006-card-bundle` and `003-debugging` explicitly require both Windows desktop and browser WebGPU verification? -> A: Both specs require final Windows desktop and browser WebGPU verification; iterative builds may target desktop only.
- Q: How should debug overlay scripts, tasks, docs, and source-facing labels be named? -> A: Rename all debug overlay scripts, tasks, docs, and source-facing labels to use `DebugHUD` so generic `HUD` remains available for a future user-facing HUD.
- Q: Should `WASD` labels visually react to key presses? -> A: `WASD` labels may visually highlight while pressed through a DebugHUD/InputSystem key-state capture, but no gameplay, camera, card, or other non-DebugHUD system may consume those keys in this spec.
- Q: Which approved DebugHUD keys are toggles? -> A: DebugHUD key labels are classified as toggle or non-toggle: `F`, `I`, and `H` are toggles; `W`, `A`, `S`, and `D` are non-toggle hold indicators.
- Q: Where should DebugHUD and its input capture live? -> A: DebugHUD UI, inspector toggling, diagnostic key classification, and the DebugHUD/InputSystem-style key-state capture are reusable system-level diagnostics and belong in `bevy/crates/shared`; `bevy/crates/game` should only compose them with card-specific features.
- Q: What is the broader purpose of this feature after renaming? -> A: `003-debugging` owns developer-facing diagnostics and QA tooling, including the rendered DebugHUD, inspector access, Card UI separation guidance, terminal self-logging, and repeatable tests.
- Q: Is Card UI part of the DebugHUD? -> A: No. Card UI is a temporary developer/prototype control surface that may render to the reviewer, but it remains separate from DebugHUD and must not be promoted to final player-facing UI by this spec.
- Q: How should implementers use logs and tests while working? -> A: Implementers may emit scoped terminal logs for their own debugging and must use repeatable tests and documented manual checks to QA their own work before handoff.
- Q: What is debug drawing? -> A: Debug drawing is runtime visual annotation used to mark requested game scene areas, such as drawing around the hand area, so implementers and reviewers can visually discuss layout, behavior, and QA observations.
- Q: How long should debug drawings remain? -> A: Debug drawings generally remain visible in the game until explicitly removed, but they are temporary debugging scaffolds and should eventually be removed or replaced by real UI, art, or production visualization.
- Q: Should hot reload behavior live in `001-project-setup` or `003-debugging`? -> A: `001-project-setup` owns the desktop hot reload script/tooling entry point; `003-debugging` owns the in-app DebugHUD `H` toggle, runtime hot reload observation, and screen reinitialization behavior.
- Q: What should happen when `H` is enabled and a hot-patch event arrives? -> A: The app should completely rebuild the currently active conceptual screen, losing that screen's local state and restarting it as if the user had just arrived on that screen.
- Q: What should happen when `H` is disabled and a hot-patch event arrives? -> A: The app may accept the hot patch, but it must not reinitialize the current screen, reset screen-local state, or restart scene presentation because of the patch.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Show DebugHUD Panel (Priority: P1)

A reviewer sees a top-left DebugHUD panel adapted from the bevy-jam-1 HUD pattern while running the card-inspection prototype.

**Why this priority**: The HUD provides lightweight review diagnostics without adding gameplay UI.

**Independent Test**: Launch the prototype and verify that a translucent top-left HUD panel appears with title/status text and the expected key labels.

**Acceptance Scenarios**:

1. **Given** the prototype is running, **When** the reviewer observes the top-left corner, **Then** a translucent DebugHUD panel is visible by default.
2. **Given** the HUD is visible, **When** the reviewer reads it, **Then** it shows the prototype title, frame/status text, and key labels for `W`, `A`, `S`, `D`, `F`, `I`, and `H`.

---

### User Story 2 - Toggle FPS With F (Priority: P1)

A reviewer presses `F` to show or hide FPS diagnostic text in the HUD.

**Why this priority**: FPS is the primary runtime diagnostic needed for this visual POC.

**Independent Test**: Press `F` while the prototype is running and verify that FPS text toggles visibility without affecting the card.

**Acceptance Scenarios**:

1. **Given** the HUD is visible and FPS is hidden, **When** the reviewer presses `F`, **Then** FPS text appears in the HUD.
2. **Given** FPS text is visible, **When** the reviewer presses `F` again, **Then** FPS text is hidden.
3. **Given** the reviewer toggles FPS, **When** the card is visible, **Then** card position, rotation target, and camera behavior are unchanged.

---

### User Story 3 - Toggle Inspector With I (Priority: P1)

A reviewer presses `I` to show or hide the inspector while preserving the POC scene behavior.

**Why this priority**: Inspector access helps review runtime entities and state without adding gameplay.

**Independent Test**: Press `I` while the prototype is running and verify that inspector visibility toggles without affecting the card.

**Acceptance Scenarios**:

1. **Given** the inspector is hidden, **When** the reviewer presses `I`, **Then** the inspector becomes visible.
2. **Given** the inspector is visible, **When** the reviewer presses `I` again, **Then** the inspector becomes hidden.
3. **Given** the reviewer toggles the inspector, **When** the card is visible, **Then** card position, rotation target, and camera behavior are unchanged.

---

### User Story 4 - Keep WASD Non-Functional (Priority: P2)

A reviewer sees `W`, `A`, `S`, and `D` in the HUD key legend, and those labels may visually react while pressed, but those keys do not trigger gameplay, camera, card movement, or selection behavior.

**Why this priority**: The visual pattern is copied from bevy-jam-1, but the card POC must not inherit aircraft movement or gameplay controls.

**Independent Test**: Press `W`, `A`, `S`, and `D` while the prototype is running and verify that no gameplay or card behavior changes.

**Acceptance Scenarios**:

1. **Given** the prototype is running, **When** the reviewer presses `W`, `A`, `S`, or `D`, **Then** the DebugHUD key legend may show the pressed key state, but the card does not move, rotate because of those keys, select, score, shoot, or trigger gameplay.
2. **Given** `WASD` key labels are visible, **When** the reviewer presses those keys, **Then** FPS and inspector visibility remain unchanged.
3. **Given** `WASD` key labels are visible, **When** the reviewer releases those keys, **Then** their pressed-state feedback clears because they are non-toggle hold indicators.

---

### User Story 5 - Use Developer QA Tooling (Priority: P2)

An implementer can rely on developer-facing tooling, including terminal logs and repeatable tests, to inspect behavior and QA their own work before review.

**Why this priority**: Debugging should support the person building and verifying the feature, not only the reviewer looking at rendered UI.

**Independent Test**: Run the documented test command from the repository root and verify that DebugHUD, inspector, Card UI separation, and diagnostic input behavior are covered or manually documented.

**Acceptance Scenarios**:

1. **Given** an implementer is debugging the feature, **When** they need runtime insight, **Then** they may add scoped terminal logging that explains relevant diagnostic state without printing secrets or noisy unrelated data.
2. **Given** implementation work changes DebugHUD, inspector, Card UI, or diagnostic input behavior, **When** the implementer finishes a change, **Then** they run the repeatable test command and record any blocked checks.
3. **Given** Card UI is present in the prototype, **When** debugging documentation describes it, **Then** it is described as a temporary developer/prototype control surface separate from DebugHUD and not final player-facing UI.

---

### User Story 6 - Mark Scene Areas With Debug Drawing (Priority: P2)

An implementer can add runtime visual marks around requested game scene areas so the implementer and reviewer can discuss the same visual target.

**Why this priority**: Some layout and gameplay-scene questions are easier to discuss when the running game draws a visible temporary mark around the exact area under review.

**Independent Test**: Ask for a debug drawing around a known scene region, such as the hand area, then launch the app and verify that the mark is visible, clearly scoped to that region, and documented as temporary debugging scaffolding.

**Acceptance Scenarios**:

1. **Given** a reviewer asks to debug draw around the hand area of the game scene, **When** the implementer adds the runtime mark, **Then** the running scene shows a visible annotation around that hand area.
2. **Given** a debug drawing is present, **When** the reviewer continues discussing or testing the scene, **Then** the drawing remains available until the reviewer asks for it to be removed or replaced.
3. **Given** a debug drawing is present, **When** implementation moves toward production polish, **Then** the drawing is treated as temporary and is removed or replaced with real UI, art, or production visualization.

---

### User Story 7 - Rebuild Active Screen On Hot Reload (Priority: P1)

A developer can turn on the DebugHUD `H` toggle, stay on any current screen, edit hot-reload-enabled code, and see that screen rebuild from scratch after the hot patch arrives.

**Why this priority**: The hot reload loop is only useful for screen iteration if the visible screen re-runs its setup path and drops stale screen-local state without requiring a manual app restart.

**Independent Test**: Start the desktop hot reload workflow, navigate to a screen such as GameScreen, enable `H`, create screen-local state, trigger a hot-patch event, and verify the same screen is rebuilt with fresh initial state. Repeat with `H` disabled and verify the patch does not reinitialize the screen.

**Acceptance Scenarios**:

1. **Given** the desktop hot reload workflow is running, `H` is enabled, and the reviewer is on GameScreen, **When** a hot-patch event is observed, **Then** GameScreen is completely rebuilt, loses screen-local state, and restarts as if the reviewer had just arrived on GameScreen.
2. **Given** `H` is enabled and the reviewer is on DeckScreen, DebugScreen, or another current app screen, **When** a hot-patch event is observed, **Then** the active screen is rebuilt using the same complete reinitialization semantics as GameScreen.
3. **Given** `H` is disabled, **When** a hot-patch event is observed, **Then** the app may apply hot-patched code but must not reinitialize the active screen, reset screen-local state, or restart scene presentation because of that patch.
4. **Given** `H` is enabled and a screen rebuild completes, **When** the DebugHUD is visible, **Then** the DebugHUD reports the current `H` state and the active screen after rebuild.

### Edge Cases

- If the application window size changes, the HUD should scale consistently with the window while staying anchored near the top-left.
- If FPS is hidden, the HUD should not reserve visible FPS text content.
- If the inspector is hidden, no inspector panel should be visible.
- If `F` or `I` is pressed repeatedly, each key press should toggle only its corresponding diagnostic state.
- If terminal logs are used during debugging, they should be scoped to the diagnostic being inspected and should not include secrets, credentials, or unrelated runtime noise.
- If Card UI and DebugHUD are visible at the same time, they should remain visually and conceptually separate developer-facing surfaces.
- If a debug drawing marks a scene area, it should be visually clear enough to support discussion without being mistaken for final art or permanent player-facing UI.
- If a debug drawing becomes obsolete because the scene layout changes, it should be updated, removed, or explicitly documented as stale.
- If the window or browser viewport changes size, DebugHUD, Card UI, inspector offsets, and debug drawings should remain inside or aligned to the aspect-ratio-safe game view.
- If a hot-patch event arrives while `H` is enabled, the active conceptual screen should be fully rebuilt even when the screen currently contains modal, selected-card, deck-editor, debug drawing, animation, or match state.
- If a hot-patch event arrives while `H` is disabled, the app should avoid any screen reinitialization side effect even if the patched code affects systems that would normally run during screen setup.
- If a hot-patch event arrives while navigation between screens is in progress, the rebuild should target the screen that is active after navigation settles, or defer until the active screen identity is stable.
- If the current target does not support desktop hot patch events, the `H` toggle may still be visible as a diagnostic toggle, but it should not imply that browser WebGPU hot reload is supported.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The prototype MUST include one top-left DebugHUD panel adapted from the bevy-jam-1 HUD pattern.
- **FR-001A**: This debugging feature replaces the old no-HUD rule now consolidated into `006-card-bundle` for the final combined app; the DebugHUD MUST be visible by default.
- **FR-002**: The DebugHUD MUST show the prototype title and frame/status text.
- **FR-003**: The DebugHUD MUST show key labels for `W`, `A`, `S`, `D`, `F`, `I`, and `H`.
- **FR-004**: The `F` key MUST toggle FPS visibility in the DebugHUD.
- **FR-005**: The `I` key MUST toggle inspector visibility.
- **FR-006**: The `W`, `A`, `S`, and `D` keys MAY be captured by the DebugHUD/InputSystem for visible key-state feedback, but MUST NOT trigger movement, gameplay, camera, card, selection, scoring, deck behavior, or any other non-DebugHUD behavior in this spec.
- **FR-006A**: Approved DebugHUD keys MUST be classified as toggle or non-toggle: `F`, `I`, and `H` are toggles, while `W`, `A`, `S`, and `D` are non-toggle hold indicators.
- **FR-007**: The HUD MUST use a translucent top-left panel style comparable to the bevy-jam-1 HUD.
- **FR-008**: The HUD MUST scale responsively when the application window size changes.
- **FR-009**: The implementation MUST include automated tests for HUD creation, `F` toggle behavior, `I` toggle behavior, and non-functional `WASD` behavior.
- **FR-010**: The repository MUST include a `RunTests` script that runs the automated test suite.
- **FR-010A**: Debug overlay scripts, tasks, docs, and source-facing labels MUST use `DebugHUD` naming rather than generic `HUD` naming so generic `HUD` remains available for a future user-facing HUD.
- **FR-010B**: The feature MUST include an InputSystem-style key-state capture for approved DebugHUD keys: `W`, `A`, `S`, `D`, `F`, `I`, and `H`.
- **FR-011**: This feature MUST NOT include bevy-jam-1 toast, minimap, reticle, autopilot, reset, shooting, health, score, or gameplay HUD behavior.
- **FR-012**: The DebugHUD MUST support both Windows desktop and browser WebGPU before completion; during implementation iterations, desktop-only builds are acceptable for fast feedback.
- **FR-013**: DebugHUD UI, inspector visibility, approved diagnostic input capture, and key classification MUST be implemented as reusable shared runtime functionality under `bevy/crates/shared`; game-specific code in `bevy/crates/game` may consume these diagnostics but MUST NOT own them.
- **FR-014**: Debugging documentation MUST identify Card UI as a temporary developer/prototype control surface that is separate from DebugHUD and not final player-facing game UI.
- **FR-015**: Implementers MAY use scoped terminal logging for self-debugging, but logs MUST avoid secrets, credentials, and unrelated noisy output.
- **FR-016**: The feature MUST document a self-QA workflow that combines automated tests, terminal/build output review, and manual acceptance checks for rendered developer tooling.
- **FR-017**: Debugging tooling MUST include the concept of debug drawing: runtime visual annotations that mark requested game scene areas for discussion, QA, and implementation alignment.
- **FR-018**: Debug drawings SHOULD remain in the running game until the reviewer asks for removal or replacement, unless they become misleading, stale, or harmful to verification.
- **FR-019**: Debug drawings MUST be treated as temporary scaffolding and MUST NOT be promoted to final player-facing UI or production art without a separate feature decision.
- **FR-020**: Debug drawings MUST be scoped to the requested visual target, such as a hand area, card zone, interaction region, layout boundary, or other concrete game scene area.
- **FR-021**: All visible debugging surfaces, including DebugHUD, Card UI, inspector offsets, and debug drawings, MUST derive placement from the aspect-ratio-safe game view rather than raw window pixels or ad hoc world coordinates.
- **FR-022**: Card UI and scene-specific debug drawing implementation MUST remain under `bevy/crates/game`; shared debugging runtime under `bevy/crates/shared` MUST own only reusable DebugHUD, inspector, and diagnostic input behavior.
- **FR-023**: The DebugHUD MUST include an `H` toggle that controls whether observed hot-patch events reinitialize the active conceptual screen.
- **FR-024**: When `H` is enabled and a hot-patch event is observed, the app MUST completely rebuild the currently active screen, including GameScreen, DeckScreen, DebugScreen, and any other screen hosted under the persistent AppScene.
- **FR-025**: A hot reload screen rebuild MUST lose screen-local state and restart the active screen as if the user had just navigated to that screen.
- **FR-026**: When `H` is disabled and a hot-patch event is observed, the app MUST NOT reinitialize the active screen, reset screen-local state, or restart scene presentation because of that patch.
- **FR-027**: Hot reload screen rebuild state and patch-observation state SHOULD be implemented under `bevy/crates/game` because screen identity and screen-local reset behavior are app-specific; shared DebugHUD input may expose only the reusable `H` toggle state.
- **FR-028**: The `H` toggle MUST affect runtime reinitialization only; `scripts/main/RunAppDesktopHotReload.ps1` remains the approved tool/script entry point defined by `001-project-setup`.

### Key Entities

- **Debugging Tooling**: Developer-facing diagnostics and QA aids that may be rendered on screen, logged to the terminal, or exercised through tests.
- **DebugHUD Panel**: The top-left diagnostic UI surface showing prototype status and key labels.
- **FPS Toggle**: The `F` key behavior that shows or hides FPS text in the HUD.
- **Inspector Toggle**: The `I` key behavior that shows or hides inspector visibility.
- **DebugHUD InputSystem**: The debug-only key-state capture for approved DebugHUD keys: `W`, `A`, `S`, `D`, `F`, `I`, and `H`, including each key's toggle or non-toggle classification.
- **Non-Gameplay WASD Labels**: Visible `W`, `A`, `S`, and `D` key labels that preserve the copied HUD pattern and may show pressed state without adding gameplay controls.
- **Card UI**: A temporary developer/prototype control surface that may be rendered during card work, remains separate from DebugHUD, and is not final player-facing UI.
- **Debug Drawing**: A runtime visual annotation that marks a requested game scene area, such as the hand area, to support shared visual discussion and QA.
- **Hot Reload Screen Reset**: Runtime behavior controlled by the DebugHUD `H` toggle that rebuilds the active conceptual screen after a hot-patch event.
- **Conceptual Screen**: A user-visible app screen such as GameScreen, DeckScreen, DebugScreen, MainMenuScreen, LightningScreen, MatchmakingScreen, or SettingsScreen, implemented as AppScene plus the active sub-screen view/root.
- **Screen-Local State**: Runtime state owned by the active conceptual screen, including modal state, selected cards, generated screen entities, view roots, animations, temporary match state, and other non-persistent presentation state.
- **Hot-Patch Event**: A runtime notification that desktop hot-patched code has been applied or observed by the running app.
- **Terminal Self-Logging**: Scoped diagnostic log output used by implementers to inspect runtime behavior while keeping output safe and focused.
- **RunTests Script**: A repeatable project script for running the automated test suite.
- **DebugHUD Naming**: The canonical naming convention for debug overlay scripts, tasks, docs, and source-facing labels.
- **Shared Debug Runtime**: Reusable DebugHUD, inspector, and diagnostic input state owned by `bevy/crates/shared`.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 100% of launch checks, one top-left DebugHUD panel is visible.
- **SC-002**: In HUD content checks, the HUD includes title/status text and labels for `W`, `A`, `S`, `D`, `F`, `I`, and `H`.
- **SC-003**: In toggle tests, pressing `F` changes FPS visibility state on each press and does not change inspector visibility.
- **SC-004**: In toggle tests, pressing `I` changes inspector visibility state on each press and does not change FPS visibility.
- **SC-005**: In keyboard behavior tests, pressing `W`, `A`, `S`, and `D` may update DebugHUD key-state feedback but produces no card movement, gameplay action, FPS toggle, inspector toggle, camera behavior, or non-DebugHUD behavior.
- **SC-005A**: In key classification tests, `F`, `I`, and `H` behave as toggles while `W`, `A`, `S`, and `D` behave as non-toggle hold indicators.
- **SC-006**: The `RunTests` script completes the automated test suite from the repository root.
- **SC-006A**: Review of scripts, tasks, docs, and source-facing labels finds `DebugHUD` naming for debug overlay-specific items and no generic debug overlay item named only `HUD`.
- **SC-007**: Reviewers identify no toast, minimap, reticle, autopilot, reset, shooting, health, score, or gameplay HUD behavior in this feature.
- **SC-008**: Final acceptance verification passes for the DebugHUD on Windows desktop and browser WebGPU, or any blocked target is documented with the exact blocker.
- **SC-009**: Debugging documentation clearly separates DebugHUD, inspector, Card UI, terminal logs, and tests as distinct developer-facing tools.
- **SC-010**: Self-QA notes or handoff output include the repeatable test command used and any blocked manual checks.
- **SC-011**: When a debug drawing is requested for a concrete scene area, the running app shows a clear temporary mark around that area in 100% of accepted debug-drawing checks.
- **SC-012**: Review of debug drawings finds that each one is identified as temporary and either still useful, explicitly requested to remain, removed, or replaced with production UI/art.
- **SC-013**: In desktop and browser layout checks, DebugHUD, Card UI, inspector offsets, and debug drawings remain inside or aligned to the aspect-ratio-safe game view after viewport resize.
- **SC-014**: In desktop hot reload checks with `H` enabled, hot-patch events rebuild the active conceptual screen and reset screen-local state in 100% of checked screens.
- **SC-015**: In desktop hot reload checks with `H` disabled, hot-patch events do not reinitialize the active screen or reset screen-local state in 100% of checked screens.
- **SC-016**: Manual or automated checks cover at least GameScreen, DeckScreen, and DebugScreen hot reload reset behavior, or record the exact blocker for any unchecked screen.

## Assumptions

- The HUD is reviewer-facing diagnostic UI, not player-facing gameplay UI.
- This feature is broader than one rendered HUD: it owns developer-facing debugging workflows that may appear on screen, in terminal logs, or in automated test output.
- The bevy-jam-1 HUD is the visual and behavioral reference for the panel, title/status text, key labels, and `F`/`I` toggle pattern.
- `WASD` remains visible to preserve the copied HUD pattern and may provide DebugHUD-only pressed feedback; it is intentionally non-functional for gameplay, card, camera, and other non-DebugHUD systems in this card POC.
- Card UI is temporary developer/prototype UI and remains separate from DebugHUD.
- Debug drawings are temporary visual annotations for collaboration and QA; they are expected to remain until removal is requested, but they are not final game art or player-facing UI.
- Visible debugging surfaces follow the same aspect-ratio-safe layout rules as other runtime overlays and presentation elements.
- The desktop hot reload tooling entry point remains specified by `001-project-setup`; this feature specifies only the app's runtime response after a hot-patch event is observed.
- Hot reload screen reset is for development iteration and does not preserve screen-local state by design when `H` is enabled.
- Terminal logs are acceptable for implementer self-debugging when scoped, safe, and paired with repeatable tests.
- Toast, minimap, reticle, and gameplay HUD systems are intentionally excluded from this feature.
- Desktop-only builds are acceptable while iterating, but final completion requires Windows desktop and browser WebGPU verification.
- `bevy/crates/shared` owns reusable DebugHUD and diagnostic input behavior; `bevy/crates/game` owns card-specific behavior that may be inspected by those diagnostics.
