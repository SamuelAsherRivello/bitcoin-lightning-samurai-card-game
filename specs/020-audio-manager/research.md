# Research: Audio Manager

## Decision: Use Existing Settings As Channel Mutes

| Field | Detail |
| ----- | ------ |
| Decision | Drive SFX audibility from `MetaGameSettingsModel.sfx_enabled` and Music audibility from `MetaGameSettingsModel.music_enabled`. |
| Rationale | The settings already exist, persist, and are shown in SettingsScreen, so the audio manager should consume them instead of introducing duplicate preference state. |
| Alternatives Considered | Add a separate audio settings resource; rejected because it would duplicate persisted state and risk drift. Hard-disable audio at each call site; rejected because channel policy belongs in one manager path. |

## Decision: Use Enum-Driven Audio Requests

| Field | Detail |
| ----- | ------ |
| Decision | Callers request audio through named enum values such as `AudioEnum.ButtonClick`, not raw asset paths. |
| Rationale | The spec requires one-to-one enum-to-file mappings and arbitrary game code needs a stable API that does not load files directly. |
| Alternatives Considered | Raw string paths at call sites; rejected because it scatters file knowledge. Separate event types for every sound; rejected because it grows boilerplate before the sound set is large enough to justify it. |

## Decision: Keep SFX Mapping To Confirmed Files

| Audio Enum | Confirmed File |
| ---------- | -------------- |
| `AudioEnum.ButtonClick` | `Click01.wav` |
| `AudioEnum.CardSlide` | `Slide01.wav` |
| `AudioEnum.LocationOpen` | `Tamborine01.wav` |
| `AudioEnum.LocationLeadChange` | `Upgrade01.wav` |

| Field | Detail |
| ----- | ------ |
| Rationale | All four files were found under `bevy/crates/game/assets/audio/sfx/`, so implementation can use real runtime assets. |
| Alternatives Considered | Placeholder sounds; rejected because confirmed files are available. |

## Decision: Centralize Button Presentation

| Field | Detail |
| ----- | ------ |
| Decision | Add `button_ui_bundle` with one default style and migrate current buttons through that bundle. |
| Rationale | The click sound should attach to accepted shared button actions, and a shared bundle gives one future extension point for multiple styles without changing every screen. |
| Alternatives Considered | Leave screen-specific button construction in place; rejected because it would make consistent button sound and style coverage harder to verify. |

## Decision: Trigger Gameplay Sounds From Accepted State Changes

| Field | Detail |
| ----- | ------ |
| Decision | Play slide, location-open, and lead-change sounds only after accepted state transitions, not preview/redraw work. |
| Rationale | The spec distinguishes meaningful card movement and location scoring changes from recalculation or visual refresh. This avoids duplicate sounds during animation and redraw systems. |
| Alternatives Considered | Play sounds from rendering systems; rejected because rendering can rerun without state changes. Play sounds from input start; rejected because some inputs can be rejected. |

## Decision: Track Previous Location Winning Side

| Field | Detail |
| ----- | ------ |
| Decision | Store or derive the previous winning side per location and emit a lead-change request only when the new side is non-tied and different. |
| Rationale | The required `0,0 -> 0,3 -> 0,5 -> 6,5` behavior depends on comparing current and previous winners, not simply detecting any nonzero score or total change. |
| Alternatives Considered | Play on every score total change; rejected by the `0,3 -> 0,5` case. Play when any side is winning after a tie only; rejected because `0,5 -> 6,5` must also play. |
