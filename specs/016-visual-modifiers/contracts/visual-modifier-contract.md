# Contract: Visual Modifier System

## Rule Contract

| Rule | Condition | Target | Treatment |
| ---- | --------- | ------ | --------- |
| `abilityoutline` | Card `PowerPointView` is linked to a `CardInstanceId` whose `CardInstanceStateModel.zone` is a location with an active non-zero ability delta affecting displayed power | The linked card power point view's circle/background child | Gold outline around the target |
| `leadingscoreoutline` | A location total `PowerPointView` value is strictly greater than the paired total in the same location | The leading location total point view's circle/background child | White outline around the target |

## Update Contract

| Operation | Inputs | Required Behavior |
| --------- | ------ | ----------------- |
| Evaluate Conditions | Rule list plus card, point, score, and location state | Produces active or inactive rule decisions without mutating presentation |
| Resolve Targets | Active rule decisions plus point view roots and target markers | Produces target entities such as point circle/background children |
| Apply Treatments | Active rule decisions plus resolved targets | Updates only the target's visual treatment, not point text or gameplay value |
| Update card ability modifiers | Card point view, `CardInstanceId`, `CardInstanceStateModel`, active location ability delta | Activates `abilityoutline` only when the linked card power point is actively modified by a non-zero ability |
| Update location leading modifiers | Paired local/opponent location total point views | Activates `leadingscoreoutline` on the higher value and clears both on ties |
| Clear stale modifiers | Missing placement, removed ability, equal totals, hidden/despawned children | Removes inactive modifier presentation without panics |
| Adapter bridge | Existing hand-index point view data plus 015 adapter helpers | Produces equivalent modifier decisions until card point views carry direct `CardInstanceId` links |

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
