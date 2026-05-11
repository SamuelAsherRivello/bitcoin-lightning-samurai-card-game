# Data Model: Point System

## Entities

| Entity | Fields | Relationships | Validation |
| ------ | ------ | ------------- | ---------- |
| `CostPointModel` | `value: i32` | Owned by card definitions and future cost-bearing entities | Display contract supports `-99` through `99`; values outside that range may exist only without readable display guarantee |
| `PowerPointModel` | `value: i32` | Owned by card definitions, effective card state, and location totals | Display contract supports `-99` through `99`; negative values are allowed |
| `CardDefinition` / `CardModel` | `cost: CostPointModel`, `base_power: PowerPointModel`, existing visual/card fields | Source data for playable cards; creates card instances | Every playable card must expose cost and power |
| `CardInstanceModel` | `definition_id`, `owner`, `reveal_state`, `location_slot`, `effective_power_delta` or resolved effective power | Runtime copy of a card definition in match state | Unrevealed cards do not contribute to visible location totals by default |
| `LocationScoreModel` | `index`, `local_total: PowerPointModel`, `opponent_total: PowerPointModel`, `local_modifier`, `opponent_modifier`, `capacity_per_player` | Derived from card instances at one shared location | Default capacity is four cards per player; extra placement is invalid unless later rules change capacity |
| `LocationControlModel` | `controller: LocationController` | Derived from one `LocationScoreModel` | Higher total controls; equal totals have no controller |
| `MatchScoreModel` | `locations: [LocationScoreModel; 3]`, `round: 1..=6` | Aggregates all shared locations | Exactly three shared locations for current feature scope |
| `MatchOutcomeModel` | `result: MatchOutcome`, `local_controlled_count`, `opponent_controlled_count`, `local_total_power`, `opponent_total_power` | Derived after final scoring from `MatchScoreModel` | Controlled-location count decides winner first; total power is the tiebreaker; equal both ways is draw |
| `CostPointView` | `model: CostPointModel`, background presentation, dynamic text presentation | Renders cost for a card or future cost-bearing entity | Must render numeric text for `-99` through `99` |
| `PowerPointView` | `model: PowerPointModel`, background presentation, dynamic text presentation | Renders card power and shared location totals | Must render numeric text for `-99` through `99` |

## State Transitions

| Transition | Input | Output |
| ---------- | ----- | ------ |
| Define card points | Card definition source data | Playable card exposes cost and base power |
| Create card instance | Card definition and owner | Runtime instance starts with base power as effective power before modifiers |
| Reveal card | Card instance reveal state changes to revealed | Card can contribute effective power to its owner's location total |
| Move card | Card instance location changes | Old location total no longer includes the card; new location total includes it after resolution if revealed |
| Apply modifier | Card or location modifier resolves | Effective power or location total changes without mutating card base power |
| Recalculate location score | Current card instances and modifiers | Local and opponent totals update for each location |
| Evaluate control | One location score | Local, opponent, or none |
| Evaluate match outcome | Final scores for three locations | Local win, opponent win, or draw |

## Rules

| Rule | Requirement Coverage |
| ---- | -------------------- |
| Cost never contributes to location totals, control, or match outcome. | FR-001, FR-003 |
| Power contributes only through revealed effective power at the card's current location by default. | FR-002, FR-005, FR-020 |
| Top location number is opponent total and bottom location number is local total from the local player's perspective. | FR-021, FR-022 |
| Tied and empty equal-zero locations have no controller. | FR-024, FR-025 |
| Final match outcome compares controlled-location count, then total power across all locations, then draw. | FR-026, FR-027, FR-028 |
| Future abilities resolve into the same effective-power, total, capacity, and control model. | FR-032 |
