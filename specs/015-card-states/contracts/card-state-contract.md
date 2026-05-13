# Contract: Card State Model

## State Axes Contract

| Axis | Must Be Independent From | Required Behavior |
| ---- | ------------------------ | ----------------- |
| Face | Zone and interaction | A card can render front or back because of flip, reveal, or animation without changing where it is. |
| Zone | Face and interaction | Deck, hand, and location membership must remain valid while a card animates, is selected, or is dragged. |
| Interaction | Durable zone | Selection and dragging overlay a source zone and must know where to return. |
| Reveal policy | Render layer internals | Hidden/revealed knowledge rules must derive visible face, not replace face-layer rendering. |
| Pose | Durable gameplay legality | Transforms and z bands must be derived from zone plus interaction and safe-area layout. |

## Legal Combination Contract

| Zone | Lock State | Reveal Policy | Allowed Interaction | Expected Face |
| ---- | ---------- | ------------- | ------------------- | ------------- |
| Deck | N/A | Owner-visible or hidden by absence | None | Back or no view |
| Hand, local human | N/A | OwnerVisible | Idle, pressed, selected, dragging, returning | Front |
| Hand, CPU/opponent | N/A | OwnerVisible to owner | Idle/passive animation | Back or front by viewer policy |
| Location, current round | CurrentRoundMovable | OwnerVisible or CurrentRoundHiddenToOpponent | Idle, selected, dragging, returning, settling placed | Front to owner; back to hidden opponent |
| Location, locked | Locked | RevealedToAll | Idle or selected | Front |
| Location, locked | Locked | CurrentRoundHiddenToOpponent | Idle/passive | Back to non-owner until reveal event |

## Invalid Combination Contract

| Combination | Required Handling |
| ----------- | ----------------- |
| Deck card selected or dragged | Reject interaction before creating active gesture. |
| Locked location card dragged | Reject drag start and leave durable zone unchanged. |
| Hidden card revealed through front face to non-owner | Derivation bug; tests must fail. |
| One card in two zones | Model validation failure. |
| Slot populated by an instance whose zone is not matching location/side/slot | Model validation failure. |
| More than one active interaction | New press is ignored or existing interaction resolves first. |
| Gesture references missing card instance | Clear interaction or reject transition. |

## Derivation Contract

| Derived Output | Inputs | Rule |
| -------------- | ------ | ---- |
| `visible_face` | `CardRevealPolicy`, viewer side, flip/animation state | Owner and revealed cards may show front; hidden current-round opposing cards show back. |
| `pose` | `CardZoneModel`, `CardInteractionModel`, safe GameScene layout | Interaction pose wins while active; otherwise zone pose wins. |
| `is_draggable` | Owner/controller, zone, lock state, interaction | Local human hand and current-round movable location cards are draggable when no other interaction blocks them. |
| `is_selectable` | Owner/controller, zone, reveal policy | Local hand and location cards are selectable when visible to the local player. |
| Slot occupancy | `CardPlacementModel` and `CardZoneModel::Location` | Occupancy and card zone must agree on location, side, and slot. |

## Compatibility Contract

| Existing Behavior | Required Preservation |
| ----------------- | --------------------- |
| Hand click selects inside `GameScene` | ✅ |
| Drag threshold suppresses click inspection | ✅ |
| Drag from hand to first empty local slot | ✅ |
| Same-round placed card can return to hand | ✅ |
| Locked placed card cannot be dragged | ✅ |
| Current-round hidden opponent card renders face down to opponent | ✅ |
| CPU-owned cards are passive to local drag/hover | ✅ |
| `CardViewBundle` creates front/back layers and point layers | ✅ |
