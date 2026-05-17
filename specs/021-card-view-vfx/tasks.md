# Tasks: Card View Bundle Dynamic VFX

| Status | ID | File | Task |
| --- | --- | --- | --- |
| Pending | CVFX-001 | `bevy/crates/game/src/runtime/resources/card_model.rs` | Extend `CardModel` with normal map, rim mask, and plasma texture handles; provide safe defaults. |
| Pending | CVFX-002 | `bevy/crates/game/src/runtime/resources/card_model_registry_model.rs` | Add load/validation path so missing optional VFX handles do not break legacy cards. |
| Pending | CVFX-003 | `bevy/crates/game/src/runtime/components/card_vfx_component.rs` | Add/confirm VFX-related component fields for per-layer tuning metadata. |
| Pending | CVFX-004 | `bevy/crates/game/src/runtime/bundles/card_view_bundle.rs` | Wire VFX handles and role tags into foreground/frame/title mesh bundles. |
| Pending | CVFX-005 | `bevy/crates/game/src/runtime/systems/card_vfx_uniform_update_system.rs` | Add/update systems to publish angle-based VFX uniforms from inspection state. |
| Pending | CVFX-006 | `bevy/crates/game/src/runtime/systems/card_vfx_timing_update_system.rs` | Add/extend timing system for 5s cycle + 1s active sweep window. |
| Pending | CVFX-007 | `bevy/crates/game/src/runtime/shaders/materials.rs` | Extend card material struct(s) with VFX fields and align bind group layout. |
| Pending | CVFX-008 | `bevy/crates/game/src/lib.rs` | Register any new/extended materials in app plugin setup. |
| Pending | CVFX-009 | `bevy/crates/game/src/runtime/shaders/materials.rs` | Ensure material extraction uploads VFX timing and tilt/angle values. |
| Pending | CVFX-010 | `bevy/crates/game/assets/shaders/card_background_mask.wgsl` | Implement normal-map decode + lighting term for foreground shading. |
| Pending | CVFX-011 | `bevy/crates/game/assets/shaders/card_background_mask.wgsl` | Implement angle-aware rim glow for frame role. |
| Pending | CVFX-012 | `bevy/crates/game/assets/shaders/card_background_mask.wgsl` | Implement diagonal plasma sweep overlay with cycle/window controls. |
| Pending | CVFX-013 | `bevy/crates/game/assets/shaders/card_background_mask.wgsl` | Add soft blend/saturation clamps to avoid visual clipping. |
| Pending | CVFX-014 | `bevy/crates/game/assets/themes/<theme>/cards/...` | Add or wire normal map, frame rim mask, and plasma mask assets. |
| Pending | CVFX-015 | `specs/021-card-view-vfx/checklists/requirements.md` | Validate each checklist item and update status marks after implementation. |

## Milestones
- Milestone 1: data wiring complete (CVFX-001 to CVFX-006).
- Milestone 2: shader effects complete (CVFX-007 to CVFX-013).
- Milestone 3: art assets + validation pass (CVFX-014 to CVFX-015).

## Test tasks
- Manual: angle response and rim response in `deck_scene` and `game_scene`.
- Manual: plasma timing and sweep direction under repeated cycles.
- Smoke: confirm no regressions in card visibility and scene transitions.
