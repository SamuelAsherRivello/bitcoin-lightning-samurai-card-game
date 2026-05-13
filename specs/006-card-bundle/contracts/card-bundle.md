# Contract: Card Bundle

## Surface Contract

| Surface | Contract |
| ------- | -------- |
| Prototype entry | The current app entry may use DeckBuilder as a prototype surface, but this spec does not promote it to final game UI |
| Card count | The card presentation bundle exposes one focused inspectable card in the current prototype surface |
| DebugHUD | DebugHUD remains visible by default, owns diagnostics and `T`/`R`/`H`, and stays separate from Card UI |
| Card UI | Temporary prototype control surface that owns the `Flip` button |
| Camera | Pointer and flip behavior never mutate the fixed primary camera transform |

## Inspection Contract

| Requirement | Contract |
| ----------- | -------- |
| Dimensions | Card uses poker proportions, 63 mm by 88 mm, with 88:63 ratio within 2% |
| Geometry | Card reads as a thin slab with no bevel and no visibly separated layer stack |
| Pointer mapping | Center maps to neutral; edges/corners map to corresponding card-facing directions |
| Tilt limit | Supported pointer tilt is clamped to 20 degrees from neutral |
| Smoothing | Runtime rotation moves toward target and reaches it within 100 ms |

## CardFront Contract

| Layer | Contract |
| ----- | -------- |
| Background | Appears behind the frame and only through the frame aperture |
| Frame | Acts as the visual reference plane, has continuous UV mapping, and carries tilt-reactive shine |
| Foreground | Appears in front of the frame and may overlap the aperture or frame |
| Title | Appears frontmost and may overlap foreground, frame, or aperture |

## User Actions

| Action | Preconditions | Expected Result |
| ------ | ------------- | --------------- |
| Move pointer | Card is visible | Card inspection tilt updates smoothly; camera remains fixed |
| Press `T` while CardFront is visible | At least one valid front exists | Active front remains valid; visible front updates when another available front exists |
| Press `T` while CardBack is visible | Card is face down | CardBack remains visible; changed CardFront appears only after flipping face up |
| Press `R` | DebugHUD input active | Reloadable AppScene card content rebuilds without changing DebugHUD toggle state |
| Press `H` | DebugHUD input active | Hot-reload auto-restart toggles and persists through local runtime state |
| Click `Flip` from CardFront idle | Card UI is visible and CardFront is active | Card starts animating toward CardBack within one frame |
| Click `Flip` from CardBack idle | Card UI is visible and CardBack is active | Card starts animating toward CardFront within one frame |
| Click `Flip` during animation | A flip is already in progress | Animation reverses direction from current progress |
| Move pointer during animation | Card is visible | Pointer-driven inspection continues to affect non-flip orientation behavior |

## Visual Side Contract

| Moment | Required Visible Side |
| ------ | --------------------- |
| Front-to-back before midpoint | CardFront |
| Front-to-back after midpoint | CardBack |
| Back-to-front before midpoint | CardBack |
| Back-to-front after midpoint | CardFront |
| Edge-on midpoint | Card appears thinnest and side graphics may switch |

## Out Of Scope

| Area | Contract |
| ---- | -------- |
| Gameplay | No scoring, rounds, targeting, rules, tabletop placement, or reveal gameplay is introduced |
| Deck browsing | No collection browser or multi-card deck UI is introduced |
| Front-specific backs | CardBack does not vary by active CardFront in this feature phase |
| CardBack content | CardBack contains no words, readable letters, characters, logos, or clear icon-like symbols |
| Future brand surfaces | Game box cover art and main menu art are not designed by this feature |
| Final game UI | DeckBuilder and Card UI are not final user-facing game UI |
