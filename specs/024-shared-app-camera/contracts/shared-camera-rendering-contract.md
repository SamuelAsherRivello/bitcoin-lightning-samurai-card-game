# Shared Camera Rendering Contract

## Purpose

Define the runtime rendering contract for replacing all per-screen cameras with one locked AppScene 3D camera while preserving current visual behavior.

## Inputs

| Input | Type | Source |
| --- | --- | --- |
| Active view | Resource | Existing `ActiveView` |
| Camera defaults | Resource | `AppCameraModel` or migrated `PrimaryCameraDefaults` |
| Safe viewport | Derived value | Existing aspect-ratio-safe viewport logic |
| Screen transition state | Resource | Existing `ScreenTransitionResource` |
| Overlay/menu/modal state | Resources/components | Existing DebugHUD, modal, deck, and card selection systems |

## Required Behavior

| Contract | Requirement |
| --- | --- |
| Camera count | Runtime uses exactly one active camera for normal rendering. |
| Camera type | The active camera is a 3D camera owned by AppScene. |
| View switching | Switching active views does not create or activate a screen-owned camera. |
| 2D camera removal | No runtime path spawns `Camera2d` for screens, overlays, text, or transitions. |
| Visual parity | Existing screen layout, scale, safe-area placement, and layering remain unchanged to the user. |
| Transition coverage | Transition fades cover all visible content through the shared camera. |
| Interaction parity | Card picking, buttons, modals, top navigation, and debug drawing remain usable. |

## Output

| Output | Consumer |
| --- | --- |
| One active AppScene 3D camera | Bevy renderer, UI, picking, debug drawing, card gesture systems |
| Shared-camera-compatible overlay ordering | Transition, modal, HUD, and point label systems |
| Updated tests and quickstart notes | Developers and reviewers |

## Failure Handling

| Case | Handling |
| --- | --- |
| UI cannot render through the shared 3D camera | Convert affected UI surface to a 3D-facing overlay or document the blocker before implementation continues. |
| Overlay does not fully cover viewport | Use a camera-facing 3D plane sized from the camera frustum and safe viewport. |
| Picking uses stale per-scene camera query | Query `AppSceneCamera` and use `ActiveView` to choose content behavior. |
| Browser rendering differs from desktop | Record blocker and keep feature incomplete until a browser-compatible path is implemented. |

## Verification Contract

| Check | Method |
| --- | --- |
| Single camera | Runtime test counts active `Camera` entities and `Camera2d` entities. |
| Stable camera | Runtime test switches views and compares camera entity and transform. |
| GameScene parity | Desktop/browser screenshots and existing GameScene tests. |
| DeckScene parity | Desktop/browser screenshots and existing DeckScene/modal tests. |
| Transition parity | Existing transition phase tests plus visual fade check. |
| Interaction parity | Existing card selection, top navigation, modal, and debug drawing tests. |
