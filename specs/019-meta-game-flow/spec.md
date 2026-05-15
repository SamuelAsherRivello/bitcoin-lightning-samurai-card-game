# Feature Specification: Meta Game Flow

**Feature Branch**: `019-meta-game-flow`  
**Created**: 2026-05-14  
**Status**: Draft  
**Input**: User description: "Open to the main screen, replace mockup-only visuals with existing or new graphics, route top navigation across Play Game, My Decks, Settings, and Debug, add Lightning login placeholder, fake matchmaking, settings persistence, and validate DeckScreen/debug behavior."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Start From Main Page (Priority: P1)

The player opens the app on the main screen, sees the reusable top navigation in the requested order, can open Lightning login, and can start a fake matchmaking flow that lands in the existing GameScreen.

**Why this priority**: This defines the new app entry point and the critical path into gameplay.

**Independent Test**: Launch the app, verify MainMenuScreen is the active screen, click Start Game, observe Searching for one second, Player 02 for one second, then GameScreen after two seconds total.

**Acceptance Scenarios**:

1. **Given** the app has just started, **When** the first frame renders, **Then** MainMenuScreen is active with Play Game selected and the Start Game button enabled.
2. **Given** the player is on MainMenuScreen, **When** Start Game is clicked, **Then** MatchmakingScreen shows Player 01 versus Searching for one second.
3. **Given** MatchmakingScreen has shown Searching for one second, **When** the fake opponent is found, **Then** it shows Player 01 versus Player 02 for one second before loading GameScreen.
4. **Given** the player is on MainMenuScreen, **When** Login with Lightning is clicked, **Then** LightningScreen is shown with a placeholder QR code, Back, and Learn About Lightning.
5. **Given** the player clicks Learn About Lightning, **When** the action is handled, **Then** the system opens a browser page about Bitcoin Lightning nodes without requiring in-game authentication.

---

### User Story 2 - Navigate And Refresh Screens (Priority: P2)

The player can use the top navigation from every meta-game screen, including re-clicking the already selected destination to refresh that screen.

**Why this priority**: Top navigation is the common shell for the meta game and provides a recovery path if a screen gets into a bad state.

**Independent Test**: Click each top navigation destination from MainMenuScreen, SettingsScreen, DeckScreen, and DebugScreen; re-click the selected button and verify the same screen reloads instead of ignoring input.

**Acceptance Scenarios**:

1. **Given** any meta-game screen is active, **When** the player clicks My Decks, **Then** DeckScreen loads with My Decks selected.
2. **Given** SettingsScreen is active, **When** Settings is clicked again, **Then** SettingsScreen reloads and keeps saved settings.
3. **Given** DeckScreen is active, **When** My Decks is clicked again, **Then** DeckScreen reloads and keeps persisted deck data.
4. **Given** DebugScreen is active, **When** Debug is clicked again, **Then** DebugScreen reloads and shows the debug card plus Card UI.
5. **Given** GameScreen is active, **When** Play Game is clicked, **Then** MainMenuScreen loads rather than reloading the live game.

---

### User Story 3 - Configure Match Before Game (Priority: P3)

The player can configure CPU Brain, match mode, SFX, and music from SettingsScreen before starting a match, and all settings save to disk.

**Why this priority**: Match mode currently lives on GameScreen and must move into pre-game configuration.

**Independent Test**: Open SettingsScreen, click each setting, restart or reload the screen, and verify persisted values are restored.

**Acceptance Scenarios**:

1. **Given** SettingsScreen is active, **When** CPU AI Brain is clicked, **Then** it remains Level 1 and still records the setting as operational.
2. **Given** SettingsScreen is active, **When** Mode is clicked, **Then** it toggles between Human versus CPU and CPU versus CPU.
3. **Given** SettingsScreen is active, **When** Toggle SFX or Toggle Music is clicked, **Then** the selected audio setting flips and saves to disk.
4. **Given** GameScreen is active, **When** controls render, **Then** match mode is no longer controlled from GameScreen.

---

### User Story 4 - Preserve Deck And Debug Workflows (Priority: P4)

The DeckScreen remains complete, Shop is clickable but non-functional, card selection opens the fullscreen overlay from either side, and DebugScreen owns the current Card UI plus card preview.

**Why this priority**: Existing deck and debug work must remain intact while the app shell changes.

**Independent Test**: Open DeckScreen, select cards in the deck and library panels, verify the modal card appears centered with actions; click Shop and verify no shop layout is introduced; open DebugScreen and verify Card UI is available there.

**Acceptance Scenarios**:

1. **Given** DeckScreen editor is active, **When** a card in the left deck panel is clicked, **Then** the selected card zooms into the center overlay and shows move/transfer/back actions.
2. **Given** DeckScreen editor is active, **When** a card in the right library panel is clicked, **Then** the selected card zooms into the center overlay and shows move/transfer/back actions.
3. **Given** DeckScreen editor is active, **When** Shop is clicked, **Then** no shop purchasing layout appears and the app remains stable.
4. **Given** DebugScreen is active, **When** the screen renders, **Then** it shows the debug card and existing Card UI controls.

### Edge Cases

- If a top navigation button is pressed while a modal overlay owns input, the modal keeps priority and lower screen actions are blocked.
- If Settings persistence fails, the screen still updates in memory and logs a warning rather than blocking navigation.
- If Learn About Lightning cannot open a browser on the current platform, the in-game flow remains on LightningScreen.
- If MatchmakingScreen is reloaded during either timer phase, the fake matchmaking timer restarts from the beginning.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST start on MainMenuScreen instead of GameScreen.
- **FR-002**: System MUST treat screens colloquially as the user-facing surface while preserving AppScene as the persistent app-level scene plus one active view/scene.
- **FR-003**: System MUST show top navigation in this exact order: Play Game, My Decks, divider, Settings, Debug.
- **FR-004**: System MUST keep every visible top navigation button clickable, including the currently selected destination.
- **FR-005**: System MUST route Play Game from GameScreen back to MainMenuScreen.
- **FR-006**: System MUST keep every Lightning-labeled button clickable but non-functional except Lightning login navigation and Learn About Lightning browser opening.
- **FR-007**: System MUST implement fake matchmaking with a one-second Searching phase and a one-second Player 02 found phase before GameScreen.
- **FR-008**: System MUST move match mode selection from GameScreen to SettingsScreen.
- **FR-009**: System MUST persist CPU Brain level, match mode, SFX enabled, and music enabled to disk.
- **FR-010**: System MUST keep CPU Brain limited to Level 1 for this pass while allowing the control to be clicked.
- **FR-011**: System MUST keep Shop clickable on DeckScreen without adding functional shop purchase layout.
- **FR-012**: System MUST preserve DeckScreen fullscreen card overlay behavior for both deck and library card clicks.
- **FR-013**: System MUST make DebugScreen the place where the current debug card preview and Card UI controls appear.
- **FR-014**: System MUST keep visible UI inside the aspect-ratio-safe content area.

### Key Entities *(include if feature involves data)*

- **Active Screen/View**: User-facing screen identity mapped onto the persistent AppScene plus a child presentation view.
- **Top Navigation Destination**: Play Game, My Decks, Settings, Debug, including selected state and reload behavior.
- **Matchmaking State**: Fake search phase, found phase, elapsed time, and automatic transition to GameScreen.
- **Meta Settings**: CPU Brain level, match mode, SFX enabled flag, and music enabled flag persisted to disk.
- **Lightning Login State**: Placeholder screen with QR code, Back, and Learn About Lightning action.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Fresh app launch shows MainMenuScreen before GameScreen in both desktop and browser builds.
- **SC-002**: Start Game reaches GameScreen after 2.0 seconds of fake matchmaking timing.
- **SC-003**: All four settings survive a screen reload and app restart when persistence is available.
- **SC-004**: Top navigation button clicks route or reload every screen without dead buttons.
- **SC-005**: Existing DeckScreen modal tests still pass, and new tests cover left/right card overlay selection and Shop non-functionality.

## Assumptions

- Existing theme card, location, and world graphics are acceptable replacements for mockup-only card/location/world visuals.
- The QR code can be a generated placeholder graphic or UI-built placeholder because real Lightning integration is out of scope.
- The Learn About Lightning target can be any stable public page about Bitcoin Lightning nodes for this pass.
- Settings persistence uses the same local JSON persistence approach as existing deck and match-mode storage.
- New Rust files and folders follow typical lowercase Rust naming conventions.
- Bevy crate folders, representative files, asset folders, and Rust coding standards use `bevy/crates/template-crate` as the proper local reference.
- Changed Bevy runtime files use one primary concept per file, Scene/Model/View naming, and HUMAN/AI purpose comments.
- Visible 2D and 3D positions derive from the aspect-ratio-safe game view.
