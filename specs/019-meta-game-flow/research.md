# Research: Meta Game Flow

## Decision: Extend `ActiveView` Rather Than Add A Separate Router

**Rationale**: The runtime already uses `ActiveView` to load one child presentation on top of persistent `AppScene`, and systems already understand how to hide/despawn Game, Deck, and Debug entities.

**Alternatives considered**: A new router resource was rejected because it would duplicate current scene ownership and require bridge logic for existing GameScene, DeckScene, and DebugScene systems.

## Decision: Reuse Existing Theme Assets For Cards, Locations, And Worlds

**Rationale**: The user stated the mockup graphics are temporary. The repository already has production-direction card, location, and world art under `bevy/crates/game/assets/themes/theme_japan/`.

**Alternatives considered**: Creating new mock assets was rejected for card/location/world surfaces because existing graphics are already organized by the constitution's theme asset rules.

## Decision: Build QR Placeholder With UI Primitives

**Rationale**: Real Lightning auth is out of scope; a UI-built placeholder avoids adding a one-off generated image while still providing the required visual affordance.

**Alternatives considered**: A generated bitmap QR was considered but would add asset management for a non-functional placeholder.

## Decision: Use Existing JSON Persistence Pattern For Settings

**Rationale**: Match mode already uses `bevy_persistent` and `data/local_storage/`. Extending that approach keeps settings explicit and local.

**Alternatives considered**: Keeping settings in memory was rejected because the requirement says all four settings save to disk.

## Decision: Treat Shop As Clickable No-Op

**Rationale**: The user explicitly said no shop layout is needed. The DeckScreen tab button can remain operational without rendering purchase content.

**Alternatives considered**: Rendering mock shop offers was rejected because it would imply unsupported Lightning purchase behavior.
