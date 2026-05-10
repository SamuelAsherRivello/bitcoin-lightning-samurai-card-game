# Quickstart: Card Polish

## Verify Tests

```powershell
scripts/other/RunTests.ps1
```

## Verify Desktop Build

```powershell
scripts/other/RunAppDesktop.ps1 -CheckOnly
```

## Verify Web Build

```powershell
scripts/other/RunAppWeb.ps1 -CheckOnly
```

## Manual Smoke Test

1. Launch the desktop app with `scripts/other/RunAppDesktop.ps1`.
2. Move the pointer across the center, corners, and edges of the window.
3. Confirm the card remains one centered inspectable card.
4. Confirm the background, frame, foreground character, and `SKYBOLT` title read as four apparent layers.
5. Confirm the frame shine changes with tilt and remains bound to the frame.
6. Press `T` and confirm the HUD recognizes the card type toggle while the card remains on `SKYBOLT`.
7. Press `R` and confirm the card scene reloads without changing DebugHUD toggle state.
8. Press `H` and confirm hot-reload auto-restart toggles independently from `F`, `I`, and `T`.
