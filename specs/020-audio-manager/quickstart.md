# Quickstart: Audio Manager

## Prerequisites

| Requirement | Command Or Path |
| ----------- | --------------- |
| Rust and project dependencies | `scripts/main/InstallDependencies.ps1` |
| Confirmed SFX assets | `bevy/crates/game/assets/audio/sfx/Click01.wav`, `Slide01.wav`, `Tamborine01.wav`, `Upgrade01.wav` |
| Main verification script | `scripts/other/RunTests.ps1` |

## Implementation Checklist

| Step | Action |
| ---- | ------ |
| 1 | Inspect `bevy/crates/template-crate` and `.codex/rules/bevy-runtime-structure.md` before adding runtime files. |
| 2 | Add an audio manager model/resource with SFX and Music channels backed by existing meta-game settings. |
| 3 | Add enum-to-file mappings for `ButtonClick`, `CardSlide`, `LocationOpen`, and `LocationLeadChange`. |
| 4 | Add a shared `button_ui_bundle` with exactly one default style and migrate current buttons to it. |
| 5 | Emit `ButtonClick` only for accepted shared button clicks. |
| 6 | Emit `CardSlide` for accepted near/far deck-to-hand and hand-to-location moves. |
| 7 | Emit `LocationOpen` only for closed-to-open location transitions. |
| 8 | Track previous location winning side and emit `LocationLeadChange` only for new non-tied winners. |
| 9 | Add tests for channel muting, enum mappings, button coverage, movement triggers, and location winner transition cases. |
| 10 | Run `scripts/other/RunTests.ps1` before marking implementation complete. |

## Verification Scenarios

| Scenario | Expected Result |
| -------- | --------------- |
| SFX enabled and a shared button is clicked | `ButtonClick` request maps to `Click01.wav`. |
| SFX disabled and a shared button is clicked | No audible SFX, button action still completes. |
| Near deck moves a card to near hand | One `CardSlide` request maps to `Slide01.wav`. |
| Far hand moves a card to a location slot | One `CardSlide` request maps to `Slide01.wav`. |
| Location changes from closed to open | One `LocationOpen` request maps to `Tamborine01.wav`. |
| Location scores change `0,0 -> 0,3 -> 0,5 -> 6,5` | Exactly two `LocationLeadChange` requests map to `Upgrade01.wav`. |
| Music disabled while SFX enabled | Music is inaudible; SFX requests still follow SFX setting. |

## Verification Commands

```powershell
scripts/other/RunTests.ps1
```

For user-visible checks after implementation, run the app through the existing desktop or browser workflow and exercise SettingsScreen, navigation buttons, card movement, and location scoring. If browser verification is blocked, record the exact blocker in the implementation notes.
