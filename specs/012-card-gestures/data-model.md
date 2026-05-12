# Data Model: Card Gestures

## Entities

| Entity | Fields | Relationships | Validation |
| ------ | ------ | ------------- | ---------- |
| `PointerGestureModel` | `pointer_id`, `start_position`, `current_position`, `pressed_card`, `card_center_offset`, `has_crossed_drag_threshold` | Feeds `CardGestureModel` while a pointer is active | A gesture becomes drag once movement crosses the threshold and cannot also resolve as click; card center offset remains stable during drag |
| `CardGestureModel` | `active_card`, `state`, `source_position`, `target_position`, `source_slot`, `target_slot`, `resolved_destination` | Owns the single active focus for inspection, drag, placement, or return | At most one card may be selected, pressed for drag, dragged, or returning because of a gesture |
| `CardGestureState` | `Idle`, `Pressed`, `SelectedInspecting`, `Dragging`, `Returning`, `Placed` | State machine for a hand card gesture | Invalid transitions are ignored or resolved back to a valid state |
| `CardGestureDestination` | `HandCardSlot`, `LocationCardSlot` | Records where a successful or cancelled drag resolves | Drag resolution can only end at the original hand card slot or an assigned destination location card slot |
| `CardStateModel` | card index to `CardState` mapping plus current-round placement identity | Tracks whether each playable card is in hand, being dragged, placed this round, or locked at a location | `Hand` cards and current-round placed cards can begin allowed drag gestures |
| `CardState` | `Hand`, `Dragging`, `LocationCurrentRound`, `LocationLocked` | Runtime gameplay state for a card during this feature | `Dragging` is temporary during an active drag; `LocationCurrentRound` cards may return to hand; `LocationLocked` cards are not draggable |
| `HandCardGestureTarget` | `card_id`, `hand_index`, entity reference, source transform/size | Attached to local-player hand card previews | Only local hand cards are valid gesture sources |
| `CardSlotBoardModel` | three location entries, each with opponent and local slot rows | Owns slot legality and occupancy independent of rendering | Exactly three location areas; each row has four slots |
| `CardSlotModel` | `location_index`, `side`, `slot_index`, `rect`, `state`, `occupant_card` | Represents one board slot under or above a location | Local-player slots may accept direct drag only when empty; opponent slots reject direct local placement; local slots are assigned in upper-left, upper-right, lower-left, lower-right order |
| `CardSlotRect` | `left`, `top`, `width`, `height` | Shared safe-GameView slot geometry used by debug drawing, hit testing, and placement transforms | Runtime slot geometry matches the debug-drawn reference lines |
| `LocationPowerPointView` | `location_index`, `side`, rendered `PowerPointModel` | Presents the current score for one player side at one shared location | Value is derived from assigned card slots, not stored as independent gameplay truth |
| `CardSlotSide` | `LocalPlayer`, `Opponent` | Distinguishes valid local placement side from opponent side | Local human player can directly place only into `LocalPlayer` slots |
| `CardPlacementModel` | `card_id`, `location_index`, `slot_index`, `final_transform/size` | Result of a successful drag release | Placement requires an empty local-player slot |
| `HandInsertionModel` | `card_id`, `candidate_index`, shifted hand transforms, gap rect | Result of dragging a current-round placed card over the hand area | Candidate index may be before, between, or after visible hand cards; release over a valid gap returns the card to that hand order |
| `CardGestureView` | rendered card entity, visual state, animation target | Presents selected, dragged, returning, or placed card states | Card aspect ratio must be preserved in all gesture states |
| `DropTargetHint` | `location_index`, visible rectangle state | User-facing highlight for a location's local slots area while dragging | Visible only for locations with at least one empty local-player slot |

## State Transitions

| Transition | Input | Output |
| ---------- | ----- | ------ |
| Press hand card | Pointer press on local hand card with no active pointer gesture | `Pressed` gesture records card and hand card slot source position |
| Press locked location card | Pointer press on card with `CardState` `LocationLocked` | Press is ignored for drag start |
| Press current-round location card | Pointer press on card with `CardState` `LocationCurrentRound` | `Pressed` gesture records card and location slot source position |
| Press while another pointer gesture is active | Pointer press while a card is already `Pressed` or `Dragging` | New press is ignored and existing active card remains authoritative |
| Resolve click/tap | Pointer release before crossing threshold | Card becomes `SelectedInspecting` in `GameView` |
| Select another card | Click another hand card while one is selected | Prior selected card returns; new card becomes selected |
| Click selected card | Pointer click/tap on selected inspected card | Card transitions to `Returning` and then `Idle` at hand position |
| Begin drag | Pointer movement crosses threshold while pressing hand card | Gesture becomes `Dragging`; inspection click is suppressed |
| Enter card drag state | Gesture becomes `Dragging` | Card state changes from `Hand` to `Dragging` |
| Begin drag scale | Gesture first becomes `Dragging` | Card grows to 150% of hand size over 0.25 seconds with ease-out motion |
| Hover valid location area | Dragged card overlaps a local-player slots area for a location with at least one empty slot | Location is eligible for automatic slot assignment on release and shows DropTargetHint |
| Release valid location area | Pointer release over an available local-player slots area | Card transitions to `Placed`; first empty local slot becomes populated, card state becomes `Location`, and resolved destination is a location card slot |
| Hover hand insertion area | Current-round placed card overlaps the player hand area while dragging | Existing hand cards shift on the x axis to show the candidate insertion gap |
| Release valid hand insertion gap | Pointer release over a valid hand insertion gap | Card transitions to `Hand`; source location slot becomes empty; hand order updates to the candidate index and the full hand group recenters |
| Recalculate location power | A location card slot becomes populated | That location side's power total becomes the sum of the power values of all cards assigned to that side's four slots |
| Release invalid target | Pointer release over opponent slot, populated slot, or empty board space | Card transitions to `Returning`, card state becomes `Hand`, and resolved destination is the original hand card slot |
| Resize/reflow | Safe visible area changes during gesture | Source, target, and slot transforms are recalculated from current safe area |

## Rules

| Rule | Requirement Coverage |
| ---- | -------------------- |
| Pointer input is generalized across mouse, touch, stylus, and equivalent devices. | FR-001 |
| GameView hand-card click/tap selects for in-place inspection and never opens DeckBuilderScene. | FR-003, FR-004, FR-005 |
| Selected inspection size is approximately 90% of safe visible height unless width-constrained. | FR-006 |
| Drag threshold determines click versus drag, and crossed-threshold gestures cannot inspect. | FR-009, FR-010 |
| Dragging preserves the pointer-to-card-center offset and grows the card to 150% of hand size over 0.25 seconds. | FR-011, FR-026 |
| Only zero or one card can be pressed for drag or actively dragged at a time. | FR-022, FR-029 |
| Cards in `Hand` state and current-round placed location state can begin allowed drag gestures; locked location cards cannot. | FR-031, FR-034, FR-035 |
| Three locations have four opponent-side slots and four local-player slots each. | FR-012, FR-013 |
| Local direct placement is limited to the twelve bottom/local-player slots. | FR-014, FR-015 |
| Only empty local-player slots accept dragged hand cards. | FR-016, FR-017, FR-020, FR-021 |
| Drop target hints show only available local-player slot areas while dragging. | FR-027, FR-028 |
| Card aspect ratio is preserved during inspection, drag preview, snap, and return. | FR-006, FR-011, FR-018 |
| Successful and cancelled drags resolve only to a hand card slot or a location card slot. | FR-030 |
| Slot rectangles are stored in the runtime slot model and reused by debug drawing and gameplay placement. | FR-032 |
| Location-side power point views derive from the card slots assigned under each location and player side. | FR-033 |
| Current-round placed cards can be dragged over the hand area, show an x-axis insertion gap, and return to the chosen hand order on release. | FR-034, FR-036, FR-037 |

## Sanity-Checked Runtime Shape

| Layer | Owns |
| ----- | ---- |
| `GameModel` | Current match-level game state |
| `RoundModel` | Current round state |
| `LocationsModel` | Three shared location entries |
| `LocationModel` | Shared location identity plus two player-side slot groups |
| `LocationPlayerSideModel` | Four `LocationModelCardSlot` entries for one player side |
| `LocationModelCardSlot` | Slot occupancy and geometry; populated slots contribute their card power to that side's location total |
