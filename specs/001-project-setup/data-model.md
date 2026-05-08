# Data Model: Project Setup

| Entity | Fields | Relationships | Validation Rules |
| ------ | ------ | ------------- | ---------------- |
| Desktop Window Placement | `window_position`, `window_size`, `monitor_name`, `monitor_position`, `monitor_size`, `relative_position` | Saved on normal close; loaded on next desktop startup | Size must be positive; restored placement must be visible on an available screen |
| Screen Identity | `monitor_name`, `monitor_position`, `monitor_size` | Associated with the saved desktop window placement | Matching prefers same named screen and size, then same named screen, then same position |
| Local Runtime State | Placement JSON file under `generated/runtime/` | Owned by the local machine and not committed | Missing, unreadable, or invalid data is ignored |
| Project Script | Dependency-check, headless compile, test, desktop run, web run, and stop entry points under root `scripts` | Used by VS Code tasks and manual terminal workflows | Must run from repository root |
| Compile Workflow | `action`, `package/workspace scope`, `target_dir`, `target_triple`, `features`, `release`, linker/cache options | Shared by desktop, web, tests, and dependency warmup scripts | Must print the Cargo command and fail on non-zero Cargo exit |
| VS Code Task | Build, test, and desktop run task definitions | Calls repository scripts with integrated terminal presentation | Must reveal output in VS Code terminal |

## State Transitions

| Event | Previous State | New State | Side Effects |
| ----- | -------------- | --------- | ------------ |
| First launch without saved placement | No placement | 800x600 centered primary window | No placement file required |
| Move or resize while running | In-memory placement unknown or stale | In-memory placement updated | No file write |
| Normal close | In-memory/current placement available | Placement persisted locally | Writes ignored JSON |
| Relaunch with valid placement | Placement file available | Exact screen, x/y, and size restored | Window opens where reviewer left it |
| Relaunch with invalid/off-screen placement | Placement file invalid or unavailable | 800x600 centered primary fallback | Invalid placement is ignored |
