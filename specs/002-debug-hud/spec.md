# Feature Specification: Debug HUD

**Feature Branch**: `001-card-inspection-poc`  
**Created**: 2026-05-09  
**Status**: Draft  
**Input**: User description: "Copy how the HUD works in https://github.com/SamuelAsherRivello/bevy-jam-1. Keep the main HUD panel, title/status text, and key legend. WASD should be visible but do nothing. Keep F and I only and bring in that functionality. Do not include toast, minimap, reticle, or other HUD-related systems. Add tests and a RunTests script."

## Clarifications

### Session 2026-05-09

- Q: Which bevy-jam-1 HUD content should be kept? -> A: Keep the top-left HUD panel, title/status text, key legend with non-functional `WASD`, and functional `F` and `I`; exclude toast, minimap, reticle, and other listed HUD systems.
- Q: How should `002-debug-hud` relate to the no-HUD rule in `001-card-inspection-poc` when both are implemented? -> A: `002-debug-hud` replaces the no-HUD rule in `001`; the final app should show the debug HUD by default.
- Q: Should `001-card-inspection-poc` and `002-debug-hud` explicitly require both Windows desktop and browser WebGPU verification? -> A: Both specs require final Windows desktop and browser WebGPU verification; iterative builds may target desktop only.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Show Debug HUD Panel (Priority: P1)

A reviewer sees a top-left debug HUD panel adapted from the bevy-jam-1 HUD pattern while running the card-inspection prototype.

**Why this priority**: The HUD provides lightweight review diagnostics without adding gameplay UI.

**Independent Test**: Launch the prototype and verify that a translucent top-left HUD panel appears with title/status text and the expected key labels.

**Acceptance Scenarios**:

1. **Given** the prototype is running, **When** the reviewer observes the top-left corner, **Then** a translucent debug HUD panel is visible by default.
2. **Given** the HUD is visible, **When** the reviewer reads it, **Then** it shows the prototype title, frame/status text, and key labels for `W`, `A`, `S`, `D`, `F`, and `I`.

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

A reviewer sees `W`, `A`, `S`, and `D` in the HUD key legend, but those keys do not trigger gameplay, camera, card movement, or selection behavior.

**Why this priority**: The visual pattern is copied from bevy-jam-1, but the card POC must not inherit aircraft movement or gameplay controls.

**Independent Test**: Press `W`, `A`, `S`, and `D` while the prototype is running and verify that no gameplay or card behavior changes.

**Acceptance Scenarios**:

1. **Given** the prototype is running, **When** the reviewer presses `W`, `A`, `S`, or `D`, **Then** the card does not move, rotate because of those keys, select, score, shoot, or trigger gameplay.
2. **Given** `WASD` key labels are visible, **When** the reviewer presses those keys, **Then** FPS and inspector visibility remain unchanged.

### Edge Cases

- If the application window size changes, the HUD should scale consistently with the window while staying anchored near the top-left.
- If FPS is hidden, the HUD should not reserve visible FPS text content.
- If the inspector is hidden, no inspector panel should be visible.
- If `F` or `I` is pressed repeatedly, each key press should toggle only its corresponding diagnostic state.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The prototype MUST include one top-left debug HUD panel adapted from the bevy-jam-1 HUD pattern.
- **FR-001A**: This debug HUD feature replaces the no-HUD rule from `001-card-inspection-poc` for the final combined app; the debug HUD MUST be visible by default.
- **FR-002**: The debug HUD MUST show the prototype title and frame/status text.
- **FR-003**: The debug HUD MUST show key labels for `W`, `A`, `S`, `D`, `F`, and `I`.
- **FR-004**: The `F` key MUST toggle FPS visibility in the debug HUD.
- **FR-005**: The `I` key MUST toggle inspector visibility.
- **FR-006**: The `W`, `A`, `S`, and `D` keys MUST NOT trigger movement, gameplay, camera, card, selection, scoring, or deck behavior.
- **FR-007**: The HUD MUST use a translucent top-left panel style comparable to the bevy-jam-1 HUD.
- **FR-008**: The HUD MUST scale responsively when the application window size changes.
- **FR-009**: The implementation MUST include automated tests for HUD creation, `F` toggle behavior, `I` toggle behavior, and non-functional `WASD` behavior.
- **FR-010**: The repository MUST include a `RunTests` script that runs the automated test suite.
- **FR-011**: This feature MUST NOT include bevy-jam-1 toast, minimap, reticle, autopilot, reset, shooting, health, score, or gameplay HUD behavior.
- **FR-012**: The debug HUD MUST support both Windows desktop and browser WebGPU before completion; during implementation iterations, desktop-only builds are acceptable for fast feedback.

### Key Entities

- **Debug HUD Panel**: The top-left diagnostic UI surface showing prototype status and key labels.
- **FPS Toggle**: The `F` key behavior that shows or hides FPS text in the HUD.
- **Inspector Toggle**: The `I` key behavior that shows or hides inspector visibility.
- **Non-Functional WASD Labels**: Visible `W`, `A`, `S`, and `D` key labels that preserve the copied HUD pattern without adding gameplay controls.
- **RunTests Script**: A repeatable project script for running the automated test suite.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 100% of launch checks, one top-left debug HUD panel is visible.
- **SC-002**: In HUD content checks, the HUD includes title/status text and labels for `W`, `A`, `S`, `D`, `F`, and `I`.
- **SC-003**: In toggle tests, pressing `F` changes FPS visibility state on each press and does not change inspector visibility.
- **SC-004**: In toggle tests, pressing `I` changes inspector visibility state on each press and does not change FPS visibility.
- **SC-005**: In keyboard behavior tests, pressing `W`, `A`, `S`, and `D` produces no card movement, gameplay action, FPS toggle, or inspector toggle.
- **SC-006**: The `RunTests` script completes the automated test suite from the repository root.
- **SC-007**: Reviewers identify no toast, minimap, reticle, autopilot, reset, shooting, health, score, or gameplay HUD behavior in this feature.
- **SC-008**: Final acceptance verification passes for the debug HUD on Windows desktop and browser WebGPU, or any blocked target is documented with the exact blocker.

## Assumptions

- The HUD is reviewer-facing diagnostic UI, not player-facing gameplay UI.
- The bevy-jam-1 HUD is the visual and behavioral reference for the panel, title/status text, key labels, and `F`/`I` toggle pattern.
- `WASD` remains visible only to preserve the copied HUD pattern; it is intentionally non-functional in this card POC.
- Toast, minimap, reticle, and gameplay HUD systems are intentionally excluded from this feature.
- Desktop-only builds are acceptable while iterating, but final completion requires Windows desktop and browser WebGPU verification.
