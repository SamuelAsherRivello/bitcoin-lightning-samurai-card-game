# Feature Specification: Card View State Model

**Feature Branch**: `015-card-states`  
**Created**: 2026-05-13  
**Status**: Draft  
**Input**: User request: "analyze the current states available for the card_view_bundle. It can be cardfront or cardback, it can be in the player deck, in the player hand, in a location, selected, or currently being dragged. Articulate all the states and state hierarchies, then propose a new data model to optimize and clarify the relationship"

## User Scenarios & Testing

### User Story 1 - Understand Current Card View State (Priority: P1)

As a developer, I need the card view state axes documented so I can reason about card front/back rendering, deck/hand/location ownership, selection, dragging, and CPU reveal behavior without reading every resource and system.

**Independent Test**: A developer can map each current state to the existing source artifact that owns it.

### User Story 2 - Separate Durable Gameplay State From Transient Presentation (Priority: P1)

As a developer, I need a proposed model that separates where a card is, which face should be visible, and what gesture or animation is active, so future features do not overload `CardState`, `CardGestureState`, or `CardViewBundle`.

**Independent Test**: Each proposed enum has one reason to change, and illegal combinations are either impossible or explicitly validated.

### User Story 3 - Preserve Existing Behavior While Enabling Cleanup (Priority: P2)

As a developer, I need a migration path that preserves current hand, location, selected, dragging, returning, placed, current-turn hidden, and revealed behavior.

**Independent Test**: Existing gesture and opponent-mode tests can be ported to the new model without changing user-visible behavior.

## Requirements

| ID | Requirement |
| -- | ----------- |
| FR-001 | The analysis MUST treat `CardViewBundle` as a visual root bundle, not as the durable owner of card gameplay state. |
| FR-002 | The analysis MUST document face state as an independent axis with `Front` and `Back`. |
| FR-003 | The analysis MUST document zone/state placement for deck, hand, location, and out-of-view card states. |
| FR-004 | The analysis MUST document transient interaction states for idle, pressed, selected, dragging, returning, and placed animation. |
| FR-005 | The proposed model MUST distinguish durable card instance state from render-facing view state. |
| FR-006 | The proposed model MUST support local player and opponent/CPU cards without separate parallel state machines. |
| FR-007 | The proposed model MUST represent same-turn movable location cards separately from locked location cards. |
| FR-008 | The proposed model MUST represent current-turn hidden opponent placements and revealed placements without coupling them to card front/back rendering internals. |
| FR-009 | The proposed model MUST define validation rules for illegal state combinations such as a deck card being selected or a locked location card being dragged. |
| FR-010 | The proposed model MUST fit the existing Bevy ECS runtime organization under `bevy/crates/game/src/runtime/`. |

## Key Entities

| Entity | Description |
| ------ | ----------- |
| `CardViewBundle` | Current Bevy bundle that spawns a `CardView` root, transform, visibility, and child face layers. |
| `CardFace` | Current front/back face enum used by flip and CPU placement systems. |
| `CardStateModel` | Current local-player hand-index state map for hand, dragging, location, and locked location cards. |
| `CardGestureModel` | Current single active gesture focus for pressed, selected, dragging, returning, and placed visual transitions. |
| `CardSlotBoardModel` | Current location slot occupancy and slot geometry model. |
| `PlacementVisibilityModel` | Current opponent-mode reveal state for placed cards. |
| `CardInstanceStateModel` | Proposed durable per-card-instance gameplay model. |
| `CardViewStateModel` | Proposed per-rendered-card presentation model derived from card instance state plus active interaction. |

## Assumptions

| Topic | Assumption |
| ----- | ---------- |
| Implementation | The initial plan is documentation-first; generated tasks define the approved follow-up implementation path. |
| Scope | The model should support the currently visible gameplay surfaces first: deck, local hand, local slots, opponent slots, selected inspection, drag preview, and CPU reveal. |
| Compatibility | Existing resources may be migrated incrementally instead of replaced in one large refactor. |

## Success Criteria

| ID | Measurable Outcome |
| -- | ------------------ |
| SC-001 | The generated plan names every current card state axis and the source artifact that owns it. |
| SC-002 | The generated data model identifies hierarchy, validation, and transitions for the proposed state model. |
| SC-003 | The generated contract gives a compact table of legal and illegal combinations. |
