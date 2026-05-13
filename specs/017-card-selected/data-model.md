# Data Model: Card Selected Modal Backdrop

## SelectableCard State

| Field | Type | Rules |
| --- | --- | --- |
| `entity` | `Entity` | Rendered card root. |
| `instance_id` | `Option<CardInstanceId>` | Present when connected to the 015 card instance model. |
| `owner` | `MatchPlayerSide` or existing owner component | Supports near/far and human/CPU ownership. |
| `source` | `CardSelectionSource` | Hand, location slot, deck-builder card, debug/settings card, or other screen-specific card source. |
| `front_visible` | `bool` | Must be ✅ to become selectable. |
| `movement_state` | `CardMovementState` | Must be `Stationary` to become selectable. |
| `affordance` | `CardInputAffordance` | `Selectable` when front-facing and stationary, otherwise `Passive` or `None`. |

## CardSelectionSource

| Variant | Description |
| --- | --- |
| `LocalHand { hand_index }` | Existing local hand card source. |
| `LocalLocation { location_index, slot_index, hand_index }` | Existing local placed card source. |
| `OpponentHand { owner, hand_index }` | CPU/far/near hand source when front-facing. |
| `OpponentLocation { owner, location_index, slot_index }` | CPU/far/near placed source when front-facing. |
| `ScreenCard { view, entity }` | Non-GameScene card displayed on another screen. |

## CardMovementState

| State | Selectable |
| --- | --- |
| `Stationary` | ✅ |
| `Pressed` | ❌ |
| `Dragging` | ❌ |
| `Returning` | ❌ |
| `Dealing` | ❌ |
| `Moving` | ❌ |
| `Revealing` | ❌ |
| `Flipping` | ❌ |

## SelectedCardModalModel

| Field | Type | Rules |
| --- | --- | --- |
| `selected_entity` | `Option<Entity>` | Only one selected card may exist. |
| `selected_source` | `Option<CardSelectionSource>` | Used to return to source. |
| `source_transform` | `Option<Transform>` | Captured before selected inspection. |
| `target_transform` | `Option<Transform>` | Center-screen selected inspection transform. |
| `fade_elapsed_seconds` | `f32` | Clamped from `0.0` to `SELECTED_CARD_MODAL_FADE_SECONDS`. |
| `max_opacity` | `f32` | Default `0.5`. |
| `dismiss_pending` | `bool` | Set by modal backdrop click, not by selected card click. |

## ModalBackgroundLayer

| Field | Type | Rules |
| --- | --- | --- |
| `opacity` | `f32` | `0.0..=0.5`, derived from selected modal progress. |
| `input_blocks_lower_layers` | `bool` | ✅ while selected. |
| `covers_fullscreen` | `bool` | ✅ including letterboxed areas if lower UI exists there; visual placement must still respect GameScene composition. |
| `render_order` | `ModalRenderOrder` | Behind selected card, above non-selected scene/card/UI content. |

## State Transitions

| From | Event | To |
| --- | --- | --- |
| `Idle` | Pointer press on selectable card | `PressedCandidate` |
| `PressedCandidate` | Pointer moves past drag threshold and card is draggable | `Dragging` |
| `PressedCandidate` | Pointer release before drag threshold | `SelectedInspecting` |
| `SelectedInspecting` | Modal backdrop click | `Returning` |
| `SelectedInspecting` | Selected card click | `SelectedInspecting` |
| `Returning` | Card reaches source transform | `Idle` |

## Validation Rules

| Rule | Requirement |
| --- | --- |
| Single active selected card | `SelectedCardModalModel.selected_entity` is unique. |
| Front face required | Back-facing and hidden/revealing CPU cards are not selectable. |
| Stationary required | Any movement animation makes a card non-selectable. |
| Modal lifetime | Backdrop exists and blocks input only while `SelectedInspecting` or returning from modal dismissal. |
| Point overlays | Non-selected card point text cannot render above selected inspection card. |
