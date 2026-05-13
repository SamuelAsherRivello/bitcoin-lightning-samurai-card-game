# Research: Visual Modifier System

## Decision: Represent visual modifiers as named state

| Field | Detail |
| ----- | ------ |
| Decision | Add explicit modifier names such as `AbilityOutline` and `LeadingScoreOutline` rather than encoding meaning only in colors or materials. |
| Rationale | Named state makes future modifiers testable, avoids scattering rule names across presentation code, and lets multiple systems reason about active UI effects without parsing visual details. |
| Alternatives considered | Directly changing materials in point value systems was simpler but would mix score calculation, ability resolution, and rendering presentation in one path. |

## Decision: Keep modifier state separate from point values

| Field | Detail |
| ----- | ------ |
| Decision | Existing point systems continue to set `PointModel` values; VMS derives presentation modifiers from the same gameplay state. |
| Rationale | A point value can be modified without requiring the value model to know why it changed. The outline rule cares about the cause of the value, while text rendering cares about the resolved value. |
| Alternatives considered | Extending `PointModel` with ability flags was rejected because `PointModel` is a shared value/type payload for card energy, card power, and location power. |

## Decision: Target the point circle/background child explicitly

| Field | Detail |
| ----- | ------ |
| Decision | Add a focused marker for point view background/circle children so outline sync can update the correct visual child. |
| Rationale | Current child names include strings such as `PowerPointView Circle`, but systems should not depend on display names for behavior. A marker keeps hierarchy traversal deterministic. |
| Alternatives considered | Name matching was rejected as brittle. Applying outlines to the point root was rejected because the user requested the circle behind the number. |

## Decision: Ability outline uses resolved active non-zero ability state

| Field | Detail |
| ----- | ------ |
| Decision | `abilityoutline` is active when a card power point view is in a location where an open location ability contributes a non-zero delta to that card's displayed power. |
| Rationale | Existing `GameLocationModel::ability_delta_for_location()` already encodes open/closed and no-ability behavior, so VMS can use the same resolved signal as point value updates. |
| Alternatives considered | Showing the outline for any location that has ability text was rejected because closed locations and no-op abilities should not imply an active modifier. |

## Decision: Leading score outline derives from paired location totals

| Field | Detail |
| ----- | ------ |
| Decision | `leadingscoreoutline` is derived by comparing the two `PointLocationView` values for a given `location_index`. |
| Rationale | Location total point views already hold location index, side, and resolved total value, which is the exact comparison surface needed for leading score feedback. |
| Alternatives considered | Recomputing totals from slots inside VMS was rejected as a default because it duplicates `update_location_power_points`; a fallback may use slot totals only if ordering makes component values unavailable. |

## Decision: Use surface-specific outline presentation behind one contract

| Field | Detail |
| ----- | ------ |
| Decision | UI location point views may use Bevy UI border presentation; world-space card point views should use an outline ring mesh, scaled child, or equivalent WebGPU-compatible presentation around the background circle. |
| Rationale | Location total badges are UI nodes with borders, while card point badges are world-space mesh children. One contract can define the modifier and color while each surface uses the appropriate Bevy primitive. |
| Alternatives considered | Converting all point views to one rendering surface was rejected as too broad for the first VMS pass. |
