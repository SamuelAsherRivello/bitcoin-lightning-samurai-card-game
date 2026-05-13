# Implementation Plan: Card Bundle

**Branch**: `006-card-bundle` | **Date**: 2026-05-10 | **Spec**: [spec.md](spec.md)  
**Input**: Consolidated feature specification from `specs/006-card-bundle/spec.md`

## Summary

Consolidate the already implemented one-card inspection, card polish, and card flip specifications into a single card presentation bundle. The bundle preserves the current runtime behavior while removing obsolete cross-spec dependencies among `004-card-inspection-poc`, `005-card-polish`, and `006-card-flip`.

## Technical Context

| Area | Decision |
| ---- | -------- |
| Runtime | Bevy ECS card game under `bevy/crates/game` |
| Shared systems | Window, camera, DebugHUD, inspector, and diagnostics stay under shared runtime ownership |
| Primary scene language | `AppScene` persists; active presentations use views/scenes such as `GameScene`, `DeckScene`, and `DebugScene` |
| Card presentation | CardStructure owns layer order, aperture, parallax, shine, shared back, and flip composition |
| Artwork | CardType/CardDefinition entries provide replaceable front art |
| Prototype UI | Card UI remains temporary and separate from DebugHUD |
| Verification | `scripts/other/RunTests.ps1`, desktop check, web check, and manual card behavior smoke when runtime is available |

## Constitution Check

| Gate | Status | Notes |
| ---- | ------ | ----- |
| Scope stays within repository | ✅ | Consolidates specs under `specs/006-card-bundle` |
| Bevy runtime ownership preserved | ✅ | Runtime behavior remains under existing Bevy workspace structure |
| Spec links remain current | ✅ | Downstream references should point to `006-card-bundle` |
| No gameplay scope creep | ✅ | Gameplay concepts stay in `007-gameplay-concepts` |
| No destructive Git operations | ✅ | Only inspection and additive/editing work is required |

## Project Structure

```text
specs/006-card-bundle/
├── checklists/
│   └── requirements.md
├── contracts/
│   └── card-bundle.md
├── data-model.md
├── plan.md
├── quickstart.md
├── research.md
├── spec.md
└── tasks.md
```

## Phase Mapping

| Former Spec | Consolidated Role |
| ----------- | ----------------- |
| `004-card-inspection-poc` | Inspection baseline within the bundle |
| `005-card-polish` | CardFront/CardStructure/CardType polish within the bundle |
| `006-card-flip` | CardFront/CardBack flip behavior within the bundle |

## Dependencies

| Dependency | Relationship |
| ---------- | ------------ |
| `002-camera-setup` | Provides the fixed primary 3D camera used by inspection |
| `003-debugging` | Provides DebugHUD and diagnostic controls used by the bundle |
| `007-gameplay-concepts` | Defines future gameplay language that remains out of scope for this bundle |
| `008-game-theme-poc` and later specs | May build on card browsing, theme assets, and card flipping now linked through `006-card-bundle` |

## Verification Strategy

| Check | Purpose |
| ----- | ------- |
| `scripts/other/RunTests.ps1` | Confirm existing Rust tests still pass after documentation-only consolidation |
| `scripts/other/RunAppDesktop.ps1 -CheckOnly` | Confirm desktop compile path remains valid |
| `scripts/other/RunAppWeb.ps1 -CheckOnly` | Confirm web compile path remains valid |
| Manual card smoke | Verify one-card inspection, `T`, `R`, `H`, and `Flip` behavior when runtime is available |
| Link audit | Ensure no references remain to removed spec directories |

## Risks

| Risk | Mitigation |
| ---- | ---------- |
| Downstream specs still name removed folders | Run `rg` for old folder names and update links to `006-card-bundle` |
| Consolidation loses behavioral detail | Preserve inspection, polish, and flip requirements as separate sections in the bundle |
| Active plan comments drift | Keep `AGENTS.md` active implementation plan on `007-gameplay-concepts` unless the user asks to switch active work |
| Dirty worktree contains unrelated changes | Avoid touching unrelated implementation files |
