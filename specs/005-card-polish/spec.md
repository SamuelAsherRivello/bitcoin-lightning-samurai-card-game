# Feature Specification: Card Polish

**Feature Branch**: `005-card-polish`  
**Created**: 2026-05-09  
**Status**: Draft  
**Input**: User description: "005 is card-polish. The card is a paradox: it is flat and has no z thickness yet it will appear like it has parallax effects. Start with Scene > Card > background, frame, character. The background appears at depth -1 relative to the frame and the character appears at depth 1 relative to the frame. The frame itself has a hologram or shine that appears to shine differently as you move it. Inspiration includes Daniel Ilett's holographic card shader repository, a linked YouTube video, a Marvel Snap style Reddit post, a linked X post, and the provided Wasp card image."

## Clarifications

### Session 2026-05-09

- Q: How should `005-card-polish` handle the visible thin-slab geometry already required by `004-card-inspection-poc`? -> A: Keep `004` thin-slab card geometry visible, but hide layered depth inside the front face.
- Q: What drives the polished card's parallax and frame shine response? -> A: Drive parallax and frame shine from the card's current smoothed tilt.
- Q: How is the background layer revealed through the frame? -> A: The frame has a hole, and the background is visible only through that hole.
- Q: How may the foreground layer relate to the frame and frame hole? -> A: Foreground may extend over the frame and hole, like premium foreground card art.
- Q: How should background and foreground clipping differ? -> A: Background may be wider than the frame but is clipped by the frame hole, so it never appears wider; foreground may be wider than the frame and may appear wider because it is not clipped by the frame.
- Q: What default artwork should the polished card use? -> A: Use a textured background, a white frame with about 10% thickness on each side and a central hole, and a foreground rectangle about 50% of the frame width and height.
- Q: Are the background, frame, and foreground separate physical depth layers? -> A: No; all three are flat 2D texture layers, with apparent depth created by parallax and masking.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Perceive Layered Card Depth (Priority: P1)

A reviewer launches the card prototype and sees one flat card that visually reads as layered: the background appears set behind the frame through a hole in the frame, the frame appears as the neutral surface, and the foreground appears in front of the frame.

**Why this priority**: The central promise of this feature is the paradoxical flat card that appears to have dimensional depth without becoming a physically thick card.

**Independent Test**: Launch the prototype, view the card in neutral and tilted positions, and verify that the background, frame, and foreground are visually separable as three apparent depth layers while the card remains a single flat presentation.

**Acceptance Scenarios**:

1. **Given** the card is in its neutral front-facing position, **When** the reviewer observes the artwork, **Then** the card shows a distinct textured background layer visible through the frame hole, a white frame layer, and a foreground rectangle.
2. **Given** the reviewer moves the pointer across the viewing area, **When** the card tilt changes, **Then** the background appears to shift as if behind the frame and the foreground appears to shift as if in front of the frame.
3. **Given** the card is inspected from any supported pointer position, **When** the reviewer evaluates its layered artwork, **Then** the parallax layers remain contained within the card's front face and do not appear as separated physical pieces.
4. **Given** the card is inspected from any supported pointer position, **When** the reviewer observes the frame area outside the hole, **Then** the background is not visible outside the frame hole.
5. **Given** the foreground artwork reaches the frame or frame hole, **When** the reviewer observes the card, **Then** the foreground may appear over the frame or hole because it is not clipped by the frame.

---

### User Story 2 - Read the Frame as Holographic (Priority: P1)

A reviewer tilts or moves the card and sees the frame produce a holographic shine that changes with the card's apparent angle instead of appearing as a static border.

**Why this priority**: The frame is the main surface that sells premium trading-card polish and makes the motion feel reactive.

**Independent Test**: Move the pointer through the center, corners, and edges of the viewing area and verify that the frame shine changes direction, intensity, or color in response to movement while remaining bound to the frame area.

**Acceptance Scenarios**:

1. **Given** the card is visible, **When** the reviewer moves the pointer from left to right, **Then** the frame shine visibly changes across the frame.
2. **Given** the card is visible, **When** the reviewer moves the pointer from top to bottom, **Then** the frame shine visibly changes in a different way than a static image would.
3. **Given** the frame shine is active, **When** the reviewer observes the card artwork, **Then** the shine remains visually associated with the frame and does not obscure the foreground or background enough to make them unreadable.

---

### User Story 3 - Preserve the Card Inspection POC Feel (Priority: P2)

A reviewer can still use the existing single-card inspection behavior while seeing richer card polish, with no gameplay, no multiple-card layout, and no menu flow added by this feature.

**Why this priority**: This feature should improve the presentation of the existing card POC without expanding into gameplay or collection UI.

**Independent Test**: Launch and inspect the prototype using the same card inspection flow from `004-card-inspection-poc`, then verify that only visual polish has been added.

**Acceptance Scenarios**:

1. **Given** the prototype launches, **When** the first visible screen appears, **Then** it still shows one centered inspectable card rather than a menu, deck view, or gameplay scene.
2. **Given** the reviewer moves the pointer, **When** the card responds, **Then** the card's overall inspection behavior remains smooth and recognizable from `004-card-inspection-poc`.
3. **Given** the reviewer observes the scene, **When** evaluating scope, **Then** there are no additional cards, turns, rules, scoring, dragging, targeting, or deck interactions introduced by this feature.

---

### User Story 4 - Distinguish Layer Boundaries Clearly (Priority: P3)

A reviewer can identify which visual elements belong to the background, frame, and foreground layers, even when the card is moving and the holographic frame shine is active.

**Why this priority**: The polish needs to be legible, not only impressive; unclear layer boundaries would weaken the depth effect.

**Independent Test**: Inspect the card during motion and confirm that reviewers can correctly name the three layer roles without being shown implementation details.

**Acceptance Scenarios**:

1. **Given** the card is moving through its supported tilt range, **When** the reviewer identifies the visual layers, **Then** the background, frame, and foreground remain distinguishable.
2. **Given** the frame shine reaches a bright point, **When** the reviewer observes the card, **Then** the shine does not erase the frame outline or layer separation.

### Edge Cases

- If the pointer is held at an extreme corner, the depth illusion should remain controlled and should not make any layer appear detached from the card.
- If the pointer stops moving, the parallax and shine should settle without visible jitter, flicker, or endless drifting.
- If the card returns to the neutral centered orientation, the layer offsets should return to a balanced neutral composition.
- If the frame shine becomes bright, the foreground and frame boundaries should remain readable.
- If the background source extends wider than the frame, it should remain clipped by the frame hole and never appear wider than that visible opening.
- If the foreground extends wider than the frame, it may visibly exceed the frame width because it is not clipped by the frame, while still remaining inside the outer card bounds.
- If the application window size changes, the layered card should remain centered and preserve its card proportions and layer alignment.
- If the approved DebugHUD from `003-debug-hud` is visible, it should not obscure the card's layered visual effect during normal inspection.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The prototype MUST present exactly one centered card using the existing card inspection flow from `004-card-inspection-poc`.
- **FR-002**: The card MUST visually contain three named artwork layers: background, frame, and foreground.
- **FR-003**: The background, frame, and foreground layers MUST be implemented as flat 2D visual layers rather than physically separated 3D slabs.
- **FR-004**: The frame layer MUST serve as the visual reference plane for the layered card effect and MUST include a hole through which the background layer is visible.
- **FR-005**: The default frame artwork MUST be white and have approximately 10% thickness on each side, with the remaining middle area forming the frame hole.
- **FR-006**: The background layer MUST be clipped by the frame hole, MUST be visible only through the frame hole, and MUST NOT appear outside the hole or on top of the frame.
- **FR-007**: The default background artwork MUST be textured so the parallax shift is visible through the frame hole.
- **FR-008**: The background layer MUST appear at an apparent depth behind the frame, equivalent to depth -1 relative to the frame.
- **FR-009**: The background layer MAY be wider than the frame internally, but it MUST NOT appear wider than the visible frame hole.
- **FR-010**: The default foreground artwork MUST be a rectangle approximately 50% of the frame's width and approximately 50% of the frame's height.
- **FR-011**: The foreground layer MUST appear at an apparent depth in front of the frame, equivalent to depth +1 relative to the frame.
- **FR-012**: The foreground layer MAY extend over the frame and frame hole as foreground card art.
- **FR-013**: The foreground layer MAY be wider than the frame and MAY appear wider than the frame because it is not clipped by the frame.
- **FR-014**: The foreground layer MUST remain inside the outer card bounds and MUST NOT appear as a detached physical piece.
- **FR-015**: The apparent depth effect MUST respond to the card's current smoothed tilt so the layers appear to shift differently as the card is tilted.
- **FR-016**: The card MUST preserve the existing `004-card-inspection-poc` thin-slab card geometry while keeping the background, frame, foreground, parallax, and shine effects visually contained on the front face without additional visible z thickness or separated physical layer geometry.
- **FR-017**: The frame MUST include a holographic shine or foil-like visual treatment.
- **FR-018**: The frame shine MUST visibly change based on the card's current smoothed tilt.
- **FR-019**: The frame shine MUST remain visually bound to the frame and MUST NOT be the primary effect on the background or foreground layers.
- **FR-020**: The layered parallax and frame shine MUST preserve the readability of the foreground, frame boundary, frame hole, and background composition during normal inspection.
- **FR-021**: The card's silhouette and proportions MUST remain consistent with the card inspection POC while the polish effects are active.
- **FR-022**: This feature MUST NOT introduce gameplay, multiple cards, deck handling, selection, dragging, scoring, turns, rules, menus, or player-facing card text requirements.
- **FR-023**: The effect MUST remain visually stable when pointer input pauses, including no distracting flicker, jitter, or unintended drift.
- **FR-024**: Final acceptance MUST include verification in the same target environments required by `004-card-inspection-poc`, or any blocked target MUST be documented with the exact blocker.

### Key Entities

- **Polished Card**: The single inspectable card whose visual treatment creates apparent depth and premium foil response while remaining flat.
- **Background Layer**: The artwork region that appears behind the frame at apparent depth -1 and is visible only through the frame hole.
- **Frame Layer**: The card border or framing surface that acts as the visual reference plane, includes the hole that reveals the background, carries the holographic shine, and defaults to a white frame with about 10% thickness on each side.
- **Foreground Layer**: The primary foreground artwork that appears in front of the frame at apparent depth +1, defaults to a rectangle about 50% of the frame width and height, may overlap the frame or frame hole, may appear wider than the frame, and stays inside the outer card bounds.
- **Apparent Depth**: The viewer-facing illusion that layers sit at different depths, without requiring physical card thickness.
- **Frame Hole**: The opening in the frame through which the background layer is visible.
- **Frame Shine**: The tilt-reactive holographic or foil-like effect bound to the frame.
- **Inspection Input**: The existing reviewer movement input that changes card orientation; the current smoothed card tilt drives the perceived parallax and shine response.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In a neutral card view, at least 4 out of 5 reviewers can correctly identify the background, frame, and foreground as distinct visual layers.
- **SC-002**: During movement tests through center, four corners, and four screen edges, at least 4 out of 5 reviewers perceive the background as behind the frame and the foreground as in front of the frame.
- **SC-003**: In 100% of tested supported pointer positions, the background is visible only through the frame hole, does not appear outside the hole, and never appears wider than the visible opening.
- **SC-004**: In neutral view, the default frame reads as white, has about 10% thickness on each side, and leaves a central hole for the background.
- **SC-005**: In neutral view, the default foreground reads as a rectangle about half the frame width and half the frame height.
- **SC-006**: In 100% of tested supported pointer positions, any foreground overlap across the frame or hole remains within the outer card bounds and reads as foreground art rather than detached geometry.
- **SC-007**: In 100% of tested supported pointer positions, the polished background, frame, and foreground layers remain visually contained within the card front face with no detached layer geometry or added visible layer thickness.
- **SC-008**: During pointer movement from left to right and top to bottom, at least 4 out of 5 reviewers observe the frame shine changing consistently with the card's smoothed tilt.
- **SC-009**: In readability review, at least 90% of sampled inspection moments preserve recognizable foreground, frame, frame hole, and background separation.
- **SC-010**: In scope review, no gameplay, menus, multiple cards, deck behavior, scoring, turns, dragging, or selection behavior is present.
- **SC-011**: Final acceptance verification passes in the target environments inherited from `004-card-inspection-poc`, or each unavailable target has a documented blocker.

## Assumptions

- The apparent depth values are relative to the frame: background is -1, frame is 0, and foreground is +1.
- "Flat and has no z thickness" describes the visual polish layers: they should not read as additional physical depth beyond the existing `004-card-inspection-poc` thin-slab card geometry.
- Background, frame, and foreground are flat 2D texture layers; apparent z separation is produced by parallax offsets and masking rather than layer geometry.
- The background is masked by the frame hole: reviewers should see it only where the frame opening reveals it, even when the source background extends wider than the frame.
- The foreground layer may overlap the frame or frame hole, may appear wider than the frame, and stays inside the outer card silhouette.
- Default artwork means a textured background, a white frame with about 10% thickness on each side and a central hole, and a foreground rectangle about 50% of the frame width and height.
- Parallax and frame shine are evaluated against the card's current smoothed tilt, not raw pointer position.
- The linked Daniel Ilett holographic card project, linked YouTube video, linked Reddit post, linked X post, and provided Wasp card image are inspiration for desired feel, not exact implementation or asset requirements.
- The first polished version may use project-owned placeholder art for the background, frame, and foreground as long as the three layer roles and motion response are clear.
- The existing pointer-driven card inspection behavior remains the primary interaction model.
- This feature builds on the single-card POC and does not replace the broader constraints from `004-card-inspection-poc` unless explicitly stated here.
