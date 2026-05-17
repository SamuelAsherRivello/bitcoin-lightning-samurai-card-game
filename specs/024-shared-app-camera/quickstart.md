# Quickstart: Shared AppScene 3D Camera

## Prerequisites

| Item | Command/Path |
| --- | --- |
| Dependencies installed | `scripts/main/InstallDependencies.ps1` |
| Test runner | `scripts/other/RunTests.ps1` |
| Desktop app run | `scripts/main/RunAppDesktop.ps1` |
| Web app run | `scripts/main/RunAppWeb.ps1` |
| Runtime screenshot workflow | BRP endpoint `http://localhost:15702` with screenshots under `target/ai-runtime-screenshots/` |

## Implement

| Step | Action |
| --- | --- |
| 1 | Add shared AppScene camera model, marker, and bundle. |
| 2 | Spawn the shared 3D camera from AppScene setup and keep it independent of active views. |
| 3 | Prove Bevy UI and 3D content render through the shared camera. |
| 4 | Remove GameScene, DeckScene, DebugScene, meta, transition, and point-text camera spawns. |
| 5 | Retarget or replace UI overlays, DebugHUD, modals, selected-card menus, and transition fades. |
| 6 | Collapse viewport constraints to one shared-camera system. |
| 7 | Update picking, gesture, debug drawing, and card selection camera queries. |
| 8 | Update tests and run desktop/browser verification. |

## Verify

| Check | Expected Result |
| --- | --- |
| Automated tests | `scripts/other/RunTests.ps1` passes. |
| Camera inspection | Exactly one active `Camera3d`; zero `Camera2d` entities. |
| GameScene visual check | World, locations, hand cards, controls, HUD, and transition fade match pre-migration layout. |
| DeckScene visual check | Grids, selected-card menu, validation prompt, preview card, and navigation match pre-migration layout. |
| Debug/meta visual check | Debug and meta screens remain inside the safe area and keep expected layering. |
| Desktop parity | Manual or screenshot-based desktop check passes. |
| Browser parity | Manual or screenshot-based browser WebGPU check passes. |

## Screen Parity Targets

| Screen | Desktop Expectation | Browser Expectation |
| --- | --- | --- |
| GameScene | World background, three locations, local hand, CPU cards, controls, card labels, DebugHUD, and fade overlay stay inside the 1280x800 safe area. | Same safe-area framing and stacking through WebGPU with no missing UI roots or card-label drift. |
| DeckScene | Deck grids, preview card, selected-card menu, validation prompt, and top navigation retain current spacing and layering. | Same DeckScene layout with no browser-only UI target loss. |
| DebugScene | Debug card, navigation, panels, and DebugHUD remain aligned to the safe area through the shared camera. | Same alignment and interaction behavior without a per-screen UI camera. |
| Meta screens | Main Menu, Lightning, Matchmaking, and Settings keep navigation, panels, buttons, and transition fade ordering unchanged. | Same meta-screen placement and fade coverage through the shared camera. |

## Entity Hierarchy Notes

| Area | Expected Runtime State |
| --- | --- |
| AppScene | Owns the single active `AppSceneCamera` with `Camera3d`; the camera is parented under the persistent app scene. |
| GameScene | Owns screen content, world background, locations, hand/card entities, controls, and HUD children, but no view-owned camera. |
| DeckScene | Owns deck screen UI/content and selected-card surfaces, but no view-owned camera. |
| DebugScene | Owns debug screen card/content and debug UI, but no view-owned camera. |
| Meta screens | Own their screen UI/content and target the shared camera for UI roots; they do not spawn local cameras. |

## Expected Result

The app renders all current screens through one locked AppScene-owned 3D camera. No normal runtime path spawns a 2D camera. User-visible layout, transition behavior, card interaction, overlays, and desktop/browser parity remain unchanged.

## Verification Notes

| Check | Status | Notes |
| --- | --- | --- |
| Automated tests | ✅ | `scripts/other/RunTests.ps1` passed on 2026-05-22: 356 game lib tests, 3 game binary tests, 13 shared lib tests, doc tests passed. |
| Source scan | ✅ | `rg` found no `Camera2d` runtime references; legacy camera marker names remain only as inert compatibility definitions/tests. |
| Manual desktop check | ❌ | Not run in this pass. |
| Manual browser check | ❌ | Not run in this pass. |
