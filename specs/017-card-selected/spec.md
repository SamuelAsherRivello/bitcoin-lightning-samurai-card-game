# Feature Specification: Card Selected Modal Backdrop

**Feature Branch**: `017-card-selected`  
**Created**: 2026-05-13  
**Status**: Draft  
**Input**: User description: "Any card that is cardfront and not currently being dragged or moved is selectable. This includes human cards, CPU cards, near player, far player, all of them on any screen. Add a Selectable state if it doesn't exist already so it's clear. Clicking a selectable card selects it; dragging is not clicking. Selected cards already scale to the center. Add a fullscreen black modal background behind the selected card and in front of everything else, fade 0% to 50% opacity over the selected-card scale-up duration, block all lower clicks/hovers/interactions, and clicking the modal ends selection. Do not end selection by clicking the card itself. Fix selected card depth so point-view white numbers behind it no longer render on top."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Select Any Front-Facing Card (Priority: P1)

Players can click any front-facing, stationary card in any view to inspect it at the center of the screen.

**Why this priority**: This defines the core selection behavior and broadens selection beyond the current local hand/local slot focus.

**Independent Test**: Start the app, show front-facing human and CPU cards in hand or placed locations, click each stationary card without dragging, and confirm the clicked card becomes the selected inspection card.

**Acceptance Scenarios**:

1. **Given** a front-facing human hand card is stationary, **When** the user clicks and releases without crossing the drag threshold, **Then** that card is selected and scales to the center.
2. **Given** a front-facing CPU, far-player, near-player, hand, or location card is stationary, **When** the user clicks and releases without crossing the drag threshold, **Then** that card is selected and scales to the center.
3. **Given** a card is being dragged, returning, moving, dealing, revealing, flipping, or otherwise animated as movement, **When** the user clicks or releases over it, **Then** it is not treated as selectable.
4. **Given** a card back is visible instead of its front face, **When** the user clicks it, **Then** it is not selected.

---

### User Story 2 - Modal Backdrop During Selection (Priority: P1)

When a card is selected, the rest of the screen darkens behind it while the selected card remains visually on top.

**Why this priority**: The selected card must read as a modal inspection target and lower scene/UI elements must not compete visually.

**Independent Test**: Select a card and observe a fullscreen black layer fade from 0% opacity to 50% opacity while the card scales up.

**Acceptance Scenarios**:

1. **Given** a selectable card has just been clicked, **When** the card begins scaling to selected inspection size, **Then** a fullscreen black modal background appears behind the card and fades from 0% to 50% opacity over 0.5 seconds.
2. **Given** the selected card is fully inspected, **When** the user views the screen, **Then** all elements behind the card are darkened by the modal layer and the selected card remains undimmed.
3. **Given** the selected card overlaps card point numbers or badges behind it, **When** the selected card is in front, **Then** point-view text and numbers from other cards do not render over the selected card.

---

### User Story 3 - Modal Blocks Lower Interaction And Dismisses Selection (Priority: P1)

While a card is selected, lower screen controls and card hover/click/drag targets are blocked, and clicking the backdrop dismisses selection.

**Why this priority**: A modal selection state must be unambiguous and prevent accidental game actions beneath the inspection card.

**Independent Test**: Select a card, click or hover lower buttons/cards/locations through the dimmed area, and verify nothing beneath reacts; click the modal background and verify the selected card returns.

**Acceptance Scenarios**:

1. **Given** a card is selected, **When** the pointer is over lower UI controls, lower cards, or location drop targets, **Then** those lower elements do not hover, click, drag, or otherwise interact.
2. **Given** a card is selected, **When** the user clicks the modal backdrop outside the selected card, **Then** selection ends and the card returns to its source.
3. **Given** a card is selected, **When** the user clicks the selected card itself, **Then** selection does not end from that card click.

### Edge Cases

| Edge Case | Expected Behavior |
| --- | --- |
| Press starts on a selectable card and crosses the drag threshold | Treat as drag when the card is draggable; do not open selected inspection. |
| Card becomes hidden, locked, or removed while selected | Dismiss selection or return to source without leaving the modal backdrop visible. |
| Selected card belongs to a CPU/far-player entity | It can be inspected if its front face is currently visible and it is not moving. |
| Multiple cards overlap under the pointer | The visually topmost selectable front-facing card wins. |
| Click occurs on modal backdrop while pointer is also over lower UI | Modal consumes the click; lower UI receives nothing. |

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST expose an explicit selectable state or affordance for rendered cards, using a clear name such as `Selectable` or `CardInputAffordance::Selectable`.
- **FR-002**: System MUST mark a card selectable only when its front face is visible and it is not currently dragged, moved, returned, dealt, revealed, flipped, or otherwise animated as movement.
- **FR-003**: System MUST support selecting human, CPU, near-player, far-player, hand, and location cards across game screens where card fronts are visible.
- **FR-004**: System MUST keep drag and click classification separate by preserving the existing drag-threshold behavior.
- **FR-005**: System MUST show a fullscreen modal background layer while a card is selected.
- **FR-006**: The modal background layer MUST be black and animate opacity from 0% to 50% over 0.5 seconds when selection starts.
- **FR-007**: The modal background layer MUST render behind the selected card and in front of all non-selected scene/card/UI content it is intended to dim.
- **FR-008**: The modal background layer MUST block clicks, hovers, drags, and interactions for lower cards, location drop targets, and UI controls while selection is active.
- **FR-009**: Clicking the modal background MUST end selected inspection and return the selected card to its source.
- **FR-010**: Clicking the selected card itself MUST NOT dismiss selected inspection.
- **FR-011**: Point-view text, badges, and other overlay numbers from non-selected cards MUST NOT render above the selected inspection card.
- **FR-012**: Selected-card and modal layout MUST remain derived from the aspect-ratio-safe game view and update correctly on resize.

### Key Entities *(include if feature involves data)*

| Entity | Description |
| --- | --- |
| Selectable Card State | Render-facing affordance that says a front-facing stationary card can enter selected inspection on click. |
| Selected Card Interaction | Transient state identifying the one card currently selected, its source pose, target inspection pose, and return behavior. |
| Modal Background Layer | Fullscreen dimming and input-capture layer shown only during selected inspection. |
| Card Point Overlay Depth | Rendering contract that keeps selected-card content above point text and badges belonging to lower cards. |

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of front-facing stationary human, CPU, near-player, far-player, hand, and location cards tested in supported screens can enter selected inspection by click.
- **SC-002**: 0 drag gestures crossing the existing threshold open selected inspection.
- **SC-003**: Modal background opacity reaches 50% within 0.5 seconds of selection start and starts from 0% opacity on each new selection.
- **SC-004**: While selected inspection is active, lower controls and cards receive 0 click, hover, or drag effects during manual QA.
- **SC-005**: In the reported overlap case, no non-selected point-view number is visible over the selected card.

## Assumptions

| Assumption | Rationale |
| --- | --- |
| Existing selected card scale-to-center behavior remains the base interaction | User said this is already done and should not be replaced. |
| Existing drag threshold remains authoritative | User said drag-versus-click already works well. |
| Modal fade uses an explicit 0.5 second duration | Current selected inspection uses interpolation rate `CARD_GESTURE_ANIMATION_RATE = 14.0`, not a fixed duration; user requested 0.5s as the desired starting value. |
| Hidden/back-face CPU placements are not selectable | User specified cardfront only. |
| New Rust runtime files follow `bevy/crates/template-crate` standards | Required by project constitution and AGENTS guidance. |
| Changed runtime items include `HUMAN:` and `AI:` purpose comments | Required by repo guidance. |
| Visible 2D and 3D positions derive from the aspect-ratio-safe game view | Required by project constitution. |
