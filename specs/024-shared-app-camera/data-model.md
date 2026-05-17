# Data Model: Shared AppScene 3D Camera

## Entities

| Entity | Type | Fields | Notes |
| --- | --- | --- | --- |
| `AppSceneCamera` | Component marker | none | Marks the single shared app runtime camera. |
| `AppCameraModel` | Resource | `position`, `rotation`, `scale`, `fov_radians`, `near`, `far`, `safe_viewport` | Centralized locked camera defaults and viewport state. |
| `SharedCameraLayer` | Enum or constants | `WorldBackground`, `LocationSurface`, `CardSurface`, `CardPointText`, `UiOverlay`, `TransitionFade` | Defines the replacement ordering policy for former camera orders. |
| `SharedCameraOverlayView` | Component marker or bundle concept | `kind`, `depth`, `alpha` where relevant | Covers transition overlays, modal backdrops, HUD-like overlays, and other camera-facing presentation surfaces. |

## Validation Rules

| Rule | Constraint |
| --- | --- |
| Single active camera | Normal app runtime has exactly one active camera entity. |
| Camera type | The only active camera is a `Camera3d`. |
| Ownership | The shared camera has `AppSceneEntity` or equivalent AppScene ownership marker and does not carry view-owned scene markers. |
| No 2D cameras | Normal app runtime spawns zero `Camera2d` entities. |
| Locked transform | View switches do not mutate shared camera position, rotation, or scale. |
| Safe viewport | Camera viewport is recalculated from the aspect-ratio-safe game view when the window or fullscreen state changes. |
| Overlay coverage | Transition fade coverage is visually fullscreen within the active viewport. |

## State Transitions

| From | Trigger | To |
| --- | --- | --- |
| No app camera | AppScene setup | Spawn `AppSceneCamera` using `AppCameraModel` defaults. |
| App camera active | View switch request | Keep same camera; switch view content only. |
| Windowed viewport | Fullscreen/browser resize | Recalculate shared camera viewport and dependent layout. |
| Transition idle | Startup or view switch transition | Render fade overlay through shared camera. |
| Transition active | Transition complete | Keep shared camera unchanged and hide or transparentize fade overlay. |
