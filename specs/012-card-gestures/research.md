# Research: Card Gestures

## Decisions

| Topic | Decision | Rationale | Alternatives Considered |
| ----- | -------- | --------- | ----------------------- |
| Pointer vocabulary | Treat mouse, touch, stylus, and equivalent inputs as pointer gestures in the game model. | The feature request explicitly says mouse input should map to mobile tap, press, swipe, and drag concepts. Bevy pointer events already give a unified interaction vocabulary for pickable entities. | Separate mouse and touch systems were rejected because they would duplicate gesture rules and increase drift. |
| Current click navigation | Stop opening `DeckBuilderScene` from hand-card clicks while `GameView` is active, and leave the Deck Builder view implementation unchanged. | The feature specifically changes GameView hand-card gestures while the user clarified that the Deck Builder scene should remain as-is. | Editing or removing the Deck Builder view was rejected because the user explicitly said not to touch it. |
| Gesture state ownership | Add a runtime `CardGestureModel` resource for one active gesture focus and small components that identify hand cards and slot targets. | One active selected/dragged card matches the spec's conflict rules and keeps click, drag, inspection, and placement behavior coordinated. | Encoding state only in entity components was rejected because resolving selection conflicts and source-return behavior needs a single authoritative focus. |
| Slot state ownership | Add a runtime `CardSlotModel` or board-slot resource for three locations, two sides, and four slots per side. | Slot legality is gameplay state that must be testable without rendering and reused later by scoring/turn rules. | Treating slots only as UI node positions was rejected because populated/empty state and local-only legality are core feature behavior. |
| Animation dependency | Use `bevy_tweening = "0.15"` for card transform/scale movement. | docs.rs lists `bevy_tweening 0.15.0` as depending on Bevy `^0.18` and its compatibility table maps `0.15` to Bevy `0.18`. The project currently uses Bevy `0.18.1`. | Hand-rolled interpolation was rejected because the user requested Bevy Tween behavior and a maintained tween plugin keeps animation concerns explicit. |
| Selected inspection pose | Reuse the Deck Builder inspection transform/size semantics as the source of truth, adapted to `GameView` safe-area coordinates. | The spec requires the selected card to appear at the same inspection position as the Deck Builder scene while staying in `GameView`. | Defining a separate GameView-only selected pose was rejected because it would drift from the requested visual parity. |
| Drag threshold | Use a documented small virtual-pixel threshold derived from the aspect-ratio-safe view, with consistent boundary behavior. | The spec requires movement of a few pixels to become drag and not click; safe-view units make desktop/browser sizing consistent. | Raw window-pixel threshold was rejected because the constitution requires aspect-ratio-relative runtime layout. |
| Invalid drop behavior | Return the card to its source position and source size when the target is invalid. | This preserves the card and matches the spec's invalid-drop edge cases. | Leaving the card at the invalid drop point was rejected because it creates ambiguous board state. |

## External References

| Reference | Relevance |
| --------- | --------- |
| docs.rs `bevy_tweening 0.15.0` | Confirms the tweening crate dependency and Bevy 0.18 compatibility for planning. |
| Bevy 0.18 release notes | Confirms the project is on the current Bevy 0.18 generation and includes first-party picking/input improvements relevant to pointer work. |
