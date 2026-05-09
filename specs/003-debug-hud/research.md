# Research: DebugHUD

| Decision | Rationale | Alternatives Considered |
| -------- | --------- | ----------------------- |
| Use Bevy UI `Text`, `TextSpan`, `Node`, and `BackgroundColor` for the HUD panel | These APIs are native to Bevy, work with ECS, and avoid adding a browser-only or desktop-only overlay dependency | Egui-only HUD was rejected because the feature requests a Bevy HUD pattern, while egui remains reserved for the inspector |
| Store FPS visibility and sample data in `DebugHudState` | A resource makes diagnostic state explicit and easy to test without coupling it to entities | Component-only state was rejected because the FPS toggle is app-level diagnostic state |
| Store inspector visibility in an `InspectorState` component | The inspector window is an entity-like diagnostic surface with position and size settings | A global bool was rejected because it would mix inspector window metadata with unrelated HUD state |
| Treat `WASD` as static labels with no pressed feedback | The spec says `WASD` should be visible but do nothing, so the implementation avoids movement and HUD state changes for those keys | Visual pressed highlighting was rejected because it could be interpreted as functional behavior |
| Use `bevy-inspector-egui` for the inspector | The dependency is already in the workspace and provides entity inspection without building custom inspector UI | A custom inspector was rejected as unnecessary scope |
| Use `scripts/other/RunTests.ps1` for test execution | The repository requires repeatable scripts under `scripts/`; this script runs the full workspace test suite | Ad hoc `cargo test` only was rejected because the feature explicitly requires a RunTests script |
