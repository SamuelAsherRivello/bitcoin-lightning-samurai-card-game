# Screen Transition Contract

## Purpose
Define the runtime behavior contract for fullscreen screen transitions.

## Inputs

| Input | Type | Source |
| --- | --- | --- |
| View change request | Event or state mutation | Existing top-nav/meta/game/deck/debug flow |
| Transition config | Resource fields | `ScreenTransitionModel` defaults |

## Required Behavior

| Contract | Requirement |
| --- | --- |
| Startup | App starts with fullscreen black overlay and fades to transparent.
| Runtime switch sequence | Fade out to black -> apply view switch -> fade in.
| Ordering | Overlay remains visually above all screen content while active.
| Timing | Default cycle is 1.2 seconds total: 1.0 seconds fade time (0.5 out + 0.5 in) plus 0.2 seconds hold at full black.
| Color | Default transition color is black.

## Output

| Output | Consumer |
| --- | --- |
| Updated `ActiveView` at black gate | Existing view systems/scenes |
| Overlay alpha updates per frame | UI renderer |

## Failure Handling

| Case | Handling |
| --- | --- |
| New transition request while non-idle | Keep current transition running and replace the queued follow-up request (`queued_view`) with the latest requested target.
| Active request to current view | Ignore request and keep phase unchanged.
| Invalid duration (`<= 0`) | Clamp to safe minimum epsilon before interpolation.

## Phase Timing

| Phase | Behavior |
| --- | --- |
| `StartupFadeIn` | Start at alpha `1.0` and fade to `0.0`, then enter `Idle`. |
| `FadeOutPendingSwitch` | Fade from current alpha to `1.0` before switching view. |
| `SwitchAtBlack` | Apply requested `ActiveView` transition while overlay is fully opaque. |
| `HoldAtBlack` | Keep overlay fully opaque for `0.2` seconds before fade-in. |
| `FadeInAfterSwitch` | Fade from `1.0` to `0.0`, then return to `Idle` or consume queued request. |
