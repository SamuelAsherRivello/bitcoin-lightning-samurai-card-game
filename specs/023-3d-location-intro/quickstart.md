# Quickstart: 3D Location Intro

## Focused Verification

```powershell
cargo test -p samurai-card-game game_scene_owns_camera_world_background_and_three_locations
cargo test -p samurai-card-game location_intro_reveals_locations_in_sequence
```

## Broader Verification

```powershell
scripts/other/RunTests.ps1
```

## Visual QA

| Step | Expected Result |
| ---- | --------------- |
| Start GameScene | Location 01 fades and shrinks from 150% to final size over 1 second. |
| Wait 0.5 seconds | Location 02 begins the same animation. |
| Wait another 0.5 seconds after location 02 completes | Location 03 begins the same animation. |
| Inspect board after 4 seconds | Three locations are fully visible, in front of the world background, below all cards, and retain background graphic, border, and two point views. |
