# Contract: Card Flip UI

## Surface

The temporary `Card UI` window inside the current `DeckBuilder` prototype entry point exposes a `Flip` button alongside card presentation controls. `Card UI` is separate from DebugHUD. `DeckBuilder`, Card UI, and DebugHUD are not treated as final user-facing game UI by this feature.

## User Actions

| Action | Preconditions | Expected Result |
| ------ | ------------- | --------------- |
| Click `Flip` from CardFront idle | Card UI is visible and CardFront is active | Card starts animating toward CardBack within one frame |
| Click `Flip` from CardBack idle | Card UI is visible and CardBack is active | Card starts animating toward CardFront within one frame |
| Click `Flip` during animation | A flip is already in progress | Animation reverses direction from current progress |
| Move pointer during animation | Card is visible | Pointer-driven inspection continues to affect the non-flip tilt behavior |
| Press `T` while CardFront is visible | At least two front entries are available | The visible CardFront changes immediately |
| Press `T` while CardBack is visible | At least two front entries are available | CardBack remains visible; the changed front appears only after flipping face up |
| Inspect CardBack art direction | CardBack is visible | CardBack reads as a bold abstract superhero-pattern design compatible with current fronts |

## Visual Contract

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
| Gameplay | No scoring, turns, targeting, dragging, or rules are introduced |
| Deck browsing | No multi-card deck or collection UI is introduced |
| Front-specific backs | CardBack does not vary by active card front in this feature phase |
| CardBack content | CardBack contains no words, readable letters, characters, logos, or clear icon-like symbols |
| Future brand surfaces | Game box cover art and main menu art are not designed by this feature |
| Final game UI | DeckBuilder is not promoted to final user-facing game UI by this feature |
| DebugHUD separation | Card UI remains separate from DebugHUD and the Flip button is not a DebugHUD toggle |
