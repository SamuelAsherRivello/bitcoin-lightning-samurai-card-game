# Feature Specification: Card Bundle

**Feature Branch**: `006-card-bundle`  
**Created**: 2026-05-10  
**Status**: Draft  
**Consolidates**: `004-card-inspection-poc`, `005-card-polish`, `006-card-flip`  
**Input**: Combine the one-card inspection proof of concept, polished layered card presentation, and front/back flip behavior into one durable card presentation bundle.

## Scope

`006-card-bundle` is the source of truth for the current inspectable card prototype. It includes the baseline one-card inspection surface, the flat layered CardStructure/CardType presentation, the temporary card controls, and the animated CardFront/CardBack flip behavior.

| Area | Included Behavior |
| ---- | ----------------- |
| Inspection | One centered poker-proportion card, fixed camera, pointer-driven tilt, smooth response, DebugHUD exception |
| Polish | Four flat apparent-depth front layers: background, frame, foreground, title |
| Card type | Replaceable CardType artwork, `T` card type toggle, valid active front selection |
| Runtime reload | DebugHUD `R` AppScene reload and `H` hot-reload auto-restart toggle |
| Flip | Temporary Card UI `Flip` button, animated front/back transition, midpoint face swap, shared CardBack |
| Boundaries | No gameplay, scoring, rounds, targeting, tabletop placement, final menu UI, or collection/deck browser scope |

## Clarifications

### Inspection Baseline

| Question | Answer |
| -------- | ------ |
| What is the initial card geometry? | A thin poker-proportion slab, 63 mm wide by 88 mm tall, with no bevel. |
| What drives card tilt? | Pointer position in the visible application area. |
| What is the tilt limit? | 20 degrees from neutral on each supported axis. |
| What is the smoothing target? | The card reaches a new target orientation within 100 ms. |
| What does pointer input affect? | Only the card orientation; the `002-camera-setup` camera remains fixed. |
| How does DebugHUD interact with the old no-HUD rule? | `003-debugging` replaces that rule; DebugHUD is visible by default and is the only HUD exception. |

### Polished Front

| Question | Answer |
| -------- | ------ |
| Are front layers physical depth layers? | ❌ They are flat front-face layers; apparent depth comes from parallax and masking. |
| What are the layers? | Background, frame, foreground, title, in that apparent-depth order. |
| What drives parallax and shine? | The current smoothed card tilt, not raw pointer position. |
| How is background revealed? | Through the frame aperture only. |
| Can foreground and title break out? | ✅ They may overlap the frame while remaining within the outer card bounds and visually attached to the card. |
| What separates behavior from art? | CardStructure owns layer order, masking, parallax, shine, reload, and toggles; CardType owns artwork and visual style choices. |

### Flip And Backface

| Question | Answer |
| -------- | ------ |
| What does `Flip` do? | Animates the card by 180 degrees around the y-axis from the current flip state. |
| What happens if `Flip` is clicked mid-animation? | The animation reverses from current progress. |
| When do graphics switch sides? | At the edge-on midpoint, approximately 90 degrees through the flip. |
| What does `T` change? | The active CardFront definition. If CardBack is visible, the change stays hidden until the card flips face up. |
| Who owns CardBack? | The card series/CardStructure, not an individual CardDefinition. |
| What is CardBack art direction? | One shared abstract superhero-pattern back matching the current fronts, with no words, readable letters, characters, logos, or clear symbols. |
| What is DeckBuilder here? | The current prototype entry point, not final user-facing game UI. |
| What is Card UI here? | A temporary prototype control surface separate from DebugHUD. |

## User Scenarios & Testing

### User Story 1 - Inspect One Centered Card (Priority: P1)

A reviewer launches the prototype and immediately reaches the current one-card presentation surface. The card is centered, uses poker-card proportions, starts front-facing, and responds smoothly to pointer movement while the camera remains fixed.

**Why this priority**: The card presentation bundle must preserve the original proof that a digital paper-card object can be reviewed predictably before any gameplay is layered in.

**Independent Test**: Launch the app, verify one centered card is visible, move the pointer to the center, edges, and corners, and confirm the card tilts smoothly without camera movement.

**Acceptance Scenarios**:

1. **Given** the app has launched, **When** the first visible prototype state appears, **Then** exactly one inspectable card is centered in the scene.
2. **Given** the card is neutral, **When** its visible proportions are checked, **Then** its height-to-width ratio matches 88:63 within 2%.
3. **Given** the pointer moves through the visible area, **When** the card responds, **Then** it tilts toward the corresponding visible direction, clamps at 20 degrees, and reaches the target within 100 ms.
4. **Given** pointer input is active, **When** runtime state is inspected, **Then** the primary camera transform remains unchanged.

### User Story 2 - Present A Flat Layered CardFront (Priority: P1)

A reviewer sees a premium front face that appears layered while remaining a flat card-front presentation. The background appears behind the frame through the aperture, the frame reads as the reference plane, the foreground appears in front, and the title appears frontmost.

**Why this priority**: The visual promise of the card prototype is a flat paper-card object that can still feel dimensional and polished.

**Independent Test**: Inspect the card at neutral and tilted positions and verify the four front layers remain legible, contained, and visually distinct.

**Acceptance Scenarios**:

1. **Given** CardFront is visible, **When** the reviewer observes the card, **Then** background, frame, foreground, and title are visually distinguishable.
2. **Given** the pointer changes card tilt, **When** parallax updates, **Then** the background, foreground, and title shift in ways consistent with their apparent depths.
3. **Given** the frame aperture is visible, **When** the background is inspected, **Then** it appears only through the aperture and never over the frame.
4. **Given** foreground or title art overlaps the frame, **When** the reviewer observes the card, **Then** the overlap reads as premium front artwork rather than detached geometry.
5. **Given** frame shine is active, **When** the card tilts, **Then** the shine changes with smoothed card tilt and remains bound to the frame.

### User Story 3 - Switch CardFront Artwork Safely (Priority: P2)

A reviewer can press `T` to cycle through available CardType/CardFront entries without invalid artwork, while CardStructure behavior remains unchanged.

**Why this priority**: Card art must be replaceable independently from card structure, masking, parallax, shine, and flip behavior.

**Independent Test**: Press `T` in front-facing and back-facing states and verify active front selection remains valid.

**Acceptance Scenarios**:

1. **Given** CardFront is visible, **When** the reviewer presses `T`, **Then** the visible front updates to the active CardType/CardFront when another entry is available.
2. **Given** only one valid front is available, **When** the reviewer presses `T`, **Then** the card stays on a valid front and does not flash, disappear, or show missing artwork.
3. **Given** CardBack is visible, **When** the reviewer presses `T`, **Then** the visible card remains face down until a later flip reveals the changed CardFront.

### User Story 4 - Reload Prototype Card Content (Priority: P2)

A reviewer can press DebugHUD `R` to rebuild reloadable AppScene card content, and can use DebugHUD `H` to persistently control whether desktop hot-reload patches invoke the same reload path.

**Why this priority**: Card presentation iteration needs repeatable local reload behavior without restarting the app.

**Independent Test**: Press `R` and verify the card scene rebuilds without changing DebugHUD toggle state; toggle `H` and verify the setting persists through the approved local runtime state path.

**Acceptance Scenarios**:

1. **Given** the prototype is running, **When** the reviewer presses `R`, **Then** reloadable AppScene card content is rebuilt without treating `R` as a toggle.
2. **Given** DebugHUD is visible, **When** the reviewer presses `H`, **Then** hot-reload auto-restart toggles independently of `F`, `I`, `R`, and `T`.
3. **Given** desktop hot reload reports a patch, **When** `H` is enabled, **Then** the app invokes the same AppScene reload path as `R`; when `H` is disabled, it does not.

### User Story 5 - Flip Between CardFront And CardBack (Priority: P1)

A reviewer opens the current DeckBuilder prototype entry point, uses the temporary Card UI `Flip` button, and sees the card animate between the active CardFront and the shared CardBack.

**Why this priority**: A card presentation bundle needs both sides of a card while preserving the inspection feel.

**Independent Test**: Click `Flip` from front and back states, move the pointer during the animation, click again mid-animation, and verify midpoint side swaps and valid final states.

**Acceptance Scenarios**:

1. **Given** CardFront is visible, **When** the reviewer clicks `Flip`, **Then** the card animates toward CardBack.
2. **Given** CardBack is visible, **When** the reviewer clicks `Flip`, **Then** the card animates toward CardFront.
3. **Given** a flip reaches the edge-on midpoint, **When** the visible side is evaluated, **Then** the graphics switch between CardFront and CardBack according to flip direction.
4. **Given** a flip is in progress, **When** the reviewer moves the pointer, **Then** pointer-driven inspection continues on the non-flip orientation behavior without snapping to neutral.
5. **Given** a flip is in progress, **When** the reviewer clicks `Flip` again, **Then** the animation reverses from current progress and settles in a valid side state.

### User Story 6 - Use A Shared CardBack (Priority: P2)

A reviewer sees one card-series back design that belongs with the current superhero fronts but does not reveal front-specific identity.

**Why this priority**: A shared backface supports future hidden-information behavior and avoids front-specific back art.

**Independent Test**: Flip to CardBack, change the active front with `T`, and verify the same abstract back remains visible and content-safe.

**Acceptance Scenarios**:

1. **Given** any available CardFront is active, **When** the card is face down, **Then** the same shared CardBack is visible.
2. **Given** CardBack is visible, **When** active CardFront changes, **Then** CardBack remains unchanged.
3. **Given** CardBack art is inspected, **When** its content is reviewed, **Then** it has no words, readable letters, characters, logos, or clear icon-like symbols.
4. **Given** CardBack is compared to CardFront art, **When** art direction is reviewed, **Then** it reads as compatible abstract superhero-game card back art.

## Edge Cases

| Case | Expected Behavior |
| ---- | ----------------- |
| Pointer leaves the window | Card holds or eases toward the last valid pointer target without jitter. |
| Window size changes | Card remains centered, preserves proportions, and keeps layer alignment. |
| Pointer at an exact corner | Card remains visible and tilt remains within the 20-degree limit. |
| No pointer movement after launch | Card remains neutral and front-facing. |
| Pointer pauses during polish | Parallax and shine settle without flicker, jitter, or drift. |
| Background texture is wider than aperture | It remains clipped to the aperture. |
| Foreground or title breaks over frame | It remains within outer card bounds and visually attached to the front. |
| Generated texture or backface fails to load | A clear fallback is shown rather than a missing, blank, or invisible card. |
| Card UI is hidden | Flip state remains valid and the card does not unexpectedly change faces. |
| App focus changes during flip | Animation continues or settles cleanly according to existing app behavior. |

## Requirements

### Functional Requirements

| ID | Requirement |
| -- | ----------- |
| FR-001 | The prototype MUST expose one centered inspectable card as the current card presentation surface. |
| FR-002 | The card MUST use poker-card proportions of 63 mm by 88 mm and preserve an 88:63 height-to-width ratio within 2%. |
| FR-003 | Pointer-driven card inspection MUST rotate only the card while the `002-camera-setup` camera remains fixed. |
| FR-004 | Pointer-driven tilt MUST map visible area positions to corresponding card facing directions, clamp at 20 degrees, and smooth to target within 100 ms. |
| FR-005 | The approved `003-debugging` DebugHUD MUST be visible by default and is the only HUD exception to the focused prototype surface. |
| FR-006 | Card-specific geometry, defaults, pointer mapping, smoothing, card structure, card type, and flip behavior MUST live under `bevy/crates/game`; reusable window, camera, DebugHUD, inspector, and diagnostic input behavior MUST remain under shared runtime ownership. |
| FR-007 | CardFront MUST use a CardStructure with flat front-face layers in this apparent-depth order: background, frame, foreground, title. |
| FR-008 | CardStructure layers MUST NOT be implemented as visibly separated physical slabs; apparent depth MUST be created through parallax, masking, material response, and presentation. |
| FR-009 | The frame MUST include a central aperture and continuous frame UV mapping so frame artwork reads as one object. |
| FR-010 | Background artwork MUST be visible only through the frame aperture and MUST NOT render over the frame. |
| FR-011 | Foreground and title artwork MAY overlap the frame or aperture but MUST remain visually associated with the card and within the outer card bounds. |
| FR-012 | Parallax and frame shine MUST be driven by current smoothed card tilt. |
| FR-013 | Frame shine MUST visibly change with tilt, remain bound to the frame, and preserve readability of title, foreground, frame boundary, aperture, and background. |
| FR-014 | CardType MUST own replaceable artwork and visual style choices independently from CardStructure behavior. |
| FR-015 | The initial CardType/CardFront set MUST include valid generated/static artwork for background, frame, foreground, and title, and active selection MUST never resolve to missing or invalid artwork. |
| FR-016 | DebugHUD `T` MUST cycle the active CardType/CardFront through available valid entries. |
| FR-017 | DebugHUD `R` MUST be a non-toggle operation that rebuilds reloadable AppScene card content without changing DebugHUD toggle state. |
| FR-018 | DebugHUD `H` MUST toggle persisted hot-reload auto-restart state, default to disabled, and invoke the same AppScene reload path as `R` only when enabled and a desktop hot-reload patch is reported. |
| FR-019 | DebugHUD key labels MUST classify `F`, `I`, `H`, and `T` as toggles and `W`, `A`, `S`, `D`, and `R` as non-toggle hold or operation indicators. |
| FR-020 | The temporary Card UI MUST include a `Flip` button and remain separate from DebugHUD. |
| FR-021 | `DeckBuilder` MAY remain the current prototype entry point but MUST NOT be promoted into final user-facing game UI by this feature. |
| FR-022 | Activating `Flip` from CardFront MUST animate toward CardBack; activating it from CardBack MUST animate toward CardFront. |
| FR-023 | Each flip activation MUST rotate side-selection by 180 degrees around the y-axis from current flip state, while preserving pointer-driven non-flip inspection behavior. |
| FR-024 | The visible side MUST switch at the edge-on midpoint of the flip. |
| FR-025 | Repeated or mid-animation `Flip` activations MUST reverse direction from current progress and settle into a valid front or back state. |
| FR-026 | CardFront MUST be the active CardDefinition/CardType front presentation. |
| FR-027 | CardBack MUST be one shared card-series/CardStructure backface independent of individual CardFront content. |
| FR-028 | CardBack artwork MUST be a compatible abstract superhero-pattern card back with no words, readable letters, characters, logos, or clear icon-like symbols. |
| FR-029 | Pressing `T` while CardBack is visible MUST change the hidden active CardFront without changing the visible CardBack until the card flips face up. |
| FR-030 | The card presentation bundle MUST NOT introduce gameplay, scoring, rounds, targeting, tabletop placement, deck browsing, collection UI, final menus, or multi-card layout scope. |
| FR-031 | Broader Game, Player, Deck, hand, placed card, shared location, and Table Top concepts MUST remain in `007-gameplay-concepts` rather than becoming implementation scope for this feature. |
| FR-032 | Final acceptance SHOULD include Windows desktop and browser WebGPU verification, or document exact blockers for unavailable targets. |

### Key Entities

| Entity | Meaning |
| ------ | ------- |
| Card Presentation Bundle | The combined one-card inspection, polished front, card type, reload, and flip behavior defined by this spec. |
| Fixed Camera | The stationary camera from `002-camera-setup`, unaffected by pointer inspection. |
| CardStructure | Reusable card presentation structure for geometry, layer order, frame aperture, masking, apparent depth, parallax, shine, reload, and shared back behavior. |
| CardType | Replaceable artwork and visual style package applied to CardStructure. |
| CardDefinition | Prototype front identity/content selected by the active front index. |
| CardFront | The active front presentation composed from CardStructure plus active CardType/CardDefinition artwork. |
| CardBack | The shared card-series backface, independent of active CardFront. |
| Card UI | Temporary prototype control surface that includes `Flip`; separate from DebugHUD. |
| DebugHUD | Developer overlay/control surface for diagnostics, `T`, `R`, `H`, and other debug labels. |
| DeckBuilder | Current prototype entry point for reviewing the card, not final game UI. |
| Flip Orientation | Side-selection y-axis animation state layered onto pointer-driven inspection. |
| Face Swap Midpoint | Edge-on point where visible graphics switch between CardFront and CardBack. |

## Success Criteria

| ID | Outcome |
| -- | ------- |
| SC-001 | In 100% of launch checks, exactly one centered card is visible in the current prototype surface. |
| SC-002 | The visible card ratio matches 88:63 within 2%. |
| SC-003 | Pointer movement to center, four corners, and four edges produces corresponding smooth card tilt with no camera movement. |
| SC-004 | At least 4 out of 5 reviewers can identify background, frame, foreground, and title as distinct CardFront layers. |
| SC-005 | In all supported pointer positions, background remains visible only through the frame aperture. |
| SC-006 | In readability review, at least 90% of sampled inspection moments preserve recognizable title, foreground, frame, aperture, and background separation. |
| SC-007 | Pressing `T` never produces missing, blank, or invalid CardFront artwork. |
| SC-008 | Pressing `R` reloads card scene content while preserving valid DebugHUD state. |
| SC-009 | Pressing `H` toggles only hot-reload auto-restart and persists through local runtime state. |
| SC-010 | In all tested front-to-back flips, CardFront remains visible before the midpoint and CardBack is visible after it. |
| SC-011 | In all tested back-to-front flips, CardBack remains visible before the midpoint and CardFront is visible after it. |
| SC-012 | During flip, pointer-driven inspection continues without snapping to neutral. |
| SC-013 | Repeated `Flip` activations always settle into a valid front or back state without disappearing, indefinite jitter, or showing both faces at once. |
| SC-014 | CardBack remains unchanged when active CardFront changes while face down. |
| SC-015 | CardBack contains no words, readable letters, characters, logos, or clear icon-like symbols. |
| SC-016 | Scope review confirms this bundle does not introduce gameplay, tabletop placement, scoring, rounds, final menus, collection UI, or multi-card layout behavior. |

## Assumptions

| Assumption |
| ---------- |
| The target reviewer is validating card presentation and prototype controls, not final gameplay. |
| Desktop-only builds are acceptable during iteration, but final completion should verify Windows desktop and browser WebGPU or document blockers. |
| Generated/static artwork is project-owned runtime asset content and may be replaced without changing CardStructure behavior. |
| The current switchable fronts stand in for future CardDefinitions. |
| Hidden placement and reveal timing are future gameplay concepts; this bundle only proves hidden front selection while CardBack remains visible. |
| Future tabletop, box cover, and main menu art may be influenced by the eventual backface direction, but they are not designed by this feature. |
