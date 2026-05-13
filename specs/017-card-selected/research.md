# Research: Card Selected Modal Backdrop

## Decision: Use an explicit selectable affordance

**Rationale**: Existing `CardStateModel::is_selectable()` covers only local hand and local placed cards by `hand_index`. Feature 017 needs the concept to apply to CPU, near/far player, hand/location cards, and screens beyond the current local `GameScene` gesture path. An explicit affordance such as `CardInputAffordance::Selectable` or a `SelectableCard` component makes eligibility clear at the rendered-card boundary.

**Alternatives considered**: Reuse `CardStateModel::is_selectable()` only. Rejected because it does not identify CPU/far-player cards or screen-wide rendered card entities.

## Decision: Preserve the existing drag threshold

**Rationale**: `PointerGestureModel` already tracks start/current positions and `CARD_GESTURE_DRAG_THRESHOLD = 8.0`; `CardGestureState::Pressed` only becomes selected on release if the threshold was not crossed. The feature should reuse this click-vs-drag distinction.

**Alternatives considered**: Add separate click timers or raw button release handlers. Rejected because that would risk changing behavior the user explicitly said works well.

## Decision: Add explicit selected modal progress

**Rationale**: Current selected inspection animation in `card_gesture_animation_system` uses `CARD_GESTURE_ANIMATION_RATE = 14.0`, which is frame-step interpolation rather than a fixed 0.5 second tween. A selected modal model should track elapsed time and expose normalized progress over `SELECTED_CARD_MODAL_FADE_SECONDS = 0.5`, allowing opacity to fade from 0% to 50% predictably.

**Alternatives considered**: Derive opacity from current card scale distance to target. Rejected because scale interpolation is not time-based and could make opacity timing hard to test.

## Decision: Use a modal input-capture layer plus render dim layer

**Rationale**: The feature has two responsibilities: darken visuals behind the selected card and block all lower interactions. Bevy UI is the most direct way to consume pointer input across the safe-area/fullscreen surface, while a render layer or safe-area-sized world plane can handle visual placement behind 3D card content. Implementation can pair them under one modal state if one entity cannot satisfy both render ordering and input blocking.

**Alternatives considered**: Only add a 3D plane. Rejected because it would not reliably block egui/UI buttons. Only add UI backdrop. Rejected unless verified to render behind the selected 3D card, because UI often renders over 3D content.

## Decision: Fix point text overlay by selected-card-aware text visibility or ordering

**Rationale**: Card point numbers use a separate `CARD_POINT_TEXT_RENDER_LAYER` and a 2D text camera, so their depth is independent from the selected 3D card mesh. The reported issue is expected if lower cards' `Text2d` overlays continue rendering at a fixed `CARD_POINT_TEXT_Z`. The reliable fix is to hide or suppress non-selected card point text while selected inspection is active, or move selected-card point text to a higher selected overlay while keeping other point text behind/hidden.

**Alternatives considered**: Increase the selected card z only. Rejected because the point text is drawn by a separate camera/layer and will not necessarily obey the 3D card depth.
