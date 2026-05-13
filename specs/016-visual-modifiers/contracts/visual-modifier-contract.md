# Contract: Visual Modifier System

## Modifier Contract

| Modifier | Input Surface | Activation Rule | Visual Output |
| -------- | ------------- | --------------- | ------------- |
| `abilityoutline` | Card `PowerPointView` | Card point value includes an active non-zero ability delta | Gold outline around the point circle behind the number |
| `leadingscoreoutline` | Location total `PowerPointView` | This location total is strictly greater than the paired total in the same location | White outline around the point circle behind the number |

## Update Contract

| Operation | Inputs | Required Behavior |
| --------- | ------ | ----------------- |
| Update card ability modifiers | Card point view, card placement, active location ability delta | Sets `abilityoutline` only when the card power point is actively modified by a non-zero ability |
| Update location leading modifiers | Paired local/opponent location total point views | Sets `leadingscoreoutline` on the higher value and clears both on ties |
| Sync outline presentation | Active modifier set and point circle/background child | Updates only the circle/background outline, not the text or the whole card/location |
| Clear stale modifiers | Missing placement, removed ability, equal totals, hidden/despawned children | Removes inactive modifier presentation without panics |

## Acceptance Samples

| Scenario | Input | Expected |
| -------- | ----- | -------- |
| Card receives +2 location ability | Base power `3`, active delta `+2`, displayed power `5` | Card power point has gold `abilityoutline` |
| Card receives -2 location ability | Base power `3`, active delta `-2`, displayed power `1` | Card power point has gold `abilityoutline` |
| Card at no-ability location | Base power `3`, active delta `0`, displayed power `3` | Card power point has no `abilityoutline` |
| Closed location with future ability | Location has ability text but is not open | Card power point has no `abilityoutline` |
| Local leads location | Local total `5`, opponent total `3` | Local location total has white `leadingscoreoutline`; opponent does not |
| Opponent leads location | Local total `2`, opponent total `7` | Opponent location total has white `leadingscoreoutline`; local does not |
| Tied location | Local total `4`, opponent total `4` | Neither location total has `leadingscoreoutline` |

## Out Of Scope

| Area | Status |
| ---- | ------ |
| Full ability authoring system | Excluded |
| Animated outline pulses | Future enhancement |
| Modifier tooltips | Future enhancement |
| Full card frame outlines | Excluded from this pass |
| Location frame control effects | Excluded from this pass |
| New runtime assets or shaders | Not planned for the first pass |
