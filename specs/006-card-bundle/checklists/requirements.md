# Requirements Checklist: Card Bundle

**Purpose**: Validate that the consolidated card presentation spec is complete and no longer depends on removed `004`, `005`, or `006-card-flip` spec folders.  
**Created**: 2026-05-12  
**Feature**: [spec.md](../spec.md)

## Content Quality

| ID | Item | Status |
| -- | ---- | ------ |
| CQ-001 | No implementation-only details appear where user behavior should be specified | ✅ |
| CQ-002 | Inspection, polish, and flip behavior remain distinct enough for QA | ✅ |
| CQ-003 | Redundant old-spec dependency language has been removed | ✅ |
| CQ-004 | Scope exclusions are explicit for gameplay, tabletop placement, final menus, collection UI, and multi-card layout | ✅ |

## Requirement Completeness

| ID | Item | Status |
| -- | ---- | ------ |
| RC-001 | One-card inspection behavior is covered | ✅ |
| RC-002 | Poker-card proportions and fixed camera behavior are covered | ✅ |
| RC-003 | CardStructure/CardType separation is covered | ✅ |
| RC-004 | Four front layers, aperture masking, parallax, and shine are covered | ✅ |
| RC-005 | `T`, `R`, and `H` prototype/debug controls are covered | ✅ |
| RC-006 | Card UI `Flip`, midpoint face swap, and mid-animation reversal are covered | ✅ |
| RC-007 | Shared CardBack ownership and content constraints are covered | ✅ |
| RC-008 | Future gameplay concepts remain delegated to `007-gameplay-concepts` | ✅ |

## Link Hygiene

| ID | Item | Status |
| -- | ---- | ------ |
| LH-001 | Downstream references should point to `006-card-bundle` instead of `004-card-inspection-poc` | ✅ |
| LH-002 | Downstream references should point to `006-card-bundle` instead of `005-card-polish` | ✅ |
| LH-003 | Downstream references should point to `006-card-bundle` instead of `006-card-flip` | ✅ |
