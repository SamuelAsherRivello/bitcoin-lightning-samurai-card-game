# Location Intro Contract

| Behavior | Contract |
| -------- | -------- |
| Location count | GameScene presents exactly three `location_bundle` roots. |
| Background surface | Each location has one 3D rectangular surface textured with the active location graphic. |
| Overlay content | Each location has title/body text, one colored border, and exactly two location power point views. |
| Initial state | Each location starts at 0% opacity and 150% destination scale until its delay elapses. |
| Location 01 timing | Starts immediately and reaches 100% opacity and 100% scale after 0.5 seconds. |
| Location 02 timing | Starts 0.5 seconds after location 01 completes and reaches final state after its own 0.5-second animation. |
| Location 03 timing | Starts 0.5 seconds after location 02 completes and reaches final state after its own 0.5-second animation. |
| Easing | Scale and opacity use ease-out timing. |
| Depth | Location surfaces are in front of the world background and below card presentation. |
| Scope | Scoring, card placement, deck, round, and location ability rules do not change. |
