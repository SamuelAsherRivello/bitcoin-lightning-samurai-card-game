# Contract: Card Selected Modal Backdrop

## Selection Eligibility

| Condition | Required |
| --- | --- |
| Rendered entity is a card front | ✅ |
| Card is stationary | ✅ |
| Card is dragged, returning, moving, dealing, revealing, or flipping | ❌ |
| Card belongs to human/local player | Optional |
| Card belongs to CPU/far/near player | Optional |
| Card is on GameView only | ❌ |

## Pointer Contract

| Interaction | Result |
| --- | --- |
| Press and release on selectable card without crossing `CARD_GESTURE_DRAG_THRESHOLD` | Enter selected inspection. |
| Press and move across `CARD_GESTURE_DRAG_THRESHOLD` on draggable card | Start drag; do not select. |
| Click selected card | Keep selected inspection active. |
| Click modal backdrop | Dismiss selected inspection and return card to source. |
| Hover/click/drag lower elements while modal is active | Lower element receives no interaction. |

## Visual Contract

| Element | Render Rule |
| --- | --- |
| Selected card | Centered in aspect-ratio-safe GameView and rendered above modal backdrop and lower point overlays. |
| Modal background | Fullscreen black, opacity animates 0.0 to 0.5 over 0.5 seconds. |
| Non-selected scene/cards/UI behind modal | Darkened and blocked while modal is active. |
| Non-selected card point text | Hidden, depth-ordered behind, or otherwise prevented from rendering on top of selected card. |
| Selected card point text | Remains visible as part of the selected card presentation if the selected card normally shows point text. |

## Verification Contract

| Check | Method |
| --- | --- |
| Selectable coverage | Manual desktop run with local hand, local placed, CPU/far/near cards where front-facing. |
| Drag distinction | Start a drag from a draggable selectable card and verify selected modal does not open. |
| Modal opacity | Screenshot or visual inspection at start and after 0.5 seconds; max opacity is 50%. |
| Input blocking | Try lower buttons, cards, and drop targets while selected; verify no hover/click/drag effect. |
| Point text bug | Reproduce attached screenshot scenario and verify lower white numbers do not draw over selected card. |
