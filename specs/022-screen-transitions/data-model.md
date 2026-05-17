# Data Model: Screen Transitions

## Entities

| Entity | Type | Fields | Notes |
| --- | --- | --- | --- |
| `ScreenTransitionModel` | Resource | `phase`, `current_alpha`, `target_color`, `total_duration_seconds`, `pending_view` | Central transition state and config.
| `ScreenTransitionPhase` | Enum | `StartupFadeIn`, `FadeOutPendingSwitch`, `SwitchAtBlack`, `HoldAtBlack`, `FadeInAfterSwitch`, `Idle` | Explicit lifecycle for startup + runtime transitions.
| `ScreenTransitionOverlay` | Component marker | none | Marks the fullscreen top-layer UI node to update each frame.

## Validation Rules

| Rule | Constraint |
| --- | --- |
| Color default | `target_color` defaults to black (`rgba(0,0,0,1)`) |
| Fade duration default | `total_duration_seconds` defaults to `1.0` (0.5 out + 0.5 in) |
| Hold default | full-black hold defaults to `0.2` seconds |
| Duration safety | Runtime clamps duration to a small positive minimum to avoid divide-by-zero |
| Switch timing | `pending_view` can only be committed when phase reaches full black gate |
| Overlay coverage | UI node uses fullscreen dimensions and highest intended draw/layer order |

## State Transitions

| From | Trigger | To |
| --- | --- | --- |
| `StartupFadeIn` | app startup initialized | animate alpha 1.0 -> 0.0 |
| `Idle` | new screen/view request | `FadeOutPendingSwitch` with `pending_view` set |
| `FadeOutPendingSwitch` | alpha reaches 1.0 | `SwitchAtBlack` |
| `SwitchAtBlack` | `ActiveView` updated | `HoldAtBlack` |
| `HoldAtBlack` | hold timer reaches 0.2 seconds | `FadeInAfterSwitch` |
| `FadeInAfterSwitch` | alpha reaches 0.0 | `Idle` and clear `pending_view` |
