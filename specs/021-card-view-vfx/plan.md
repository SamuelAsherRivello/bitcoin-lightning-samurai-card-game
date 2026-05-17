# Plan: Card View Bundle Dynamic VFX

## Objective
Implement three linked VFX features on `card_view_bundle`:
- Foreground normal-mapped lighting driven by card angle from mouse.
- Frame rim glow driven by the same angle input.
- Diagonal plasma sweep across frame/foreground/title every 5s with 1s duration.

## Scope
- Focus on runtime visuals only; no gameplay logic changes.
- Preserve current card bundling and parallax semantics.
- Keep all changes additive and behind sane defaults.

## Execution plan

### Phase 1 — Data and plumbing (low risk)
1. Extend card model metadata with vfx texture handles and default fallbacks.
2. Add lightweight timing resource for VFX phase calculations in card runtime systems.
3. Thread card-specific and global VFX parameters into card spawn bundle/component metadata.
4. Ensure no card spawn path breaks when optional assets are absent.

### Phase 2 — Foreground normal mapping + rim glow
1. Extend `CardBackgroundMaskMaterial` (or split material if required by role leakage during tuning).
2. Update WGSL to:
   - sample foreground normal map,
   - decode tangent normals,
   - apply angle-aware lighting term,
   - add rim term for frame contribution.
3. Wire `CardInspectionState.target_rotation` to material uniforms for responsive updates.
4. Ensure frame/foreground layers retain current depth ordering and transparency behavior.

### Phase 3 — Plasma sweep overlay
1. Add periodic timer logic in a scene-independent runtime system.
2. Pass sweep parameters to card shader materials.
3. In shader, compute UL→LR diagonal sweep:
   - periodic phase at 5.0s,
   - active only for 1.0s,
   - soft falloff edge and low alpha to avoid masking the title/foreground.
4. Apply same sweep UVs to frame, foreground, and title materials/binds.

### Phase 4 — Tune and harden
1. Review with quick art tuning pass for strength and contrast.
2. Confirm performance budget by comparing frame time in reference scenes.
3. Add guardrails for missing textures and zero/NaN rotations.

## Implementation risks
- Role mixing in a shared material might leak rim color into non-frame layers; solve via role masks or split materials.
- Normal map tangent basis may become unstable with current mesh orientation; validate UV and normal basis on generated card mesh.
- Plasma timing drift under frame hitching should remain stable and loop-corrected.

## Definition of done
- All three effects are visible in game + deck scenes.
- Effects scale with card angle and respond immediately to pointer-driven tilt updates.
- All new effects can be disabled without altering visual correctness.
- Checklist in `checklists/requirements.md` is fully satisfied.

## Files touched
- `bevy/crates/game/src/runtime/resources/mod.rs`
- `bevy/crates/game/src/runtime/components/mod.rs`
- `bevy/crates/game/src/runtime/bundles/card_view_bundle.rs`
- `bevy/crates/game/src/runtime/shaders/materials.rs`
- `bevy/crates/game/src/runtime/shaders.rs` (if helper wiring exists)
- `bevy/crates/game/src/runtime/systems/card_animation_systems.rs`
- `bevy/crates/game/src/runtime/systems/mod.rs`
- `bevy/crates/game/assets/shaders/card_background_mask.wgsl`
- `bevy/crates/game/src/lib.rs`
- `bevy/crates/game/assets/shaders/card_vfx_common.wgsl` (if split constants are preferred)

