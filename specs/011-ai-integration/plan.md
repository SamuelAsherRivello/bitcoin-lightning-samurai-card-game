# Implementation Plan: AI Integration

**Branch**: `[main]` | **Date**: 2026-05-11 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/011-ai-integration/spec.md`

**Note**: The normal Spec Kit feature creation script was not used because it runs `git checkout -b`, which is disallowed by this repository's Git safety rules. This plan was created directly from the plan template for `specs/011-ai-integration`.

## Summary

Create a durable AI integration plan that unifies prior debugging, visual QA, gameplay, theme, and point-system goals with future Codex runtime workflows. Choose `bevy_debugger_mcp` as the primary MCP for local desktop Bevy runtime inspection and screenshot capture because it is built around Bevy Remote Protocol, exposes agent-oriented observation and screenshot tools, and fits Codex's MCP model when installed/configured. Keep Playwright or the in-app browser as complementary browser WebGPU screenshot QA rather than the selected runtime MCP.

## Technical Context

**Language/Version**: Rust 2024  
**Primary Dependencies**: Bevy 0.18.1, Bevy Remote Protocol via `bevy::remote`, `bevy_debugger_mcp` as an external dev MCP candidate, existing `bevy-inspector-egui`, existing Codex/Spec Kit workflows  
**Storage**: No persistent game storage changes; transient screenshots and runtime snapshots must stay under project-local generated-output paths  
**Testing**: `scripts/other/RunTests.ps1`, `scripts/other/RunAppDesktop.ps1 -CheckOnly`, `scripts/other/RunAppWeb.ps1 -CheckOnly`, MCP smoke checks documented in [quickstart.md](./quickstart.md)  
**Target Platform**: Windows desktop for the selected MCP runtime bridge; browser WebGPU remains covered by complementary browser QA and documented gaps  
**Project Type**: Bevy ECS game runtime in the existing Cargo workspace plus repo-local Spec Kit planning artifacts  
**Performance Goals**: AI runtime tooling must be opt-in, local-only, and disabled from normal app startup; any enabled runtime bridge must avoid materially changing gameplay behavior outside dev sessions  
**Constraints**: Do not edit global Codex config without explicit user request; keep Bevy source under `bevy/crates`; keep scripts under `scripts`; keep screenshots and generated diagnostics project-local; preserve desktop/browser parity by documenting target-specific coverage; avoid exposing local-only services publicly  
**Scale/Scope**: Planning and contracts for one primary MCP selection, one dev-only Bevy Remote Protocol bridge, screenshot capture flow, runtime observation flow, and future AI QA artifacts; no MCP installation or runtime code implementation in this planning step

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Gate | Status | Notes |
| ---- | ------ | ----- |
| Active spec, constitution, and repo-local guidance followed | PASS | Plan follows `specs/011-ai-integration/spec.md`, AGENTS guidance, and `.codex/rules/bevy-runtime-structure.md`. |
| Source, assets, scripts, docs, and tests stay in defined locations | PASS | This planning step changes only `specs/011-ai-integration`, `.specify/feature.json`, and AGENTS active-plan metadata. Future source changes are scoped to `bevy/crates` and scripts. |
| Rust workspace paths use lowercase conventions | PASS | Planned feature names and future paths use lowercase `snake_case` conventions under `bevy/`. |
| Visible feedback preserved | PASS | Feature does not alter current loading, cache, refresh, or database UI feedback. |
| Browser build constraints preserved | PASS | BRP/MCP is desktop-first; browser WebGPU QA remains separate and any gap is documented. |
| Native database setup preserved | PASS | Feature does not touch native database setup or seed behavior. |
| Browser-visible verification path exists | PASS | Browser screenshots and smoke tests continue through served-web verification and browser automation. |
| Language and framework standards followed | PASS | Future implementation keeps Bevy ECS ownership in components, resources, systems, and plugins. |
| Responsive positioning and scaling preserved | PASS | AI QA must verify that visible 2D and 3D elements derive position and scale from the aspect-ratio-safe game view at fullscreen and smaller window sizes. |
| Framework-specific constraints documented | PASS | Research and contracts document Bevy Remote Protocol, reflection, native HTTP transport, local-only endpoint, and screenshot handler constraints. |

## Project Structure

### Documentation (this feature)

```text
specs/011-ai-integration/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/
    └── ai-runtime-mcp-contract.md
```

### Source Code (repository root)

```text
bevy/crates/game/
├── Cargo.toml
├── assets/
└── src/runtime/
    ├── components/
    ├── plugins/
    ├── resources/
    └── systems/

bevy/crates/shared/
└── src/

scripts/
├── main/
│   └── RunAppDesktopHotReload.ps1
└── other/
    ├── RunTests.ps1
    ├── RunAppDesktop.ps1
    └── RunAppWeb.ps1

specs/011-ai-integration/
└── contracts/
```

**Structure Decision**: Keep AI integration as a development tooling layer over the existing Bevy runtime rather than a new app or service inside the repository. Future implementation should add dev-only Bevy Remote Protocol and screenshot wiring through feature plugins under `bevy/crates/game/src/runtime/plugins/`, keep reusable diagnostics in `bevy/crates/shared` only when they are project-agnostic, and keep repeatable MCP smoke scripts under `scripts/` if automation is added.

## Complexity Tracking

No constitution violations are planned.

## Phase 0: Research Summary

Research is captured in [research.md](./research.md). The primary decision is to use `bevy_debugger_mcp` as the selected MCP and to keep direct BRP scripts, `bevy-inspector-egui`, terminal/tests, and browser automation as supporting alternatives.

## Phase 1: Design Summary

Design artifacts are captured in [data-model.md](./data-model.md), [quickstart.md](./quickstart.md), and [contracts/ai-runtime-mcp-contract.md](./contracts/ai-runtime-mcp-contract.md).

## Post-Design Constitution Check

| Gate | Status | Notes |
| ---- | ------ | ----- |
| Active spec and repo guidance remain aligned | PASS | Design artifacts preserve local-only MCP setup and Spec Kit workflow boundaries. |
| Desktop/browser parity is represented honestly | PASS | Desktop MCP is selected; browser WebGPU QA remains a complementary verification path with documented coverage differences. |
| Safety boundaries are explicit | PASS | The contract classifies operations and requires opt-in, localhost-scoped startup. |
| No unresolved clarifications remain | PASS | The MCP choice, alternatives, screenshot path, and configuration ownership are all resolved for planning. |
