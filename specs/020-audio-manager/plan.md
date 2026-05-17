# Implementation Plan: Audio Manager

**Branch**: `main` | **Date**: 2026-05-15 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `specs/020-audio-manager/spec.md`

**Note**: This template is filled in by the `/speckit-plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

Add a game audio manager that accepts enum-driven SFX and Music requests, applies the existing persisted SFX/Music settings as independent channel mutes, and wires the first required sounds to shared button clicks, accepted card movement, location open transitions, and new location winning-side changes. Introduce a shared `button_ui_bundle` with exactly one default style, migrate existing buttons to it, and keep the behavior observable through tests that inspect audio requests and state transitions without depending on human hearing.

## Technical Context

| Field | Detail |
| ----- | ------ |
| Language/Version | Rust 2024 workspace edition |
| Primary Dependencies | Bevy 0.18.1, `bevy_aspect_ratio_mask` 0.4, `bevy-persistent` 0.10.0, existing Bevy ECS runtime modules |
| Storage | Existing `MetaGameSettingsModel` JSON persistence under `data/local_storage/meta-game-settings.json`; no new persistence required |
| Testing | `scripts/other/RunTests.ps1`; focused Cargo tests may be used while iterating |
| Target Platform | Windows desktop and browser WebGPU |
| Project Type | Bevy ECS game workspace under `bevy/crates/game` |
| Performance Goals | Audio request handling should be event-driven, avoid per-frame asset lookups, and add no visible frame stalls during button clicks, card movement, or location scoring |
| Constraints | Use existing SFX/Music settings as source of truth; keep runtime assets under `bevy/crates/game/assets`; use lowercase Rust module paths; changed primary runtime items require `HUMAN:` and `AI:` comments |
| Scale/Scope | Four SFX enum mappings, two channels, one default button style, all current game buttons, required gameplay/location triggers only |

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Gate | Status | Notes |
| ---- | ------ | ----- |
| Active spec, constitution, and repo-local guidance followed | ✅ | Plan is based on `specs/020-audio-manager/spec.md` and AGENTS guidance. |
| Source, assets, scripts, docs, and tests stay in defined locations | ✅ | Runtime code remains under `bevy/crates/game`; no new assets required. |
| `bevy/crates/template-crate` used as Bevy reference | ✅ | Implementation tasks should inspect it before adding runtime files. |
| Rust folders/files use lowercase conventions | ✅ | Proposed modules use `snake_case` file names. |
| Runtime files are focused around one primary concept | ✅ | Audio manager, button bundle, and trigger systems are separated by concept. |
| Changed runtime primary items include `HUMAN:` and `AI:` comments | ✅ | Required by implementation tasks. |
| Runtime system functions follow `[domain]_[schedule]_system` | ✅ | Planned systems should use names such as `audio_update_system`. |
| Scene/Model/View distinctions preserved | ✅ | Audio state is a model/resource; button presentation remains a bundle/view concept. |
| Theme-owned asset organization preserved | ✅ | Feature uses existing shared audio assets, not theme-owned card/location/world assets. |
| Visible loading/toast feedback remains for async/data workflows | ✅ | No async data workflow changes. |
| Browser builds keep localStorage snapshots and avoid browser SQLite/OPFS | ✅ | No storage changes beyond reading existing settings. |
| Native database setup remains unchanged | ✅ | No database changes. |
| Browser-visible changes have served-web verification path | ✅ | Browser verification should use existing app run workflow if implemented. |
| Language/framework standards followed | ✅ | Bevy ECS components/resources/systems/plugins remain the implementation pattern. |
| On-screen positions derive from aspect-ratio-safe view | ✅ | Button migration must preserve current safe-area UI placement. |
| Framework-specific API constraints documented | ✅ | Use Bevy audio playback/events and asset handles through ECS resources; avoid direct per-caller file loading. |

## Project Structure

### Documentation (this feature)

```text
specs/020-audio-manager/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── checklists/
    └── requirements.md
```

### Source Code (repository root)

```text
bevy/crates/game/
├── assets/
│   └── audio/
│       └── sfx/
│           ├── Click01.wav
│           ├── Slide01.wav
│           ├── Tamborine01.wav
│           └── Upgrade01.wav
├── src/
│   ├── runtime/
│   │   ├── bundles/
│   │   │   ├── button_ui_bundle.rs
│   │   │   └── mod.rs
│   │   ├── plugins/
│   │   │   └── mod.rs
│   │   ├── resources/
│   │   │   ├── audio_manager_model.rs
│   │   │   └── mod.rs
│   │   └── systems/
│   │       ├── audio_update_system.rs
│   │       ├── card_gesture_update_system.rs
│   │       ├── mod.rs
│   │       └── visual_modifier_update_system.rs
│   └── tests/
│       └── runtime/
│           ├── resources/
│           │   └── resources_tests.rs
│           └── systems/
│               └── systems_tests.rs
```

**Structure Decision**: Use the existing Bevy game crate runtime structure. Add a focused audio manager model/resource, add a focused shared button UI bundle, wire them through existing plugin/system registration, and update existing gameplay systems only where they already own the accepted state transitions.

## Complexity Tracking

No constitution violations are planned.

## Phase 0: Research

See [research.md](research.md).

## Phase 1: Design

See [data-model.md](data-model.md) and [quickstart.md](quickstart.md). No external API contracts are needed because this feature is internal to the Bevy game runtime.

## Post-Design Constitution Check

| Gate | Status | Notes |
| ---- | ------ | ----- |
| Design keeps behavior under active spec scope | ✅ | Data model covers only SFX/Music channels, enum mappings, button bundle, and specified triggers. |
| Design keeps implementation in project runtime locations | ✅ | All planned source changes stay under `bevy/crates/game/src/runtime`. |
| Design preserves Windows desktop and browser WebGPU parity | ✅ | Bevy audio and UI input paths are shared; verification covers both when practical. |
| Design preserves aspect-ratio-safe UI placement | ✅ | Shared button bundle is presentation-only and must retain existing parent layout constraints. |
| Design avoids new persistence, database, or external service complexity | ✅ | Existing settings are consumed; no new storage contract. |
