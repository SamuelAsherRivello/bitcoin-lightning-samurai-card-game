# Feature Specification: Card Polish

**Feature Branch**: `005-card-polish`  
**Created**: 2026-05-09  
**Status**: Draft  
**Input**: User description: "005 is card-polish. The card is a paradox: it is flat and has no z thickness yet it will appear like it has parallax effects. Start with Scene > Card > background, frame, character. The background appears at depth -1 relative to the frame and the character appears at depth 1 relative to the frame. The frame itself has a hologram or shine that appears to shine differently as you move it. Inspiration includes Daniel Ilett's holographic card shader repository, a linked YouTube video, a Marvel Snap style Reddit post, a linked X post, and the provided Wasp card image. Update 005 to define a CardStructure with four parallax layers: background, frame, foreground, and title. Define artwork as a CardType applied to that structure. The requested initial CardType uses generated static textures and material shaders instead of manually constructed dots or primitive placeholder art: a blue-and-white repeated icon-like cloud background, a subtle 45-degree grey/off-grey pinstripe frame, a flat superhero-inspired foreground character breaking out of the frame, and a title layer in front of the foreground that also partially breaks out of the frame. Add HUD key T to toggle CardType through a two-card-type list, with only this requested card type available for now."

## Clarifications

### Session 2026-05-09

- Q: How should `005-card-polish` handle the visible thin-slab geometry already required by `004-card-inspection-poc`? -> A: Keep `004` thin-slab card geometry visible, but hide layered depth inside the front face.
- Q: What drives the polished card's parallax and frame shine response? -> A: Drive parallax and frame shine from the card's current smoothed tilt.
- Q: How is the background layer revealed through the frame? -> A: The frame has a hole, and the background is visible only through that hole.
- Q: How may the foreground layer relate to the frame and frame hole? -> A: Foreground may extend over the frame and hole, like premium foreground card art.
- Q: How should background and foreground clipping differ? -> A: Background may be wider than the frame but is clipped by the frame hole, so it never appears wider; foreground may be wider than the frame and may appear wider because it is not clipped by the frame.
- Q: What default artwork should the polished card use? -> A: Use the requested initial CardType: a blue-and-white repeated icon-like cloud background, a subtle 45-degree grey/off-grey pinstripe frame, a flat superhero-inspired foreground character that breaks out of the frame, and a title layer in front of the foreground that also partially breaks out of the frame.
- Q: Are the background, frame, foreground, and title separate physical depth layers? -> A: No; all four are flat 2D texture layers, with apparent depth created by parallax and masking.
- Q: How should `005-card-polish` separate artwork from card behavior? -> A: Treat the card's layer order, parallax, masking, and HUD interaction as `CardStructure`; treat all artwork and visual-style assets applied to that structure as `CardType`.
- Q: How many apparent layers does the polished card structure have? -> A: Four flat front-face layers in depth order: background, frame, foreground, and title.
- Q: What is the initial `CardType`? -> A: A generated-texture card type with blue-and-white repeated icon-like clouds for the background, subtle 45-degree grey/off-grey pinstripes for the frame, a flat superhero-inspired foreground character that breaks out of part of the frame, and a title that sits in front of the foreground and also partially breaks out of the frame.
- Q: How should texture artwork be produced for the initial card type? -> A: Use static generated image textures for the background, frame, and foreground artwork instead of manually constructing dot patterns or placeholder primitives; each visual style includes a texture material shader.
- Q: How should the frame be represented structurally? -> A: CardStructure defines one frame object with a central cutout stencil/aperture and continuous full-card UVs; CardType only chooses the frame texture applied to that one object.
- Q: How should card type switching work initially? -> A: Add a HUD `T` key that toggles through the CardType registry sized for two card types; with SkyBolt and Tar available, the toggle cycles between those two card types and keeps the active artwork valid.
- Q: Which `bevy-zoo-game` `001`/`003` key behaviors should this feature carry forward? -> A: Keep `R` as a non-toggle DebugHUD operation that reloads the reloadable `AppScene` card scene content, and keep `H` as a persisted DebugHUD toggle that controls whether desktop hot-reload patches automatically invoke that same reload path.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Perceive Layered Card Depth (Priority: P1)

A reviewer launches the card prototype and sees one flat card that visually reads as layered: the background appears set behind the frame through a hole in the frame, the frame appears as the reference surface, the foreground character appears in front of the frame, and the title appears in front of the foreground.

**Why this priority**: The central promise of this feature is the paradoxical flat card that appears to have dimensional depth without becoming a physically thick card.

**Independent Test**: Launch the prototype, view the card in neutral and tilted positions, and verify that the background, frame, foreground, and title are visually separable as four apparent depth layers while the card remains a single flat presentation.

**Acceptance Scenarios**:

1. **Given** the card is in its neutral front-facing position, **When** the reviewer observes the artwork, **Then** the card shows a distinct blue-and-white cloud background visible through the frame hole, a subtle diagonal pinstripe frame, a foreground superhero-inspired character, and a title layer.
2. **Given** the reviewer moves the pointer across the viewing area, **When** the card tilt changes, **Then** the background appears to shift as if behind the frame, the foreground appears to shift as if in front of the frame, and the title appears closest to the viewer.
3. **Given** the card is inspected from any supported pointer position, **When** the reviewer evaluates its layered artwork, **Then** the parallax layers remain contained within the card's front face and do not appear as separated physical pieces.
4. **Given** the card is inspected from any supported pointer position, **When** the reviewer observes the frame area outside the hole, **Then** the background is not visible outside the frame hole.
5. **Given** the foreground artwork reaches the frame or frame hole, **When** the reviewer observes the card, **Then** the foreground character may appear over the frame or hole because it is not clipped by the frame.
6. **Given** the title artwork reaches the frame or frame hole, **When** the reviewer observes the card, **Then** the title may appear over the foreground, frame, or hole because it is the frontmost layer.

---

### User Story 2 - Read the Frame as Holographic (Priority: P1)

A reviewer tilts or moves the card and sees the frame produce a holographic shine that changes with the card's apparent angle instead of appearing as a static border.

**Why this priority**: The frame is the main surface that sells premium trading-card polish and makes the motion feel reactive.

**Independent Test**: Move the pointer through the center, corners, and edges of the viewing area and verify that the frame shine changes direction, intensity, or color in response to movement while remaining bound to the frame area.

**Acceptance Scenarios**:

1. **Given** the card is visible, **When** the reviewer moves the pointer from left to right, **Then** the frame shine visibly changes across the frame.
2. **Given** the card is visible, **When** the reviewer moves the pointer from top to bottom, **Then** the frame shine visibly changes in a different way than a static image would.
3. **Given** the frame shine is active, **When** the reviewer observes the card artwork, **Then** the shine remains visually associated with the frame and does not obscure the foreground, title, or background enough to make them unreadable.

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

A reviewer can identify which visual elements belong to the background, frame, foreground, and title layers, even when the card is moving and the holographic frame shine is active.

**Why this priority**: The polish needs to be legible, not only impressive; unclear layer boundaries would weaken the depth effect.

**Independent Test**: Inspect the card during motion and confirm that reviewers can correctly name the four layer roles without being shown implementation details.

**Acceptance Scenarios**:

1. **Given** the card is moving through its supported tilt range, **When** the reviewer identifies the visual layers, **Then** the background, frame, foreground, and title remain distinguishable.
2. **Given** the frame shine reaches a bright point, **When** the reviewer observes the card, **Then** the shine does not erase the frame outline or layer separation.

---

### User Story 5 - Toggle Card Card Type Slot (Priority: P3)

A reviewer can press the HUD `T` key to cycle the card type selection, so the polish system is ready for a second card type without changing the card structure.

**Why this priority**: Card type switching proves the artwork is organized as interchangeable `CardType` data instead of being fused into the structural parallax behavior.

**Independent Test**: Launch the prototype, press `T`, and verify the HUD action is recognized while the card remains on the requested initial card type when no second card type is available.

**Acceptance Scenarios**:

1. **Given** the prototype has only the requested initial CardType available, **When** the reviewer presses `T`, **Then** the card remains visually stable and does not enter an invalid or blank card type state.
2. **Given** the HUD is visible, **When** the reviewer presses `T`, **Then** the HUD communicates the active CardType state clearly enough for testing.
3. **Given** SkyBolt and Tar CardTypes are available, **When** the reviewer presses `T`, **Then** the same control cycles between the two card type entries without changing the CardStructure.

---

### User Story 6 - Reload Scene Content From DebugHUD (Priority: P3)

A reviewer can press the DebugHUD `R` key to rebuild the reloadable app scene content without restarting the whole app, and can press `H` to persistently toggle whether desktop hot-reload patches perform that same reload.

**Why this priority**: This carries forward the reviewer workflow from `bevy-zoo-game` specs `001` and `003`, making card polish iteration compatible with manual scene reloads and optional hot-reload auto-restart.

**Independent Test**: Launch the prototype, press `R`, and verify the card scene is rebuilt while HUD toggle state remains valid; press `H`, restart or reload as supported, and verify the hot-reload auto-restart state is represented as a persistent DebugHUD toggle.

**Acceptance Scenarios**:

1. **Given** the prototype is running, **When** the reviewer presses `R`, **Then** the app reloads the `AppScene` card content, including the primary camera and card structure, without treating `R` as a toggle.
2. **Given** the DebugHUD is visible, **When** the reviewer presses `H`, **Then** the hot-reload auto-restart value toggles independently of `F`, `I`, `R`, and `T`.
3. **Given** desktop hot reload is active and `H` is enabled, **When** a hot-reload patch is applied, **Then** the app invokes the same `AppScene` reload behavior as pressing `R`.
4. **Given** desktop hot reload is active and `H` is disabled, **When** a hot-reload patch is applied, **Then** the patch does not automatically reload `AppScene`.

### Edge Cases

- If the pointer is held at an extreme corner, the depth illusion should remain controlled and should not make any layer appear detached from the card.
- If the pointer stops moving, the parallax and shine should settle without visible jitter, flicker, or endless drifting.
- If the card returns to the neutral centered orientation, the layer offsets should return to a balanced neutral composition.
- If the frame shine becomes bright, the foreground and frame boundaries should remain readable.
- If the background source extends wider than the frame, it should remain clipped by the frame hole and never appear wider than that visible opening.
- If the foreground extends wider than the frame, it may visibly exceed the frame width because it is not clipped by the frame, while still remaining inside the outer card bounds.
- If the application window size changes, the layered card should remain centered and preserve its card proportions and layer alignment.
- If the approved DebugHUD from `003-debug-hud` is visible, it should not obscure the card's layered visual effect during normal inspection.
- If the reviewer presses `T`, the card should cycle between available card types and should not flash, disappear, or show missing artwork.
- If the generated texture assets fail to load, the prototype should show a clear fallback rather than silently reverting to manually constructed dot patterns or primitive placeholder art.
- If the reviewer presses `R` repeatedly, each press should reload `AppScene` without toggling `F`, `I`, `H`, or `T`.
- If the reviewer presses `H` repeatedly, each press should toggle only hot-reload auto-restart and should persist through the approved local runtime state path.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The prototype MUST present exactly one centered card using the existing card inspection flow from `004-card-inspection-poc`.
- **FR-002**: The card MUST define a `CardStructure` that visually contains four named artwork layers in depth order: background, frame, foreground, and title.
- **FR-003**: The background, frame, foreground, and title layers MUST be implemented as flat 2D visual layers rather than physically separated 3D slabs.
- **FR-004**: The frame layer MUST serve as the visual reference plane for the layered card effect and MUST include a hole through which the background layer is visible.
- **FR-005**: The requested initial `CardType` MUST apply generated static artwork assets to the CardStructure rather than relying on manually constructed dot patterns, manually assembled primitive pattern elements, or plain placeholder rectangles as the final card type artwork.
- **FR-006**: The background layer MUST be clipped by the frame hole, MUST be visible only through the frame hole, and MUST NOT appear outside the hole or on top of the frame.
- **FR-007**: The requested initial CardType's background visual style MUST use a static generated texture with repeated icon-like clouds in blue and white, and it MUST include a texture material shader.
- **FR-008**: The background layer MUST appear at an apparent depth behind the frame, equivalent to depth -1 relative to the frame.
- **FR-009**: The background layer MAY be wider than the frame internally, but it MUST NOT appear wider than the visible frame hole.
- **FR-010**: The requested initial CardType's frame visual style MUST use a static generated texture with subtle grey and off-grey pinstripes angled approximately 45 degrees upward to the right, and it MUST include a texture material shader.
- **FR-011**: The foreground layer MUST appear at an apparent depth in front of the frame, equivalent to depth +1 relative to the frame.
- **FR-012**: The requested initial CardType's foreground visual style MUST use a static generated texture of a flat superhero-inspired character, using the attached example's character fit and cape only as inspiration, and it MUST include a texture material shader.
- **FR-013**: The foreground layer MAY be wider than the frame and MAY appear wider than the frame because it is not clipped by the frame.
- **FR-014**: The foreground character MUST partially break out over some part of the frame while still reading as attached card artwork rather than detached physical geometry.
- **FR-015**: The apparent depth effect MUST respond to the card's current smoothed tilt so the layers appear to shift differently as the card is tilted.
- **FR-016**: The card MUST preserve the existing `004-card-inspection-poc` thin-slab card geometry while keeping the background, frame, foreground, title, parallax, and shine effects visually contained on the front face without additional visible z thickness or separated physical layer geometry.
- **FR-017**: The frame MUST include a holographic shine or foil-like visual treatment.
- **FR-018**: The frame shine MUST visibly change based on the card's current smoothed tilt.
- **FR-019**: The frame shine MUST remain visually bound to the frame and MUST NOT be the primary effect on the background or foreground layers.
- **FR-020**: The layered parallax and frame shine MUST preserve the readability of the title, foreground, frame boundary, frame hole, and background composition during normal inspection.
- **FR-021**: The card's silhouette and proportions MUST remain consistent with the card inspection POC while the polish effects are active.
- **FR-022**: This feature MUST NOT introduce gameplay, multiple cards, deck handling, selection, dragging, scoring, turns, rules, menus, or broader player-facing card text systems beyond the single title artwork layer required for the polished card.
- **FR-023**: The effect MUST remain visually stable when pointer input pauses, including no distracting flicker, jitter, or unintended drift.
- **FR-024**: Final acceptance MUST include verification in the same target environments required by `004-card-inspection-poc`, or any blocked target MUST be documented with the exact blocker.
- **FR-025**: The title layer MUST appear in front of the foreground layer, equivalent to a depth greater than +1 relative to the frame.
- **FR-026**: The requested initial CardType's title artwork MUST partially break out over the frame in a premium trading-card style while remaining visually associated with the card.
- **FR-027**: The application MUST define a CardType registry intended to hold two card type entries for this feature phase, with the requested initial CardType as the first available card type.
- **FR-028**: The DebugHUD MUST include a `T` key interaction that toggles the active CardType selection through the registry.
- **FR-029**: Pressing `T` MUST keep the card on a valid available CardType and MUST NOT show missing, blank, or invalid artwork.
- **FR-030**: CardType artwork MUST be replaceable without changing CardStructure behavior, including layer order, clipping, apparent depth rules, parallax response, and HUD card type toggle behavior.
- **FR-037**: The frame layer MUST be one CardStructure object with a center cutout and continuous frame UV mapping, so a CardType frame texture appears continuous rather than as four separately mapped border objects.
- **FR-038**: The DebugHUD MUST include `R` as a non-toggle key that invokes the `AppScene` reload method carried forward from `bevy-zoo-game` `001-project-setup` and `003-debug-hud`.
- **FR-039**: Pressing `R` MUST rebuild reloadable card scene content, including the primary camera and card structure, without changing DebugHUD toggle state.
- **FR-040**: The DebugHUD MUST include `H` as a toggle key for hot-reload auto-restart.
- **FR-041**: The `H` hot-reload auto-restart value MUST default to `false`, persist as local runtime state through the project-approved `bevy-persistent` pattern, and restore on later desktop launches.
- **FR-042**: When desktop hot reload reports a patch and `H` is enabled, the app MUST invoke the same `AppScene` reload behavior as pressing `R`; when `H` is disabled, the patch MUST NOT auto-reload `AppScene`.
- **FR-043**: DebugHUD key labels MUST classify `F`, `I`, `H`, and `T` as toggles and `W`, `A`, `S`, `D`, and `R` as non-toggle hold or operation indicators.

### Key Entities

- **Polished Card**: The single inspectable card whose visual treatment creates apparent depth and premium foil response while remaining flat.
- **CardStructure**: The reusable card presentation structure that owns layer order, clipping, apparent depth, parallax response, frame shine behavior, and HUD card type switching independent of any specific artwork.
- **CardType**: The interchangeable artwork package applied to a CardStructure, including generated static textures and texture material shader choices for the background, frame, foreground, and title presentation.
- **Background Layer**: The artwork region that appears behind the frame at apparent depth -1 and is visible only through the frame hole.
- **Frame Layer**: The card border or framing surface that acts as the visual reference plane, includes the hole that reveals the background, carries the holographic shine, and receives the CardType frame visual style.
- **Foreground Layer**: The primary character artwork that appears in front of the frame at apparent depth +1, may overlap the frame or frame hole, may appear wider than the frame, and stays visually associated with the card.
- **Title Layer**: The frontmost title artwork layer that appears in front of the foreground, may overlap the foreground, frame, or frame hole, and may partially break out of the frame.
- **Apparent Depth**: The viewer-facing illusion that layers sit at different depths, without requiring physical card thickness.
- **Frame Hole**: The opening in the frame through which the background layer is visible.
- **Frame Shine**: The tilt-reactive holographic or foil-like effect bound to the frame.
- **Visual Style**: The CardType-provided look for a layer, including its generated static texture and the texture material shader used to display it.
- **Inspection Input**: The existing reviewer movement input that changes card orientation; the current smoothed card tilt drives the perceived parallax and shine response.
- **Card Type Toggle Input**: The HUD `T` key interaction that cycles the active CardType selection.
- **AppScene Reload Input**: The HUD `R` key interaction that rebuilds reloadable scene-owned card content.
- **Hot Reload Auto-Restart Toggle**: The HUD `H` key interaction that persists whether desktop hot-reload patches automatically reload `AppScene`.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In a neutral card view, at least 4 out of 5 reviewers can correctly identify the background, frame, foreground, and title as distinct visual layers.
- **SC-002**: During movement tests through center, four corners, and four screen edges, at least 4 out of 5 reviewers perceive the background as behind the frame, the foreground as in front of the frame, and the title as the frontmost layer.
- **SC-003**: In 100% of tested supported pointer positions, the background is visible only through the frame hole, does not appear outside the hole, and never appears wider than the visible opening.
- **SC-004**: In neutral view, the requested initial CardType reads as blue-and-white repeated icon-like clouds in the background, subtle grey/off-grey diagonal pinstripes in the frame, a flat superhero-inspired foreground character, and a title in front of the character.
- **SC-005**: In neutral view, the foreground character and title both visibly break out over part of the frame while remaining visually associated with the card.
- **SC-006**: In 100% of tested supported pointer positions, any foreground overlap across the frame or hole remains within the outer card bounds and reads as foreground art rather than detached geometry.
- **SC-007**: In 100% of tested supported pointer positions, the polished background, frame, foreground, and title layers remain visually contained within the card front face with no detached layer geometry or added visible layer thickness.
- **SC-008**: During pointer movement from left to right and top to bottom, at least 4 out of 5 reviewers observe the frame shine changing consistently with the card's smoothed tilt.
- **SC-009**: In readability review, at least 90% of sampled inspection moments preserve recognizable title, foreground, frame, frame hole, and background separation.
- **SC-010**: In scope review, no gameplay, menus, multiple cards, deck behavior, scoring, turns, dragging, or selection behavior is present.
- **SC-011**: Final acceptance verification passes in the target environments inherited from `004-card-inspection-poc`, or each unavailable target has a documented blocker.
- **SC-012**: Pressing `T` with only the requested initial CardType available produces no missing-art, blank-art, crash, or invalid-card type state in 100% of tested attempts.
- **SC-013**: Pressing `R` reloads the primary camera and card structure while preserving valid DebugHUD state in 100% of tested attempts.
- **SC-014**: Pressing `H` toggles only hot-reload auto-restart, restores through local persisted DebugHUD input state, and does not affect card type selection in 100% of tested attempts.

## Assumptions

- The apparent depth values are relative to the frame: background is -1, frame is 0, foreground is +1, and title is in front of the foreground.
- "Flat and has no z thickness" describes the visual polish layers: they should not read as additional physical depth beyond the existing `004-card-inspection-poc` thin-slab card geometry.
- Background, frame, foreground, and title are flat 2D texture layers; apparent z separation is produced by parallax offsets and masking rather than layer geometry.
- The background is masked by the frame hole: reviewers should see it only where the frame opening reveals it, even when the source background extends wider than the frame.
- The foreground layer may overlap the frame or frame hole, may appear wider than the frame, and stays visually associated with the card silhouette.
- The title layer is the frontmost apparent-depth layer and may overlap the foreground, frame, or frame hole.
- The requested initial CardType replaces previous placeholder artwork assumptions, including the white frame and rectangle foreground.
- Static generated texture assets may be produced with ChatGPT Images or an equivalent project-approved image generation workflow, then committed as ordinary project-owned assets.
- Parallax and frame shine are evaluated against the card's current smoothed tilt, not raw pointer position.
- The linked Daniel Ilett holographic card project, linked YouTube video, linked Reddit post, linked X post, and provided Wasp card image are inspiration for desired feel, not exact implementation or asset requirements.
- The attached example image is inspiration for character fit, cape placement, and premium breakout composition only; the requested initial CardType should not copy protected character identity, logos, or exact artwork.
- The first polished version should use generated static card type textures for the requested background, frame, and foreground rather than manually drawn dots or primitive placeholder geometry.
- The existing pointer-driven card inspection behavior remains the primary interaction model.
- `R`/`H` behavior is imported only as it relates to reloadable scene content, DebugHUD key classification, and hot-reload auto-restart; unrelated `bevy-zoo-game` model browser or gameplay systems are out of scope.
- This feature builds on the single-card POC and does not replace the broader constraints from `004-card-inspection-poc` unless explicitly stated here.
