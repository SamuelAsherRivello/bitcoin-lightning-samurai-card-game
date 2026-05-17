# Research Notes: Card View Bundle Dynamic VFX

## Current pipeline context
- Card visuals are assembled from separate mesh/material layers with roles and parallax behavior already present.
- Existing shader support is centralized in `CardBackgroundMaskMaterial`.
- Frame shine is already driven by inspection rotation, so we can reuse angle semantics.

## Shader strategy options
1. Extend `CardBackgroundMaskMaterial`
   - Single material receives normal map + glow params + plasma state.
   - Lowest material-type churn, smaller plugin surface.
   - Requires careful handling of frame-specific contribution without blending artifacts.
2. Introduce dedicated foreground and frame materials
   - Cleaner role separation and independent tuning.
   - More Bevy asset/plugin setup work and extra passes.

Recommended default: start with option 1 to keep the change surgical, then split into dedicated materials if role isolation becomes hard to tune.

## Technical approach
- **Normal mapping**
  - Add a normal map texture handle to card model data.
  - Convert sampled tangent-space normals to world lighting vector using card-local basis.
  - Compute `ndotL` and feed it into base albedo + subtle spec/roughness-like modulation.
- **Rim glow**
  - Use the same tilt-derived light direction.
  - `rim = pow(1.0 - max(dot(view_dir, normal), 0.0), rim_power)` with card-specific intensity.
  - Restrict to frame role via uniform mask or role material branch.
- **Plasma sweep**
  - Keep global timer in a scene/resource.
  - `phase = (time - last_cycle_start) / cycle_seconds`.
  - Sweep window active if `phase < active_window_ratio`.
  - Sweep position on UV-space: `u + v` offset by time.
  - Use a soft ramp edge to avoid hard clipping.

## Open questions
- Are normal maps generated per-card art in current art pipeline, or do we start with temporary test normals?
- Can rim color be shared across theme assets or should it remain artist-editable per card?
- Should plasma overlay use a procedural texture, a palette texture, or a fixed noise pattern texture?

