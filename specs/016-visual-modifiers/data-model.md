# Data Model: Visual Modifier System

## Entities

| Entity | Fields | Relationships | Validation |
| ------ | ------ | ------------- | ---------- |
| `VisualModificationRule` | `name`, `condition`, `target`, `treatment` | Declarative unit evaluated by VMS | Every rule must define exactly one Condition, one Target selector, and one Treatment. |
| `VisualModificationCondition` | `CardPowerModifiedByAbility`, `LocationTotalIsLeading` | Reads gameplay/model state without mutating presentation | Conditions must be deterministic and must not perform rendering work. |
| `VisualModificationTarget` | `CardPowerPointCircle`, `LocationTotalPointCircle` | Resolves to a concrete point view circle/background child | Missing targets must be tolerated without panic. |
| `VisualModificationTreatment` | `Outline { color, width }` | Applied to resolved targets by presentation sync | Treatments must not change point values or text content. |
| `VisualModifier` | `AbilityOutline`, `LeadingScoreOutline` | Stored in `PointViewVisualModifiers` for a `PointView` root as active rule output | Only known variants are applied; unknown visual names are not accepted. |
| `PointViewVisualModifiers` | Active modifier set for one point view | Attached to the same entity as `PointView` or derived for that entity during sync | Modifier set is idempotent; applying the same modifier twice has no visual duplication. |
| `PointViewCircle` | Optional surface type, outline target role | Attached to the circle/background child of a point view | Exactly one outline target is preferred per point view root; missing targets are tolerated without panic. |
| `PointViewCardInstanceLink` | `CardInstanceId` | Links a card-owned point view root to the 015 durable card instance state | Must reference an existing instance when the 015 model is authoritative; adapter fallback may synthesize the link from hand index temporarily. |
| `AbilityOutlineRule` | Condition, Target, Treatment | Reads `PointView`, 015 card instance state, and `GameLocationModel` | Condition: card power modified by active non-zero ability; Target: card power point circle; Treatment: gold outline. |
| `LeadingScoreOutlineRule` | Condition, Target, Treatment | Reads paired `PointLocationView` point values | Condition: location total is strictly higher than paired total; Target: leading location total point circle; Treatment: white outline. |

## Rule Semantics

| Rule | Condition | Target | Treatment |
| ---- | --------- | ------ | --------- |
| `abilityoutline` | Linked card power point value includes an active non-zero ability delta | Card `PowerPointView` circle/background | Gold outline around the circle behind the number |
| `leadingscoreoutline` | Location total is strictly greater than the paired total in the same location | Leading location total `PowerPointView` circle/background | White outline around the circle behind the number |

## State Transitions

| Trigger | Previous State | Next State |
| ------- | -------------- | ---------- |
| Card instance enters open ability location | No `AbilityOutline` | Add `AbilityOutline` to linked card power point view |
| Card leaves ability location | `AbilityOutline` | Remove `AbilityOutline` |
| Location opens and ability delta becomes active | No `AbilityOutline` | Add `AbilityOutline` to affected placed card power point views |
| Round reset or location no longer active | `AbilityOutline` | Remove `AbilityOutline` |
| Location total becomes strictly higher than paired total | No `LeadingScoreOutline` | Add `LeadingScoreOutline` to higher location total point view |
| Location totals become equal | One side has `LeadingScoreOutline` | Remove `LeadingScoreOutline` from both location total point views |
| Lead changes sides | One side has `LeadingScoreOutline` | Move `LeadingScoreOutline` to the new higher side |

## Validation Rules

| Rule | Requirement |
| ---- | ----------- |
| Rule completeness | Every visual modification must include a Condition, Target, and Treatment before implementation. |
| Condition separation | Conditions must not mutate Bevy entities or rendering assets. |
| Target separation | Targets must identify the smallest intended render element, such as the circle/background rather than the full card. |
| Treatment separation | Treatments must not recalculate gameplay values. |
| Ability modifier scope | `AbilityOutline` target selection must not include card energy point views or location total point views in the first pass. |
| Leading modifier scope | `LeadingScoreOutline` target selection must not include card point views in the first pass. |
| Tie behavior | Equal location totals must clear `LeadingScoreOutline` from both paired point views. |
| Hidden point views | Hidden point views may keep modifier state, but outline presentation must respect inherited visibility. |
| 015 view state | `CardViewStateModel` hidden/revealed face behavior must win over VMS presentation; outlines never reveal a hidden card point view. |
| Missing visual child | Missing `PointViewCircle` target must skip presentation update without failing the system. |
| Multiple modifiers | Multiple active modifiers must be deterministic; initial scope keeps the two modifiers on different point view categories. |

## Derived Data Flow

| Step | Input | Output |
| ---- | ----- | ------ |
| Resolve point values | Slot board, card registry, hand model, location model | Updated `PointView.model.value` and point text |
| Evaluate card ability Condition | `PointViewCardInstanceLink`, `CardInstanceStateModel`, point type, active location ability delta | `AbilityOutline` active or inactive for the card point view |
| Evaluate location leading Condition | Paired location total point view values | `LeadingScoreOutline` active on higher side or inactive on tie |
| Resolve Targets | Active rule, point view root, circle/background marker | Concrete child entity or no target |
| Apply Treatments | Active rule treatment and resolved target | Border/ring/material state updated around the point circle |

## 015 Integration

| 015 Model | VMS Use |
| --------- | ------- |
| `CardInstanceId` | Stable key linking rendered card point views to durable card state. |
| `CardInstanceStateModel` | Preferred source for owner, card model ID, location zone, side, slot, and reveal policy. |
| `CardZoneModel::Location` | Determines whether a card point view is eligible for `AbilityOutline` and which `location_index` to query. |
| `CardViewStateModel` | Ensures outline sync follows derived visibility, face, z-band, and input-affordance behavior rather than inventing separate render rules. |
| 015 adapter helpers | Temporary bridge from current `GameHandModel`, `CardStateModel`, `CardSlotBoardModel`, and CPU views until point views carry direct instance links. |
