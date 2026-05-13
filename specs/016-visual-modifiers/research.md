# Research: Visual Modifier System

## Decision: Represent visual modifications as Condition, Target, and Treatment rules

| Field | Detail |
| ----- | ------ |
| Decision | Model every VMS entry as a `VisualModificationRule` with a Condition, Target, and Treatment. |
| Rationale | The rule shape makes each effect explainable: why it activates, what render element it touches, and how it changes that element. This is clearer for future changes than a flat list of special-case outline systems. |
| Alternatives considered | Directly changing materials in point value systems was simpler but would mix score calculation, ability resolution, target selection, and rendering presentation in one path. A flat named-modifier enum was clearer than raw colors but still did not separate activation from target selection and treatment. |

## Decision: Keep modifier state separate from point values

| Field | Detail |
| ----- | ------ |
| Decision | Existing point systems continue to set `PointModel` values; VMS Conditions derive presentation decisions from the same gameplay state. |
| Rationale | A point value can be modified without requiring the value model to know why it changed. The Condition cares about the cause of the value, while text rendering cares about the resolved value. |
| Alternatives considered | Extending `PointModel` with ability flags was rejected because `PointModel` is a shared value/type payload for card energy, card power, and location power. |

## Decision: Use the 015 card instance state model for card placement

| Field | Detail |
| ----- | ------ |
| Decision | Resolve card-owned `abilityoutline` from `CardInstanceId`, `CardInstanceStateModel`, `CardZoneModel::Location`, and `CardViewStateModel` where those are available. |
| Rationale | `015-card-states` has already introduced durable instance identity, location zone details, reveal policy, and view-state derivation. VMS should consume that model instead of adding another placement or visibility model. |
| Alternatives considered | Continuing to derive ability outlines directly from `HandCardGestureTarget` plus `CardSlotBoardModel::local_slot_for_card()` was retained only as an adapter fallback while existing render systems still expose hand indices. A new VMS-owned placement model was rejected as duplicate state. |

## Decision: Target the point circle/background child explicitly

| Field | Detail |
| ----- | ------ |
| Decision | Add a focused marker for point view background/circle children so rule Target resolution can update the correct visual child. |
| Rationale | Current child names include strings such as `PowerPointView Circle`, but systems should not depend on display names for behavior. A marker keeps Target resolution deterministic. |
| Alternatives considered | Name matching was rejected as brittle. Applying outlines to the point root was rejected because the user requested the circle behind the number. |

## Decision: Ability outline uses resolved active non-zero ability state

| Field | Detail |
| ----- | ------ |
| Decision | `abilityoutline` is active when a card power point view maps to a `CardInstanceStateModel` whose zone is `CardZoneModel::Location` and whose open location ability contributes a non-zero delta to that card's displayed power. |
| Rationale | Existing `GameLocationModel::ability_delta_for_location()` already encodes open/closed and no-ability behavior, while 015 now provides the card's authoritative location identity. |
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
| Rationale | Location total badges are UI nodes with borders, while card point badges are world-space mesh children. One Treatment contract can define outline intent while each surface uses the appropriate Bevy primitive. |
| Alternatives considered | Converting all point views to one rendering surface was rejected as too broad for the first VMS pass. |
