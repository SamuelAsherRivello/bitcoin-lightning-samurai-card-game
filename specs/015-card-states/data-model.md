# Data Model: Card View State Model

## Current State Hierarchy

| Hierarchy | Current Artifact | State Values | Relationship |
| --------- | ---------------- | ------------ | ------------ |
| Visual root | `CardViewBundle` / `CardView` | Spawned root, `Transform`, `Visibility` | Root for child layers; not authoritative for gameplay location. |
| Face presentation | `CardFace`, `CardFaceLayer`, `CardFlipState` | `Front`, `Back` | Selects child layer visibility for non-CPU cards. |
| Card layer composition | `CardParallaxLayer`, `CardLayerRole`, `CardBackgroundLayer`, `CardFrameLayer` | Background, frame, safe area, foreground, title | Visual-only hierarchy below the root. |
| Durable local card zone | `CardStateModel` | `Hand`, `Dragging`, `Location`, `LocationLocked` | Tracks local hand-index state; `Location` means current-round movable location card. |
| Durable slot occupancy | `CardSlotBoardModel` / `CardSlotState` | Empty, populated with `hand_index` and `card_id` | Tracks location/side/slot occupancy and layout rect. |
| Active interaction | `CardGestureModel` | `Idle`, `Pressed`, `SelectedInspecting`, `Dragging`, `Returning`, `Placed` | One active local card interaction overlay at a time. |
| Hand identity/order | `GameHandModel`, `HandCardGestureTarget` | Card IDs and hand indices | Provides rendered hand cards and gesture targets. |
| Opponent reveal | `PlacementVisibilityModel` | `CurrentTurnHidden`, `Revealed` | Determines whether non-owner viewers know a placed card. |
| CPU presentation | `CpuHandCardView`, `CpuPlacedCardView`, `CpuPlacedCardAnimation` | Passive hand/placed card, visible face, animation phase | Parallel render path for CPU-owned cards. |

## Current Legal Combinations

| Durable State | Face | Interaction | Legal? | Notes |
| ------------- | ---- | ----------- | ------ | ----- |
| Deck | Back or none | None | ✅ | Deck cards are usually model-only and not `CardGestureView` roots. |
| Hand | Front for local, often back for opponent/CPU | Idle, pressed, selected, dragging, returning | ✅ | Local hand cards are gesture targets; CPU hand cards are passive. |
| Location current turn | Front for owner, back if hidden to non-owner | Idle, selected, dragging, returning, placed animation | ✅ | Current implementation names this `CardState::Location`. |
| Location locked | Front when revealed, back when hidden | Idle or selected only | ✅ | Locked cards cannot begin drag. |
| Deck | Any | Selected or dragging | ❌ | Deck cards are not interactive card views. |
| Location locked | Any | Dragging | ❌ | Prior-round/locked placements are not draggable. |
| Any card | Any | Multiple active gestures | ❌ | `CardGestureModel` allows one active focus. |

## Proposed Entities

| Entity | Fields | Relationships | Validation |
| ------ | ------ | ------------- | ---------- |
| `CardInstanceId` | Stable numeric or compact string ID | Primary key for runtime card instance state, views, slots, and gestures | Must be unique within an active match. |
| `CardInstanceStateModel` | `instance_id`, `card_model_id`, `owner`, `zone`, `reveal_policy` | Durable card truth independent of render entity | A card has exactly one zone at a time. |
| `CardZoneModel` | `Deck { deck_index }`, `Hand { order_index }`, `Location { location_index, side, slot_index, lock_state }`, `OutOfPlay` | Replaces overloaded hand-index state | Only `Hand` and same-turn `Location` may start local drag. |
| `LocationLockState` | `CurrentTurnMovable`, `Locked` | Detail inside `CardZoneModel::Location` | End turn changes current-turn movable cards to locked after reveal/turn resolution. |
| `CardRevealPolicy` | `OwnerVisible`, `CurrentTurnHiddenToOpponent`, `RevealedToAll` | Derives per-viewer knowledge and desired face | Hidden-to-opponent is valid only for location placements. |
| `CardInteractionModel` | `state`, `instance_id`, `pointer`, `source_pose`, `target_pose`, `resolved_destination` | Replaces active-hand-index focus with instance focus | At most one active interaction; selected and dragging are overlays, not zones. |
| `CardInteractionState` | `Idle`, `Pressed`, `SelectedInspecting`, `Dragging`, `Returning`, `SettlingPlaced` | Temporary input/animation state | `Dragging` requires a draggable durable zone. |
| `CardViewStateModel` | `instance_id`, `visible_face`, `pose`, `z_band`, `is_input_enabled`, `layer_visibility` | Derived by systems and applied to `CardView` entities | Does not decide gameplay legality. |
| `CardPlacementModel` | `instance_id`, `location_index`, `side`, `slot_index`, `placed_turn` | Slot occupancy by instance ID | Slot may contain zero or one card instance. |
| `HandOrderModel` | Ordered `CardInstanceId` list by owner | Replaces hand order embedded in `CardStateModel` | Contains only cards whose zone is `Hand`. |

## Proposed State Axes

| Axis | Values | Authoritative Owner |
| ---- | ------ | ------------------- |
| Identity | `CardInstanceId`, `card_model_id`, owner side | `CardInstanceStateModel` |
| Zone | Deck, hand, location, out of play | `CardZoneModel` |
| Location detail | Location index, side, slot index, current-turn movable/locked | `CardZoneModel::Location` |
| Reveal | Owner-visible, current-turn hidden to opponent, revealed to all | `CardRevealPolicy` |
| Interaction | Idle, pressed, selected, dragging, returning, settling placed | `CardInteractionModel` |
| Face | Front, back | `CardViewStateModel.visible_face`, derived from reveal/viewer/animation |
| Pose | Deck, hand slot, selected inspection, drag preview, location slot, return target | `CardViewStateModel.pose`, derived from zone plus interaction |
| Input affordance | None, selectable, draggable, passive | Derived from owner/controller, zone, lock state, and interaction |

## Proposed Transitions

| Transition | From | To | Rule |
| ---------- | ---- | -- | ---- |
| Draw card | `Deck` | `Hand { order_index }` | Removes from deck order and appends to hand order. |
| Press card | `Hand` or `Location(CurrentTurnMovable)` | Interaction `Pressed` | Requires no active interaction and local human control. |
| Select card | Interaction `Pressed` | Interaction `SelectedInspecting` | Durable zone does not change. |
| Begin drag | Interaction `Pressed` | Interaction `Dragging` | Durable zone must be draggable. |
| Place card | `Hand` + dragging | `Location(CurrentTurnMovable)` + `SettlingPlaced` | Requires empty legal local slot and energy/rule success. |
| Return current-turn card | `Location(CurrentTurnMovable)` + dragging | `Hand { order_index }` + `Returning` | Frees slot and restores hand order/energy. |
| Reject drag | Dragging | Original durable zone + `Returning` | Durable state returns to captured source. |
| End turn reveal | `Location(CurrentTurnMovable)` | `Location(Locked)` | Reveal policy becomes `RevealedToAll` after end-turn reveal. |
| CPU place hidden | CPU hand | `Location(CurrentTurnMovable)` with `CurrentTurnHiddenToOpponent` | Visible face is back for non-owner until reveal. |

## Migration Notes

| Current Artifact | Proposed Fate |
| ---------------- | ------------- |
| `CardViewBundle` | Keep as render-root bundle. |
| `CardFace` | Keep as render-facing visible face enum. |
| `CardStateModel` | Replace or wrap with `CardInstanceStateModel` and `HandOrderModel`. |
| `CardSlotBoardModel` | Keep geometry; change occupancy from `hand_index` to `CardInstanceId` when migration reaches slots. |
| `CardGestureModel` | Replace `active_hand_index` with `active_instance_id`; keep pointer/source/target behavior. |
| `PlacementVisibilityModel` | Fold into `CardRevealPolicy` or keep as opponent-mode adapter until migration completes. |
| `CpuHandCardView` / `CpuPlacedCardView` | Derive from `CardViewStateModel` and owner/controller affordance. |

## Implemented Migration Helpers

| Helper | Source State | Output | Notes |
| ------ | ------------ | ------ | ----- |
| `local_instances_from_existing_state` | `GameHandModel`, `CardStateModel`, `CardSlotBoardModel` | `CardInstanceStateCollectionModel` | Maps current `hand_index` identity into stable `CardInstanceId` values while preserving hand order and local slot placement. |
| `instance_from_cpu_hand_view` | `CpuHandCardView` | `CardInstanceStateModel` | Keeps CPU hand cards passive by representing ownership and hand zone without gesture state. |
| `instance_from_cpu_placed_view` | `CpuPlacedCardView` plus optional `PlacementVisibility` | `CardInstanceStateModel` | Maps hidden current-turn placements to `CurrentTurnHiddenToOpponent` and revealed placements to `RevealedToAll`. |
| `reveal_policy_from_placement` | `PlacementVisibilityModel` | `CardRevealPolicy` | Provides a bridge while opponent-mode reveal state remains in the current match model. |

## Slot Occupancy Migration Path

| Step | Change | Validation |
| ---- | ------ | ---------- |
| 1 | Keep `CardSlotBoardModel` geometry and current `hand_index` occupancy. | Adapter tests confirm existing state maps to `CardInstanceStateModel`. |
| 2 | Add `CardPlacementModel` as the instance-id slot identity beside existing slot occupancy. | `validate_slot_occupancy` checks that placement identity and card zone agree. |
| 3 | Replace `CardSlotState::Populated { hand_index, card_id }` with an instance-id-backed occupant in a follow-up implementation. | Existing gesture and opponent tests should pass through adapters before direct replacement. |
| 4 | Update gesture systems to use `active_instance_id` and slot instance identity. | Drag, return, lock, and reveal workflows remain behavior-compatible. |
