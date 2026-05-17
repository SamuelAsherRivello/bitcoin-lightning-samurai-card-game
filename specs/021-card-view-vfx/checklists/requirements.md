# Requirements Checklist: Card View Bundle Dynamic VFX

| Requirement | Status | Notes |
| --- | --- | --- |
| Foreground normal map path exists in card model and is loaded from assets | ❌ | Add `foreground_normal_texture` to card model metadata |
| Normal-mapped shading uses mouse/card tilt as light axis input | ❌ | Reuse `CardInspectionState.target_rotation` as primary input |
| Rim glow responds to angle and is frame-only by role | ❌ | Update frame role/material pass only |
| Plasma sweep triggers on a repeating 5s period | ❌ | Implement timer logic in card VFX state |
| Plasma sweep active duration is ~1s | ❌ | Clamp/attenuate plasma strength outside sweep window |
| Diagonal sweep aligns to UL→LR UV direction | ❌ | Compute with UV transform in shader |
| Overlay covers frame, foreground, and title simultaneously | ❌ | Add shared sweep parameter to all affected material passes |
| Existing parallax and inspection feel preserved | ❌ | Verify against pre-change behavior in game scene |
| No safe-area breakage (HUD masking unaffected) | ❌ | Keep card layers under existing `bevy_aspect_ratio_mask` flow |
| No breaking API changes to runtime card spawning API without migration | ❌ | Use additive model fields with defaults |

## Optional polish checks
| Item | Status | Notes |
| --- | --- | --- |
| Add optional per-card intensity tuners for artists | ❌ | Extend card model with VFX coefficients |
| Add debug visual toggle for all VFX in debug HUD | ❌ | Nice-to-have for tuning |

