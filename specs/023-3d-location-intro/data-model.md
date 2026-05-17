# Data Model: 3D Location Intro

| Entity | Fields | Relationships | Validation |
| ------ | ------ | ------------- | ---------- |
| `LocationBundle` | `location_index` | Marks the combined location presentation for one shared GameScene location. | Index must correspond to one of the three shared locations. |
| `LocationBundleSurface` | `location_index`, material handle via mesh material component | Marks the 3D textured rectangle surface. | Surface opacity must match intro progress. |
| `LocationBundleOverlay` | `location_index` | Marks the safe-area UI overlay containing title/body, border, and point views. | Overlay scale and opacity must match intro progress. |
| `LocationBundleIntro` | `location_index`, `elapsed_seconds` | Drives the intro state for the surface and overlay for one location. | Progress clamps to 0.0 before start and 1.0 after completion. |
| `LocationBundleIntroVisual` | `location_index` | Marks child visuals whose colors should fade with the intro. | Visual alpha must match eased intro progress. |

## State Transitions

| State | Timing | Opacity | Scale |
| ----- | ------ | ------- | ----- |
| Pending | Before start delay | 0% | 150% |
| Animating | During the 0.5-second ease-out animation | 0% to 100% | 150% to 100% |
| Complete | After animation duration | 100% | 100% |

## Sequence

| Location | Start Delay | Animation Duration | Completion |
| -------- | ----------- | ------------------ | ---------- |
| Location 01 | 0.0 seconds | 0.5 seconds | 0.5 seconds |
| Location 02 | 1.0 seconds | 0.5 seconds | 1.5 seconds |
| Location 03 | 2.0 seconds | 0.5 seconds | 2.5 seconds |
