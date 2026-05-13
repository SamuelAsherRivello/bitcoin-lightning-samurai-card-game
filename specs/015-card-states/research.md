# Research: Card View State Model

## Current State Ownership

| State Axis | Current Owner | Current Values | Notes |
| ---------- | ------------- | -------------- | ----- |
| Render root | `CardViewBundle` and `CardView` | Spawned/visible root with transform | The bundle does not know deck, hand, location, selected, or drag semantics. |
| Face layer | `CardFace`, `CardFaceLayer`, `CardFlipState`, `update_card_face_visibility` | `Front`, `Back` | Global flip state controls non-CPU card layers; CPU placed cards opt out through `CpuPlacedCardFaceLayer`. |
| Layer role | `CardParallaxLayer`, `CardBackgroundLayer`, `CardFrameLayer`, `CardUiState` | Background, frame, safe area, foreground, title | This is visual composition state, not gameplay state. |
| Local gameplay zone | `CardStateModel` | `Hand`, `Dragging`, `Location`, `LocationLocked` | The implementation uses `Location`; the older gesture spec names this concept `LocationCurrentRound`. |
| Hand membership | `GameHandModel`, `HandCardGestureTarget`, `CardStateModel.hand_order` | Hand card IDs and hand indices | Hand index is currently both identity and ordering handle for local cards. |
| Deck membership | `GameDeckModel`, `OpponentMatchModel` player decks | In deck, drawn out of deck | Deck cards generally have no `CardViewBundle` until rendered as hand/deck animation. |
| Location occupancy | `CardSlotBoardModel`, `CardSlotState` | Empty or populated with `hand_index` and `card_id` | Slot occupancy duplicates part of local card zone state. |
| Active gesture | `CardGestureModel` | `Idle`, `Pressed`, `SelectedInspecting`, `Dragging`, `Returning`, `Placed` | Single active focus; owns source/target transforms and pointer offset. |
| Drop feedback | `DropTargetHint` | Hidden/visible and close/available colors | Derived from dragging state and slot availability. |
| Opponent reveal | `PlacementVisibilityModel` | `CurrentTurnHidden`, `Revealed` | This is gameplay knowledge/reveal state, separate from front/back layer mechanics. |
| CPU card rendering | `CpuHandCardView`, `CpuPlacedCardView`, `CpuPlacedCardAnimation` | Hand/placed, visible face, animation phase | Parallel CPU path bypasses local gesture markers. |

## Decisions

| Decision | Rationale | Alternatives Considered |
| -------- | --------- | ----------------------- |
| Keep `CardViewBundle` presentation-only | The bundle currently creates a root entity and child layers from a `CardModel`; making it own gameplay state would mix view construction with match rules. | Adding deck/hand/location fields to the bundle was rejected because the same visual root can represent hand, selected, dragged, or placed poses over time. |
| Model state as independent axes | Face, zone, reveal, interaction, and pose change for different reasons and on different schedules. | One `CardViewState` enum was rejected because combinations such as `Location + Selected` and `Location + CurrentTurnHidden + Back` would explode into many variants. |
| Use stable card instance identity | Current local state keys by `hand_index`, which becomes awkward once cards move between deck, hand, location, and opponent ownership. | Continuing to key by hand index was rejected because location slots already store both `hand_index` and `card_id`, and opponent placements need side-aware identity. |
| Treat selection and dragging as interaction overlays | A card can be durable `Hand` or `LocationCurrentRound` while transiently selected, dragged, returning, or settling. | Baking `Selected` and `Dragging` into the durable zone enum was rejected because it obscures where the card returns after interaction. |
| Derive visible face from reveal policy plus render orientation | `PlacementVisibility` answers what the viewer may know; `CardFace` answers which mesh layers are visible. | Using only `CardFace` was rejected because a face-down opponent card and a manually flipped Deck Builder card have different rule meaning. |

## State Hierarchy

| Level | Owns | Examples |
| ----- | ---- | -------- |
| Match/card instance | Durable identity and legal card location | In deck, in hand, in location slot, discarded/future out-of-play |
| Zone detail | Zone-specific references | Deck index, hand order, location/side/slot, same-turn movable/locked |
| Visibility policy | What a viewer/controller may know | Owner visible, current-turn hidden to opponent, revealed |
| Interaction overlay | Temporary user input state | Pressed, selected, dragging, returning, placed animation |
| Render view | Derived pose and layers | Front/back visible face, z band, transform, scale, layer visibility |

## Risks

| Risk | Mitigation |
| ---- | ---------- |
| Existing code relies on `hand_index` for many transitions | Add an adapter that maps `CardInstanceId` to current hand index/order during migration. |
| CPU and local paths already diverge | Model owner/controller as data, then let interaction affordances decide whether a card is draggable or passive. |
| Face and reveal semantics can be confused | Name fields `reveal_policy` and `visible_face` separately and test their derivation. |
| Large refactor could destabilize gestures | Start with tests and model-only changes, then migrate gesture systems in small steps. |
