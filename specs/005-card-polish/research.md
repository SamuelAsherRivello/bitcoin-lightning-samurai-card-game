# Research: Card Polish

## Decision: Use Bevy `StandardMaterial` texture planes for visual styles

**Rationale**: The current prototype already renders a small set of flat front-face planes and drives apparent depth by translating child entities from card tilt. `StandardMaterial.base_color_texture` keeps the implementation compatible with desktop and WebGPU without adding a custom material pipeline.

**Alternatives considered**: A custom shader material was considered for every layer, but it would add WebGPU shader compatibility risk before the card type system is proven. Manual geometry patterns were rejected because the spec explicitly replaces dot construction with static generated textures.

## Decision: Keep frame masking geometric for this phase

**Rationale**: The existing frame is four planes around a central aperture. Sizing the background plane to the frame hole keeps the background visible only through the aperture while retaining the current parallax model.

**Alternatives considered**: GPU stencil or alpha-mask clipping would be more general, but it is unnecessary for the current one-card proof and introduces avoidable render pipeline complexity.

## Decision: Use a two-slot in-memory CardType registry

**Rationale**: The feature only needs one active card type and one reserved slot. A resource-backed registry is enough to validate card type separation and the HUD `T` toggle without introducing external data files or serialization.

**Alternatives considered**: Asset manifests or serialized card type files were rejected for this phase because the second card type is not defined yet and the spec does not require runtime authoring.

## Decision: Use generated PNG assets with chroma-key removal for breakout layers

**Rationale**: Static PNGs satisfy the generated texture requirement. Transparent foreground and title assets allow the character and title to break over the frame without extra masking systems.

**Alternatives considered**: Hand-coded vector art was rejected because the spec requests generated static textures. Native transparent image generation was not required because the built-in image workflow plus local chroma-key removal produced alpha PNGs.

## Decision: Import only `R`/`H` AppScene workflow from `bevy-zoo-game`

**Rationale**: The related specs define `R` as a non-toggle AppScene reload operation and `H` as a persisted hot-reload auto-restart toggle. For this card app, the reloadable scope is the primary camera and card structure, so the reviewer workflow transfers without importing zoo models, model browser state, or unrelated gameplay systems.

**Alternatives considered**: Importing the whole zoo scene lifecycle was rejected because it would add non-card assets and browser systems outside this feature. Leaving `R`/`H` undocumented in 005 was rejected because the user explicitly requested bringing those spec behaviors into the local specs before implementation.
