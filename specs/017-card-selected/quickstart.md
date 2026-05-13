# Quickstart: Card Selected Modal Backdrop

## Implementation Notes

| Area | Guidance |
| --- | --- |
| Existing click/drag behavior | Preserve `PointerGestureModel` and `CARD_GESTURE_DRAG_THRESHOLD`. |
| Existing selected transform | Reuse `selected_inspection_transform()` for center-screen card pose. |
| Modal duration | Add a named `0.5` second constant for modal fade/progress; current selected transform does not have a fixed duration. |
| Input blocking | Ensure modal backdrop is checked before lower game controls, card gestures, hover, drop hints, and egui/UI controls can react. |
| Point text fix | Account for `CARD_POINT_TEXT_RENDER_LAYER` and `CardPointTextCamera`; selected card z alone is not enough. |

## Verification

| Step | Command or Action | Expected |
| --- | --- | --- |
| Run tests | `scripts/other/RunTests.ps1` | Rust tests pass. |
| Start desktop app | `scripts/main/RunAppDesktop.ps1` or hot-reload script with AI runtime as needed | Game opens. |
| Select front cards | Click stationary front-facing local and CPU cards | Card centers and modal fades in. |
| Test drag | Drag a draggable card past threshold | Drag starts; no modal selection. |
| Test modal block | While selected, interact with lower buttons/cards/locations | No lower interaction occurs. |
| Dismiss modal | Click backdrop outside selected card | Card returns; modal disappears. |
| Check point text | Select a card overlapping other point numbers | Lower point numbers do not render over selected card. |

## Runtime Peek

| If Needed | Action |
| --- | --- |
| AI runtime endpoint is available | Query `http://localhost:15702` and capture `bevy_debugger/screenshot` to `target/ai-runtime-screenshots/`. |
| Endpoint is unavailable | Ask user to start `scripts/main/RunAppDesktopHotReload.ps1` or `scripts/other/RunAppDesktop.ps1 -AiRuntime`. |
