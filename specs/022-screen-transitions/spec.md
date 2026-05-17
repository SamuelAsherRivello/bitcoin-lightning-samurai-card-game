# Spec: Screen Transitions

## Status
Planned

## Owner
Runtime UI team

## Problem
Screen changes happen instantly and can feel abrupt. The game currently has no fullscreen transition layer to hide screen swaps or provide smooth entry when the app starts.

## Goals
1. Add a dedicated transition UI layer rendered in front of all other game content.
2. On initial app load, start fully black and fade into gameplay.
3. On any screen change, fade out to black, apply the screen switch while black is fully opaque, then fade back in.
4. Make transition color and timing configurable, with default black color, 1.0 seconds of fade time (0.5 out + 0.5 in), and a 0.2 second hold at full black.

## Non-goals
- No new gameplay logic.
- No scene data model redesign.
- No audio transition work in this feature.

## User-visible behavior
- The first rendered frame appears black and quickly fades in.
- Triggering any screen transition produces a short fade-to-black, then fades back from black after the new screen becomes active.
- Transition overlay always appears above HUD, cards, and scene visuals.

## Requirements
- Transition layer is fullscreen and draws in front of all other content.
- Default transition color is solid black.
- Default fade duration is 1.0 seconds total, split across fade out and fade in.
- Default full-black hold duration is 0.2 seconds.
- Screen switch operation occurs only at full black (between the two fade phases).
- Transition trigger path must support all existing screen/view changes.

## Acceptance criteria
- On cold app start, content appears only after black fades away.
- Switching between any two existing screens/views visibly performs: fade out -> switch -> fade in.
- During transition, no underlying screen popping is visible.
- Transition duration is perceptibly 1.2 seconds total under normal frame rate (1.0 seconds fade + 0.2 seconds full-black hold).
- Overlay remains in the aspect-ratio-safe presentation stack and does not break desktop/browser parity.

## Dependencies
- Screen/view transition trigger system in `bevy/crates/game/src/runtime/`.
- UI/HUD layering setup that currently controls render order.
- Any existing screen state resource/event used to request scene/view changes.
