# Contract: AI Runtime MCP

## Purpose

Define the expected contract between Codex, the selected MCP server, and the running Bevy game for development-only AI runtime inspection and screenshot capture.

## Selected MCP

| Field | Value |
| ----- | ----- |
| MCP name | `bevy-debugger` |
| Implementation | `bevy_debugger_mcp` |
| Codex-to-MCP transport | stdio |
| MCP-to-game transport | Bevy Remote Protocol over local HTTP |
| Default Bevy host | `localhost` |
| Default Bevy port | `15702` |
| Runtime target | Windows desktop dev runtime |
| Browser WebGPU coverage | Complementary browser automation, not this MCP |

## Required Tool Capabilities

| Capability | Operation Class | Required Behavior |
| ---------- | --------------- | ----------------- |
| List tools | observational | Codex can verify available MCP tools before using runtime workflows. |
| Observe ECS state | observational | Codex can query reflected entities, components, and resources. |
| Discover schemas | observational | Codex can inspect registered reflected types when available. |
| Capture screenshot | screenshot-only | Codex can request a desktop game screenshot written to a project-local path. |
| Run experiment | mutating | Codex may alter runtime state only with task-specific reason and reset/rollback guidance. |
| Replay or session analysis | observational | Codex may read recorded session data if generated locally and scoped to the task. |

## Bevy Runtime Requirements

| Requirement | Contract |
| ----------- | -------- |
| Endpoint opt-in | Normal app startup must not expose BRP/MCP. |
| Host binding | Endpoint binds to localhost unless a future spec explicitly approves broader access. |
| Reflection | Components/resources needed for AI inspection must derive/register reflection when safe. |
| Custom screenshot method | Desktop screenshot capture must be exposed through a dev-only BRP method if the MCP requires it. |
| Generated output | Screenshots and snapshots must be written inside the repository. |
| Responsive layout evidence | Runtime screenshots and state queries must support checking that visible positions and scales derive from the aspect-ratio-safe game view across window sizes. |
| Browser limitation | Native-only BRP support must be documented; browser QA uses separate browser tools. |

## Codex Workflow Requirements

| Requirement | Contract |
| ----------- | -------- |
| Configuration ownership | Codex documents MCP config but does not edit global Codex settings unless explicitly asked. |
| Failure handling | Codex reports when the MCP server, game endpoint, or reflected type is unavailable. |
| Source of truth | Codex does not infer runtime state when the MCP query fails. |
| Secret safety | Codex must not request or print secrets, credentials, cookies, or tokens. |
| Mutation discipline | Codex prefers observation and screenshots; mutation requires reason plus reset/rollback. |

## Minimum Smoke Test

| Step | Expected Result |
| ---- | --------------- |
| Start MCP server | Server responds to MCP initialization. |
| List MCP tools | Observation and screenshot tools are present. |
| Start desktop game with AI runtime tooling | Game exposes local BRP endpoint. |
| Query a known reflected type | Structured state returns successfully. |
| Capture screenshot | Image appears at the requested project-local path. |
| Capture fullscreen and smaller-window screenshots | Visible 2D and 3D positions and scales remain responsive to the aspect-ratio-safe game view. |
| Stop game/server | No lingering public endpoint remains. |

## Non-Goals

| Non-Goal | Reason |
| -------- | ------ |
| Production AI endpoint | This feature is development tooling only. |
| Public remote debugging | Workspace rules forbid exposing local-only services without explicit approval. |
| Replacing tests | MCP inspection complements repository tests; it does not replace them. |
| Replacing browser QA | Desktop runtime MCP does not prove browser WebGPU behavior. |
