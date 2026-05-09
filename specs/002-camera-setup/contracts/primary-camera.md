# Contract: Primary 3D Camera

| Requirement | Contract |
| ---- | ---- |
| Camera count | After startup, exactly one entity has the `PrimarySceneCamera` marker and `Camera3d` component |
| Camera name | The primary camera entity is named `Primary 3D Camera` |
| Transform | The camera starts at `(0, 0, 5)` and looks at `Vec3::ZERO` with `Vec3::Y` as up |
| Projection | The camera uses perspective projection with documented FOV, near, and far defaults |
| Background | The app clear color is neutral dark gray |
| Input behavior | No system in this feature changes the camera transform in response to keyboard, mouse, controller, or gameplay input |
| Overlay compatibility | Existing DebugHUD UI and egui inspector entities remain able to render above the 3D scene |

## Verification

| Check | Expected Result |
| ---- | ---- |
| Unit startup query | One `PrimarySceneCamera` plus one `Camera3d` exists |
| Defaults query | Transform and projection match `PrimaryCameraDefaults` |
| Input stability query | Pressed keyboard input does not mutate the primary camera transform after an update |
| Combined startup query | DebugHUD and inspector entities still exist after camera setup |
