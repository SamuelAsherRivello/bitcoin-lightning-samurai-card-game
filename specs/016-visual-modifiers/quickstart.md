# Quickstart: Visual Modifier System

## Review Workflow

| Step | Action | Expected Result |
| ---- | ------ | --------------- |
| 1 | Read [spec.md](./spec.md) | Confirm each visual modification is described as Condition, Target, and Treatment. |
| 2 | Read [data-model.md](./data-model.md) | Confirm Conditions are separate from point values, Targets select the point circle/background, and Treatments describe the visual change. |
| 3 | Read [contracts/visual-modifier-contract.md](./contracts/visual-modifier-contract.md) | Confirm rule Conditions, Targets, Treatments, colors, and tie behavior are unambiguous. |
| 4 | Inspect current point view code | Confirm `PointView`, `PointLocationView`, and card/location point spawning are the implementation targets. |
| 5 | Inspect 015 card-state code | Confirm `CardInstanceId`, `CardInstanceStateModel`, `CardZoneModel`, and `CardViewStateModel` are the preferred card-state inputs for VMS. |

## Future Implementation Checks

| Check | Command Or Workflow | Pass Condition |
| ----- | ------------------- | -------------- |
| Rust tests | `scripts/other/RunTests.ps1` | Component/system tests pass. |
| Desktop visual smoke | `scripts/main/RunAppDesktop.ps1` or AI runtime workflow | Card power point views show gold outlines only under active ability modifiers. |
| Location lead visual smoke | Desktop GameScene with unequal and tied totals | Higher location total shows white outline; ties clear both outlines. |
| Browser parity | Project browser WebGPU workflow when practical | Outlines render with the same activation behavior as desktop. |

## Implementation Notes

| Topic | Note |
| ----- | ---- |
| Runtime conventions | VMS runtime code follows focused component/system files under `bevy/crates/game/src/runtime/`, lowercase Rust module paths, `HUMAN:`/`AI:` comments on primary items, and repository script verification. |
| First pass | Implement and verify `abilityoutline` independently before `leadingscoreoutline`. |
| Second pass | Implement location total comparison after the point total update path is stable. |
| Tests first | Prefer pure Condition/Target/Treatment rule tests before presentation sync tests. |
| 015 integration | Prefer direct `CardInstanceId` links from point views to `CardInstanceStateModel`; use `local_instances_from_existing_state` and CPU adapter helpers only as a transition path. |
| Presentation | Use UI borders for UI point views and a WebGPU-compatible ring/child mesh or equivalent for world-space card point views. |
| Safety | Modifier sync systems should tolerate missing children and hidden point views. |

## Verification Log

| Date | Check | Result |
| ---- | ----- | ------ |
| 2026-05-13 | `cargo test -p samurai-card-game visual_modifier --no-default-features` | Passed: 8 VMS-focused tests. Existing warning remains for `local_slots_area_hit_target`. |
| 2026-05-13 | `cargo test -p samurai-card-game visual_modifier --features fast-dev` | Passed: 8 VMS-focused tests. Existing warning remains for `local_slots_area_hit_target`. |
| 2026-05-13 | `scripts/other/RunTests.ps1` | Failed: all 8 VMS-focused tests passed, but 19 broader `systems_tests` failed in existing card/view setup and visibility cases. |
| 2026-05-13 | `git diff --check` | Passed with line-ending warnings only. |
