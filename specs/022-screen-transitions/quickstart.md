# Quickstart: Screen Transitions

## Prerequisites

| Item | Command/Path |
| --- | --- |
| Dependencies installed | `scripts/main/InstallDependencies.ps1` |
| Test runner | `scripts/other/RunTests.ps1` |
| Desktop app run | `scripts/main/RunAppDesktop.ps1` |
| Web app run | `scripts/main/RunAppWeb.ps1` |

## Implement

1. Add transition model/resource and phase enum under `bevy/crates/game/src/runtime/resources/`.
2. Add a fullscreen transition overlay bundle and marker component under runtime `bundles/` and `components/`.
3. Add transition update system under runtime `systems/` to animate alpha and commit queued view changes at full black.
4. Wire resource + system scheduling in `core_game_plugin.rs` so startup begins in `StartupFadeIn` and runtime view switches route through transition orchestration.
5. Ensure overlay uses AppScene-safe UI layering so it renders above all screens.

## Verify

1. Run `scripts/other/RunTests.ps1` and ensure no regressions.
2. Run desktop build and check: startup fades from black on first load.
3. Navigate among game/deck/debug/meta screens and check: fade out, switch while black, fade in.
4. Run web build and repeat the same visual checks for parity.

## Expected Result

Transition behavior is consistent and deterministic across all existing screens with default black color, 1.0 seconds fade time, and a 0.2 second full-black hold (1.2 seconds total cycle).

## Verification Notes

| Check | Status | Notes |
| --- | --- | --- |
| Automated tests (`scripts/other/RunTests.ps1`) | ✅ | Passed (`317` runtime tests + shared crate tests). |
| Manual desktop transition check | ❌ | Not run in this implementation pass. |
| Manual web transition check | ❌ | Not run in this implementation pass. |
