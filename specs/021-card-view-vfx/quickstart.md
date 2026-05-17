# Quickstart: Card View Bundle Dynamic VFX

## Precheck
1. Ensure card assets are available under theme-specific directories:
   - `bevy/crates/game/assets/themes/theme_<name>/cards/card_<name>/...`
2. Confirm a 2D normal map for foreground and optional plasma/rim masks are present.
3. Start runtime:
   - `scripts/main/RunAppDesktop.ps1` (or hot-reload variant).

## Launch and validate
1. Open deck scene and place one card.
2. Slowly tilt card with mouse/inspection input.
3. Confirm:
   - foreground surface normals change with tilt,
   - frame rim brightens at grazing angles,
   - a diagonal plasma flash appears every 5 seconds for ~1 second.

## Visual checks
- Check at low and high tilt angles.
- Observe all layers (frame, foreground, title) stay legible.
- Verify no color clipping at sweep peak.

## Failure diagnosis
- If no effect is visible: verify shader asset recompilation and material registration.
- If frame-only effect leaks: inspect role-based branching for role masks or split materials.
- If timing feels off: verify cycle and active-window seconds in the shared timing state.

