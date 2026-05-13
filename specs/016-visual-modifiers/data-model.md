# Data Model: Visual Modifier System

## Entities

| Entity | Fields | Relationships | Validation |
| ------ | ------ | ------------- | ---------- |
| `VisualModifier` | `AbilityOutline`, `LeadingScoreOutline` | Stored in `PointViewVisualModifiers` for a `PointView` root | Only known variants are applied; unknown visual names are not accepted. |
| `PointViewVisualModifiers` | Active modifier set for one point view | Attached to the same entity as `PointView` or derived for that entity during sync | Modifier set is idempotent; applying the same modifier twice has no visual duplication. |
| `PointViewCircle` | Optional surface type, outline target role | Attached to the circle/background child of a point view | Exactly one outline target is preferred per point view root; missing targets are tolerated without panic. |
| `AbilityOutlineRule` | Point type, card placement, active ability delta | Reads `PointView`, card placement state, and `GameLocationModel` | Applies only to card power point views with a non-zero active ability delta. |
| `LeadingScoreOutlineRule` | Location index, local total, opponent total | Reads paired `PointLocationView` point values | Applies only to the strictly higher location total; ties clear both sides. |

## Modifier Semantics

| Modifier | Applies To | Active When | Presentation |
| -------- | ---------- | ----------- | ------------ |
| `AbilityOutline` | Card `PowerPointView` circle/background | The card's displayed power includes an active non-zero ability delta | Gold outline around the circle behind the number |
| `LeadingScoreOutline` | Location total `PowerPointView` circle/background | The total is strictly greater than the paired total in the same location | White outline around the circle behind the number |

## State Transitions

| Trigger | Previous State | Next State |
| ------- | -------------- | ---------- |
| Card enters open ability location | No `AbilityOutline` | Add `AbilityOutline` to card power point view |
| Card leaves ability location | `AbilityOutline` | Remove `AbilityOutline` |
| Location opens and ability delta becomes active | No `AbilityOutline` | Add `AbilityOutline` to affected placed card power point views |
| Round reset or location no longer active | `AbilityOutline` | Remove `AbilityOutline` |
| Location total becomes strictly higher than paired total | No `LeadingScoreOutline` | Add `LeadingScoreOutline` to higher location total point view |
| Location totals become equal | One side has `LeadingScoreOutline` | Remove `LeadingScoreOutline` from both location total point views |
| Lead changes sides | One side has `LeadingScoreOutline` | Move `LeadingScoreOutline` to the new higher side |

## Validation Rules

| Rule | Requirement |
| ---- | ----------- |
| Ability modifier scope | `AbilityOutline` must not apply to card energy point views or location total point views in the first pass. |
| Leading modifier scope | `LeadingScoreOutline` must not apply to card point views in the first pass. |
| Tie behavior | Equal location totals must clear `LeadingScoreOutline` from both paired point views. |
| Hidden point views | Hidden point views may keep modifier state, but outline presentation must respect inherited visibility. |
| Missing visual child | Missing `PointViewCircle` target must skip presentation update without failing the system. |
| Multiple modifiers | Multiple active modifiers must be deterministic; initial scope keeps the two modifiers on different point view categories. |

## Derived Data Flow

| Step | Input | Output |
| ---- | ----- | ------ |
| Resolve point values | Slot board, card registry, hand model, location model | Updated `PointView.model.value` and point text |
| Resolve card ability outlines | Card placement, point type, active location ability delta | `AbilityOutline` present or absent on card power point views |
| Resolve location leading outlines | Paired location total point view values | `LeadingScoreOutline` present on higher side or absent on tie |
| Sync presentation | Point view modifier set and circle/background child | Border/ring/material state updated around the point circle |
