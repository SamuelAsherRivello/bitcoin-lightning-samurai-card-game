# Contract: Card Gestures

## Gesture Contract

| Operation | Inputs | Output | Required Behavior |
| --------- | ------ | ------ | ----------------- |
| Press hand card | Pointer press, local hand card target | Active `Pressed` gesture | Records pointer start, card identity, and source transform without changing active view |
| Press non-hand card | Pointer press on a card in `Dragging` or `Location` state | No drag gesture starts | A card is only draggable while its card state is `Hand` |
| Press second hand card during active pointer gesture | Pointer press while another card is `Pressed` or `Dragging` | Existing active gesture unchanged | Guards against concurrent drags; only zero or one card can be pressed for drag or actively dragged at a time |
| Release as click/tap | Active `Pressed` gesture, pointer release within threshold | Selected inspected card | Keeps `GameView` active and animates the card to center inspection presentation |
| Click selected card | Pointer click/tap on selected inspected card | Returning card | Animates the selected card back to source hand position and source size |
| Cross drag threshold | Active `Pressed` gesture, pointer movement beyond threshold | Active `Dragging` gesture | Suppresses click inspection for that pointer sequence |
| Drag hold | Active `Dragging` gesture, pointer movement | Drag preview target | Keeps the original pointer-to-card-center offset and animates to 150% of hand size over 0.25 seconds with ease-out motion |
| Release over valid location slots area | Active `Dragging` gesture, available local-player slots area | Placed card and populated slot | Assigns the first empty destination location card slot in upper-left, upper-right, lower-left, lower-right order and fits card within slot aspect-preserving bounds |
| Release over invalid target | Active `Dragging` gesture, opponent slot, populated slot, or no slot | Returning card | Rejects placement and returns card to its original hand card slot position and size |

## Card State Contract

| Card State | Draggable | Meaning |
| ---------- | --------- | ------- |
| `Hand` | ✅ | Card is in its original hand card slot and can start a drag |
| `Dragging` | ❌ | Card is controlled by the active pointer gesture |
| `Location` | ❌ | Card is placed in a destination location card slot |

## Destination Contract

| Drag Result | Legal Destination |
| ----------- | ----------------- |
| Successful drag | Destination location card slot assigned by the location's first-empty local slot order |
| Cancelled or rejected drag | Original hand card slot captured when the pointer gesture began |

## Slot Contract

| Slot Type | Count | Local Direct Placement | Valid When |
| --------- | ----- | ---------------------- | ---------- |
| Opponent-side slot above a location | 12 total, four per location | ❌ | Never valid for direct local drag in this feature |
| Local-player slot below a location | 12 total, four per location | ✅ | Slot exists and is empty |

## Slot Geometry Contract

| Runtime Model | Required Consumers |
| ------------- | ------------------ |
| `CardSlotModel.rect` | Debug drawing rectangles, location slot hit testing, DropTargetHint area bounds, and card placement transforms |

## Location Power Contract

| Event | Required Behavior |
| ----- | ----------------- |
| Empty location side | Visible power point value is `0` |
| Card assigned to one of the side's four location card slots | Visible power point value is recalculated from all cards assigned to that location side |
| Multiple cards assigned to one location side | Visible power point value is the sum of those cards' power values |

## Drop Target Hint Contract

| State | Required Behavior |
| ----- | ----------------- |
| Dragging over GameView with available locations | Show one light blue DropTargetHint rectangle over the full local-player slots area for each location with at least one empty local-player slot |
| Dragging over GameView with full locations | Hide DropTargetHint for full locations and reject drops there |
| Not dragging or not in GameView | Hide all DropTargetHint rectangles |

## View Contract

| View State | Required Presentation |
| ---------- | --------------------- |
| Hand | Card appears in the local hand area as a hand-sized card preview |
| Selected inspection | Card appears in the center `GameView` inspection position, matching Deck Builder inspection as closely as safe-area layout allows, at approximately 90% safe visible height |
| Dragging | Card follows pointer movement inside the game view with aspect ratio preserved |
| Placed | Card snaps into the selected local-player slot and fits within the slot while preserving aspect ratio |
| Returning | Card animates back to its source hand or slot presentation without changing active view |

## Acceptance Samples

| Scenario | Input | Expected |
| -------- | ----- | -------- |
| Inspect hand card | Press and release within threshold on local hand card | GameView remains active; selected card enlarges at center |
| Dismiss inspected card | Press and release selected card | Card returns to hand source |
| Drag from hand | Press hand card and move beyond threshold | Gesture becomes drag and does not inspect |
| Valid placement | Release dragged card over empty bottom/local slot | Card snaps into slot and slot becomes populated |
| Concurrent drag guard | Press one hand card, move or hold, then press another hand card before release | Existing active card remains the only draggable card |
| Automatic slot assignment | Release dragged card over an available location's local slots area | First empty slot is chosen in upper-left, upper-right, lower-left, lower-right order |
| Opponent-side drop | Release dragged card over top/opponent slot | Drop rejected and card returns |
| Populated-slot drop | Release dragged card over occupied local slot | Drop rejected and card returns |
| Off-board drop | Release dragged card outside valid slots | Drop rejected and card returns to its original hand card slot |

## Deck Builder Boundary

| Boundary | Required Behavior |
| -------- | ----------------- |
| GameView hand-card click/tap | Does not open or activate the Deck Builder view |
| Other GameView card gestures | Do not create a user-facing route to the Deck Builder view |
| Existing Deck Builder implementation | Remains unchanged by this feature |

## Out Of Scope

| Area | Status |
| ---- | ------ |
| Turn legality and energy costs | Excluded |
| CPU or opponent direct placement | Excluded |
| Card reveal timing | Excluded |
| Scoring resolution | Excluded |
| Full mobile packaging/release | Excluded, though pointer semantics must be touch-compatible |
| Deck Builder view changes | Excluded; the view remains in the game but unreachable through GameView user gestures after this feature |
