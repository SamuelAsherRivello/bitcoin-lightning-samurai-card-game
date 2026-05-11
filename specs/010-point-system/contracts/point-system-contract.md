# Contract: Point System

## Model Contract

| Operation | Inputs | Output | Required Behavior |
| --------- | ------ | ------ | ----------------- |
| Create cost point | Signed integer value | `CostPointModel` | Preserves value and exposes display text for `-99` through `99` |
| Create power point | Signed integer value | `PowerPointModel` | Preserves value, including negative values, and exposes display text for `-99` through `99` |
| Calculate effective power | Card definition base power plus resolved card-level modifiers | `PowerPointModel` | Does not mutate base card definition data |
| Calculate location total | Player-owned revealed card instances at one location plus resolved location-level modifiers | `PowerPointModel` | Excludes unrevealed cards by default; includes at most the allowed capacity unless later rules change capacity |
| Evaluate location control | Local and opponent location totals | `LocationController` | Higher total controls; equal totals return no controller |
| Evaluate match outcome | Three location controls and totals | `MatchOutcome` | More controlled locations wins; tied control count uses total power; tied total power draws |

## View Contract

| View | Inputs | Required Presentation |
| ---- | ------ | --------------------- |
| `CostPointView` | `CostPointModel`, owning entity layout | Artful background plus dynamic foreground numeric text |
| `PowerPointView` on card | `PowerPointModel`, owning card layout | Artful background plus dynamic foreground numeric text distinct from cost |
| `PowerPointView` on location total | `PowerPointModel`, location layout, player perspective | Opponent total appears at top; local total appears at bottom |

## Acceptance Samples

| Scenario | Input | Expected |
| -------- | ----- | -------- |
| Local leads location | Local total `5`, opponent total `3` | Local controls location |
| Opponent leads location | Local total `2`, opponent total `7` | Opponent controls location |
| Tied location | Local total `4`, opponent total `4` | No controller |
| Empty location | Local total `0`, opponent total `0` | No controller |
| Location-count win | Local controls 2, opponent controls 1 | Local wins |
| Total-power tiebreak | Both control 1 location, local total power `12`, opponent total power `10` | Local wins |
| Draw | Same controlled count and same total power | Draw |

## Out Of Scope

| Area | Status |
| ---- | ------ |
| Deckbuilding | Excluded |
| Card drawing | Excluded |
| Energy/play-budget rules | Future integration only |
| CPU strategy | Excluded |
| Complete turn resolution | Excluded |
| Card/location ability authoring | Excluded except for compatibility with resolved modifiers |
| Final production UI | Excluded |
