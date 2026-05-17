# Research: Shared AppScene 3D Camera

## Decision: Introduce One AppScene-Owned 3D Camera

| Field | Value |
| --- | --- |
| Decision | Spawn one locked `AppSceneCamera` from the persistent AppScene setup path and use it for every view. |
| Rationale | The user explicitly wants one 3D camera in `AppScene`, not one camera per screen. AppScene is always present, so camera lifetime becomes independent of view switching. |
| Alternatives considered | Keeping one camera per view was rejected because it preserves the current architecture. Keeping a separate 2D overlay camera was rejected because the requirement says remove all 2D cameras. |

## Decision: Prove Bevy UI Through Shared 3D Camera Before Broad Refactor

| Field | Value |
| --- | --- |
| Decision | Start implementation with a small verification slice that renders Bevy UI, text, buttons, and 3D content through the shared 3D camera. |
| Rationale | Most current screens use Bevy UI and `UiTargetCamera`; deleting 2D cameras without proving UI behavior would create high regression risk. |
| Alternatives considered | Rewriting all UI to 3D meshes immediately was rejected because it is much larger and risks changing visible behavior. |

## Decision: Preserve Existing Safe-Area Coordinate System

| Field | Value |
| --- | --- |
| Decision | Keep `GAME_SCENE_WIDTH`, `GAME_SCENE_HEIGHT`, safe-area viewport calculations, and existing world/screen conversion helpers as the visual coordinate source of truth. |
| Rationale | Visual parity depends on preserving current virtual 1280x800 positioning across desktop and browser. |
| Alternatives considered | Moving to raw window pixels was rejected by the constitution. Rebuilding layout in arbitrary world coordinates was rejected because it would be harder to prove parity. |

## Decision: Replace Camera Order With Shared-Camera Layering

| Field | Value |
| --- | --- |
| Decision | Convert current camera order semantics into explicit Z-depth, render layers, and UI z-index policy under the shared camera. |
| Rationale | Current rendering uses several cameras with orders such as 0, 1, 2, 3, and 10. A single camera cannot preserve those semantics unless ordering is expressed in scene content. |
| Alternatives considered | Multiple cameras with different orders were rejected by the feature requirement. Relying on spawn order was rejected because it is fragile. |

## Decision: Prefer 3D Fade Plane For Screen Transitions

| Field | Value |
| --- | --- |
| Decision | If Bevy UI transition overlay cannot reliably cover all content through the shared 3D camera, use a camera-facing 3D fade plane placed closest to the camera. |
| Rationale | The transition overlay currently depends on a dedicated `ScreenTransitionCamera`. A 3D plane can preserve fullscreen black fades without needing a 2D camera. |
| Alternatives considered | Keeping `ScreenTransitionCamera` was rejected because it is a 2D camera. Removing transitions was rejected because `022-screen-transitions` is active behavior. |

## Decision: Use Active View As Context, Not Camera Ownership

| Field | Value |
| --- | --- |
| Decision | Systems that currently infer behavior from scene-owned cameras should query the shared camera and use `ActiveView` or scene markers on content for view-specific behavior. |
| Rationale | After migration, the camera no longer belongs to GameScene, DeckScene, or DebugScene. |
| Alternatives considered | Moving the shared camera between scene markers was rejected because it would make ownership ambiguous and could break AppScene lifetime guarantees. |

## Technical Risks

| Risk | Mitigation |
| --- | --- |
| Bevy UI interaction with `Camera3d` differs between desktop and browser | Verify both targets before removing fallback paths. |
| Text rendering changes when card point labels move off the 2D text camera | Add focused visual and transform tests for point label alignment. |
| Existing tests assert exact camera counts or scene markers | Update tests to assert the new camera contract and existing content hierarchy. |
| Hidden camera dependencies in modals or debug systems | Search all `Camera2d`, `Camera3d`, `UiTargetCamera`, `IsDefaultUiCamera`, `PrimaryViewCamera`, and `CardPointTextCamera` references during implementation. |
