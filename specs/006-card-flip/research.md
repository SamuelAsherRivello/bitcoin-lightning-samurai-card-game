# Research: Card Flip

## Decision: Represent flip state as game runtime ECS state separate from pointer tilt

**Rationale**: The current card inspection flow stores pointer target rotation in `CardInspectionState` and applies it in `smooth_card_rotation`. Keeping flip progress in separate runtime state lets the card continue using pointer-driven inspection while the y-axis side-selection animation advances independently.

**Alternatives considered**: Replacing `CardInspectionState.target_rotation` with a flip-only target was rejected because it would snap or suppress the existing pointer-driven feel. Encoding flip state in the UI state was rejected because Card UI state currently controls presentation tuning, not animation lifecycle.

## Decision: Compose final card rotation from inspection rotation plus flip rotation

**Rationale**: The spec requires x and z behavior to remain fed by pointer position while each flip advances y by 180 degrees from the current flip angle. A composed transform keeps inspection and flip concerns separable and makes midpoint face-swap checks depend on flip progress rather than absolute world orientation.

**Alternatives considered**: Slerping the root transform directly to a new quaternion was rejected because it mixes pointer tilt and flip state, making mid-animation pointer movement harder to reason about. Rotating child faces only was rejected because the whole card silhouette must visibly flip.

## Decision: Reverse mid-animation `Flip` clicks from current progress

**Rationale**: The clarification for this feature states that clicking `Flip` while a flip animation is already in progress reverses direction from current progress. This keeps the control responsive and avoids queued state surprises.

**Alternatives considered**: Ignoring clicks was rejected because it makes the button feel unresponsive. Queuing a follow-up flip was rejected because it can surprise reviewers after the current animation finishes.

## Decision: Swap CardFront/CardBack visibility at normalized flip midpoint

**Rationale**: The face graphics must change at the edge-on point, approximately 90 degrees from front-facing. Tracking normalized flip progress gives a deterministic midpoint independent of pointer tilt.

**Alternatives considered**: Swapping based on transform yaw was rejected because pointer tilt also contributes to yaw. Showing both faces and relying on backface culling was rejected for the first implementation because the current front is a multi-layer card with transparent/offset artwork and needs explicit side control.

## Decision: Store the backface as CardStructure asset content

**Rationale**: The user specified that the card series owns one shared backface and that the backface is not specific to an individual card front. `bevy/crates/game/assets/cards/card_structure/` is already the project-approved location for shared card structure assets.

**Alternatives considered**: Storing the backface under each front-art folder was rejected because it would make the back specific to an individual card front. Procedural-only material with no asset was rejected because the request asks to place backface art in the CardStructure folder.

## Decision: Treat existing front toggle as active card-front selection

**Rationale**: The current `T` behavior changes between existing front entries. For this feature, that behavior represents changing the active card definition/front content. If the card is face down, the shared CardBack remains visible and the change only becomes obvious after flipping face up.

**Alternatives considered**: Renaming the feature around full gameplay card definitions was rejected for `006` because broader Game, Player, Deck, and Table Top concepts have been moved to `007-gameplay-concepts`. Disabling `T` while face down was rejected because hidden front changes are part of the desired proof.

## Decision: Use one bold abstract superhero-pattern CardBack design

**Rationale**: The backface should feel like a game-wide card back that belongs beside the current superhero front card types. The provided trading-card back-cover inspiration uses ornate card-back composition, centered pattern energy, framed structure, and premium contrast; this feature translates those qualities into a superhero direction with compatible blues, whites, greys, and dark accents instead of medieval fantasy.

**Alternatives considered**: A plain white rectangle was rejected because it no longer meets the updated art-direction goal. A character, logo, readable letter, or explicit power emblem was rejected because the back must have no words, no characters, and no clear symbology. Designing box cover or main menu art now was rejected because those surfaces are future scope.

## Decision: Keep verification in existing repository scripts

**Rationale**: The repo guidance prefers repeatable scripts. `scripts/other/RunTests.ps1` covers Rust tests, `scripts/other/RunAppDesktop.ps1` covers Windows desktop behavior, and `scripts/other/RunAppWeb.ps1` covers browser WebGPU behavior when wasm tooling is available.

**Alternatives considered**: Ad hoc cargo commands were rejected as the primary documented workflow because repository scripts already encode project target directories, features, and serving behavior.
