# Quickstart: Visual Modifier System

## Review Workflow

| Step | Action | Expected Result |
| ---- | ------ | --------------- |
| 1 | Read [spec.md](./spec.md) | Confirm `abilityoutline` and `leadingscoreoutline` scope matches the requested first and second passes. |
| 2 | Read [data-model.md](./data-model.md) | Confirm modifier state is separate from point values and targets the point circle/background. |
| 3 | Read [contracts/visual-modifier-contract.md](./contracts/visual-modifier-contract.md) | Confirm activation rules, colors, and tie behavior are unambiguous. |
| 4 | Inspect current point view code | Confirm `PointView`, `PointLocationView`, and card/location point spawning are the implementation targets. |

## Future Implementation Checks

| Check | Command Or Workflow | Pass Condition |
| ----- | ------------------- | -------------- |
| Rust tests | `scripts/other/RunTests.ps1` | Component/system tests pass. |
| Desktop visual smoke | `scripts/main/RunAppDesktop.ps1` or AI runtime workflow | Card power point views show gold outlines only under active ability modifiers. |
| Location lead visual smoke | Desktop GameView with unequal and tied totals | Higher location total shows white outline; ties clear both outlines. |
| Browser parity | Project browser WebGPU workflow when practical | Outlines render with the same activation behavior as desktop. |

## Implementation Notes

| Topic | Note |
| ----- | ---- |
| First pass | Implement and verify `abilityoutline` independently before `leadingscoreoutline`. |
| Second pass | Implement location total comparison after the point total update path is stable. |
| Tests first | Prefer pure modifier rule tests before presentation sync tests. |
| Presentation | Use UI borders for UI point views and a WebGPU-compatible ring/child mesh or equivalent for world-space card point views. |
| Safety | Modifier sync systems should tolerate missing children and hidden point views. |
