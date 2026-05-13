# Quickstart: Point System

## Prerequisites

| Step | Command |
| ---- | ------- |
| Verify local dependencies once per machine | `scripts/main/InstallDependencies.ps1` |
| Run automated tests | `scripts/other/RunTests.ps1` |
| Run desktop app | `scripts/main/RunAppDesktop.ps1` |
| Stop app/build processes | `scripts/other/StopApp.ps1` |

## Implementation Checkpoints

| Checkpoint | Expected Result |
| ---------- | --------------- |
| Model tests | Cost and power are distinct; cost never contributes to scoring; power supports negative values |
| Location scoring tests | Revealed cards contribute to the owning player's current location; unrevealed cards do not by default |
| Capacity tests | Default four-card-per-player capacity is enforced or invalid placement is represented explicitly |
| Control tests | Local lead, opponent lead, tied totals, and empty equal-zero totals return the expected controller |
| Match outcome tests | Controlled-location count wins first; total power breaks tied control count; equal both ways draws |
| View tests or inspection | Cost and power point views display values from `-99` through `99` with dynamic text |

## Desktop Verification

1. Run `scripts/other/RunTests.ps1`.
2. Run `scripts/main/RunAppDesktop.ps1`.
3. Inspect `GameView` and confirm the three shared locations still appear inside the aspect-ratio-safe area.
4. Confirm location top values represent opponent totals and bottom values represent local totals.
5. Confirm visible card point views keep cost and power visually distinct.
6. Run `scripts/other/StopApp.ps1` when finished.

## Browser WebGPU Verification

1. Run the existing web workflow documented by the repository, typically `scripts/other/RunAppWeb.ps1`.
2. Open the served local browser target.
3. Confirm point text renders, remains inside the safe `GameView` layout, and matches desktop semantics.
4. If browser WebGPU cannot be verified, record the exact blocker before considering the feature complete.

## Notes For Task Generation

| Topic | Guidance |
| ----- | -------- |
| First tasks | Add deterministic model tests before rendering changes where practical |
| File scope | Prefer focused point model/view files over expanding already-large aggregate modules |
| Naming | Use `Model` for data and `View` for rendering/presentation |
| Purpose comments | Add required `HUMAN:` and `AI:` lines above new or changed primary runtime items |
| Template reference | Use `bevy/crates/template-crate` as the proper reference for Bevy crate folders, representative files, asset folders, and Rust coding standards |
| Exclusions | Do not add deckbuilding, drawing, CPU strategy, full round rules, card abilities, or production UI |
