# Feature Specification: Basic Setup

**Feature Branch**: `001-basic-setup`  
**Created**: 2026-05-09  
**Status**: Draft  
**Input**: User description: "Basic setup includes repeatable build and run scripts, VS Code task support, an 800x600 desktop window, and remembering the window's last size, x/y position, and screen so the app reopens where the reviewer left it."

## Clarifications

### Session 2026-05-09

- Q: How should saved desktop window placement restore across launches? -> A: Restore exact x/y and size on the same screen, including two-screen setups; fall back only if that saved screen is unavailable or the saved position would open off-screen.
- Q: What fallback should the app use when saved placement is invalid or off-screen? -> A: Open centered on the primary screen at 800x600.
- Q: When should desktop window placement be saved? -> A: Save placement only when the window closes normally.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Run Repeatable Project Scripts (Priority: P1)

A developer or reviewer can build, test, and run the project through documented repository scripts instead of ad hoc commands.

**Why this priority**: Reliable scripts are the foundation for every later feature and keep local verification repeatable.

**Independent Test**: From the repository root, run the documented build, test, and desktop run entry points and verify each one starts from the expected working directory.

**Acceptance Scenarios**:

1. **Given** the repository is checked out, **When** the reviewer runs the build script, **Then** the desktop project builds successfully or reports a clear build error.
2. **Given** the repository is checked out, **When** the reviewer runs the test script, **Then** the automated test suite runs from the repository root.
3. **Given** the repository is checked out in VS Code, **When** the reviewer starts the desktop run task, **Then** command output appears in the VS Code integrated terminal.

---

### User Story 2 - Open a Predictable Desktop Window (Priority: P1)

A reviewer launches the desktop app and sees a normal resizable window at the project-approved default size.

**Why this priority**: A predictable desktop surface is required before camera, DebugHUD, or card inspection behavior can be reviewed consistently.

**Independent Test**: Launch the app without a saved placement and verify the initial window is 800x600.

**Acceptance Scenarios**:

1. **Given** no saved window placement exists, **When** the reviewer launches the desktop app, **Then** the window opens at 800x600.
2. **Given** the window is visible, **When** the reviewer resizes it, **Then** the app remains usable and does not lose the ability to close normally.

---

### User Story 3 - Restore Last Window Placement (Priority: P2)

A reviewer moves or resizes the desktop window, closes the app, and sees the app reopen on the same screen at the remembered placement.

**Why this priority**: Reopening in the previous location reduces friction during repeated local review sessions.

**Independent Test**: Move the window to a known screen and position, resize it, close the app, reopen it, and verify the window returns to the saved screen and placement.

**Acceptance Scenarios**:

1. **Given** the reviewer has moved or resized the app window to a screen and position, **When** the reviewer closes the app normally, **Then** the app records the latest window size, x/y position, and screen identity as local runtime state.
2. **Given** saved placement exists and the same screen is available, **When** the reviewer launches the app again, **Then** the app opens on that screen at the saved x/y position and size, including when the saved screen is one of two connected screens.
3. **Given** saved placement exists but the previous screen is unavailable, invalid, or off-screen, **When** the reviewer launches the app, **Then** the app opens centered on the primary screen at 800x600.

### Edge Cases

- If saved placement data is missing, the app should use the default 800x600 desktop window.
- If saved placement data is invalid or unreadable, the app should ignore it and continue launching centered on the primary screen at 800x600.
- If the saved screen is disconnected, the app should fall back to the primary screen centered at 800x600.
- If the window is closed without being moved, the app should still preserve the best known placement on normal close.
- If the app runs in a browser target, desktop window placement persistence should not block browser startup.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The repository MUST provide repeatable scripts for building, testing, and running the desktop app.
- **FR-001A**: The repository MUST keep repeatable scripts under root `scripts`.
- **FR-001B**: The repository SHOULD provide helper scripts for dependency checks and stopping project-local app/build processes.
- **FR-002**: The repository MUST provide VS Code task entries for build, test, and desktop run workflows.
- **FR-003**: The desktop run workflow MUST show command output in the VS Code integrated terminal when started through the VS Code task.
- **FR-004**: The desktop app MUST default to an 800x600 window when no valid saved placement exists.
- **FR-005**: The desktop app MUST remember the latest window x/y position, size, and screen identity when the window closes.
- **FR-005A**: The desktop app MUST save placement only on normal window close, not continuously during every move or resize.
- **FR-006**: The desktop app MUST restore the remembered x/y position, size, and screen identity on the next desktop launch when that screen is available, including two-screen desktop setups.
- **FR-007**: The desktop app MUST open centered on the primary screen at 800x600 when the remembered screen or position is unavailable, invalid, or off-screen.
- **FR-008**: Window placement state MUST be local runtime state and MUST NOT be committed as source content.
- **FR-009**: Build and test scripts MUST work from the repository root.
- **FR-010**: This feature MUST NOT include card rendering, DebugHUD controls, gameplay, or card-inspection interaction behavior.

### Key Entities

- **Project Script**: A repository entry point for build, test, or run workflows.
- **Desktop Window Placement**: The remembered window x/y position, size, and screen identity from the last desktop session.
- **Screen Identity**: The screen information used to reopen the app on the same display when possible.
- **Local Runtime State**: Generated local data used by the app but excluded from version control.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: The build script completes the desktop build workflow from the repository root.
- **SC-002**: The test script completes the automated test suite from the repository root.
- **SC-003**: In 100% of first-launch checks without saved placement, the desktop window opens at 800x600.
- **SC-004**: In placement restore checks on either screen in a two-screen setup, the app reopens within 20 physical pixels of the saved x/y position and restores the saved window size.
- **SC-005**: In disconnected-screen, invalid-data, and off-screen placement checks, the app opens centered on the primary screen at 800x600 instead of restoring off-screen.
- **SC-006**: VS Code desktop run task output appears in the integrated terminal in 100% of task-launch checks.

## Assumptions

- The primary development environment is Windows desktop with VS Code.
- Window placement persistence is reviewer convenience state, not gameplay state.
- Local runtime state may live under an ignored generated-output location in the repository.
- Browser WebGPU remains supported by later features, but desktop placement restore only applies to desktop windows.
