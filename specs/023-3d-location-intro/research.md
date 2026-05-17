# Research: 3D Location Intro

## Decision: Use 3D mesh rectangles for the location background graphic

**Rationale**: The current GameScene already renders the world background as a 3D rectangle and cards through 3D cameras. A matching rectangle mesh for each location gives the requested 3D asset while allowing size and position to derive from `CardSlotBoardModel::location_area_rect`.

**Alternatives considered**: Keeping the location background as UI only would not satisfy the 3D requirement. Rebuilding the whole location, including text and point views, as custom 3D text would add risk and duplicate existing point view behavior.

## Decision: Keep title, body, border, and point overlays in the existing safe-area UI layer

**Rationale**: The existing UI overlay already owns readable title/body text, two location point views, and the colored border. It renders below the card overlay camera and above the world background, matching the requested visual ordering when combined with the 3D rectangle.

**Alternatives considered**: Moving all overlays into 3D would require new text and point rendering paths and would risk breaking current point update behavior. Removing overlays would fail the same-look requirement.

## Decision: Animate each location with deterministic per-location delay

**Rationale**: The requested sequence can be represented by a start delay based on location index: 0.0 seconds, 1.0 seconds, and 2.0 seconds. Each animation lasts 0.5 seconds, so the 0.5-second pauses are preserved without a separate sequencing resource.

**Alternatives considered**: A global intro resource could serialize states, but the current requirement is fixed to exactly three locations and does not need a broader state machine.

## Decision: Use ease-out cubic timing

**Rationale**: Ease-out cubic gives the requested fast-start, slow-finish feel and is deterministic in tests.

**Alternatives considered**: Linear timing was rejected because the spec explicitly requires ease-out. Spring timing was rejected because the requested final timing has strict 0.5-second animation and wait measurements.
