# Research: DeckScreen Meta Game UI

## Decision: Add reusable top navigation concepts, scoped to DeckScreen mounting

**Rationale**: The mockups show the same top nav pattern across pages. A reusable model/view avoids duplicating destination labels and selected-state rules later, while mounting it only on DeckScreen keeps this feature scoped.

**Alternatives considered**: Hard-coding DeckScreen-only nav was rejected because it would create immediate duplication when other screens adopt the same navigation. Implementing all other pages now was rejected because the user explicitly scoped this feature to DeckScreen.

## Decision: Use a DeckScreen model resource for screen-local state

**Rationale**: DeckScreen has deck selection, editor, library tab, shop tab, and modal overlay states. A focused `DeckScreenModel` keeps those separate from gameplay round, hand, slot, and selected-card inspection models.

**Alternatives considered**: Encoding DeckScreen state directly in `ActiveView` was rejected because `ActiveView` only identifies the active child scene.

## Decision: Reuse existing persisted player deck collection

**Rationale**: The runtime already has `PlayerDeckCollectionModel`, deck names, card IDs, and persistence paths. Reusing it avoids duplicate deck sources and supports restart persistence.

**Alternatives considered**: A new deck storage file was rejected because it would duplicate ownership and ordering data.

## Decision: Keep modal separate from gameplay selected-card modal

**Rationale**: DeckScreen modal has action rail commands and persistence effects, while gameplay selected inspection is match-specific. Separate models avoid cross-screen coupling.

**Alternatives considered**: Extending the gameplay selected modal was rejected because it would mix match input blocking, placement gestures, and deck editing commands.

## Decision: Shop UI is visible but purchase execution remains out of scope

**Rationale**: The mockup requires the Shop tab, prices, and Lightning-styled Buy buttons. Actual Lightning settlement requires authentication/payment behavior that is not specified by this feature.

**Alternatives considered**: Implementing real purchase flow was rejected as a separate feature requiring payment contracts and secret handling rules.

## Deck View Constraint

| Requirement | Detail |
| ----------- | ------ |
| DeckViewBundle | Implementation MUST create a DeckViewBundle that renders a deck tile using the existing card back asset and the deck name only. |
