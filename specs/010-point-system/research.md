# Research: Point System

## Decision: Represent point values with small typed models

| Field | Decision |
| ----- | -------- |
| Choice | Use dedicated `CostPointModel` and `PowerPointModel` wrappers around signed integer values instead of raw `i32` fields at every call site. |
| Rationale | Cost and power have different gameplay meanings even when both render as numbers. Typed models prevent cost from accidentally participating in scoring and keep display-range validation close to the value contract. |
| Alternatives considered | Raw numeric card fields were simpler but made requirement FR-003 easier to violate. A generic `PointModel` enum was rejected because the spec gives cost and power distinct responsibilities and view contracts. |

## Decision: Keep base card data separate from match-state effective power

| Field | Decision |
| ----- | -------- |
| Choice | Add card-definition cost/base-power values separately from a runtime card-instance model that can hold resolved effective-power changes. |
| Rationale | FR-005 and FR-030 require match-state changes to be separate from base card definition data, and 007 already distinguishes `CardDefinition` from `CardInstance`. |
| Alternatives considered | Mutating `CardModel` power during play was rejected because it would mix source card data with match state and make modifiers difficult to reason about. |

## Decision: Compute location totals from revealed effective power

| Field | Decision |
| ----- | -------- |
| Choice | A location total is the sum of revealed card instances owned by that player at the location, up to the default four-card capacity, plus resolved location-level modifiers. |
| Rationale | This directly implements FR-020 and the edge cases for unrevealed cards, moved cards, negative values, and modifiers. |
| Alternatives considered | Storing totals as manually edited display fields was rejected because displayed totals must reflect the resolved current match state. |

## Decision: Use deterministic enum results for control and outcome

| Field | Decision |
| ----- | -------- |
| Choice | Model location control as local, opponent, or none; model match outcome as local win, opponent win, or draw. |
| Rationale | The spec requires ties and empty equal-zero locations to have no controller, then final outcome to compare controlled-location counts before total power. |
| Alternatives considered | Boolean control flags were rejected because they cannot represent no-controller ties cleanly. |

## Decision: Render point views through existing Bevy UI/GameView patterns

| Field | Decision |
| ----- | -------- |
| Choice | Implement `CostPointView` and `PowerPointView` as focused Bevy presentation concepts with dynamic foreground text and an artful background, positioned from existing card and location layouts. |
| Rationale | The repo already uses Bevy ECS and UI concepts for GameView, card presentation, and location overlays. This keeps desktop/browser behavior aligned and avoids introducing a new UI framework. |
| Alternatives considered | Texture-only baked numbers were rejected because the spec requires dynamic foreground text for `-99` through `99`. |

## Decision: Defer full gameplay systems outside point scoring

| Field | Decision |
| ----- | -------- |
| Choice | Do not implement deckbuilding, card draw, energy, CPU strategy, complete turn rules, card abilities, or production UI in this feature. |
| Rationale | FR-031 explicitly excludes those systems. The point model should provide stable integration targets for later features. |
| Alternatives considered | Building a full playable loop now was rejected because it would expand the feature beyond its spec and create unclear dependencies. |
