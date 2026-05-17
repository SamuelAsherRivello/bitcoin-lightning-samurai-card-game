# Spec: Card View Bundle Dynamic VFX

## Status
Planned

## Owner
Card runtime team

## Problem
The current card view bundle has static foreground, frame, and title textures. The card tilt interaction is only used by parallax and a frame shine effect, so the card still reads as flat art. We need depth- and motion-driven VFX that react to user inspection input and give the card a premium look.

## Goals
1. Add foreground normal-mapping so mouse-driven card angle visibly changes perceived surface lighting.
2. Add angle-aware rim glow for the frame to reinforce edge lighting and silhouette depth.
3. Add a diagonal plasma sweep animation that passes across frame, foreground, and title every 5 seconds over a 1 second window.
4. Keep the implementation compatible with the existing card bundle pipeline and safe-area rendering constraints.

## Non-goals
- No new gameplay mechanics.
- No new camera systems.
- No runtime network or asset-server changes; all visuals come from local assets + shader time.

## User-visible behavior
- As the card rotates due to mouse movement, foreground surfaces subtly catch and shift light.
- The frame glow should become strongest when the tilt is grazing and weakest at a top-down face orientation.
- Every 5 seconds, a diagonal plasma streak should sweep from upper-left to lower-right across the card content in a visible but brief 1-second pass.
- The effect should be readable on background, frame edges, foreground art, and title decals without obscuring card art.

## Requirements
- The foreground normal map must exist as an input texture and be sampled by shader math.
- Rim glow must be computed from card tilt and applied to frame rendering.
- Plasma overlay must move deterministically across each card with:
  - period: 5.0 seconds,
  - active duration: 1.0 second,
  - direction: top-left to bottom-right,
  - layered coverage on frame, foreground, and title.
- Effects must scale with card mesh size and keep existing parallax behavior.
- Existing card inspection controls and safe-area HUD masking must remain unchanged.

## Acceptance criteria
- At a static card angle of ~0°, the lighting stack is subtle with low rim visibility.
- At extreme tilt values, foreground shading and frame rim change materially and in the correct direction.
- The plasma pass is synchronized to a repeating 5-second cycle and is visibly aligned to diagonal motion.
- No frame-rate regressions from normal/shader time logic are introduced in local profiling on a reference scene.
- No unrelated scene logic (deck/game state, card data) is required to trigger VFX.

## Dependencies
- `bevy/crates/game/assets/shaders/card_background_mask.wgsl` extension for VFX uniforms and compositing.
- `bevy/crates/game/src/runtime/shaders/materials.rs` material definitions.
- `bevy/crates/game/src/runtime/systems` update path for card inspection, frame shine, and time-driven overlays.
- `bevy/crates/game/src/runtime/bundles/card_view_bundle.rs` and `runtime/resources/mod.rs` for new VFX assets and bundle wiring.
- `bevy/crates/game/src/lib.rs` material plugin registration if new materials are introduced.

