# Feature Specification: Game Theme POC

**Feature Branch**: `[008-game-theme-poc]`  
**Created**: 2026-05-10  
**Status**: Draft  
**Input**: User description: "Create a visual and interaction proof-of-concept for a stylized tactical card game currently titled 'Card Katana'. The title is temporary and should not yet appear in namespaces, architecture, or technical systems. This prototype is focused entirely on atmosphere, visual identity, interactivity, and thematic cohesion rather than gameplay implementation. Theme is JW, Japanese Warriors, with Bamboo Forest and Coastal Harbor worlds, reusable tactical locations, four bottom cards, cursor/touch card tilt, card selection into a Deck Builder, card flipping, card theme cycling, and strict separation between world themes and card themes. The art style is Japan Realism: cinematic feudal Japanese realism, chiaroscuro lighting, realistic materials, mature tactical tone, subtle fog/rain/smoke/natural lighting, and no visible magic or glowing fantasy effects."

## Clarifications

### Session 2026-05-10

- Q: What changes when the Deck Builder changes the current card theme? → A: CardUI settings are stored globally and applied to all cards; the flip button only flips the currently viewed browser card and does not persist flip state.
- Q: Is feature 008 a temporary overlay or a permanent game change? → A: It permanently replaces the existing game cards and world background with newly created art for the 008 theme POC.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Demonstrate Match Atmosphere (Priority: P1)

A player enters the game scene and immediately sees a cohesive tactical card game mood: one active world background, atmospheric lighting, three centered tactical locations, and four character cards along the bottom.

**Why this priority**: The proof-of-concept exists to validate atmosphere, visual identity, and screen composition before gameplay rules are implemented.

**Independent Test**: Can be fully tested by opening the game scene and confirming that the visible composition communicates the selected world, tactical locations, and character card lineup without requiring gameplay actions.

**Acceptance Scenarios**:

1. **Given** the game scene is opened for the first time, **When** the scene finishes loading, **Then** the player sees either Bamboo Forest or Coastal Harbor as the active world with matching background, lighting, and three centered locations.
2. **Given** the game scene is visible, **When** the player looks at the lower screen area, **Then** four cards are displayed along the bottom and remain visually distinct from the active world.
3. **Given** the active world is Bamboo Forest, **When** the player views the scene, **Then** the background and locations communicate moonlit bamboo, fog, stealth, traps, shadow, and motion.
4. **Given** the active world is Coastal Harbor, **When** the player views the scene, **Then** the background and locations communicate docks, ships, stormy ocean, market energy, rain-soaked streets, banners, and maritime activity.

---

### User Story 2 - Cycle World Themes (Priority: P1)

A player presses `T` during the game scene to switch between the Bamboo Forest and Coastal Harbor world themes while preserving the current card lineup and card theme choices.

**Why this priority**: World cycling proves that environment identity, lighting, and location rendering can change independently from cards.

**Independent Test**: Can be fully tested by pressing `T` in the game scene and verifying that the world presentation changes while the cards do not change.

**Acceptance Scenarios**:

1. **Given** the game scene is showing Bamboo Forest, **When** the player presses `T`, **Then** the active world changes to Coastal Harbor.
2. **Given** the game scene is showing Coastal Harbor, **When** the player presses `T`, **Then** the active world changes to Bamboo Forest.
3. **Given** the world changes, **When** the new world is displayed, **Then** three locations are randomly selected from the six reusable tactical locations and rendered across the center of the screen.
4. **Given** the world changes, **When** the cards are visible, **Then** the same four card identities remain present and do not automatically adopt the active world theme.

---

### User Story 3 - Interact With Cards Elegantly (Priority: P2)

A player moves the cursor or touch position and sees the bottom cards subtly respond by leaning or looking toward that position.

**Why this priority**: Lightweight physical response supports the premium tactile feel of the prototype without implying full gameplay.

**Independent Test**: Can be fully tested by moving the cursor or touch position across the screen and verifying restrained, readable card motion.

**Acceptance Scenarios**:

1. **Given** the game scene is visible, **When** the cursor moves left of the card row, **Then** cards lean slightly left without obscuring each other.
2. **Given** the game scene is visible, **When** the cursor moves above the card row, **Then** cards tilt slightly upward without leaving the bottom card area.
3. **Given** a touch position is active, **When** the player touches or drags near the card row, **Then** the cards respond consistently with cursor-based leaning.

---

### User Story 4 - Browse a Selected Card (Priority: P2)

A player clicks or taps one of the four permanent game-scene cards to enter a focused Deck Builder for that specific card, where the selected card is enlarged and can be flipped temporarily or adjusted through global CardUI settings without changing the match world.

**Why this priority**: The Deck Builder proves individual character presentation, card flipping, and card theme independence from the active world.

**Independent Test**: Can be fully tested by selecting any bottom card, flipping it, changing CardUI settings with `T`, returning to confirm the world did not change, and confirming CardUI settings apply across all cards while flip state does not persist.

**Acceptance Scenarios**:

1. **Given** the game scene is visible, **When** the player clicks or taps Kage Ren, Lord Daichi, Sister Hotaru, or the placeholder Yokai card, **Then** the Deck Builder opens focused on that selected card.
2. **Given** the Deck Builder is open, **When** the player triggers a card flip, **Then** the selected enlarged card flips between front and back presentation.
3. **Given** the Deck Builder is open, **When** the player presses `T`, **Then** the stored CardUI settings change and the current card visuals update as appropriate.
4. **Given** the Deck Builder changes CardUI settings, **When** the player views any card, **Then** those settings are applied consistently to all cards.
5. **Given** the Deck Builder flips the current card, **When** the player leaves or selects another card, **Then** the flip state is not stored as a durable card state.
6. **Given** the Deck Builder changes CardUI settings, **When** the player returns to the game scene, **Then** the active world remains unchanged.

---

### Edge Cases

- If a world change randomly selects the same three locations as the previous world, the locations still re-render with the new world's visual adaptation.
- If a card is selected while cards are tilted toward the cursor, the Deck Builder opens the selected card without carrying over an exaggerated tilted pose.
- If `T` is pressed rapidly in the game scene, the active world remains one of the two supported worlds and the card lineup remains stable.
- If `T` is pressed rapidly in the Deck Builder, only global CardUI settings change and the match world remains stable.
- If a card is flipped in the Deck Builder and the player opens another card, the newly opened card is not required to inherit the prior card's temporary flipped state.
- If pointer or touch input is unavailable, the cards remain readable and selectable without tilt.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The proof-of-concept MUST present a game scene focused on atmosphere, visual identity, interactivity, and thematic cohesion rather than playable combat rules.
- **FR-002**: The temporary title "Card Katana" MUST be treated as presentation copy only and MUST NOT be required for namespaces, architecture labels, technical systems, or durable data identifiers.
- **FR-003**: The game scene MUST support exactly two switchable world themes for this proof-of-concept: Bamboo Forest and Coastal Harbor.
- **FR-004**: The Bamboo Forest world MUST communicate a dense moonlit bamboo wilderness associated with ninjas, stealth, hidden movement, traps, shadows, wind, drifting fog, and mystery.
- **FR-005**: The Coastal Harbor world MUST communicate wooden docks, merchant ships, stormy oceans, crowded marketplaces, banners, warehouses, taverns, rain-soaked streets, energy, and maritime activity.
- **FR-006**: The game scene MUST display the active world background and atmospheric lighting using newly created art for this feature.
- **FR-007**: The game scene MUST maintain a reusable pool of six tactical locations: Fortress Gate, Bamboo Crossing, Shrine Ruins, Battlefield, Spirit Well, and Market Square.
- **FR-008**: Whenever the active world changes in the game scene, the scene MUST randomly render three of the six tactical locations across the center of the screen.
- **FR-009**: Tactical locations MAY visually adapt to the active world while preserving their location identity.
- **FR-010**: The game scene MUST permanently display exactly one of each of the four proof-of-concept cards along the bottom of the screen.
- **FR-011**: Three cards MUST represent named unique characters: Kage Ren, Lord Daichi, and Sister Hotaru.
- **FR-012**: Kage Ren MUST read as a Ninja character: small, agile, stealthy, hidden, masked, fast, crouched low with dual blades and rope darts, against a moonlit bamboo forest with drifting fog, using a thin matte black frame with subdued green lacquer or inlay accents.
- **FR-013**: Lord Daichi MUST read as a Samurai character: large, armored, disciplined, physically imposing, holding a massive katana with layered crimson armor, against a burning battlefield with smoke and war banners, using a heavy dark steel frame with gold engraved corners.
- **FR-014**: Sister Hotaru MUST read as a Monk character: spiritual, calm, ceremonial, and grounded, holding a lantern-lit ceremonial staff and prayer beads, against a spiritual shrine with hanging or drifting lanterns and soft fog, using a soft wooden frame with warm orange lantern accents.
- **FR-015**: The fourth card MUST be a temporary placeholder Yokai card for prototype testing, reading as supernatural, asymmetrical, strange, and haunting.
- **FR-016**: Each card MUST feel like a unique individual character even when sharing a broader visual family.
- **FR-017**: Cards in the game scene MUST subtly rotate, lean, or look toward the cursor or touch position with restrained motion.
- **FR-018**: Clicking or tapping a card MUST open a Deck Builder focused on that specific card.
- **FR-019**: The Deck Builder MUST enlarge the selected card and focus on one card at a time.
- **FR-020**: The Deck Builder MUST support flipping the selected card between front and back presentation.
- **FR-021**: Pressing `T` in the Deck Builder MUST change stored CardUI settings and update card visuals as appropriate.
- **FR-022**: CardUI settings MUST be stored globally and applied to all cards rather than stored independently per card.
- **FR-023**: The Deck Builder flip control MUST only flip the currently viewed card for animation testing and MUST NOT store durable flip state.
- **FR-024**: World themes and CardUI settings MUST be separate systems from the user's perspective: changing the world MUST NOT change CardUI settings, and changing CardUI settings MUST NOT change the world.
- **FR-025**: The presentation MUST support the broader JW theme direction: a timeless mythic Japanese-warrior-inspired setting where samurai, ninjas, ronin, monks, pirates, yokai, and spirit warriors can coexist naturally.
- **FR-026**: The visual direction MUST use Japan Realism: cinematic feudal Japanese realism, chiaroscuro lighting, grounded atmospheric environments, realistic armor, cloth, wood, metal, rain, mist, smoke, torch fire, lantern fire, and natural lighting.
- **FR-027**: The visual direction MUST feel stylized, cinematic, atmospheric, grounded, mature, tactical, painterly, and realistic without becoming comic-book-like or arcade-fantasy-like.
- **FR-028**: The proof-of-concept MUST avoid visible magic, glowing fantasy effects, exaggerated supernatural energy, and non-believable weapons or silhouettes; grounded real-world effects such as mist, smoke, rain, embers, torch fire, and lantern light are allowed.
- **FR-029**: Cards MUST render with a 2:3 vertical card silhouette with primarily full-body characters.
- **FR-030**: Card front source textures MUST use one shared `840 x 1440` layer canvas for each front layer in the current proof-of-concept, and runtime card geometry MUST present those layers in the approved 2:3 rendered silhouette.
- **FR-031**: Card fronts MUST render, from furthest from camera to closest, as background layer, frame layer, safe-area reference layer, foreground character layer, and title layer.
- **FR-032**: The safe-area reference layer MUST be a transparent card-structure overlay with a green guide rectangle inset 40 pixels from each canvas edge and MUST render immediately in front of the frame layer when visible.
- **FR-033**: CardUI MUST expose a persisted `Show Safe Area` checkbox that toggles only the safe-area reference layer and does not change art assets, layer scale values, flip state, card identity, or world theme.
- **FR-034**: Card front background layers MUST fill the full `840 x 1440` canvas with opaque environment art and MUST NOT contain alpha padding, character silhouette, character body, title text, or frame art in the source texture.
- **FR-035**: A background MAY be runtime-masked by the frame aperture when the card design needs the background visible only inside the frame; this masking MUST NOT change the source requirement that the background texture itself is full-canvas opaque environment art.
- **FR-036**: Card front frame layers MUST be drawn primarily inside the safe-area guide and MUST provide the card's structural edge, border rhythm, and material identity.
- **FR-037**: Frame layers MAY be approximately rectangular rather than perfectly rectangular, may use slightly varied corner angles, may include inner and outer line treatments, and MUST vary by card identity rather than becoming one universal frame design.
- **FR-038**: Frame layers MUST keep transparent alpha outside their painted frame marks and MUST NOT include character art, title text, or baked safe-area guide lines.
- **FR-039**: Card front foreground layers MUST contain only the character art with alpha around the character silhouette.
- **FR-040**: Foreground characters MUST be mostly contained within the safe-area guide while being allowed to break out of the guide for intentional emphasis such as a weapon, foot, arm, head, cloak edge, or pose gesture.
- **FR-041**: Foreground art MUST never be clipped by the `840 x 1440` image borders; every opaque or antialiased foreground pixel MUST remain inside the canvas with enough transparent padding to avoid hard edge cuts.
- **FR-042**: Foreground breakout MUST feel designed and selective rather than simply oversized; the primary face, torso, and stance SHOULD remain readable within or near the safe-area guide.
- **FR-043**: Card front title layers MUST contain only the character name as unique stylized raster art with alpha around the title shape; they MUST NOT use dynamic Bevy UI text, subtitles, plaques, or invented extra copy.
- **FR-044**: Title art MUST be mostly inside the safe-area guide but MAY intentionally break out of the guide when it improves composition.
- **FR-045**: Title art MAY be positioned at the bottom, top, or slightly off-center as long as it supports the card identity, remains readable at game-scene and Deck Builder sizes, and never clips against the `840 x 1440` image borders.
- **FR-046**: AI-generated source images for card foreground and title alpha extraction MUST use `#ff00ff` as the chroma-key color and MUST NOT use that color inside the subject or title artwork.
- **FR-047**: CardUI MUST expose four layer scale sliders named `LayerScale: Background`, `LayerScale: Frame`, `LayerScale: Foreground`, and `LayerScale: Title`.
- **FR-048**: Each layer scale MUST default to `1.0`, allow values from `0.0` through `2.0`, apply uniformly to x and y scale, and preserve the layer image center point while scaling.
- **FR-049**: Each layer scale slider MUST expose a nearby reset control that restores that layer scale to its default value of `1.0` and persists the reset through global CardUI settings.
- **FR-050**: Card art SHOULD be authored so a visually strong default composition is achieved with all layer scale sliders at `1.0`; slider overrides are tuning aids and MUST NOT be required for a card to avoid clipping or basic misalignment.
- **FR-051**: Card character silhouettes and stances MUST be readable at game-scene card size and enlarged Deck Builder size.
- **FR-052**: The character on each card SHOULD occupy roughly 70-80% of the card height before intentional breakout treatment, with the environment supporting atmosphere while remaining secondary to the character.
- **FR-053**: Cards MUST feel like cinematic character posters rather than comic-book splash art.
- **FR-054**: The visual direction MAY draw tonal inspiration from grounded cinematic Japanese warrior media such as Ghost of Tsushima, Shogun, 13 Assassins, and Seven Samurai without directly copying protected characters, shots, marks, or compositions.
- **FR-055**: Completing this proof-of-concept MUST permanently change the game scene to use the four-card bottom lineup and selected-card Deck Builder flow.
- **FR-056**: Completing this proof-of-concept MUST permanently replace the existing game cards and world background with newly created Japan Realism theme art.
- **FR-057**: The proof-of-concept MUST stay scoped to theme presentation and interaction validation; it MUST NOT require combat resolution, deckbuilding, economy, AI turns, scoring, or full match rules.

### Key Entities

- **World Theme**: A match environment identity that controls background, atmospheric lighting, and visual adaptation of tactical locations. For this proof-of-concept, supported values are Bamboo Forest and Coastal Harbor.
- **Tactical Location**: A reusable gameplay space shown in the center of the game scene. The six location identities are Fortress Gate, Bamboo Crossing, Shrine Ruins, Battlefield, Spirit Well, and Market Square.
- **Card Character**: A unique visible card identity with a name, title, visual family, background, foreground subject, and frame treatment.
- **CardUI Settings**: Stored visual settings for card presentation that are applied globally to all cards and intentionally separate from the active world theme.
- **Deck Builder**: A focused card viewing mode that enlarges one selected card and supports temporary flipping plus global CardUI setting changes.
- **Art Direction**: The visual contract for the proof-of-concept, defined as Japan Realism with grounded cinematic realism, restrained atmosphere, believable materials, real-world fire and weather effects, and no visible fantasy glow or magic effects.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A tester can identify whether the active world is Bamboo Forest or Coastal Harbor within 5 seconds of viewing the game scene.
- **SC-002**: In 10 consecutive presses of `T` in the game scene, the active world alternates only between Bamboo Forest and Coastal Harbor, and the four card identities remain unchanged every time.
- **SC-003**: After each game-scene world change, exactly three visible tactical locations are shown across the center of the screen.
- **SC-004**: At least 4 out of 5 testers can correctly distinguish Ninja, Samurai, Monk, and Yokai visual families from the four bottom cards without reading implementation notes.
- **SC-005**: A tester can open the Deck Builder from any of the four cards in no more than one click or tap.
- **SC-006**: In 10 consecutive presses of `T` inside the Deck Builder, global CardUI settings change without changing the active game-scene world.
- **SC-007**: Card tilt response remains subtle enough that all card names and primary silhouettes remain readable during normal cursor or touch movement.
- **SC-008**: The complete proof-of-concept can be demonstrated in under 2 minutes: view game scene, cycle worlds, select a card, flip it, cycle CardUI settings, and confirm world/CardUI separation.
- **SC-009**: At least 4 out of 5 testers describe the card and world presentation as grounded, cinematic, mature, and tactical rather than cartoonish, magical, or arcade-fantasy.
- **SC-010**: At least 4 out of 5 testers can read the primary stance and silhouette of each named character at both bottom-row card size and enlarged Deck Builder size.
- **SC-011**: After changing CardUI settings in the Deck Builder, a tester can view multiple cards and confirm the same CardUI settings apply to all cards.
- **SC-012**: After flipping a card in the Deck Builder and opening another card, a tester can confirm the prior flip state was not stored as durable card state.
- **SC-013**: A tester can confirm the game scene no longer uses the pre-008 cards or world background after the feature is complete.

## Assumptions

- The provided inspiration image informs mood, lighting, framing density, and premium card treatment, while the final POC should use the approved 2:3 vertical card silhouette rather than the reference image's horizontal card layout.
- The prototype prioritizes static or lightly animated newly created art over finalized production art.
- The Deck Builder includes at least two CardUI setting states so that pressing `T` visibly changes card presentation.
- The active world at first load may be either Bamboo Forest or Coastal Harbor.
- Random location selection may repeat across world changes unless a later requirement explicitly forbids repeats.
- The placeholder Yokai card may use provisional naming or labeling as long as it is clearly temporary and visually tests the Yokai/spirit-warrior direction.
