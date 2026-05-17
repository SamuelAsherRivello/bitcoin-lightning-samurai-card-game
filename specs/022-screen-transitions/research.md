# Research: Screen Transitions

## Decision 1: Use one persistent AppScene-owned fullscreen UI overlay
- Decision: Spawn a single transition overlay node under the AppScene HUD tree, keep it alive for the full session, and animate only background alpha.
- Rationale: Avoids repeated spawn/despawn and guarantees top-layer ordering across all views.
- Alternatives considered: Recreate overlay per screen switch (more churn and ordering risk); per-view overlays (duplicates logic).

## Decision 2: Drive transition sequencing with explicit phase state machine
- Decision: Model phases as `StartupFadeIn`, `FadeOutPendingSwitch`, `SwitchAtBlack`, `FadeInAfterSwitch`, `Idle`.
- Rationale: Makes “switch only at full black” deterministic and testable.
- Alternatives considered: Timer-only implicit logic (harder to prove correctness and can switch early/late on frame spikes).

## Decision 3: Duration policy is total cycle = 0.5s split equally
- Decision: Set default total duration to 0.5s and derive each leg (`fade_out`, `fade_in`) as 0.25s.
- Rationale: Directly matches product requirement while keeping future tuning centralized in one config field.
- Alternatives considered: Independent durations from day one (more flexible but unnecessary now).

## Decision 4: Integrate with existing ActiveView change request path
- Decision: Intercept/queue requested view changes and commit `ActiveView` update only when overlay alpha reaches full opacity.
- Rationale: Enforces visual masking contract without touching unrelated gameplay state.
- Alternatives considered: Direct immediate `ActiveView` writes plus delayed overlay (would expose popping).

## Decision 5: Verification for desktop + browser parity
- Decision: Verify via desktop and browser runs using existing repo scripts; manually observe initial load fade and multi-screen transition loop.
- Rationale: Constitution requires parity validation for user-visible rendering behavior.
- Alternatives considered: Desktop-only verification (insufficient for parity rule).
