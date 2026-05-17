# Requirements Checklist: Shared AppScene 3D Camera

| Item | Status | Notes |
| --- | --- | --- |
| No implementation details leak into user scenarios | ✅ | User stories describe runtime outcomes and visual parity. |
| Requirements are testable | ✅ | Camera count, absence of `Camera2d`, stable camera entity, and visual parity are measurable. |
| Scope covers all current screens | ✅ | Game, Deck, Debug, and meta screens are included. |
| Transitions and overlays are covered | ✅ | Screen transitions, DebugHUD, modals, point labels, selected-card menus, and debug drawing are explicit. |
| Desktop and browser parity are required | ✅ | Both targets are required in success criteria and quickstart verification. |
| Known risks are documented | ✅ | Bevy UI targeting, camera ordering, text overlays, and picking risks are captured. |
| No unrelated gameplay or asset changes are required | ✅ | Scope is rendering architecture only. |
