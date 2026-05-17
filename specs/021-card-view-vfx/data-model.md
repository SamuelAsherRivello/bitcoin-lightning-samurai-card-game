# Data Model: Card View Bundle Dynamic VFX

## Card content data

| Entity | Current field | New/Updated field | Purpose |
| --- | --- | --- | --- |
| `CardModel` (global card metadata) | `foreground_texture` | `foreground_texture`, `foreground_normal_texture: Handle<Image>` | Add normal map source for foreground shading |
| `CardModel` | `frame_texture` | `frame_texture`, `frame_rim_mask_texture: Option<Handle<Image>>` | Optional rim mask support for frame-only glow shaping |
| `CardModel` | `title_texture` | `title_texture` + optional `title_plasma_mask` | Reuse in overlay pass or default to full-coverage title UV |
| `CardModel` | none | `plasma_mask_texture: Option<Handle<Image>>` | Optional stylized plasma shape/noise texture |

## Runtime VFX state

| State type | Responsibility |
| --- | --- |
| `CardVfxUniforms` (shader uniform struct) | Passes tilt vector, light direction, time, and sweep params to GPU |
| `CardVfxTiming` resource | Tracks global elapsed time and emits periodic cycle progress for all cards |
| `CardVfxLayerConfig` component (optional) | Per-entity overrides for intensity, rim power, pulse timing |

## Suggested new shader inputs

| Uniform/Input | Type | Notes |
| --- | --- | --- |
| `vfx_tilt` | `vec4<f32>` | `x,y` for tilt axis; `z` strength; `w` optional rim enable flag |
| `normal_map_enabled` | `f32` | 1.0 when normal map bound |
| `plasma_time` | `f32` | seconds since cycle start |
| `plasma_cycle` | `f32` | total cycle length (`5.0`) |
| `plasma_active_window` | `f32` | active sweep length (`1.0`) |
| `rim_intensity` | `f32` | strength multiplier |
| `rim_power` | `f32` | controls edge falloff |

## Persistence and defaults
- New fields should use safe default handles (placeholder neutral textures) so all existing card definitions remain loadable.
- If missing/disabled normal or plasma assets, effects fall back to base color behavior with no added artifacts.
- VFX toggles should avoid blocking card spawn if handles are null.

