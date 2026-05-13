# Quickstart: Card View State Model

## Review Workflow

| Step | Command Or File | Expected Result |
| ---- | --------------- | --------------- |
| Inspect Bevy template conventions | `bevy/crates/template-crate/src/runtime/resources/template_resource.rs` and `bevy/crates/template-crate/src/tests/runtime/resources/template_resource_tests.rs` | Runtime resource files use one focused primary item, `HUMAN:`/`AI:` purpose comments, `#[cfg(test)]` path modules, and tests under `src/tests/runtime/resources/`. |
| Inspect current visual bundle | `bevy/crates/game/src/runtime/bundles/card_view_bundle.rs` | Confirms `CardViewBundle` owns render root only. |
| Inspect current local card state | `bevy/crates/game/src/runtime/resources/card_slot_model.rs` | Confirms `CardStateModel`, `CardState`, and slot occupancy. |
| Inspect current gestures | `bevy/crates/game/src/runtime/resources/card_gesture_model.rs` | Confirms one active gesture focus and interaction states. |
| Inspect current gesture systems | `bevy/crates/game/src/runtime/systems/card_gesture_update_system.rs` | Confirms hand/location press, selected, dragging, placement, return rules. |
| Inspect current face layers | `bevy/crates/game/src/runtime/systems/mod.rs` | Confirms front/back layer spawning and face visibility update. |
| Inspect opponent reveal | `bevy/crates/game/src/runtime/resources/opponent_match_model.rs` | Confirms current-round hidden and revealed placement state. |

## Future Implementation Verification

| Check | Command Or Workflow | Expected Result |
| ----- | ------------------- | --------------- |
| Model tests | `scripts/other/RunTests.ps1` | Card instance transition tests pass. |
| Hand selection | Run GameScene and click a local hand card | Card enlarges at selected inspection pose without changing durable zone. |
| Drag to location | Drag local hand card to local location slot area | Card occupies first empty local slot and durable zone becomes current-round location. |
| Return same-round card | Drag same-round placed card back to hand area | Slot frees and card returns to chosen hand order. |
| Lock behavior | End round, then try dragging prior placed card | Drag is rejected; card remains placed. |
| Opponent hidden reveal | CPU/opponent places current-round card | Non-owner sees back until reveal; owner/rules retain identity. |
| Browser parity | Served browser WebGPU run after implementation | Same visible state transitions as desktop, or blocker documented. |

## Implemented Model Checks

| Check | Command | Result |
| ----- | ------- | ------ |
| Targeted model tests | `cargo test -p bevy-card-game card_instance_state_model_tests` from `bevy/` | 13 passed, 0 failed. |
| Full project test script | `scripts/other/RunTests.ps1` from repository root | Failed: 189 passed, 20 failed. The failing tests are existing `runtime::systems::systems_tests::*` view/scene/card-structure assertions; the new `card_instance_state_model_tests` passed inside the run. |

## Future Gesture Integration Notes

| System | Migration Note |
| ------ | -------------- |
| `card_gesture_update_system` | Replace `active_hand_index` lookup with `CardInteractionModel.active_instance_id`; use `CardInstanceStateModel.zone` to decide whether the source is hand or current-round location. |
| `card_gesture_animation_system` | Derive hand, selected, drag, return, and slot poses from `CardViewStateModel.pose` while continuing to calculate transforms from the aspect-ratio-safe GameScene helpers. |
| `CardSlotBoardModel` consumers | Keep slot rect geometry in `CardSlotBoardModel`; migrate occupancy checks to `CardPlacementModel` or instance-id slot occupants after adapter parity is proven. |
| CPU reveal rendering | Use `CardRevealPolicy` to derive `visible_face`; keep `CpuPlacedCardFaceLayer` as the render-layer control mechanism until the CPU and local card view paths are unified. |
