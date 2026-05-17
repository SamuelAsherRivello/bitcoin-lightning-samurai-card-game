# Feature Specification: Shared AppScene 3D Camera

**Feature Branch**: `[024-shared-app-camera]`  
**Created**: 2026-05-18  
**Status**: Draft  
**Input**: User description: "Remove all 2D cameras. Have one 3D camera in the AppScene that is used for all screens. All screens have a mix of 2D and/or 3D and we want one 3D camera locked in one position, rotation, scale, that is used for all. The end results will look the same for the user. Analyze how to do it, any challenges, and solutions for those challenges to meet the needs."

## User Scenarios & Testing

### User Story 1 - Single Shared Runtime Camera (Priority: P1)

As a developer, I want the app runtime to own exactly one locked 3D camera under `AppScene` so every screen renders through the same stable view.

**Why this priority**: This is the architectural requirement that all later parity work depends on.

**Independent Test**: Launch the app and inspect runtime ECS state to confirm there is one active camera entity, it has `Camera3d`, it belongs to `AppScene`, and there are no runtime `Camera2d` entities.

**Acceptance Scenarios**:

1. **Given** the app starts on any initial view, **When** camera entities are inspected, **Then** exactly one active camera exists and it is the locked AppScene 3D camera.
2. **Given** the user switches among Game, Deck, Debug, Main Menu, Lightning, Matchmaking, and Settings views, **When** the active view changes, **Then** the same AppScene 3D camera remains active and unchanged.

---

### User Story 2 - Visual Parity Across Screens (Priority: P2)

As a player, I want Game, Deck, Debug, and meta screens to look the same after the camera consolidation so the change is not visible as a gameplay or UI regression.

**Why this priority**: The camera architecture change is only acceptable if the player-facing composition remains unchanged.

**Independent Test**: Compare desktop and browser screenshots before and after the migration for each screen and verify visible layout, scale, safe-area framing, and layering match the existing approved behavior.

**Acceptance Scenarios**:

1. **Given** GameScene is active, **When** the screen is rendered, **Then** the world background, three locations, hand cards, controls, DebugHUD, and transition fade appear in the same positions and stacking order as before.
2. **Given** DeckScene is active, **When** grids, card previews, top navigation, validation prompts, and selected-card menus are rendered, **Then** they appear in the same positions and stacking order as before.
3. **Given** DebugScene or a meta screen is active, **When** the screen is rendered, **Then** navigation, panels, text, and controls remain inside the aspect-ratio-safe view.

---

### User Story 3 - Camera-Independent Screen Transitions and Overlays (Priority: P3)

As a developer, I want transition overlays, modals, point labels, debug drawings, and HUD content to no longer depend on per-screen 2D cameras so future screens can mix UI and 3D without creating more cameras.

**Why this priority**: The current overlay and UI targeting logic is tightly coupled to `Camera2d`, so this cleanup is needed to keep the new architecture maintainable.

**Independent Test**: Trigger startup fade, screen transitions, modal prompts, card point labels, and debug drawing with only the shared 3D camera active.

**Acceptance Scenarios**:

1. **Given** startup or screen-switch transition is active, **When** the fade overlay renders, **Then** it fully covers every screen through the shared 3D camera.
2. **Given** a modal, selected-card menu, or DebugHUD is visible, **When** the active view changes or the viewport resizes, **Then** the overlay remains correctly targeted and inside the safe-area layout without a 2D camera.
3. **Given** card point labels or debug drawing are visible, **When** cards or debug shapes are rendered, **Then** labels and shapes align with their associated 3D or safe-area positions through the shared camera.

### Edge Cases

| Edge Case | Expected Handling |
| --- | --- |
| Bevy UI cannot target a `Camera3d` with required parity | Replace affected UI surfaces with 3D-facing panels/text or a documented compatibility layer that still preserves the no-runtime-`Camera2d` requirement. |
| Fullscreen or browser resize changes the safe viewport | Recalculate camera viewport and all screen layout from the aspect-ratio-safe game view. |
| Transition starts during a view switch | The 3D fade plane or shared-camera overlay remains closest to the camera and fully masks the switch. |
| Render layers previously depended on camera order | Replace camera order with explicit Z-depth, render layers compatible with the shared camera, and `GlobalZIndex` where Bevy UI remains in use. |
| Existing tests expect per-scene cameras | Update tests to assert one shared AppScene 3D camera and parity of screen-owned entities rather than per-view camera ownership. |

## Requirements

### Functional Requirements

- **FR-001**: The runtime MUST spawn exactly one active app camera for normal app rendering.
- **FR-002**: The app camera MUST be a 3D camera owned by `AppScene`.
- **FR-003**: The app camera position, rotation, scale, projection defaults, and viewport behavior MUST be centralized and remain stable across view switches.
- **FR-004**: The runtime MUST NOT spawn `Camera2d` entities for GameScene, DeckScene, DebugScene, meta screens, transition overlays, DebugHUD, point labels, modals, or selected-card menus.
- **FR-005**: GameScene, DeckScene, DebugScene, and meta screens MUST render through the shared AppScene 3D camera.
- **FR-006**: Existing user-visible layout, scale, safe-area placement, and layering MUST remain visually unchanged after the migration.
- **FR-007**: Screen transitions MUST continue to fade over all screen content during startup and view switches.
- **FR-008**: Bevy UI roots, if retained, MUST target the shared AppScene 3D camera or use a camera-independent default path that does not require per-screen 2D cameras.
- **FR-009**: 3D card rendering, world backgrounds, location surfaces, card point labels, debug drawing, modals, and HUD overlays MUST use deterministic ordering through shared-camera-compatible Z-depth, render layers, or UI z-index.
- **FR-010**: Pointer picking, card selection, gesture placement, and debug drawing coordinate conversions MUST use the shared AppScene camera and the active view context.
- **FR-011**: View switching MUST not create, despawn, activate, or retarget screen-owned cameras.
- **FR-012**: Camera viewport constraints MUST be implemented once for the shared AppScene camera and must preserve the existing aspect-ratio-safe view behavior.
- **FR-013**: Runtime tests MUST cover single-camera ownership, absence of 2D cameras, transition overlay coverage, and screen rendering parity expectations.
- **FR-014**: Desktop and browser verification MUST be recorded before the feature is considered complete.

### Key Entities

| Entity | Description |
| --- | --- |
| `AppSceneCamera` | The single shared 3D camera entity owned by persistent `AppScene`. |
| `AppCameraModel` | Centralized configuration for locked camera transform, projection, viewport, and safe-area behavior. |
| `SharedCameraRenderLayering` | The ordering policy that replaces per-camera render order with Z-depth, render layers, and UI z-index. |
| `SharedCameraOverlayView` | Presentation layer for transition fades, modals, DebugHUD, and screen overlays through the shared camera. |

## Success Criteria

### Measurable Outcomes

- **SC-001**: Runtime ECS inspection finds one active camera entity and zero `Camera2d` entities during normal app operation.
- **SC-002**: Switching through all current views leaves the shared AppScene camera entity unchanged.
- **SC-003**: Automated runtime tests pass for GameScene, DeckScene, DebugScene, meta screens, transitions, modals, card selection, and debug drawing.
- **SC-004**: Desktop visual verification confirms no visible layout or layering regressions across current screens.
- **SC-005**: Browser WebGPU visual verification confirms no visible layout or layering regressions across current screens.
- **SC-006**: The updated screen hierarchy for GameScreen reports the AppScene camera once and reports no view-owned cameras.

## Assumptions

| Assumption | Rationale |
| --- | --- |
| Bevy UI may be retained if it can target or render through the shared 3D camera | This minimizes churn and preserves existing button/text behavior. |
| A 3D fade plane is acceptable for transitions if Bevy UI overlay targeting cannot satisfy the no-2D-camera rule | It keeps the user-visible fade identical while removing the transition camera. |
| Existing safe-area constants remain the virtual layout source of truth | This preserves desktop and browser composition. |
| No gameplay rules, deck persistence, card data, or theme assets change | The feature is rendering architecture only. |
| The active screen-transition feature remains behaviorally intact but may need implementation changes | Transitions currently use a dedicated 2D camera and must be adapted. |
