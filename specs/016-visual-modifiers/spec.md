# Feature Specification: Visual Modifier System

**Feature Branch**: `016-visual-modifiers`  
**Created**: 2026-05-13  
**Status**: Draft  
**Input**: User request: "The visual modifier system or VMS is a feature that visually updates some ui elements in some cases. First pass: if any card point_views have an active ability applied then put a gold border around the element, around the circle behind the number, called abilityoutline. Second pass: if the point_view within a location is higher than the other point_view within that location then put a white border around it, called leadingscoreoutline. Generalize VMS so each visual modification has a Condition, Target, and Treatment."

## User Scenarios & Testing

### User Story 1 - Highlight Ability-Modified Card Points (Priority: P1)

As a player, I need card power point views affected by an active location ability to show a gold outline around the circular point background so I can distinguish base values from modified values.

**Independent Test**: Place a card at an open location with a non-zero active ability delta and verify the card's power point circle receives `abilityoutline`; place or view the same card without an active ability and verify the outline is absent.

**Acceptance Scenarios**:

| Scenario | Given | When | Then |
| -------- | ----- | ---- | ---- |
| Ability applies | A card power point view is in a location whose open ability changes card power | The card point value is rendered or updated | A gold `abilityoutline` appears around the point circle behind the number |
| Ability does not apply | A card power point view is in hand, in a closed location, or in a location with no non-zero ability delta | The card point value is rendered or updated | No `abilityoutline` appears |
| Ability state changes | A location opens, closes through reset, or card placement changes which ability applies | Point views synchronize | `abilityoutline` appears or disappears with the resolved ability state |

### User Story 2 - Highlight Leading Location Totals (Priority: P2)

As a player, I need the higher location total point view to show a white outline so I can quickly see who is leading each location.

**Independent Test**: Set one location's local and opponent totals to unequal values and verify only the higher total point circle receives `leadingscoreoutline`; set equal totals and verify neither total has it.

**Acceptance Scenarios**:

| Scenario | Given | When | Then |
| -------- | ----- | ---- | ---- |
| Local leads | A location local total is higher than the opponent total | Location totals are rendered or updated | The local location `PowerPointView` circle has a white `leadingscoreoutline` |
| Opponent leads | A location opponent total is higher than the local total | Location totals are rendered or updated | The opponent location `PowerPointView` circle has a white `leadingscoreoutline` |
| Tie | Both point views in a location have equal values | Location totals are rendered or updated | Neither location total point circle has `leadingscoreoutline` |
| Empty location | Both totals are zero | Location totals are rendered or updated | Neither location total point circle has `leadingscoreoutline` |

### User Story 3 - Define Reusable Visual Modification Rules (Priority: P3)

As a developer, I need every visual modification to be represented as a clear rule with a Condition, Target, and Treatment so later card, location, score, and UI effects can be added without inventing new ad hoc rendering paths.

**Independent Test**: A developer can inspect the VMS rule list and identify for each visual modification why it activates, what UI element it changes, and what treatment is applied.

## Edge Cases

| Case | Expected Behavior |
| ---- | ----------------- |
| Card has modified value of zero | `abilityoutline` is based on an active non-zero ability application, not on whether the final point value is positive. |
| Ability delta is zero or no ability | No `abilityoutline` is shown because no visual modifier is active. |
| Location totals tie after ability modifiers | `leadingscoreoutline` is removed from both location total point views. |
| A point view is hidden by card-face visibility | Modifier state may be stored, but hidden point views must not create visible outlines until the point view is visible. |
| A point view is despawned or reparented | VMS synchronization must tolerate missing children and avoid panics. |
| Both rules apply to different point views in one location | Card point views may receive the `abilityoutline` treatment while the leading location total point view receives the `leadingscoreoutline` treatment; the rules remain independent because their targets differ. |

## Requirements

| ID | Requirement |
| -- | ----------- |
| FR-001 | The system MUST define a Visual Modifier System concept where each visual modification rule has a Condition, Target, and Treatment. |
| FR-002 | The system MUST evaluate visual modification Conditions separately from applying Treatments. |
| FR-003 | The system MUST define the `abilityoutline` rule with Condition: a card power point value is modified by an active non-zero ability; Target: that card power point view's circular background; Treatment: a gold outline. |
| FR-004 | The system MUST remove the `abilityoutline` Treatment when its Condition is false or its Target no longer exists. |
| FR-005 | The system MUST define the `leadingscoreoutline` rule with Condition: a location total point value is strictly higher than the paired total in the same location; Target: the leading location total point view's circular background; Treatment: a white outline. |
| FR-006 | The system MUST apply `leadingscoreoutline` only to the higher of the two location total point view Targets within the same location. |
| FR-007 | The system MUST remove `leadingscoreoutline` from both location total point view Targets when their values are equal. |
| FR-008 | The system MUST keep point value calculation separate from visual modifier presentation. |
| FR-009 | The system MUST work with existing `PointView`, `PointLocationView`, `CardSlotBoardModel`, `GameLocationModel`, and card power update behavior. |
| FR-010 | The system MUST keep outlines attached to the circle/background element rather than the text element or entire card/location. |
| FR-011 | The system MUST preserve Windows desktop and browser WebGPU parity without introducing target-specific rendering dependencies. |
| FR-012 | The system MUST keep all point view outline positions derived from the existing point view layout inside the aspect-ratio-safe GameView. |
| FR-013 | The system MUST use `CardInstanceId`/`CardInstanceStateModel` zone identity from `015-card-states` as the preferred card-state source when available, with adapters allowed only while older hand-index slot state remains authoritative. |

## Key Entities

| Entity | Description |
| ------ | ----------- |
| `VisualModificationRule` | Reusable VMS rule made of one Condition, one Target selector, and one Treatment. |
| `VisualModificationCondition` | Predicate that determines whether a rule is active, such as "card power is modified" or "location total is leading". |
| `VisualModificationTarget` | Selector for the render element to change, such as a point view circle/background child. |
| `VisualModificationTreatment` | Presentation operation applied to the target, such as a gold or white outline. |
| `VisualModifier` | Named presentation state emitted by a rule, initially `AbilityOutline` and `LeadingScoreOutline`. |
| `PointViewVisualModifiers` | Proposed component or model that stores active visual modifiers for a `PointView` entity. |
| `PointViewCircle` | Proposed marker for the circle/background child that receives outline presentation. |
| `abilityoutline` | Initial rule whose Condition is modified card power, Target is the card power point circle, and Treatment is a gold outline. |
| `leadingscoreoutline` | Initial rule whose Condition is leading location score, Target is the leading location total point circle, and Treatment is a white outline. |
| `PointView` | Existing point badge value/type component for card energy, card power, and location power. |
| `PointLocationView` | Existing location total point marker containing location index and side. |
| `CardInstanceStateModel` | Completed 015 durable card state model used to determine whether a card point view belongs to a location and which active ability applies. |
| `CardViewStateModel` | Completed 015 render-facing card state model used to respect hidden/revealed face and visibility behavior when syncing point outlines. |

## Assumptions

| Topic | Assumption |
| ----- | ---------- |
| Scope | First implementation targets point views only, not full card frames, location frames, tooltips, or animation effects. |
| Ability source | Existing non-zero open location ability deltas are the first active ability source for `abilityoutline`. |
| Card points | `abilityoutline` applies to card power point views, not energy/cost point views, unless a future ability modifies cost. |
| Leading score | `leadingscoreoutline` applies to location total point views only, not individual card power point views. |
| Presentation | Outlines can be implemented with Bevy UI border properties for UI point views and an additional ring/child mesh or equivalent material-backed presentation for world-space card point views. |
| Compatibility | The current point value systems may calculate modifier flags while preserving current numeric text behavior. |
| 015 dependency | `015-card-states` is complete; VMS should prefer its card instance, zone, reveal, and view-state helpers over adding another card placement model. |

## Success Criteria

| ID | Measurable Outcome |
| -- | ------------------ |
| SC-001 | A card power point view affected by an active non-zero ability shows exactly one gold outline around its circle. |
| SC-002 | A card power point view not affected by an active non-zero ability shows no gold ability outline. |
| SC-003 | For each location with unequal totals, exactly one location total point view shows a white leading score outline. |
| SC-004 | For each location with equal totals, neither location total point view shows a white leading score outline. |
| SC-005 | Modifier outlines update after card placement, card movement, round/location opening, and score recalculation without requiring a respawn. |
