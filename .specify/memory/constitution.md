<!--
Sync Impact Report
Version change: 1.6.0 -> 1.6.1
Modified principles:
- VII. Implementation Standards: names `bevy/crates/template-crate` as the proper local reference for Bevy folder, file, asset, and Rust coding standards.
Added sections:
None
Removed sections: None
Templates requiring updates:
- ✅ .specify/templates/plan-template.md
- ✅ .specify/templates/tasks-template.md
Follow-up TODOs: None
-->

# Bevy Card Game Constitution

## Core Principles

### I. Project Intent Is The Contract
All implementation work MUST follow the active specification, the project constitution, and the repo-local agent guidance. Bevy-specific rules belong in project specs, README guidance, and repo-local Codex memory.

### II. Clear Project Boundaries
Project behavior belongs in the Bevy source tree under `bevy/crates`. Runtime assets belong under `bevy/crates/game/assets`; repeatable scripts belong under `scripts`.

### III. Visible User Feedback
User-facing workflows SHOULD provide visible loading, progress, success, and failure feedback when work is asynchronous or can fail.

### IV. Local State Stays Explicit
Local files, caches, generated outputs, and user preferences MUST be documented and kept out of version control unless a spec explicitly makes them source artifacts.

### V. Data Changes Are Explicit
Schema changes, migrations, seed data, and destructive data operations MUST be explicit in specs and plans. Destructive database operations are not performed by agents.

### VI. Verify Real Behavior
User-visible changes SHOULD be verified through the real target workflow when practical. Prefer repository scripts in `scripts` after a generated project defines them.

### VII. Implementation Standards
All implementation code MUST follow Rust and Bevy conventions, including explicit ECS ownership through components, resources, systems, and plugins.

Bevy crate folder, representative file, asset folder, and Rust coding standards MUST use `bevy/crates/template-crate` as the proper local reference before agents add or reorganize Bevy runtime code or assets.

All folders and files under Rust implementation trees MUST use typical Rust project conventions: lowercase `snake_case` module, crate, source, test, and asset directory names, with `Cargo.toml` and Rust-required filenames kept in their standard forms. All folders under `bevy/`, including runtime asset folders, MUST use lowercase `snake_case` names.

Runtime source changed by a feature MUST be organized around one primary runtime concept per file. A primary runtime concept is a focused plugin, component, scene, view, model, or system. File and item names MUST be purposeful, using names such as `FooPlugin`, `FooComponent`, `BarScene`, `BazView`, `QuxModel`, or `TempSystem` when those terms match the concept being represented.

Runtime naming MUST distinguish app structure, data, and rendering. Use `Scene` for the persistent app-level scene, `Model` for data-holding concepts, and `View` for rendering or presentation concepts. The persistent app container is `AppScene`; active sub-screen presentations are views such as `GameView` and `CardBrowserView`.

Runtime system functions changed or created by a feature MUST use `[domain]_[schedule]_system`, such as `player_update_system`. Each primary runtime item changed or created by a feature MUST have a terse purpose comment immediately above it with exactly one `HUMAN:` line for human-level intent and one `AI:` line for implementation context or future AI guidance.

Specs and plans SHOULD record any framework-specific constraints before implementation begins.

Generated projects SHOULD keep implementation, tests, documentation, scripts, and assets in clearly named locations.

### VIII. Windows Desktop And Browser WebGPU Parity
The project and all user-visible capabilities MUST work on Windows desktop and in a browser through WebGPU unless a spec explicitly narrows the target for a temporary experiment. Rendering features, shaders, assets, UI overlays, input behavior, diagnostics, and tests SHOULD be designed with both targets in mind before implementation starts.

Plans and tasks MUST call out any target-specific risk, fallback, or verification gap for Windows desktop or browser WebGPU. A feature is not complete until the relevant desktop and browser workflows are both verified, or the unverified target is explicitly documented as blocked with the reason.

### IX. Aspect-Ratio-Relative Runtime Layout
The game MUST use the project aspect-ratio system for on-screen layout. All visible 2D UI elements and 3D presentation elements that appear on screen MUST be positioned relative to the aspect-ratio-safe game view rather than raw window pixels or ad hoc world coordinates.

When the screen size, window size, or target viewport changes, on-screen positions MUST be recalculated from the aspect-ratio-safe game view so the composition remains stable across supported desktop and browser WebGPU targets. Fixed pixel values MAY be used only as dimensions inside the virtual game view; placement MUST still derive from the aspect-ratio layout.

### X. Theme Asset Organization
Theme-owned card, location, and world assets MUST live under `bevy/crates/game/assets/themes/theme_<theme_name>/{cards,locations,worlds}/`. Theme-owned asset folders MUST use category prefixes such as `card_<card_name>`, `location_<location_name>`, and `world_<world_name>` and MUST NOT repeat the theme name because the containing theme root provides that identity.

Reusable assets that are not owned by one theme, such as shared shaders, MUST stay outside the theme-owned card, location, and world categories. Card data and card rendering MUST remain distinct: card data is represented as a `CardModel`, rendered card presentation as a `CardView`, and the visual bundle that creates the rendered card as a `CardViewBundle`.

## Project Constraints

- Keep project-specific source layout documented in `README.md` and active specs.
- Keep runtime assets under `bevy/crates/game/assets`.
- Keep `bevy/crates/template-crate` as the proper reference for Bevy crate folders, representative files, asset folders, and Rust coding standards.
- Keep Rust workspace folders and files in typical Rust naming conventions, including lowercase crate and module directories.
- Keep theme-owned card, location, and world assets under `bevy/crates/game/assets/themes/theme_<theme_name>/{cards,locations,worlds}/` with category-prefixed folders, and keep shared shaders under `bevy/crates/game/assets/shaders/`.
- Keep Bevy runtime files focused on one primary runtime concept and use Scene/Model/View naming where it distinguishes app structure, data, and presentation.
- Preserve the Codex and Specify workflow files unless a generated project intentionally replaces them.
- Keep `documentation/` updated when README images or supporting docs change.
- Keep `documentation/images/Overview01.png` and `documentation/images/Workflow01.png` as replaceable README image slots.
- Do not introduce unrelated refactors while implementing a feature spec.
- Do not introduce rendering, shader, asset, UI, or input capabilities that only work on one target unless the active spec documents the temporary limitation and follow-up path.
- Keep on-screen `GameView` and `CardBrowserView` layout positions derived from the aspect-ratio-safe game view, including 3D overlays that visually align with 2D UI.

## Development Workflow

1. Inspect current files before editing and keep changes scoped to the requested behavior.
2. Prefer repository scripts under `scripts` over ad hoc commands for setup, web serving, desktop serving, and tests.
3. For framework or dependency guidance, use official documentation and the project-local Codex rules before changing code.
4. If a port or build artifact is stale or locked, diagnose the actual process or path instead of assuming a clean environment.
5. Treat build output, dependency caches, runtime data, and test output as generated artifacts unless a spec explicitly says otherwise.
6. Verify Windows desktop and browser WebGPU workflows for user-visible features when practical; if one target cannot be verified, record the exact blocker.

## Governance

This constitution applies to all future Spec Kit specifications, plans, and task lists for this repository. Specs may add narrower acceptance criteria, but they must not contradict these principles without explicitly updating this constitution and documenting the reason.

**Version**: 1.6.1 | **Ratified**: 2026-04-30 | **Last Amended**: 2026-05-12
