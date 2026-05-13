# Quickstart: AI Integration

This quickstart is for future implementation and verification. It documents the intended workflow; it does not require this planning task to install MCP servers or edit global Codex configuration.

## 1. Verify Baseline Repository Workflows

| Step | Command | Expected Result |
| ---- | ------- | --------------- |
| Run tests | `scripts/other/RunTests.ps1` | Rust tests pass or report actionable failures. |
| Check desktop app | `scripts/other/RunAppDesktop.ps1 -CheckOnly` | Desktop build/startup check passes. |
| Check web app | `scripts/other/RunAppWeb.ps1 -CheckOnly` | Browser WebGPU build/startup check passes or documents a blocker. |

## 2. Install MCP Tooling Manually When Requested

| Step | Command | Expected Result |
| ---- | ------- | --------------- |
| Install selected MCP | `cargo install bevy_debugger_mcp` | `bevy-debugger-mcp` is available on PATH. |
| Inspect server help | `bevy-debugger-mcp --help` | MCP server usage is printed. |
| List MCP tools | Use the MCP `tools/list` smoke check from the upstream README. | Tools include observation and screenshot capabilities. |

Codex should not edit global Codex MCP configuration as part of repo implementation unless the user explicitly asks for that setup. The expected future MCP server profile is named `bevy-debugger` and points at `bevy-debugger-mcp --stdio` with `BEVY_BRP_HOST=localhost` and `BEVY_BRP_PORT=15702`. The game side is enabled by the Cargo feature `ai-runtime`, exposed through the desktop script flag `-AiRuntime`.

## 3. Start The Game With AI Runtime Tooling

| Step | Command | Expected Result |
| ---- | ------- | --------------- |
| Enable dev runtime bridge | `scripts/other/RunAppDesktop.ps1 -AiRuntime` | Game starts with Bevy Remote Protocol enabled locally. |
| Enable hot-reload bridge | `scripts/main/RunAppDesktopHotReload.ps1` | Game starts with hot reload and `ai-runtime` enabled by default. |
| Check dev runtime bridge | `scripts/other/RunAppDesktop.ps1 -CheckOnly -AiRuntime` | Desktop build compiles with the `ai-runtime` feature. |
| Confirm local endpoint | MCP connection check or direct BRP health/discovery request. | Connection succeeds on localhost only. |
| Query runtime state | MCP observation request for a known reflected component/resource. | Structured ECS data is returned. |
| Capture screenshot | MCP screenshot request with a project-local output path. | Image file is written under the documented generated-output location. |

Use `documentation/images/` only for screenshots promoted to README or documentation assets. Keep transient screenshots under a future project-local generated-output path and do not commit them unless a task explicitly promotes them.

## 3A. Peek Workflow

When the user says "peek" at the app, running app, game, or desktop runtime, Codex should use this workflow.

| Step | Runtime Action | Expected Result |
| ---- | -------------- | --------------- |
| Discover BRP | POST `rpc.discover` to `http://localhost:15702` | Runtime methods are listed, including `bevy_debugger/screenshot`. |
| Query available state | Use observational methods such as `world.list_resources`, `world.query`, or `registry.schema` | Codex reports only reflected runtime facts and clearly labels visual inferences. |
| Capture screenshot | Call `bevy_debugger/screenshot` with a path under `target/ai-runtime-screenshots/` | A transient screenshot is saved for inspection. |
| Inspect scene/view | Read DebugHUD text and visible layout from the screenshot | Report `AppScene` as always-present and the active view, such as `GameScene`, separately. |
| Handle missing endpoint | If `localhost:15702` refuses connection | Report that `ai-runtime` is unavailable and ask the user to start an AI-enabled desktop run. |

## 4. Verify Browser QA Separately

| Step | Tool | Expected Result |
| ---- | ---- | --------------- |
| Serve web build | Existing web run/check script. | Browser WebGPU target runs or documents blocker. |
| Capture browser screenshot | Playwright or in-app browser. | Viewport screenshot shows the current web game surface. |
| Compare with desktop evidence | Manual or automated QA report. | Differences are documented as target-specific behavior or defects. |

## 5. Verify Responsive Positioning And Scaling

Responsive positioning means all visible positions and scales respect the aspect-ratio-safe game view. This applies to 2D UI and 3D presentation elements.

| Step | Tool | Expected Result |
| ---- | ---- | --------------- |
| Capture fullscreen layout | Desktop MCP screenshot or browser screenshot. | Visible 2D and 3D elements are centered, anchored, and scaled from the safe game view. |
| Capture smaller or tall-window layout | Desktop MCP screenshot or browser screenshot. | Elements keep the same composition inside the aspect-ratio-safe game view and do not drift into letterbox areas. |
| Inspect runtime state when available | MCP ECS query for camera viewport, transforms, and layout resources. | Positions and scales are derived from safe-view dimensions rather than raw window pixels or unrelated world constants. |

## 6. Safety Checklist

| Check | Required Result |
| ----- | --------------- |
| Localhost only | ✅ |
| Runtime MCP disabled in normal app startup | ✅ |
| No secrets required | ✅ |
| Screenshot paths project-local | ✅ |
| Responsive positioning and scaling checked at multiple window sizes | ✅ |
| Mutating operations documented with reset/rollback | ✅ |
| Browser WebGPU coverage documented separately | ✅ |

Mutating MCP operations should not be part of routine QA. If a task uses mutation, record the reason, the exact state being changed, and the reset plan, such as restarting the app or loading a known scenario.
