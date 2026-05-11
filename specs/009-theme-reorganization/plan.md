# Implementation Plan: Theme Reorganization

**Branch**: `[main]` | **Date**: 2026-05-11 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/009-theme-reorganization/spec.md`

**Note**: `.specify/scripts/powershell/setup-plan.ps1 -Json` reported `BRANCH` as `main` and copied the plan template into this 009 feature directory. The plan content below is filled for `specs/009-theme-reorganization`.

## Summary

Reorganize the current Japan proof-of-concept assets under a theme root so cards, locations, and worlds are grouped as `bevy/crates/game/assets/themes/theme_japan/{cards,locations,worlds}`. Rename theme-owned folders with category prefixes that do not repeat `japan`, update Bevy asset path references to load through the new tree, introduce purposeful Scene/Model/View naming so `AppScene` is always present, active sub-screen presentations are `GameView` and `CardBrowserView`, card data is `CardModel`, and visual spawning is `CardViewBundle`, and verify the existing card browsing, card flipping, world display, and tactical location presentation remain unchanged.

## Technical Context

**Language/Version**: Rust 2024  
**Primary Dependencies**: Bevy 0.18.1, `bevy_aspect_ratio_mask`, `bevy-inspector-egui`, `bevy-persistent`, `serde`, `serde_json`  
**Storage**: No new persistence; existing local settings stores remain unchanged  
**Testing**: `scripts/other/RunTests.ps1`, `scripts/other/RunAppDesktop.ps1 -CheckOnly`, and `scripts/other/RunAppWeb.ps1 -CheckOnly`  
**Target Platform**: Windows desktop and browser WebGPU parity  
**Project Type**: Bevy ECS game runtime in the existing Cargo workspace  
**Performance Goals**: Asset path reorganization must not add runtime asset discovery scans or change the current bounded card, world, and location counts  
**Constraints**: Keep all paths under `bevy/` lowercase `snake_case`; keep runtime assets under `bevy/crates/game/assets`; preserve ECS role boundaries; use `Scene` for the persistent app-level scene, `Model` for data concepts, and `View` for rendering/presentation concepts; split changed runtime source by one primary concept per file; name changed system functions as `[domain]_[schedule]_system`; add `HUMAN:` / `AI:` purpose comments above changed primary runtime items; do not change card, world, location, flip, or CardUI behavior; distinguish theme-owned assets from shared assets; update docs that still describe pre-009 asset locations  
**Scale/Scope**: One theme root, four card models with corresponding card view bundles, two worlds, six tactical locations, one shared shader directory, and no new gameplay content

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Gate | Status | Notes |
| ---- | ------ | ----- |
| Active spec, constitution, and repo-local guidance followed | PASS | Plan follows `specs/009-theme-reorganization/spec.md`, AGENTS guidance, and `.codex/rules/bevy-runtime-structure.md`; AGENTS path guidance now points at the 009 theme layout because 009 intentionally supersedes the old card/world/location layout. |
| Source, assets, scripts, docs, and tests stay in defined locations | PASS | Runtime changes stay under `bevy/crates/game`; assets stay under `bevy/crates/game/assets`; docs stay in README/spec artifacts; verification uses `scripts/`. |
| Rust workspace paths use lowercase conventions | PASS | New directories use lowercase names such as `themes/theme_japan/cards/card_kage_ren`. |
| Runtime source organization uses one primary concept per changed file | PASS | 009 implementation must split changed runtime source around purposeful Plugin, Component, Scene, View, Model, and System names. |
| Runtime systems and purpose comments follow project standards | PASS | Changed systems must use `[domain]_[schedule]_system`; changed primary items must include terse `HUMAN:` and `AI:` comments. |
| Scene/Model/View naming is explicit | PASS | `AppScene`, `GameView`, `CardBrowserView`, `CardModel`, `CardView`, and `CardViewBundle` are the canonical target names. |
| Theme asset layout follows project standard | PASS | Theme-owned cards, locations, and worlds move under `assets/themes/theme_japan/{cards,locations,worlds}` with category-prefixed folders. |
| Visible feedback preserved | PASS | Feature does not alter asynchronous loading or settings workflows. |
| Browser build constraints preserved | PASS | Feature changes static asset paths only and does not introduce browser storage, SQLite, or OPFS behavior. |
| Native database setup preserved | PASS | Feature does not touch database setup or seed behavior. |
| Browser-visible verification path exists | PASS | Use `scripts/other/RunAppWeb.ps1 -CheckOnly`; run real browser smoke if practical after implementation. |
| Language and framework standards followed | PASS | Bevy ECS ownership remains in resources, systems, components, and plugins; asset registry data remains explicit. |
| Framework-specific constraints documented | PASS | Plan documents Bevy asset-root behavior, explicit path references, desktop/browser parity, and no runtime directory scanning. |

## Project Structure

### Documentation (this feature)

```text
specs/009-theme-reorganization/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── asset-organization-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
bevy/crates/game/
├── assets/
│   ├── shaders/
│   │   └── card_background_mask.wgsl
│   └── themes/
│       └── theme_japan/
│           ├── cards/
│           │   ├── card_back.png
│           │   ├── safe_area.png
│           │   ├── card_kage_ren/
│           │   ├── card_lord_daichi/
│           │   ├── card_sister_hotaru/
│           │   └── card_yokai_placeholder/
│           ├── locations/
│           │   ├── location_fortress_gate/
│           │   ├── location_bamboo_crossing/
│           │   ├── location_shrine_ruins/
│           │   ├── location_battlefield/
│           │   ├── location_spirit_well/
│           │   └── location_market_square/
│           └── worlds/
│               ├── world_bamboo_forest/
│               └── world_coastal_harbor/
└── src/runtime/
    ├── components/mod.rs
    ├── plugins/mod.rs
    ├── resources/mod.rs
    └── systems/mod.rs

documentation/images/
└── Workflow01.png

scripts/
├── main/
│   └── RunAppDesktopHotReload.ps1
└── other/
    ├── GenerateCardFrameAssets.py
    ├── RunTests.ps1
    ├── RunAppDesktop.ps1
    └── RunAppWeb.ps1
```

**Structure Decision**: Keep the Bevy asset root unchanged and move only theme-owned card, location, and world assets below `assets/themes/theme_japan`. Shaders stay shared under `assets/shaders`. Runtime registries in `resources` remain the source of explicit card, world, and location asset paths; systems keep spawning the same entities from those resources. Documentation and implementation naming should distinguish app container, data, and rendering: `AppScene` is always present, `GameView` or `CardBrowserView` is the active sub-screen presentation loaded on top, `CardModel` stores card identity and asset data, `CardView` describes rendered card presentation, and `CardViewBundle` is the bundle that creates the card visuals with front presentation, back presentation, all layers, and view behavior.

## Complexity Tracking

No constitution violations are planned.
