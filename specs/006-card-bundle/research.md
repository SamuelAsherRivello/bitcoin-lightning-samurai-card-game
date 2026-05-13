# Research: Card Bundle

## Consolidation Decisions

| Decision | Rationale | Alternatives Considered |
| -------- | --------- | ----------------------- |
| Combine inspection, polish, and flip into one feature named `006-card-bundle` | The three specs define one continuous card presentation stack and downstream specs need one stable dependency | Keeping three folders preserves history but creates redundant requirements and stale cross-links |
| Preserve phase boundaries inside the merged spec | Inspection, polish, and flip each carry distinct acceptance criteria that are still useful for QA | Flattening into one generic card spec would make regressions harder to diagnose |
| Use `006-card-bundle` as the target path | The user requested this exact consolidated spec name | Renumbering every later spec would create unnecessary churn |
| Keep `007-gameplay-concepts` separate | Gameplay concepts intentionally describe future model language and should not become card presentation implementation scope | Pulling gameplay concepts into the bundle would blur prototype and future gameplay boundaries |
| Keep Deck and Card UI marked temporary | Existing behavior uses them as prototype surfaces, but multiple specs state they are not final game UI | Treating them as player-facing would create false requirements for later app flow |

## Runtime Design Notes

| Topic | Decision |
| ----- | -------- |
| Pointer inspection | Smooth card rotation remains layered under polish and flip; camera stays fixed |
| Apparent depth | CardStructure uses flat 2D front-face layers with parallax, masking, and material response |
| Art replacement | CardType/CardDefinition front art changes must not alter CardStructure behavior |
| Backface | CardBack is shared by the card series/CardStructure and stays independent of active front content |
| Flip composition | Flip orientation is side-selection around y layered onto existing inspection tilt |
| Midpoint side switch | Graphics swap at edge-on progress rather than spinning one face through the whole 180 degrees |
| Hidden front update | `T` can change the active front while face down without exposing it until a front-facing flip |

## Verification Notes

| Area | Evidence To Preserve |
| ---- | -------------------- |
| Inspection | One card, 88:63 ratio, 20-degree tilt clamp, 100 ms smoothing, fixed camera |
| Polish | Four layers, aperture clipping, parallax, frame shine, valid CardType selection |
| Reload | `R` non-toggle reload and persisted `H` hot-reload auto-restart toggle |
| Flip | Card UI `Flip`, midpoint side swap, reversal from current progress, shared CardBack |
| Scope | No gameplay, tabletop placement, deck browsing, final menus, or multi-card layout |
